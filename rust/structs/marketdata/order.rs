#[derive(Debug)]
pub struct Order {
    pub order_id: u32,
    pub order_ts: u64,
    pub side: String,
    pub price: f64,
    pub amount: f64,
}

impl Order {
    pub fn new(order_id: u32, order_ts: u64, side: String, price: f64, amount: f64) -> Self {
        Order {
            order_id,
            order_ts,
            side,
            price,
            amount,
        }
    }
}
