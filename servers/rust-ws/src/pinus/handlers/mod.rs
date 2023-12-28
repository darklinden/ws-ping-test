use super::super::pinus::msg::Msg;
use anyhow::Result;

mod pingpong;

#[allow(dead_code)]
pub const ROUTE_LIST: &'static [&'static str] = &["no-route", "connector.entryHandler.pingpong"];

#[allow(dead_code)]
pub async fn handle_data_msg(msg: Msg) -> Option<Msg> {
    let route_str = msg.route.to_owned().name.unwrap();
    let route = route_str.as_str();
    let result = match route {
        "connector.entryHandler.pingpong" => pingpong::pingpong(msg).await, // RequestUserEnter ResponseUserEnter
        _ => Result::Err(anyhow::anyhow!("route not found")),
    };

    if result.is_err() {
        log::error!("handle_data_msg error: {:?}", result.err());
        return None;
    }

    let result = result.unwrap();
    return result;
}
