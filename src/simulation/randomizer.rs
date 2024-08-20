use crate::data::market::order::Order;
use rand::distributions::Uniform;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

// Create a random Order according to the provided parameters
// it uses structs::marketdata::order::Order

pub fn randomize_order(side: String, price: f64) -> Order {
    let mut uni_rand = rand::thread_rng();

    // Randomize order_ts
    let now_ts = SystemTime::now();
    // println!("now_ts: {:?}", now_ts);

    let since_epoch_ts = now_ts
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
    // println!("since_epoch_ts: {:?}", since_epoch_ts);

    // Random millis between orders ~Uniform(1,30)
    let ms_offset = uni_rand.sample(Uniform::new(1, 30));
    // println!("ms_offset: {:?}", ms_offset);

    let order_ts = since_epoch_ts as u64 + ms_offset;
    // println!("order_ts: {}", order_ts);

    // Randomize amount
    let amount = uni_rand.sample(Uniform::new(0.01, 10.0));

    // Randomize order_id
    let order_id: u32 = 123;

    Order {
        order_id,
        order_ts,
        side,
        price,
        amount,
    }
}
