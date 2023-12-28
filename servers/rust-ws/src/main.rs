use log::debug;
use std::{env, time::Instant};

use actix::*;
use actix_web::{middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

mod pinus;
mod server;
mod session;
mod utils;

use utils::config::{get_config, ConfigKeys};

/// Entry point for our websocket route
async fn route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::WsServer>>,
) -> Result<HttpResponse, Error> {
    log::debug!("web route request: {:?}", req);

    ws::start(
        session::WsSession {
            id: 0,
            heartbeat: Instant::now(),
            room: "main".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    env::set_var("RUST_LOG", &log_level);
    println!("Log Level: {:?}", &log_level);
    env_logger::init();

    if log_level == "debug" {
        debug!("Debug mode");
        env::set_var("RUST_BACKTRACE", "1");
    }

    // config for server
    let server_port = get_config::<u16>(ConfigKeys::HTTP_SERVER_PORT).unwrap();
    let cluster_enable = get_config::<usize>(ConfigKeys::HTTP_SERVER_CLUSTER).unwrap();
    let cluster_count: usize;
    if cluster_enable > 0 {
        cluster_count = std::thread::available_parallelism().map_or(2, std::num::NonZeroUsize::get);
    } else {
        cluster_count = 1;
    }

    // start chat server actor
    let server = server::WsServer::new().start();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone()))
            .route("/", web::get().to(route))
            .wrap(Logger::default())
    })
    .workers(cluster_count)
    .bind(("0.0.0.0", server_port))?
    .run();

    log::info!("starting HTTP server at http://localhost:{}", server_port);

    server.await
}
