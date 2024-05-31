use std::time::Instant;

use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

pub mod server;
pub mod session;

/// Entry point for our websocket route
pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        session::WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "main".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

// async fn index() -> impl Responder {
//     NamedFile::open_async("./static/index.html").await.unwrap()
// }

// /// Displays state
// async fn get_count(count: web::Data<AtomicUsize>) -> impl Responder {
//     let current_count = count.load(Ordering::SeqCst);
//     format!("Visitors: {current_count}")
// }