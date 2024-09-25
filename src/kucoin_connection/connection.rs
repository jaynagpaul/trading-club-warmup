use super::error::Error;
use super::types::*;

use std::net::TcpStream;

use serde_json::json;

const PUBLIC_TOKEN_URL: &str = "https://api-futures.kucoin.com/api/v1/bullet-public";

type Websocket = tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<TcpStream>>;

/// A connection to the KuCoin futures websocket API
pub struct KuCoinConnection {
    connection: Websocket,
}

impl KuCoinConnection {
    /// Request a public token and create a connection
    pub fn request_and_create() -> Self {
        let client = reqwest::blocking::Client::new();

        let public_token = get_public_token(&client).unwrap();

        let token = public_token.data.token;

        let instance_server = public_token
            .data
            .instance_servers
            .into_iter()
            .find(|server| server.protocol == "websocket")
            .unwrap();

        let ws_url = format!("{}?token={}", instance_server.endpoint, token);

        let (ws, _) = tungstenite::connect(ws_url).unwrap();

        let mut connection = KuCoinConnection { connection: ws };
        match connection.read().unwrap() {
            KuCoinMessage::Welcome {} => connection,
            _ => panic!("Expected welcome message"),
        }
    }

    /// Subscribe to the level2 orderbook websocket topic for a symbol
    pub fn subscribe_to_level2_orderbook(&mut self, symbol: &str) -> Result<(), Error> {
        let msg = json!({
            "id": 1545910660740_i64,
            "type": "subscribe",
            "topic": format!("/contractMarket/level2:{}", symbol),
            "response": true
        });

        self.write(msg.to_string())?;
        match self.read().unwrap() {
            KuCoinMessage::Ack { .. } => Ok(()),
            _ => panic!("Expected ack message"),
        }
    }

    /// Read a message from the websocket
    pub fn read(&mut self) -> Result<KuCoinMessage, Error> {
        let msg = self.connection.read().unwrap();
        let message: KuCoinMessage = serde_json::from_str(&msg.to_string())?;

        Ok(message)
    }

    /// Write a message to the websocket
    fn write(&mut self, msg: String) -> Result<(), Error> {
        self.connection.write(tungstenite::Message::text(msg))?;
        self.connection.flush()?;
        Ok(())
    }
}

fn get_public_token(
    client: &reqwest::blocking::Client,
) -> Result<PublicTokenResponse, reqwest::Error> {
    let response = client.post(PUBLIC_TOKEN_URL).send()?;
    let json: PublicTokenResponse = response.json()?;

    Ok(json)
}
