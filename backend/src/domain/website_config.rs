use serde::{Deserialize, Serialize};
use zino_core::Uuid;

#[derive(
    Debug,
    Clone,
    Default,
    Serialize,
    Deserialize,
)]
pub struct WebsiteConfig {
    pub id: Option<String>,
    pub title: Option<String>,
    pub welcome_slogan: Option<String>,
    pub site_key: Option<String>,
    pub user_id: Uuid,
    pub position: Option<String>
}
