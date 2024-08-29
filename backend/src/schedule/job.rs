use std::collections::HashSet;

use zino::prelude::*;
use zino_model::User;

use crate::{model::ChatWebsite, service::room_message_state::MessageStatusManager};

pub fn every_15s(job_id: Uuid, job_data: &mut Map, last_tick: DateTime) {
    let counter = job_data
        .get("counter")
        .map(|c| c.as_u64().unwrap_or_default() + 1)
        .unwrap_or_default();
    job_data.upsert("counter", counter);
    job_data.upsert("current", DateTime::now());
    job_data.upsert("last_tick", last_tick);
    job_data.upsert("job_id", job_id.to_string());
}

pub fn every_20s(job_id: Uuid, job_data: &mut Map, last_tick: DateTime) {
    let counter: u64 = job_data
        .get("counter")
        .map(|c| c.as_u64().unwrap_or_default() + 1)
        .unwrap_or_default();
    job_data.upsert("counter", counter);
    job_data.upsert("current", DateTime::now());
    job_data.upsert("last_tick", last_tick);
    job_data.upsert("job_id", job_id.to_string());
}

pub fn every_10s(job_id: Uuid, job_data: &mut Map, last_tick: DateTime) -> BoxFuture {
    Box::pin(async {
        // let query = Query::from_entry("status", "confirmed");
        // if let Ok(sites) = ChatWebsite::find::<ChatWebsite>(&query).await {
        //     for site in sites {
        //         let rooms = MessageStatusManager::get_site_rooms(&site.site_key).await;
        //         // let a = MessageStatusManager::reset_all_counts(&site.site_key).await;
        //         // let a = MessageStatusManager::set_site_rooms(&site.site_key, &HashSet::new()).await;
        //         // tracing::info!("every 10 seconds: {:?}: {:?}", &site, &rooms);
        //     }
        // }
        tracing::info!("every 10 seconds");
    })
}

pub fn every_hour(job_id: Uuid, job_data: &mut Map, last_tick: DateTime) -> BoxFuture {
    let counter = job_data
        .get("counter")
        .map(|c| c.as_u64().unwrap_or_default() + 1)
        .unwrap_or_default();
    job_data.upsert("counter", counter);
    job_data.upsert("current", DateTime::now());
    job_data.upsert("last_tick", last_tick);
    job_data.upsert("job_id", job_id.to_string());
    Box::pin(async {
        let query = Query::default();
        let columns = [("*", true), ("roles", true)];
        if let Ok(mut map) = User::count_many(&query, &columns).await {
            job_data.append(&mut map);
        }
    })
}
