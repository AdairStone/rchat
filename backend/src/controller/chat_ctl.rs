use std::collections::HashMap;

use actix_web::HttpResponse;
use tera::{Context, Tera};
use zino::prelude::Error;
use zino::prelude::RequestContext;
use zino::{Request, Response, Result};
use zino_core::{
    auth::UserSession,
    extension::JsonObjectExt,
    json,
    model::Query,
    orm::Schema,
    response::{ExtractRejection, Rejection, StatusCode},
    validation::{UuidValidator, Validation, Validator},
    warn, Map, Uuid,
};

use crate::utils::date_utils::current_s;
use crate::utils::str_from_map;
use crate::utils::str_from_map_required;
use crate::utils::str_to_usize;
use crate::utils::usize_from_map_default;
use crate::{
    domain::website_config::WebsiteConfig,
    model::{ChatRoom, ChatWebsite},
    service::chat_service::ChatService,
    utils::{self},
};

pub async fn admin_config_website(mut req: Request) -> Result {
    let user_session = req
        .get_data::<UserSession<_>>()
        .ok_or_else(|| warn!("401 Unauthorized: the user session is invalid"))
        .extract(&req)?;
    let user_id: &Uuid = user_session.user_id();
    let body: Map = req.parse_body().await?;
    let website_config = WebsiteConfig {
        title: str_from_map("title", &body)?,
        welcome_slogan: str_from_map("welcome_slogan", &body)?,
        site_key: None,
        user_id: user_id.clone(),
        position: str_from_map("position", &body)?,
        id: str_from_map("id", &body)?,
    };
    let res = &mut Response::default().context(&req);
    match ChatService::config_site(&website_config).await {
        Ok(data) => {
            res.set_json_data(json!(data.unwrap()));
            res.set_code(StatusCode::OK);
        }
        Err(e) => {
            res.set_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.set_error_message(e);
        }
    }
    Ok(res.clone().into())
}

pub async fn save_site_config(mut req: Request) -> Result {
    let user_session = req
        .get_data::<UserSession<_>>()
        .ok_or_else(|| warn!("401 Unauthorized: the user session is invalid"))
        .extract(&req)?;
    let user_id: &Uuid = user_session.user_id();
    let body: Map = req.parse_body().await?;

    let website_config = WebsiteConfig {
        title: str_from_map("title", &body)?,
        welcome_slogan: str_from_map("welcome_slogan", &body)?,
        site_key: str_from_map("site_key", &body)?,
        user_id: user_id.clone(),
        position: str_from_map("position", &body)?,
        id: str_from_map("id", &body)?,
    };
    let res = &mut Response::default().context(&req);
    match ChatService::save_site(&website_config).await {
        Ok(data) => {
            res.set_json_data(json!(data.unwrap()));
            res.set_code(StatusCode::OK);
        }
        Err(e) => {
            res.set_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.set_error_message(e);
        }
    }
    Ok(res.clone().into())
}

