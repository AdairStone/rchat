use std::sync::{atomic::AtomicUsize, Arc};

use crate::{
    controller::{auth, chat_ctl, file, stats, user},
    middleware,
    model::Tag, wsserver::{self, server::{self, ChatServer}},
};
use actix::{Actor, Addr};
use actix_web::web::{self, get, post, scope, ServiceConfig};
use zino::{DefaultController, RouterConfigure};
use zino_model::User;

lazy_static! {
    static ref APP_STATE: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    static ref SERVER: Addr<ChatServer> = server::ChatServer::new(APP_STATE.clone()).start();
    static ref CURRENT_VISITORS: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));

}

pub fn main_routes() -> Vec<RouterConfigure> {
    vec![
        auth_router as RouterConfigure,
        ws_router as RouterConfigure,
        file_router as RouterConfigure,
        chat_router as RouterConfigure,
    ]
}

pub fn debug_routes() -> Vec<RouterConfigure> {
    vec![
        user_router as RouterConfigure,
        tag_router as RouterConfigure,
        stats_router as RouterConfigure,
        user_debug_router as RouterConfigure,
        tag_debug_router as RouterConfigure,
    ]
}

fn auth_router(cfg: &mut ServiceConfig) {
    cfg.route("/auth/login", post().to(auth::login));
    cfg.service(
        scope("/auth")
            .route("/refresh", get().to(auth::refresh))
            .route("/logout", post().to(auth::logout))
            .wrap(middleware::UserSessionInitializer),
    );
}

fn ws_router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/ws")
            .route("/chat", web::get().to(wsserver::chat_route))
            .app_data(web::Data::from(APP_STATE.clone()))
            // .app_data(web::Data::from(CURRENT_VISITORS))
            .app_data(web::Data::new(SERVER.clone()))
            .wrap(middleware::UserSessionInitializer)
            ,
    );
    cfg.route("/clientchat", web::get().to(wsserver::chat_route))
    .app_data(web::Data::from(APP_STATE.clone()))
    // .app_data(web::Data::from(CURRENT_VISITORS))
    .app_data(web::Data::new(SERVER.clone()));
}

fn file_router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/file")
            .route("/upload", post().to(file::upload))
            .route("/decrypt", get().to(file::decrypt))
            .wrap(middleware::UserSessionInitializer),
    );
}

fn user_router(cfg: &mut ServiceConfig) {
    cfg.route("/user/new", post().to(user::new))
        .route("/user/{id}/delete", post().to(User::soft_delete))
        .route("/user/{id}/update", post().to(User::update))
        .route("/user/{id}/view", get().to(user::view))
        .route("/user/list", get().to(User::list))
        .route("/user/import", post().to(User::import))
        .route("/user/export", get().to(User::export));
}

fn tag_router(cfg: &mut ServiceConfig) {
    cfg.route("/tag/new", post().to(Tag::new))
        .route("/tag/{id}/delete", post().to(Tag::soft_delete))
        .route("/tag/{id}/update", post().to(Tag::update))
        .route("/tag/{id}/view", get().to(Tag::view))
        .route("/tag/list", get().to(Tag::list))
        .route("/tag/tree", get().to(Tag::tree));
}

fn stats_router(cfg: &mut ServiceConfig) {
    cfg.route("/stats", get().to(stats::index));
}

fn user_debug_router(cfg: &mut ServiceConfig) {
    cfg.route("/user/schema", get().to(User::schema))
        .route("/user/definition", get().to(User::definition))
        .route("/user/mock", get().to(User::mock));
}

fn tag_debug_router(cfg: &mut ServiceConfig) {
    cfg.route("/tag/schema", get().to(Tag::schema))
        .route("/tag/definition", get().to(Tag::definition))
        .route("/tag/mock", get().to(Tag::mock));
}


fn chat_router(cfg: &mut ServiceConfig){
    cfg.service(
        scope("/service")
            .route("/config-site", post().to(chat_ctl::admin_config_website))
            .route("/save-site", post().to(chat_ctl::save_site_config))
            .route("/list-rooms", get().to(chat_ctl::list_rooms))
            .route("/list-chatmessage", get().to(chat_ctl::list_chatmessage))
            .wrap(middleware::UserSessionInitializer),
    );
    cfg.service(
        scope("/load")
            .route("/load.js", get().to(chat_ctl::load_site_js))
            .route("/messages", get().to(chat_ctl::list_chatmessage_from_chat))
            ,
    );
}
