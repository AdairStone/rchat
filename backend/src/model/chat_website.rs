use serde::{Deserialize, Serialize};
use zino::prelude::*;
use zino_derive::{DecodeRow, Model, ModelAccessor, ModelHooks, Schema};
use zino_model::User;

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
#[schema(unique_on="site_key")]
pub struct ChatWebsite {
    #[schema(primary_key, read_only, constructor = "Uuid::now_v7")]
    pub id: Uuid,
    #[schema(not_null, unique, comment = "uniq string for frontend")]
    pub site_key: String,
    #[schema(default_value = "inited", index_type = "hash")] // inited confirmed
    pub status: String,
    pub domain: Option<String>,
    pub title: Option<String>,
    pub welcome_slogan: Option<String>,
    pub position: Option<String>,
    #[schema(
        snapshot,
        reference = "User",
        fetch_as = "user",
        comment = "website ownner"
    )]
    pub user_id: Uuid,
    #[schema(read_only, default_value = "now", index_type = "btree")]
    pub create_at: DateTime,
    #[schema(default_value = "now", index_type = "btree")]
    pub update_at: DateTime,
    pub version: u64,
    #[schema(ignore)]
    pub script_home: String,
}