pub async fn load_site_js(req: Request) -> Result<HttpResponse> {
    if let Some(key) = req.get_query("key") {
        match ChatService::load_site(&key.to_string()).await {
            Ok(res) => {
                if res {
                    let query: Query = Query::new(Map::from_entry("site_key", key));
                    match ChatWebsite::find_one::<ChatWebsite>(&query).await {
                        Ok(site) => {
                            if site.is_some() {
                                let tera: Tera = Tera::new("static/**/*").unwrap();
                                let mut context = Context::new();
                                context.insert("script_home", "http://chat.local.com"); //replcace from config
                                context.insert("site_key", key); //replcace from config
                                let ukey = match ChatService::gen_room_key().await {
                                    Ok(key) => key,
                                    Err(e) => {
                                        return Err(Rejection::internal_server_error(e).into())
                                    }
                                };
                                context.insert("ukey", &ukey);

                                let rendered = tera.render("load.js", &context).unwrap();
                                let r = HttpResponse::Ok()
                                    // .append_header(("Cache-Control", "max-age=3600"))
                                    .content_type("application/javascript")
                                    .body(rendered);
                                return Ok(r);
                            } else {
                                return Err(Rejection::internal_server_error(warn!(
                                    "site not found"
                                ))
                                .into());
                            }
                        }
                        Err(e) => return Err(Rejection::internal_server_error(e).into()),
                    }
                } else {
                    return Err(Rejection::internal_server_error(warn!("site inactive")).into());
                }
            }
            Err(e) => {
                return Err(Rejection::internal_server_error(e).into());
            }
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
    let mut query = Query::from_entry("user_id", user_id.to_string());
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
        Ok(site) => {
            tracing::info!("chat website:{:?}", &site);
            if site.is_some() {
                site.unwrap()
            } else {
                return Err(Rejection::from_error(warn!("room site not found")).into());
            }
        }
        Err(e) => return Err(Rejection::from_error(e).into()),
    };
    let page = if req.get_query("page").is_none() {
        "1"
    } else {
        req.get_query("page").unwrap()
    };
    let page_size = if req.get_query("page_size").is_none() {
        "10"
    } else {
        req.get_query("page_size").unwrap()
    };

    let res = &mut Response::default().context(&req);
    match ChatService::list_rooms(
        &chat_site.id,
        utils::str_to_usize(page)?,
        utils::str_to_usize(page_size)?,
    )
    .await
    {
        Ok(data) => {
            res.set_json_data(json!(data));
            res.set_code(StatusCode::OK);
        }
        Err(e) => {
            res.set_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.set_error_message(e);
        }
    }
    Ok(res.clone().into())
}

pub async fn list_chatmessage(mut req: Request) -> Result {
    let body = req.parse_body::<Map>().await?;
    let user_session = req
        .get_data::<UserSession<_>>()
        .ok_or_else(|| warn!("401 Unauthorized: the user session is invalid"))
        .extract(&req)?;
    let user_id: &Uuid = user_session.user_id();
    let mut query = Query::from_entry("user_id", user_id.to_string());
   
    let site_id = str_from_map_required("site_id", &body)?;

    query.add_filter("site_id", site_id);

    let chat_site = match ChatWebsite::find_one::<ChatWebsite>(&query).await {
        Ok(site) => {
            if site.is_some() {
                site.unwrap()
            } else {
                return Err(Rejection::from_error(warn!("room site not found")).into());
            }
        }
        Err(e) => return Err(Rejection::from_error(e).into()),
    };
    let room_id = str_from_map_required("room_id", &body)?;
    // let room_uuid = match uuid::Uuid::parse_str(room_id) {
    //     Ok(id) => id,
    //     Err(e) => return Err(Rejection::from_error(e).into()),
    // };
    let mut room_query = Query::from_entry("id", room_id);
    room_query.add_filter("key", chat_site.id.to_string());
    let room = match ChatRoom::find_one::<ChatRoom>(&room_query).await {
        Ok(ro) => {
            if ro.is_some() {
                ro.unwrap()
            } else {
                return Err(Rejection::from_error(warn!("room forbidden")).into());
            }
        }
        Err(e) => return Err(Rejection::from_error(e).into()),
    };

    let page = usize_from_map_default("page", &body, 1)?;
    let page_size = usize_from_map_default("page_size", &body, 10)?;
    let date;
    if page == 1 {
        date = current_s() as usize;
    } else {
        date = str_to_usize(&str_from_map_required("ts", &body)?)?;
    }
    let res = &mut Response::default().context(&req);
    let data = ChatService::list_messages(&room, page, page_size, date)
        .await
        .extract(&req)?;
    let mut res_map = HashMap::new();
    res_map.insert("data", json!(data));
    res_map.insert("ts", date.into());
    res.set_json_data(json!(res_map));
    res.set_code(StatusCode::OK);
    Ok(res.clone().into())
}

// rest for chat_front
pub async fn list_chatmessage_from_chat(mut req: Request) -> Result {
    let body = req.parse_body::<Map>().await?;
    
    let site_key = str_from_map_required("site_key", &body)?;
    let room_key = str_from_map_required("room_key", &body)?;
    
    let query = Query::from_entry("site_key", site_key);
    // query.add_filter("site_key", site_key);
    let chat_site = match ChatWebsite::find_one::<ChatWebsite>(&query).await {
        Ok(site) => {
            if site.is_some() {
                site.unwrap()
            } else {
                return Err(Rejection::from_error(warn!("room site not found")).into());
            }
        }
        Err(e) => return Err(Rejection::from_error(e).into()),
    };

    // let mut room_query = Query::from_entry("id", room_key);
    let mut room_query = Query::from_entry("room_key", room_key);
    room_query.add_filter("room_site_id", chat_site.id.to_string());
    let room = match ChatRoom::find_one::<ChatRoom>(&room_query).await {
        Ok(ro) => {
            if ro.is_some() {
                ro.unwrap()
            } else {
                return Err(Rejection::from_error(warn!("room forbidden")).into());
            }
        }
        Err(e) => return Err(Rejection::from_error(e).into()),
    };
    let page = usize_from_map_default("page", &body, 1)?;
    let page_size = usize_from_map_default("page_size", &body, 10)?;
    let date;
    if page == 1 {
        date = current_s() as usize;
    } else {
        date = str_to_usize(&str_from_map_required("ts", &body)?)?;
    }
    let res = &mut Response::default().context(&req);
    let data = ChatService::list_messages(&room, page, page_size, date)
        .await
        .extract(&req)?;
    let mut res_map = HashMap::new();
    res_map.insert("data", json!(data));
    res_map.insert("ts", date.into());
    res.set_json_data(json!(res_map));
    res.set_code(StatusCode::OK);
    Ok(res.clone().into())
}
