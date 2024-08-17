use crate::{
    simulation::randomizer::randomize_order, structs::marketdata::level::Level,
    structs::marketdata::order::Order,
};

#[derive(Debug)]
pub struct Orderbook {
    pub orderbook_id: u32,
    pub orderbook_ts: f64,
    pub symbol: String,
    pub bids: Vec<Level>,
    pub asks: Vec<Level>,
}

impl Orderbook {
    pub fn new(
        orderbook_id: u32,
        orderbook_ts: f64,
        symbol: String,
        bids: Vec<Level>,
        asks: Vec<Level>,
    ) -> Self {
        Orderbook {
            orderbook_id,
            orderbook_ts,
            symbol,
            bids,
            asks,
        }
    }

    pub fn synthetize(
        bid_price: f64,
        ask_price: f64,
        tick_size: f64,
        n_levels: u32,
        n_orders: u32,
    ) -> Self {
        let mut i_bids = Vec::new();
        let mut i_asks = Vec::new();

        for i in 1..=n_levels {
            let i_bid_price = bid_price - (&tick_size * i as f64);
            let i_bid_side = "BUY";

            let mut v_bid_orders: Vec<Order> = (0..n_orders)
                .map(|_| randomize_order(i_bid_side.to_string(), i_bid_price))
                .collect();

            v_bid_orders.sort_by_key(|order| order.order_ts);

            i_bids.push(Level {
                level_id: i,
                side: i_bid_side.to_string(),
                price: i_bid_price,
                orders: v_bid_orders,
            });

            let i_ask_price = ask_price + (&tick_size * i as f64);
            let i_ask_side = "SELL";

            let mut v_ask_orders: Vec<Order> = (0..n_orders)
                .map(|_| randomize_order(i_ask_side.to_string(), i_ask_price))
                .collect();

            v_ask_orders.sort_by_key(|order| order.order_ts);

            i_asks.push(Level {
                level_id: i,
                side: i_ask_side.to_string(),
                price: i_ask_price,
                orders: v_ask_orders,
            });
        }

        Orderbook {
            orderbook_id: 123,
            orderbook_ts: 321.0,
            symbol: String::from("BTCUSDT"),
            bids: i_bids,
            asks: i_asks,
        }
    }
}
