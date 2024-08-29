use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;
use zino_core::{datetime::DateTime, Uuid};
use zino_model::User;

use crate::{
    model::{ChatMessage, ChatRoom},
    service::room_message_state::MessageStatusManager,
};

use super::server;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(20);

#[derive(Debug, Clone)]
pub struct WsChatSession {
    pub site_key: String,
    /// unique session id
    pub id: usize,
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    pub hb: Instant,
    /// joined room
    pub room_obj: ChatRoom,

    pub room: String,
    // 当前用户
    pub user: Option<User>,
    /// peer name
    pub name: Option<String>,
    pub user_type: usize, // 1 -client 0-customer service
    /// Chat server
    pub addr: Addr<server::ChatServer>,
}

impl WsChatSession {
    /// helper method that sends ping to client every 5 seconds (HEARTBEAT_INTERVAL).
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        // let session = self.clone();
        ctx.run_interval(HEARTBEAT_INTERVAL, move |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                tracing::info!("Websocket Client heartbeat failed, disconnecting!");
                // notify chat server
                act.addr.do_send(server::Disconnect {
                    id: act.id,
                    session: act.clone(),
                });

                // stop actor
                ctx.stop();
                // don't try to send a ping
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;
    /// Method is called on actor start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        tracing::info!("Websocket Client connected 1");
        // we'll start heartbeat process on session start.
        self.hb(ctx);
        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        tracing::info!("Websocket Client connected 2");
        let addr = ctx.address();
        self.addr
            .send(server::Connect {
                session: self.clone(),
                addr: addr.recipient(),
                room: self.room.clone(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat server
        self.addr.do_send(server::Disconnect {
            id: self.id,
            session: self.clone(),
        });
        // todo!(); // 更新房间状态 leave
        Running::Stop
    }
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<server::Message> for WsChatSession {
    type Result = ();
    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(e) => {
                tracing::warn!("ws stoped for: {}", e);
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        // tracing::info!("WEBSOCKET MESSAGE: {msg:?}");
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let m = text.trim();
                // we check for /sss type of messages
                if m.starts_with('/') {
                    let v: Vec<&str> = m.splitn(2, ' ').collect();
                    match v[0] {
                        "/list" => {
                            // Send ListRooms message to chat server and wait for
                            // response
                            log::info!("List rooms");
                            self.addr
                                .send(server::ListRooms)
                                .into_actor(self)
                                .then(|res, _, ctx| {
                                    match res {
                                        Ok(rooms) => {
                                            for room in rooms {
                                                ctx.text(room);
                                            }
                                        }
                                        _ => println!("Something is wrong"),
                                    }
                                    fut::ready(())
                                })
                                .wait(ctx)
                        }
                        "/join" => {
                            if v.len() == 2 {
                                self.room = v[1].to_owned();
                                self.addr.do_send(server::Join {
                                    id: self.id,
                                    room_id: self.room.clone(),
                                    session: self.clone(),
                                });
                                // ctx.text("joined");
                            } else {
                                // ctx.text("!!! room name is required");
                            }
                        }
                        "/name" => {
                            if v.len() == 2 {
                                self.name = Some(v[1].to_owned());
                            } else {
                                ctx.text("!!! name is required");
                            }
                        }
                        _ => ctx.text(format!("!!! unknown command: {m:?}")),
                    }
                } else {
                    let mesage = match serde_json::from_str::<ChatMessage>(m) {
                        Ok(im) => Some(im),
                        Err(ie) => {
                            ctx.text(format!("error! {}", ie));
                            None
                        }
                    };
                    if let Some(mut mess) = mesage {
                        mess.id = Uuid::now_v7();
                        mess.name = if self.name.is_some() {
                            self.name.clone().unwrap()
                        } else {
                            "".to_owned()
                        };
                        mess.room_id = Uuid::parse_str(&self.room).unwrap();
                        mess.create_at = DateTime::now();
                        mess.update_at = DateTime::now();
                        mess.status = "sended".to_string();
                        mess.user_id = match &self.user {
                            Some(u) => Some(u.user_session().user_id().clone()),
                            None => None,
                        };

                        // let room_id = mess.clone().room_id.clone().to_string();
                        // let site = self.site_key.clone();
                        // let from_server = self.user.is_some();// 来自服务
                        // let r = async move {
                        //     if !from_server{
                        //         MessageStatusManager::increase_latest_count(&site, room_id.as_str(), 1)
                        //         .await
                        //     } else {
                        //         Ok(())    
                        //     }
                        // }
                        // .into_actor(self)
                        // .map(|result, _act, _ctx| match result {
                        //     Ok(_r) => (),
                        //     Err(e) => {
                        //         tracing::warn!("message save error: {:?}", e);
                        //     }
                        // });
                        // ctx.spawn(r);

                        self.addr.do_send(server::ClientMessage {
                            id: self.id,
                            msg: mess.content.clone(),
                            room: self.room.clone(),
                            mess,
                            session: self.clone(),
                        });
                    }
                }
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}
