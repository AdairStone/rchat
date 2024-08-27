use std::collections::HashSet;

use crate::middleware::redis::REDIS_MANAGER;
use anyhow::Result;

pub struct MessageStatusManager;

impl MessageStatusManager {
    // 增加房间的最新消息数和总消息数
    pub async fn increase_latest_count(site: &str, room_id: &str, count: usize) -> Result<()> {
        let latest_key = format!("site:{}:room:{}:latest_count", site, room_id);
        let total_key = format!("site:{}:room:{}:total_count", site, room_id);
        let unread_key = format!("site:{}:total_unread", site);

        // 增加最新消息数
        REDIS_MANAGER.incr(&latest_key, count as isize).await?;
        // 增加总消息数
        REDIS_MANAGER.incr(&total_key, count as isize).await?;
        // 增加站点的总未读消息数
        REDIS_MANAGER.incr(&unread_key, count as isize).await?;
        Ok(())
    }

    // 重置房间的最新消息数
    pub async fn reset_latest_count(site: &str, room_id: &str) -> Result<()> {
        let latest_key = format!("site:{}:room:{}:latest_count", site, room_id);
        let unread_key = format!("site:{}:total_unread", site);

        // 获取最新消息数
        let latest_count: i64 = REDIS_MANAGER.get(&latest_key).await?.unwrap_or(0);
        // 减去总未读消息数中的最新消息数
        REDIS_MANAGER
            .incr(&unread_key, -(latest_count) as isize)
            .await?;
        // 重置最新消息数
        REDIS_MANAGER.set(&latest_key, 0).await?;
        Ok(())
    }

    // 获取房间的最新消息数和总消息数
    pub async fn get_room_message_counts(site: &str, room_id: &str) -> Result<(i64, i64)> {
        let latest_key = format!("site:{}:room:{}:latest_count", site, room_id);
        let total_key = format!("site:{}:room:{}:total_count", site, room_id);

        let latest_count: i64 = REDIS_MANAGER.get(&latest_key).await?.unwrap_or(0);
        let total_count: i64 = REDIS_MANAGER.get(&total_key).await?.unwrap_or(0);

        Ok((latest_count, total_count))
    }

    // 获取站点的总未读消息数量
    pub async fn get_total_unread(site: &str) -> Result<i64> {
        let unread_key = format!("site:{}:total_unread", site);
        let total_unread: i64 = REDIS_MANAGER.get(&unread_key).await?.unwrap_or(0);
        Ok(total_unread)
    }

    pub async fn set_site_rooms(site: &str, rooms: &HashSet<String>) -> Result<()> {
        let room_key = format!("site:{}:rooms", site);
        let val: Vec<String> = rooms.clone().into_iter().collect();
        REDIS_MANAGER.set(&room_key, val.join(",")).await?;
        Ok(())
    }

    pub async fn get_site_rooms(site: &str) -> Result<HashSet<String>> {
        let room_key = format!("site:{}:rooms", site);
        let rooms: Option<String> = REDIS_MANAGER.get(&room_key).await?;
        tracing::info!("rooms: {:?}", rooms);
        let mut hs = HashSet::<String>::new();
        if let Some(rs) = rooms {
            rs.split(",").for_each(|r| {
                hs.insert(r.to_string());
            });
        }
        Ok(hs)
        // Ok(())
    }
}
