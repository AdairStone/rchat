use crate::{
    app_config::SETTINGS, domain::website_config::WebsiteConfig, model::{ChatMessage, ChatRoom, ChatWebsite}, utils::{self, generate_random_string}
};
use zino_core::{
    datetime::DateTime, error::Error, extension::JsonObjectExt, json, model::Query, orm::{ModelAccessor, Schema}, warn, JsonValue, Map, Uuid
};

use super::room_message_state::MessageStatusManager;

pub struct ChatService;

/**
 * 1.客服配置网站
 * 2.网站客户网站发起聊天，加载聊天窗口
 * 3.客服获取聊天列表（优先显示在线的，按发起聊天时间排序）
 * 4.选择客服列表拉取聊天消息
 * 5.发送聊天消息并保存到数据库
 * 6.聊天消息撤回，聊天消息删除
 */

impl ChatService {
    // 1
    pub async fn config_site(website_config: &WebsiteConfig) -> Result<Option<ChatWebsite>, Error> {
        let mut query: Query = Query::new(Map::from_entry(
            "user_id",
            website_config.user_id.to_string(),
        ));
        if let Some(mut chat_website) = ChatWebsite::find_one::<ChatWebsite>(&query).await? {
            chat_website.script_home = SETTINGS.script_home.clone();
            return Ok(Some(chat_website));
        } else {
            let mut chat_website = ChatWebsite::default();
            chat_website.id = Uuid::now_v7();
            chat_website.site_key = Self::gen_site_key().await?;
            chat_website.title = website_config.title.clone();
            chat_website.welcome_slogan = website_config.welcome_slogan.clone();
            chat_website.position = website_config.position.clone();
            chat_website.user_id = website_config.user_id;
            chat_website.script_home = SETTINGS.script_home.clone();
            let result = chat_website.clone();
            chat_website.insert().await?;
            return Ok(Some(result));
        }
    }
    // 1.1
    pub async fn save_site(website_config: &WebsiteConfig) -> Result<Option<ChatWebsite>, Error> {
        let mut query: Query = Query::new(Map::from_entry(
            "user_id",
            website_config.user_id.to_string(),
        ));
        query.add_filter("id", website_config.id.clone().unwrap().to_string());
        if let Some(mut chat_website) = ChatWebsite::find_one::<ChatWebsite>(&query).await? {
            chat_website.title = website_config.title.clone();
            chat_website.welcome_slogan = website_config.welcome_slogan.clone();
            chat_website.position = website_config.position.clone();
            chat_website.update_at = DateTime::now();
            chat_website.clone().update().await?;
            chat_website.script_home = SETTINGS.script_home.clone();
            return Ok(Some(chat_website));
        } else {
            return Err(warn!("error save website config"));
        }
    }

    async fn gen_site_key() -> Result<String, Error> {
        // retry three times
        let key_size = 15;
        let mut i = 0;
        let mut site_key = generate_random_string(key_size);
        loop {
            let query = Query::new(Map::from_entry("site_key", site_key.clone()));
            if let Some(_site) = ChatWebsite::find_one::<ChatWebsite>(&query).await? {
                i = i + 1;
                if i > 4 {
                    return Err(warn!("gen key error for retry 3 times"));
                }
                tracing::warn!("site key {site_key} duplicated, retry {i}");
                site_key = generate_random_string(key_size);
                continue;
            } else {
                break;
            }
        }
        return Ok(site_key);
    }

    // 1.1 配置之后等待前台加载并访问
    pub async fn load_site(site_key: &String) -> Result<bool, Error> {
        let query: Query = Query::new(Map::from_entry("site_key", site_key.clone()));
        if let Some(mut chat_website) = ChatWebsite::find_one::<ChatWebsite>(&query).await? {
            chat_website.status = "confirmed".to_string();
            chat_website.update().await?;
            return Ok(true);
        }
        Ok(false)
    }

    // 2 发起聊天
    pub async fn new_room(
        site_key: String,
        room_key: Option<String>,
    ) -> Result<Option<ChatRoom>, Error> {
        let rkey = if let Some(k) = room_key {
            k
        } else {
            Self::gen_room_key().await?
        };
        let mut query: Query = Query::new(Map::from_entry("room_key", rkey.clone()));

        let mut site_query: Query = Query::new(Map::from_entry("site_key", site_key.clone()));
        site_query.add_filter("status", "confirmed");
        if let Some(chat_website) = ChatWebsite::find_one::<ChatWebsite>(&site_query).await? {
            let site_id = chat_website.id();
            query.add_filter("room_site_id", site_id.clone().to_string());
            if let Ok(room_exists) = ChatRoom::find_one::<ChatRoom>(&query).await {
                if let Some(mut room) = room_exists {
                    // room_exists
                    room.status = "active".to_owned();
                    let room_clone = room.clone();
                    room.update().await?;
                    Ok(Some(room_clone))
                } else {
                    let mut chat_room = ChatRoom::default();
                    chat_room.id = Uuid::now_v7();
                    chat_room.room_site_id = site_id.clone();
                    chat_room.room_key = rkey;
                    let room = chat_room.clone();
                    chat_room.insert().await?;
                    Ok(Some(room))
                }
            } else {
                Err(warn!("init room error"))
            }
        } else {
            Err(warn!("site not found or site already confirmed"))
        }
    }

