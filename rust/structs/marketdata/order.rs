
#[derive(Debug)]
pub struct Order {
    
    pub order_id: u32,
    pub order_ts: f64,
    pub side: bool,
    pub price: f64,
    pub amount: f64,
}

impl Order {

    pub fn new(order_id: u32, order_ts:f64, side: bool, price:f64, amount:f64) -> Self {
        Order {
            order_id,
            order_ts,
            side,
            price,
            amount,
        }
    }

}
