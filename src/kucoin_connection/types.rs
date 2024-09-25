use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
/// A message from the KuCoin futures websocket API
pub enum KuCoinMessage {
    Welcome {},

    Ping {
        id: String,
    },
    Ack {
        id: String,
    },
    Message {
        topic: String,
        data: OrderBookMessage,
    },

    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
/// https://www.kucoin.com/docs/websocket/futures-trading/public-channels/level2-market-data
pub struct OrderBookMessage {
    #[serde(deserialize_with = "deserialize_change_tuple")]
    pub change: (f64, String, u64),
    pub sequence: u64,
    pub timestamp: u64,
}

fn deserialize_change_tuple<'de, D>(deserializer: D) -> Result<(f64, String, u64), D::Error>
where
    D: Deserializer<'de>,
{
    let change = String::deserialize(deserializer)?;
    let parts = change.split(',').collect::<Vec<&str>>();
    Ok((
        parts[0].parse().unwrap(),
        parts[1].to_string(),
        parts[2].parse().unwrap(),
    ))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
/// https://www.kucoin.com/docs/websocket/basic-info/apply-connect-token/public-token-no-authentication-required-
pub struct PublicTokenResponse {
    code: String,
    pub data: PublicTokenData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicTokenData {
    pub token: String,
    pub instance_servers: Vec<InstanceServer>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceServer {
    pub endpoint: String,
    encrypt: bool,
    pub protocol: String,
    ping_interval: u32,
    ping_timeout: u32,
}
