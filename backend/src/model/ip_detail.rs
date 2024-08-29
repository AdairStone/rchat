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
pub struct IpDetail {
    #[schema(primary_key, read_only)]
    pub ip: String,
    pub city: Option<String>,
    pub region: Option<String>,
    pub country: Option<String>,
    pub loc: Option<String>,
    pub org: Option<String>,
    #[schema(read_only, default_value = "now", index_type = "btree")]
    pub create_at: DateTime,
    #[schema(default_value = "now", index_type = "btree")]
    pub update_at: DateTime,
    pub version: u64,
}