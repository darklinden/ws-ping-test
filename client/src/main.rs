use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::env;
use std::{collections::HashMap, env::args};
use tungstenite::{connect, Message};
use url::Url;

mod utils;
use utils::config::{get_config_str, ConfigKeys};

mod pinus;
use pinus::pkg::PkgType;

use crate::pinus::{
    msg::{Msg, MsgType, Route},
    pkg::{Pkg, PkgBody},
};

use std::time::{SystemTime, UNIX_EPOCH};

fn get_epoch_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

#[derive(Serialize, Deserialize)]
pub struct Sys {
    heartbeat: u16,
    dict: HashMap<String, u16>,
}

#[derive(Serialize, Deserialize)]
pub struct Handshake {
    code: u16,
    sys: Sys,
}

#[derive(Serialize, Deserialize)]
pub struct Pingpong {
    c: i64,
    s: i64,
}

fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let arg_name = args().nth(1).unwrap_or_else(|| "pingpong".to_string());

    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    env::set_var("RUST_LOG", &log_level);
    println!("{} Log Level: {:?}", arg_name, &log_level);
    env_logger::init();

    if log_level == "debug" {
        debug!("{} Debug mode", arg_name);
        env::set_var("RUST_BACKTRACE", "1");
    }

    let server_url = get_config_str(ConfigKeys::WS_URL);

    let (mut socket, _response) = connect(Url::parse(&server_url).unwrap()).expect("Can't connect");

    debug!("{} Connected to the server", arg_name);

    // send handshake
    let handshake = r#"{"sys":{"type":"ws","version":"0.0.1","rsa":{}},"user":{}}"#.to_string();
    let pkg = Pkg {
        pkg_type: PkgType::Handshake,
        content: PkgBody::StrMsg(handshake),
    };
    let wsm = pkg.encode().expect("Error encoding handshake pkg");
    socket.send(Message::Binary(wsm)).unwrap();

    let route_str = "connector.entryHandler.pingpong";
    let mut route: u16 = 0;

    let msg = socket.read().expect("Error reading message");
    if msg.is_binary() {
        let pkg = Pkg::decode(&msg.into_data()).expect("Error decoding handshake pkg");
        for p in pkg.iter() {
            match p.content {
                PkgBody::StrMsg(ref s) => {
                    let handshake: Handshake = serde_json::from_str(s).unwrap();
                    handshake.sys.dict.get(route_str).map(|r| route = *r);
                    info!("{} Handshake get route: {:?}", arg_name, route);
                }
                _ => {
                    error!("{} Handshake: unexpected pkg type", arg_name);
                }
            }
        }
    } else {
        info!("{} Handshake: {}", arg_name, msg);
    }

    // send handshake ack
    let pkg = Pkg {
        pkg_type: PkgType::HandshakeAck,
        content: PkgBody::None,
    };
    let wsm = pkg.encode().expect("Error encoding handshake ack pkg");
    socket.send(Message::Binary(wsm)).unwrap();

    // wait 1 second
    info!("{} Waiting 1 second", arg_name);
    std::thread::sleep(std::time::Duration::from_secs(1));

    // send pingpong
    info!("{} Send Pingpong", arg_name);
    let start = get_epoch_ms();
    let send_data = format!(r#"{{"c": {}}}"#, start);
    let pkg = Pkg {
        pkg_type: PkgType::Data,
        content: PkgBody::Msg(Msg {
            id: 1,
            msg_type: MsgType::Request,
            route: Route {
                code: Some(route),
                name: None,
            },
            body: Some(send_data.as_bytes().to_vec()),
        }),
    };
    let wsm = pkg.encode().expect("Error encoding pingpong pkg");

    debug!("{} pingpong message Sending", arg_name);
    socket.send(Message::Binary(wsm)).unwrap();
    debug!("{} pingpong message Sent", arg_name);

    'l: loop {
        let msg = socket.read().expect("Error reading pingpong message");

        if msg.is_binary() {
            let pkg = Pkg::decode(&msg.into_data()).expect("Error decoding pingpong pkg");
            for p in pkg.iter() {
                match p.content {
                    PkgBody::Msg(ref msg) => {
                        let body = msg.body.as_ref().unwrap();
                        let recv_data = String::from_utf8(body.to_vec()).unwrap();

                        let pingpong: Pingpong = serde_json::from_str(&recv_data).unwrap();
                        let end = get_epoch_ms();
                        info!(
                            "{} Pingpong: {} cost {} server time: {}",
                            arg_name,
                            recv_data,
                            end - pingpong.c,
                            pingpong.s
                        );
                        break 'l;
                    }
                    _ => {
                        info!("{} recv: unexpected pkg type {}", arg_name, p.pkg_type);
                    }
                }
            }
        } else {
            info!("{} Pingpong: {}", arg_name, msg);
        }
    }

    let _ = socket.close(None).expect("Error closing connection");

    Ok(())
}
