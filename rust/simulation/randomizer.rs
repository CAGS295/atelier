
use rand::Rng;
use::std::time::{Instant, Duration};

pub fn randomize_order() -> (f64, f64, f64, bool) {
    
    let mut uni_rand = rand::thread_rng();
    
    // Randomize ts
    let now_ts: f64 = Instant::now().elapsed().as_millis() as f64; // .as_secs_f64();
    let ms_offset: Duration  = Duration::from_millis(uni_rand.gen::<u64>() as u64);
    let ts: f64 = now_ts + (ms_offset.as_millis() as f64) / 1e9;

    // Randomize price
    let price = (uni_rand.gen::<f64>() * 10.0) + 1.0;
    
    // Randomize amount
    let amount = (uni_rand.gen::<f64>() * 100.0) + 1.0;

    // Randomize is_buy
    let side = if uni_rand.gen::<f64>() < 0.5 { true } else { false };
    
    (ts, price, amount, side)

}

