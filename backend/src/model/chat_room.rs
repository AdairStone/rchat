use serde::{Deserialize, Serialize};
use zino::prelude::*;
use zino_derive::{DecodeRow, Model, ModelAccessor, ModelHooks, Schema};

use super::ChatWebsite;

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
#[schema(unique_on="room_key")]
pub struct ChatRoom {
    #[schema(primary_key, read_only, constructor = "Uuid::now_v7")]
    pub id: Uuid,
    #[schema(not_null, unique, comment = "uniq key for users to join")]
    pub room_key: String,
    #[schema(default_value = "active", index_type = "hash")] // active disconnect
    pub status: String,
    #[schema(
        snapshot,
        reference = "ChatWebsite",
        fetch_as = "room_site",
        comment = "room ownner"
    )]
    pub room_site_id: Uuid,
    #[schema(read_only, default_value = "now", index_type = "btree")]
    pub create_at: DateTime,
    #[schema(default_value = "now", index_type = "btree")]
    pub update_at: DateTime,
    pub version: u64,
}
