
use actix_files::NamedFile;
use zino::{Request, Response, Result};
use zino_core::{auth::UserSession, json, model::Query, orm::Schema, response::{ExtractRejection, Rejection, StatusCode}, validation::{self, UuidValidator, Validation, Validator}, warn, Map, Uuid};
use zino::prelude::Error;
use zino::prelude::RequestContext;

use crate::{domain::website_config::WebsiteConfig, model::{ChatRoom, ChatWebsite}, service::chat_service::ChatService, utils};

pub async fn admin_config_website(mut req: Request) -> Result {
    let user_session = req
        .get_data::<UserSession<_>>()
        .ok_or_else(|| warn!("401 Unauthorized: the user session is invalid"))
        .extract(&req)?;
    let user_id: &Uuid = user_session.user_id();
    let body: Map = req.parse_body().await?;
    let welcome_slogan = if body.get("welcome_slogan").is_none(){ "".to_string() } else { body.get("welcome_slogan").unwrap().to_string()};
    let title = if body.get("title").is_none(){ "".to_string() } else { body.get("title").unwrap().to_string()};
    let position = if body.get("position").is_none(){ "".to_string() } else { body.get("position").unwrap().to_string()};

    let website_config = WebsiteConfig {
        title: Some(title),
        welcome_slogan: Some(welcome_slogan),
        site_key: None,
        user_id: user_id.clone(),
        position: Some(position),
    };
    let res = &mut Response::default().context(&req);
    match ChatService::config_site(&website_config).await {
        Ok(data) => {
            res.set_json_data(json!(data.unwrap()));
            res.set_code(StatusCode::OK);
        },
        Err(e) => {
            res.set_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.set_error_message(e);
        }
    }
    Ok(res.clone().into())
}


pub async fn load_site_js(req: Request) -> Result<NamedFile> {
    if let Some(key) = req.get_query("key") {
        match ChatService::load_site(&key.to_string()).await {
            Ok(res) => {
                if res {
                    match NamedFile::open_async("./static/load.js").await {
                        Ok(file) => return Ok(file),
                        Err(e) => return Err(Rejection::internal_server_error(e).into()),
                    }
                } else {
                    return Err(Rejection::internal_server_error(warn!("inactive")).into());            
                }
            },
            Err(e) => {
                return Err(Rejection::internal_server_error(e).into());
            },
        }
    } else {
        return Err(Rejection::internal_server_error(warn!("need key")).into());
    }
}


pub async fn list_rooms(req: Request) -> Result {
    let user_session = req
        .get_data::<UserSession<_>>()
        .ok_or_else(|| warn!("401 Unauthorized: the user session is invalid"))
        .extract(&req)?;
    let user_id: &Uuid = user_session.user_id();
    tracing::info!("user_id:{}", user_id);
    let mut query = Query::from_entry("user_id",user_id.to_string());
    let site_id = if let Some(s_id) = req.get_query("site_id") { 
        if let Err(e) = UuidValidator.validate(s_id) {
            return Err(Rejection::from_error(e).into());    
        }
        s_id
    } else {
        let mut validation = Validation::new();
        validation.record("site_id", "should provide site_id");
        return Err(Rejection::bad_request(validation).into());
    };
    query.add_filter("site_id", site_id);
    tracing::info!("site_id:{}", site_id);
    let chat_site = match ChatWebsite::find_one::<ChatWebsite>(&query).await {
        Ok(site) => if site.is_some() { site.unwrap() } 
        else {return Err(Rejection::from_error(warn!("room site not found")).into());},
        Err(e) => return Err(Rejection::from_error(e).into()),
    };
    let page = if req.get_query("page").is_none(){ "1" } else { req.get_query("page").unwrap()};
    let page_size = if req.get_query("page_size").is_none(){ "10" } else { req.get_query("page_size").unwrap()};
    
    let res = &mut Response::default().context(&req);
    match ChatService::list_rooms(&chat_site.id, utils::str_to_usize(page)?, utils::str_to_usize(page_size)?).await {
        Ok(data) => {
            res.set_json_data(json!(data));
            res.set_code(StatusCode::OK);
        },
        Err(e) => {
            res.set_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.set_error_message(e);
        }
    }

    Ok(res.clone().into())
}

pub async fn list_chatmessage(req: Request) -> Result {
    let user_session = req
        .get_data::<UserSession<_>>()
        .ok_or_else(|| warn!("401 Unauthorized: the user session is invalid"))
        .extract(&req)?;
    let user_id: &Uuid = user_session.user_id();
    let mut query = Query::from_entry("user_id",user_id.to_string());
    let site_id = if let Some(s_id) = req.get_query("site_id") { 
        if let Err(e) = UuidValidator.validate(s_id) {
            return Err(Rejection::from_error(e).into());    
        }
        s_id
    } else {
        let mut validation = Validation::new();
        validation.record("site_id", "should provide site_id");
        return Err(Rejection::bad_request(validation).into());
    };
    query.add_filter("site_id", site_id);

    let chat_site = match ChatWebsite::find_one::<ChatWebsite>(&query).await {
        Ok(site) => if site.is_some() { site.unwrap() } 
        else {return Err(Rejection::from_error(warn!("room site not found")).into());},
        Err(e) => return Err(Rejection::from_error(e).into()),
    };
    let room_id = if let Some(ro_id) = req.get_query("room_id") { 
        if let Err(e) = UuidValidator.validate(ro_id) {
            return Err(Rejection::from_error(e).into());    
        }
        ro_id
    } else {
        let mut validation = Validation::new();
        validation.record("room_id", "should provide room_id");
        return Err(Rejection::bad_request(validation).into());
    };
    // let room_uuid = match uuid::Uuid::parse_str(room_id) {
    //     Ok(id) => id,
    //     Err(e) => return Err(Rejection::from_error(e).into()),
    // };
    let mut room_query = Query::from_entry("id", room_id);
    room_query.add_filter("key", chat_site.id.to_string());
    let room = match ChatRoom::find_one::<ChatRoom>(&room_query).await{
        Ok(ro) => if ro.is_some() { ro.unwrap() } else { return Err(Rejection::from_error(warn!("room forbidden")).into()) },
        Err(e) => return Err(Rejection::from_error(e).into()),
    };
    let page = if req.get_query("page").is_none(){ "1" } else { req.get_query("page").unwrap()};
    let page_size = if req.get_query("page_size").is_none(){ "10" } else { req.get_query("page_size").unwrap()};
    
    let res = &mut Response::default().context(&req);
    match ChatService::list_messages(&room, utils::str_to_usize(page)?, utils::str_to_usize(page_size)?).await {
        Ok(data) => {
            res.set_json_data(json!(data));
            res.set_code(StatusCode::OK);
        },
        Err(e) => {
            res.set_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.set_error_message(e);
        }
    }

    Ok(res.clone().into())
}




