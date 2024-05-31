
use serde::{Deserialize, Serialize};
use zino::prelude::*;
use zino_derive::{DecodeRow, Model, ModelAccessor, ModelHooks, Schema};
use zino_model::User;

use super::ChatMedia;
use super::ChatRoom;

#[derive(
    Debug,
    Clone,
    Default,
    Serialize,
    Deserialize,
    DecodeRow,
    Schema,
    ModelAccessor,
    ModelHooks,
    Model,
)]
#[serde(default)]
pub struct ChatMessage {
    #[schema(primary_key, read_only, constructor = "Uuid::now_v7")]
    pub id: Uuid,
    #[schema(not_null, comment = "sender user name")]
    pub name: String,
    pub avatar: String,
    #[schema(
        reference = "User",
    )]
    pub user_id: Option<Uuid>,
    #[schema(default_value = "sended", index_type = "hash")]// sended readed recall delete
    pub status: String,
    #[schema(
        snapshot,
        reference = "ChatMessage",
        fetch_as = "reply_to_message",
        comment = "message replied to"
    )]
    pub reply_to_id: Option<Uuid>,
    pub content: String,
    #[schema(
        max_items = 5,
        reference = "ChatMedia",
        fetch_as = "files",
    )]
    pub file_ids: Vec<Uuid>,
    #[schema(
        snapshot,
        reference = "ChatRoom",
        fetch_as = "room",
        comment = "message replied to"
    )]
    pub room_id: Uuid,
    #[schema(read_only, default_value = "now", index_type = "btree")]
    pub create_at: DateTime,
    #[schema(default_value = "now", index_type = "btree")]
    pub update_at: DateTime,
    pub version: u64,
}
