use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

/// A simple orderbook implementation
/// Keeps track of the top `depth` asks and bids
pub struct OrderBook {
    asks: Vec<(f64, u64)>,
    bids: Vec<(f64, u64)>,

    depth: usize,
}

impl OrderBook {
    /// Create a new orderbook with a given depth
    pub fn with_depth(depth: usize) -> Self {
        OrderBook {
            asks: Vec::with_capacity(depth),
            bids: Vec::with_capacity(depth),
            depth,
        }
    }

    /// Add a bid to the orderbook
    pub fn add_bid(&mut self, price: f64, amount: u64) {
        self.add_order(price, amount, Side::Bid);
    }

    /// Add an ask to the orderbook
    pub fn add_ask(&mut self, price: f64, amount: u64) {
        self.add_order(price, amount, Side::Ask);
    }

    fn add_order(&mut self, price: f64, amount: u64, side: Side) {
        let orders = match side {
            Side::Ask => &mut self.asks,
            Side::Bid => &mut self.bids,
        };

        if amount == 0 {
            orders.retain(|(p, _)| *p != price);
            return;
        }

        for (p, a) in orders.iter_mut() {
            if *p == price {
                *a = amount;
                return;
            }
        }

        let compare = match side {
            Side::Ask => |a: f64, b: f64| a.partial_cmp(&b).unwrap(),
            Side::Bid => |a: f64, b: f64| b.partial_cmp(&a).unwrap(),
        };

        if orders.len() < self.depth {
            orders.push((price, amount));
            orders.sort_by(|a, b| compare(a.0, b.0));
        } else if compare(price, orders[self.depth - 1].0) == Ordering::Less {
            orders.pop();
            orders.push((price, amount));
            orders.sort_by(|a, b| compare(a.0, b.0));
        }
    }
}

impl Display for OrderBook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Price    Amount")?;
        writeln!(f, "Bids")?;
        for (price, amount) in &self.bids {
            writeln!(f, "{:8.2} {:8}", price, amount)?;
        }

        writeln!(f, "\n   -----    ------\n")?;

        for (price, amount) in &self.asks {
            writeln!(f, "{:8.2} {:8}", price, amount)?;
        }
        writeln!(f, "Asks")?;

        Ok(())
    }
}

enum Side {
    Ask,
    Bid,
}
