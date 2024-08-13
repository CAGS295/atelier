
use rand::Rng;

pub fn randomize_order() -> (f64, f64, f64, bool) {
    
    let mut uni_rand = rand::thread_rng();
    
    // Randomize ts
    // let ts_inc = (uni_rand.gen::<f64>() * 1e9) as u32;
    // let ts = Instant::now() + Duration::from_nanos(ts_inc);
    let ts = 1234.1234;

    // Randomize price
    let price = (uni_rand.gen::<f64>() * 10.0) + 1.0;
    
    // Randomize amount
    let amount = (uni_rand.gen::<f64>() * 100.0) + 1.0;

    // Randomize is_buy
    let is_buy = if uni_rand.gen::<f64>() < 0.5 { true } else { false };
    
    (ts, price, amount, is_buy)
}

