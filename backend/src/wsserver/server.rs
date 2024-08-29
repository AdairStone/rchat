//! `ChatServer` is an actor. It maintains list of connection client session.
//! And manages available rooms. Peers send messages to other peers in same
//! room through `ChatServer`.
use actix::prelude::*;
use futures::sink::Send;
use rand::{rngs::ThreadRng, Rng};
use std::collections::{HashMap, HashSet};
use tokio::{sync::oneshot, task};
use zino::prelude::{DateTime, ModelAccessor, Query};
use zino_core::orm::Schema;

use crate::{
    dto::chat_message_entity::{ChatMessageDto, ChatNotify, ChatNotifyMessageDto},
    model::{ChatMessage, ChatRoom},
    service::{chat_service::ChatService, room_message_state::MessageStatusManager},
};

use super::session::WsChatSession;

/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// Message for chat server communications

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub session: WsChatSession,
    pub room: String,
    pub addr: Recipient<Message>,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
    pub session: WsChatSession,
}

/// Send message to specific room
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct ClientMessage {
    /// Id of the client session
    pub id: usize,
    /// Peer message
    pub msg: String,
    /// Room name
    pub room: String,
    /// 具体消息
    pub mess: ChatMessage,
    pub session: WsChatSession,
}

/// List of available rooms
pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

/// Join room, if room does not exists create new one.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    /// Client ID
    pub id: usize,
    /// Room name
    pub room_id: String,
    pub session: WsChatSession,
}

#[derive(Debug, Clone)]
pub struct ChatServer {
    // 用于接收消息
    sessions: HashMap<usize, Recipient<Message>>,
    // 用于发送管理消息
    server_sessions: HashMap<String, (String, Recipient<Message>)>,
    rooms: HashMap<String, HashSet<usize>>,
    // 记录站点关联房间
    site_rooms: HashMap<String, HashSet<String>>,
    rng: ThreadRng,
    // visitor_count: Arc<AtomicUsize>,
}

impl ChatServer {
    pub fn new() -> ChatServer {
        let rooms = HashMap::new();
        ChatServer {
            sessions: HashMap::new(),
            server_sessions: HashMap::new(),
            rooms,
            rng: rand::thread_rng(),
            // visitor_count,
            site_rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, room_site: &str, room: &str, addr: Recipient<Message>) -> usize {
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, addr);
        // 添加房间跟连接依赖关系以及跟站点依赖关系
        self.rooms.entry(room.to_owned()).or_default().insert(id);
        self.site_rooms
            .entry(room_site.to_string())
            .or_default()
            .insert(room.to_string());
        id
    }
}

impl ChatServer {
    /// Send message to all users in the room
    fn send_message(&self, room: &str, message: &str, skip_id: usize) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        addr.do_send(Message(message.to_owned()));
                    }
                }
            }
        }
    }

    /// 给服务人员发送消息
    fn send_server_message(&self, site_key: &str, message: &str) {
        if let Some((room, addr)) = self.server_sessions.get(site_key) {
            addr.do_send(Message(message.to_owned()));
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
    type Result = usize;
    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        // 连接开启一个房间 条件：获取site_key
        tracing::info!("joining...");
        let user = msg.session.user;
        tracing::info!("{:?} joined {}", &user, &msg.room);
        if user.is_some() {
            self.server_sessions.insert(
                msg.session.site_key.clone(),
                (msg.room.clone(), msg.addr.clone()),
            );
        }
        let room: &String = &msg.room;
        let id = self.add_room(&msg.session.site_key, room, msg.addr);
        id
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: Disconnect, ctx: &mut Context<Self>) {
        tracing::info!("Someone disconnected");
        let mut rooms: Vec<String> = Vec::new();

        // Remove session from all rooms
        if self.sessions.remove(&msg.id).is_some() {
            for (room_id, sessions) in &mut self.rooms {
                if sessions.remove(&msg.id) {
                    rooms.push(room_id.to_owned());
                }
            }
        }

        // Server offline logic
        if let Some(user) = msg.session.user {
            self.server_sessions.remove(&msg.session.site_key);
        } else {
            // Update room status
            let mut chat_room = msg.session.room_obj.clone();
            chat_room.status = "offline".to_string();
            chat_room.update_at = DateTime::now();

            // Clear room status cache
            let site_key = msg.session.site_key.clone();
            let room_id = msg.session.room.clone();
            self.site_rooms
                .entry(site_key.clone())
                .or_default()
                .remove(&room_id);

            // Handle Redis update and room status update asynchronously
            if let Some(rooms_set) = self.site_rooms.get(&site_key) {
                let rooms_set_cloned = rooms_set.clone();
                let site_key_clone = site_key.clone();
                let task = async move {
                    if let Err(e) =
                        MessageStatusManager::set_site_rooms(&site_key_clone, &rooms_set_cloned)
                            .await
                    {
                        tracing::error!("Update redis site:{} rooms error: {}", &site_key_clone, e);
                    } else {
                        tracing::info!(
                            "Update redis site:{} rooms:{:?}",
                            &site_key_clone,
                            &rooms_set_cloned
                        );
                    }

                    if let Err(e) = chat_room.update().await {
                        tracing::warn!("message save error: {:?}", e);
                    }
                }
                .into_actor(self);

                ctx.spawn(task);
            }
        }
    }
}

