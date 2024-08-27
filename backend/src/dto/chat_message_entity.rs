use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::{service::room_message_state::MessageStatusManager, utils::date_utils::{current_date, date_ymdhms, format_date_ymdhms}};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ChatMessageDto {
    pub text: String,
    pub time: String,
    pub user: bool,
    pub user_name: Option<String>,
    pub str_files: Option<String>,
    pub notify: String,
    pub room_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ChatMessageFileDto {
    pub url: String,
    pub name: String,
}

// { text: '在系统后台选择充值服务，可以进行充值😊', time: '10:34:12', user: false, userName: '', fileName: '', files: [{ url: 'http://192.168.0.105/q33.png', name: '' }, { url: 'http://192.168.0.105/q33.png', name: '' }, { url: 'http://192.168.0.105/q.zip', name: 'q.zip' }], notify: '' },

impl ChatMessageDto {

    pub fn new_text_msg(text:&str, from_user: bool, user_name: Option<String>, room_id: Option<String>) -> Self {
        // let time = chrono::Local::now().format("%H:%M:%S").to_string();
        let time = date_ymdhms(current_date());
        let mut user = from_user;
        if user_name.is_none() {
            user = !from_user;
        }
        Self {
            text: text.to_string(),
            time: time,
            user: user,
            user_name: user_name,
            str_files: Some("".to_string()),
            notify: "".to_string(),
            room_id,
        }
    }

    pub fn new_notify_msg(notify:&str, from_user: bool, user_name: Option<String>, room_id: Option<String>) -> Self {
        // let time = chrono::Local::now().format("%H:%M:%S").to_string();
        let time = date_ymdhms(current_date());
        Self {
            text: "".to_string(),
            time: time,
            user: from_user,
            user_name: user_name,
            str_files: Some("".to_string()),
            notify: notify.to_string(),
            room_id,
        }
    }

    pub fn new_text_str_files_msg(text:&str, files: Option<String>, from_user: bool, user_name: Option<String>, room_id: Option<String>) -> Self {
        let time = date_ymdhms(current_date());
        let mut user = from_user;
        if user_name.is_none() {
            user = !from_user;
        }
        Self {
            text: text.to_string(),
            time: time,
            user: user,
            user_name: user_name,
            str_files: files,
            notify: "".to_string(),
            room_id,
        }
    }

}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ChatNotify {
    pub to_server: bool,
    pub message: ChatNotifyMessageDto,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ChatNotifyMessageDto {
    pub total_unread: i32,
    pub new_message: bool,
    pub message_counts: HashMap<String, i32>,
}

impl ChatNotify {
    pub async fn new_from_redis(site_key: &str) -> Self {
        let total_unread = MessageStatusManager::get_total_unread(site_key).await.unwrap_or(0);
        
        let room_set = MessageStatusManager::get_site_rooms(site_key).await.unwrap_or(HashSet::new());
        tracing::info!("site {} room_set: {:?}", &site_key, &room_set);
        let mut room_counts: HashMap<String,i32> = HashMap::new();
        for room_id in room_set.iter() {
            let (unread_counts, _) = MessageStatusManager::get_room_message_counts(site_key, room_id).await.unwrap_or((0,0));
            room_counts.insert(room_id.clone(), unread_counts as i32);
        }

        let message = ChatNotifyMessageDto {
            total_unread: total_unread as i32,
            new_message: true,
            message_counts: room_counts,
        };
        Self {
            to_server: true,
            message: message,
        }
    }
}