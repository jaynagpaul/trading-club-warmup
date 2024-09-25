mod kucoin_connection;
mod orderbook;

use kucoin_connection::{KuCoinConnection, KuCoinMessage, OrderBookMessage};
use orderbook::OrderBook;

const SYMBOL: &str = "ETHUSDTM";
const DEPTH: usize = 5;

fn main() {
    let mut connection = KuCoinConnection::request_and_create();

    connection.subscribe_to_level2_orderbook(SYMBOL).unwrap();

    let mut orderbook = OrderBook::with_depth(DEPTH);

    loop {
        let msg = connection.read().unwrap();
        if let KuCoinMessage::Message { data, .. } = msg {
            update_orderbook(&mut orderbook, data);
            // clear terminal
            println!("\x1B[2J\x1B[H");
            println!("{}", orderbook);
        }
    }
}

fn update_orderbook(orderbook: &mut OrderBook, msg: OrderBookMessage) {
    let (price, change_type, amount) = msg.change;

    match change_type.as_str() {
        "buy" => orderbook.add_bid(price, amount),
        "sell" => orderbook.add_ask(price, amount),
        unknown => println!("unknown change type: {}", unknown),
    }
}
