use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Default,
    Serialize,
    Deserialize,
)]
#[serde(default)]
pub struct 
ChatFiles {
    file_key: String,
    url: String,
    file_name: String,
}
