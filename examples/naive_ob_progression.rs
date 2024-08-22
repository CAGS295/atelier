use atelier::simulation::{randomizer, generators};
use atelier::data::market::Orderbook;

fn main() {

    let bid_price = 50_000.00;
    let ask_price = 50_100.00;
    let mid_price = (bid_price + ask_price) / 2.0;
    
    let mu = 0.001;
    let sigma = 0.0050;
    let dwt = generators::pdf_standard_normal();
    let ret_gbm = randomizer::gbm_return(mid_price, mu, sigma, dwt, 1.0) as f64;
   
    println!("ret_gbm: {}", ret_gbm);

    let tick_size = 100.0;
    let n_levels = 200;
    let n_orders = 10;

    for _ in 1..3 {

    let mut n_orderbooks: Vec<Orderbook> = vec![];
    n_orderbooks.push(Orderbook::synthetize(bid_price - ret_gbm, ask_price + ret_gbm,
        tick_size, n_levels, n_orders));
    }


}