    pub async fn gen_room_key() -> Result<String, Error> {
        // retry three times
        let key_size = 15;
        let mut i = 0;
        let mut room_key = utils::generate_random_string(key_size);
        loop {
            let query = Query::new(Map::from_entry("room_key", room_key.clone()));
            if let Some(_room) = ChatRoom::find_one::<ChatRoom>(&query).await? {
                i = i + 1;
                if i > 4 {
                    return Err(warn!("gen key error for retry 3 times"));
                }
                tracing::warn!("site key {room_key} duplicated, retry {i}");
                room_key = utils::generate_random_string(key_size);
                continue;
            } else {
                break;
            }
        }
        return Ok(room_key);
    }

    // 2.1 断开连接
    pub async fn room_disconnect(room: &mut ChatRoom) -> Result<(), Error> {
        room.status = "disconnect".to_owned();
        room.update_at = DateTime::now();
        room.clone().update().await?;
        Ok(())
    }

    // 3 获取所有聊天房间列表（客服人员）
    pub async fn list_rooms(
        site_id: &Uuid,
        page: usize,
        page_num: usize,
    ) -> Result<Map, Error> {
        let mut query: Query = Query::new(Map::from_entry("room_site_id", site_id.to_string()));
        let count_query = query.clone();
        query.order_by("create_at", true);
        query.set_limit(page_num);
        query.set_offset((page - 1) * page_num);
        let total = ChatRoom::count(&count_query).await?;
        let data = ChatRoom::find::<ChatRoom>(&query).await?;
        let mut res  = Map::new();
        tracing::info!("list rooms 1: {data:?}");
        let md = data.iter().map(|r| -> JsonValue {
            serde_json::to_value(r).unwrap()
        }).collect::<Vec<JsonValue>>();
        tracing::info!("list rooms 2: {md:?}");
        res.append(&mut Map::from_entry("data", md));
        res.append(&mut Map::from_entry("total", total));
        Ok(res)
    }

    // 3.1 （客服人员） 加入房间，消息变成已读。
    pub async fn join_room(room: &ChatRoom) -> Result<(), Error> {
        let mut query: Query = Query::new(Map::from_entry("room_id", room.id.to_string()));
        query.add_filter("status", "sended");
        let mut messages = ChatMessage::find::<ChatMessage>(&query).await?;
        for message in messages.iter_mut() {
            message.status = "readed".to_owned();
            message.update_at = DateTime::now();
            message.clone().update().await?;
        }
        let site_query: Query = Query::new(Map::from_entry("id", room.room_site_id.to_string()));
        let site = ChatWebsite::find_one::<ChatWebsite>(&site_query).await?;
        if site.is_some() {// 读取消息后重置消息状态
            match MessageStatusManager::reset_latest_count(&site.unwrap().site_key, &room.id.to_string()).await {
                Ok(s) => {
                    tracing::info!("reset latest count success");
                },
                Err(e) => {
                   tracing::error!("reset latest count error: {}", e);
                }
            }
        }
        Ok(())
    }

    // 3.1 保存消息
    pub async fn save_message(message: &ChatMessage) -> Result<(), Error> {
        message.clone().insert().await?;
        Ok(())
    }

    // 3.2 获取消息列表
    pub async fn list_messages(
        room: &ChatRoom,
        page: usize,
        page_num: usize,
        ts: usize,
    ) -> Result<Vec<ChatMessage>, Error> {
        let mut query = Query::new(Map::from_entry("room_id", room.id.to_string()));
        query.order_desc("create_at");
        let start = DateTime::from_timestamp(ts as  i64);
        query.add_filter("create_at", json!({"$le": start}));
        query.set_limit(page_num);
        query.set_offset((page - 1) * page_num);
        // Self::join_room(room).await?;
        tracing::info!("list messages condition: start:{}: {:?}", &start, &query);
        Ok(ChatMessage::find(&query).await?)
    }
}
