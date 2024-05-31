use serde::{Deserialize, Serialize};
use zino::prelude::*;
use zino_derive::{DecodeRow, Model, ModelAccessor, ModelHooks, Schema};

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
pub struct ChatMedia {
    #[schema(primary_key, read_only, constructor = "Uuid::now_v7")]
    id: Uuid,
    file_name: String,
    path: String,
    file_type: String,
    #[schema(read_only, default_value = "now", index_type = "btree")]
    create_at: DateTime,
    #[schema(default_value = "now", index_type = "btree")]
    update_at: DateTime,
    version: u64,
}
