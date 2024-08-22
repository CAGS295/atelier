use crate::simulation::randomizer::randomize_order;

#[derive(Debug, Copy, Clone)]
pub enum Side {
    Bids,
    Asks,
}

// ------------------------------------------------------------------------- //

#[derive(Debug, Copy, Clone)]
pub enum OrderType {
    Market,
    Limit,
}

#[derive(Debug, Copy, Clone)]
pub struct Order {
    pub order_id: u32,
    pub order_ts: u64,
    pub order_type: OrderType,
    pub side: Side,
    pub price: f64,
    pub amount: f64,
}

impl Order {
    pub fn new(
        order_id: u32,
        order_ts: u64,
        order_type: OrderType,
        side: Side,
        price: f64,
        amount: f64,
    ) -> Self {
        match side {
            Side::Bids => Order {
                order_id,
                order_ts,
                order_type,
                side: Side::Bids,
                price,
                amount,
            },
            Side::Asks => Order {
                order_id,
                order_ts,
                order_type,
                side: Side::Asks,
                price,
                amount,
            },
        };

        Order {
            order_id,
            order_ts,
            order_type,
            side,
            price,
            amount,
        }
    }
}

// ------------------------------------------------------------------------- //

#[derive(Debug)]
pub struct Level {
    pub level_id: u32,
    pub side: Side,
    pub price: f64,
    pub orders: Vec<Order>,
}

impl Level {
    pub fn new(level_id: u32, side: Side, price: f64, orders: Vec<Order>) -> Self {
        match side {
            Side::Bids => Level {
                level_id,
                side: Side::Bids,
                price,
                orders: orders.clone(),
            },
            Side::Asks => Level {
                level_id,
                side: Side::Asks,
                price,
                orders: orders.clone(),
            },
        };

        Level {
            level_id,
            side,
            price,
            orders,
        }
    }
}

// ------------------------------------------------------------------------- //

#[derive(Debug)]
pub struct Orderbook {
    pub orderbook_id: u32,
    pub orderbook_ts: f64,
    pub symbol: String,
    pub bids: Vec<Level>,
    pub asks: Vec<Level>,
}

impl Orderbook {
    // Create a new abstraction of Orderbook
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
            let i_bid_side = Side::Bids;
            let i_order_type = OrderType::Limit;

            let mut v_bid_orders: Vec<Order> = (0..n_orders)
                .map(|_| randomize_order(i_bid_side, i_bid_price, i_order_type))
                .collect();

            v_bid_orders.sort_by_key(|order| order.order_ts);

            i_bids.push(Level {
                level_id: i,
                side: i_bid_side,
                price: i_bid_price,
                orders: v_bid_orders,
            });

            let i_ask_price = ask_price + (&tick_size * i as f64);
            let i_ask_side = Side::Asks;

            let mut v_ask_orders: Vec<Order> = (0..n_orders)
                .map(|_| randomize_order(i_ask_side, i_ask_price, i_order_type))
                .collect();

            v_ask_orders.sort_by_key(|order| order.order_ts);

            i_asks.push(Level {
                level_id: i,
                side: i_ask_side,
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
