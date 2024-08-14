
use rand::Rng;
use::std::time::{Instant, Duration};
use crate::structs::marketdata::order::Order;

// Create a random Order according to the provided parameters
// it uses structs::marketdata::order::Order 

pub fn randomize_order() -> Order {
    
    let mut uni_rand = rand::thread_rng();
    
    // Randomize order_ts
    let now_ts: f64 = Instant::now().elapsed().as_millis() as f64;
    let ms_offset: Duration  = Duration::from_millis(uni_rand.gen::<u64>() as u64);
    let order_ts: f64 = now_ts + (ms_offset.as_millis() as f64) / 1e9;

    // Randomize price
    let price = (uni_rand.gen::<f64>() * 10.0) + 1.0;
    
    // Randomize amount
    let amount = (uni_rand.gen::<f64>() * 100.0) + 1.0;

    // Randomize side
    let side = if uni_rand.gen::<f64>() < 0.5 { true } else { false };
    
    // Randomize order_id
    let order_id: u32 = 123;

    Order {
        order_id,
        order_ts,
        side,
        price,
        amount
    }

}

