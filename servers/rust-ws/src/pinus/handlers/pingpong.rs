use crate::pinus::msg::{Msg, MsgType};
use anyhow::Result;
use serde::{Deserialize, Serialize};

use std::time::{SystemTime, UNIX_EPOCH};

fn get_epoch_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

#[derive(Serialize, Deserialize)]
pub struct Ping {
    c: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Pong {
    c: i64,
    s: i64,
}

pub async fn pingpong(_msg: Msg) -> Result<Option<Msg>> {
    // in_msg : RequestUserEnter
    // out_msg : ResponseUserEnter

    if _msg.body.is_none() {
        return Err(anyhow::anyhow!("body is none"));
    }

    let buf = _msg.body.unwrap();
    let ping: Ping = serde_json::from_slice(&buf).unwrap();

    let pong = Pong {
        c: ping.c,
        s: get_epoch_ms(),
    };

    let buf = serde_json::to_vec(&pong).unwrap();

    let ret_msg_type = if _msg.msg_type == MsgType::Request {
        MsgType::Response
    } else {
        MsgType::Push
    };

    log::debug!("ret_msg_type {:?}", ret_msg_type);

    Ok(Some(Msg {
        id: _msg.id,
        msg_type: ret_msg_type,
        route: _msg.route,
        body: Some(buf.to_vec()),
    }))
}