/// Handler for Message message.
impl Handler<ClientMessage> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: ClientMessage, context: &mut Context<Self>) {
        // 发送前保存
        tracing::info!("ClientMessage: {:?}", &msg);
        let site_key = msg.session.site_key.clone();
        let str_files = msg.mess.str_files.clone();
        let s_session = self.server_sessions.get(&site_key).clone();
        tracing::info!("server_sessions: {:?}", &s_session);
        let s_in_room = if s_session.is_some() {
            s_session.unwrap().0 == msg.room
        } else {
            false
        };
        let (tx, mut rx) = oneshot::channel::<String>();
        let room_id = msg.id.clone();
        let room_key = msg.room.clone();
        // 异步任务
        let fut = async move {
            if s_in_room {
                tracing::info!("send_message");
                let _ = tx.send("send_message".to_string());
            } else {
                let _ = MessageStatusManager::increase_latest_count(&site_key, &room_key, 1).await;
                // 不在房间，发送通知
                let notify_message = ChatNotify::new_from_redis(&site_key).await;
                match serde_json::to_string(&notify_message) {
                    Ok(notify_json) => {
                        tracing::info!("send notify: {}", &notify_json);
                        let _ = tx.send(notify_json);
                    }
                    Err(e) => {
                        tracing::warn!("message save error: {:?}", e);
                    }
                }
            }
            let result = msg.mess.insert().await;
            result
        }
        .into_actor(self)
        .map(move |result, act, _ctx| {
            tracing::info!("handle result");
            match result {
                Ok(_) => {
                    tracing::info!("handle result in ok");
                    if let Ok(rs) = rx.try_recv() {
                        if rs == "send_message" {
                            tracing::info!("msg.session: {:?}", &msg.session);
                            // let str_files = msg.mess.str_files.as_ref().map(|s| s.clone());
                            let user_name = msg.session.user.as_ref().map(|u| u.name().to_string());
                            let message_data = ChatMessageDto::new_text_str_files_msg(
                                &msg.msg,
                                str_files,
                                msg.session.user.is_none(),
                                user_name,
                                Some(msg.room.clone()),
                            );
                            let json = match serde_json::to_string(&message_data) {
                                Ok(s) => s,
                                Err(e) => {
                                    tracing::error!("message handle error: {:?}", e);
                                    "".to_string()
                                }
                            };
                            tracing::info!("real send_message: {}: {}", &rs, &json);
                            act.send_message(&msg.room, json.as_str(), room_id);
                        } else {
                            if msg.session.user.is_none() {
                                act.send_server_message(&msg.session.site_key, &rs);
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("message save error: {:?}", e);
                }
            }
        });
        context.spawn(fut);
    }
}

/// Handler for `ListRooms` message.
impl Handler<ListRooms> for ChatServer {
    type Result = MessageResult<ListRooms>;
    fn handle(&mut self, _: ListRooms, _: &mut Context<Self>) -> Self::Result {
        let mut rooms = Vec::new();
        for key in self.rooms.keys() {
            rooms.push(key.to_owned());
        }
        MessageResult(rooms)
    }
}

/// Join room, send disconnect message to old room
/// send join message to new room
impl Handler<Join> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: Join, context: &mut Context<Self>) {
        let Join {
            id,
            room_id,
            session,
        } = msg;
        let site_key = session.site_key.clone();
        let mut rooms = Vec::new();
        for (n, sessions) in &mut self.rooms {
            if sessions.remove(&id) {
                rooms.push(n.to_owned());
            }
        }

        let user = session.user;
        tracing::info!("{:?} joined {}", &user, &room_id);
        if user.is_some() {
            if let Some((room, _addr)) = self.server_sessions.get_mut(&site_key) {
                *room = room_id.clone();
            }
        }

        self.rooms.entry(room_id.clone()).or_default().insert(id);

        // 发起服务通知
        let from_user = user.is_none();
        if !from_user {
            // 管理端加入
            let user_name = user.as_ref().map(|u| u.name().to_string().clone());
            let msg = ChatMessageDto::new_notify_msg(
                &format!("{} 为你提供服务", &user_name.clone().unwrap_or_default()),
                from_user,
                user_name,
                Some(room_id.clone()),
            );
            let msg_str = serde_json::to_string(&msg).unwrap();
            self.send_message(&room_id, &msg_str, id);
        }

        let (tx, mut rx) = oneshot::channel::<String>();
        let sk_clone = site_key.clone();
        let room_key = room_id.clone();
        let fut = async move {
            let query = Query::from_entry("id", room_key.clone());
            if let Ok(chat_room) = ChatRoom::find_one::<ChatRoom>(&query).await {
                if let Some(cr) = chat_room {
                    let _r = ChatService::join_room(&cr).await;
                    // 加入之后 更新房间消息
                    let notify_message = ChatNotify::new_from_redis(&site_key).await;
                    match serde_json::to_string(&notify_message) {
                        Ok(notify_json) => {
                            tracing::info!("send notify: {}", &notify_json);
                            let _ = tx.send(notify_json);
                        }
                        Err(e) => {
                            tracing::warn!("message save error: {:?}", e);
                            let _ = tx.send("error".to_string());
                        }
                    }
                } else {
                    let _ = tx.send("error".to_string());
                }
            } else {
                let _ = tx.send("error".to_string());
            }
            
        }
        .into_actor(self)
        .map(move |result, act, _ctx| {
            if let Ok(rs) = rx.try_recv() {
                if "error" != &rs {
                    act.send_server_message(&sk_clone, &rs);
                } else {
                    tracing::warn!("send notify error and ignore");
                }
            }
        });
        context.spawn(fut);
    }
}
