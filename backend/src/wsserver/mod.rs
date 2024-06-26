use std::{ops::Deref, time::Instant};

use actix::Addr;
use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use zino::Request;
use zino_core::{auth::{JwtClaims, UserSession}, model::Query, orm::Schema, Uuid};
use zino_model::User;

use crate::{model::ChatRoom, service::chat_service::ChatService};
use zino::prelude::RequestContext;

pub mod server;
pub mod session;

/// Entry point for our websocket route
pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    tracing::info!("start chat...");
    let query_params = web::Query::<std::collections::HashMap<String, String>>::from_query(req.query_string()).unwrap();
    let key = if query_params.get("site_key").is_some() {  query_params.get("site_key").unwrap() } else { return Err(error::ErrorBadRequest("site key must provied"));};
    let client = if query_params.get("client").is_some() {  query_params.get("client").unwrap() } else { return Err(error::ErrorBadRequest("client must provied"));};
    let mut user: Option<User> = None;
    let mut name: Option<String> = None;
    let mut room_key = query_params.get("room_key").map(|el| el.clone());
    let mut room = ChatRoom::default();
    let mut user_type = 0 as usize;
    tracing::info!("join chat with:{}", client);
    if client == "0" {// 服务端加入
        // req.hea
        tracing::info!("from server user...");
        let new_req = Request::from(req.clone());
        match new_req.parse_jwt_claims(JwtClaims::shared_key()) {
            Ok(claims) => {
                if let Ok(user_session) = UserSession::<Uuid>::try_from_jwt_claims(claims) {
                    match User::find_by_id::<User>(user_session.user_id()).await {
                        Ok(us) => {
                            tracing::info!("asigin user...");
                            user = us
                        },
                        Err(e) => return Err(error::ErrorForbidden(e)),
                    };
                } else {
                    return Err(error::ErrorBadRequest("token session invalid"));
                }
            }
            Err(rejection) => {
                tracing::warn!("server join room error for: {:?}", rejection);
                return Err(error::ErrorBadRequest("token session invalid"));
            }
        }
        if room_key.is_none() { return Err(error::ErrorBadRequest("room_key must provied")); }
        room = match ChatRoom::find_one::<ChatRoom>(&Query::from_entry("room_key", room_key.clone().unwrap().to_string())).await {
            Ok(ro) => if ro.is_some() { ro.unwrap() } else { return Err(error::ErrorBadRequest("token session invalid")); },
            Err(e) =>  return Err(error::ErrorBadRequest(e)) ,
        };
        user_type = 0;
    } else {
        room = match ChatService::new_room(key.clone(), room_key).await {
            Ok(ro) => if ro.is_some() { ro.unwrap() } else { return Err(error::ErrorBadRequest("chat start error for not created"));},
            Err(e) =>  {
                tracing::error!("start fail for error:{}", e);
                return Err(error::ErrorInternalServerError(format!("start fail for error:{}",e)));
            },
        };
        user_type = 1;
    }
    tracing::info!("join room: {:?} user：{:?} ", &room, &user);
    // todo 收集一些远端信息 ip domian client agent
    ws::start(
        session::WsChatSession {
            site_key: key.clone(),
            id: 0,
            hb: Instant::now(),
            room: room.id.to_string(),
            room_obj: room,
            user: user,
            name: name,
            user_type: user_type,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

fn get_token(req: &HttpRequest) -> Result<Option<String>, Error> {
    let headers = req.headers();
    let mut token = "";
    if let Some(authorization) = headers.get("Authorization") {
        let tsr = match authorization.to_str() {
            Ok(dta) => dta,
            Err(e) =>  return Err(error::ErrorBadRequest(format!("paser error:{}" , e))),
        };
        token = tsr
            .strip_prefix("Bearer ")
            .unwrap_or(tsr);
    }
    if token.is_empty() {
        return Ok(None);
    }
    Ok(Some(token.to_string()))
}