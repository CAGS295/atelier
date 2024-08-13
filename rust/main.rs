
pub mod data_types;

fn main (){
    let ob = data_types::Orderbook {         
        orderbookid: 1234,
        symbol: String::from("BTCUSDT"),
        timestamp: 1234,
        bids: data_types::Bids {prices: vec![123.1], volumes: vec![321.1]},
        asks: data_types::Asks {prices: vec![321.1], volumes: vec![123.1]},
    };

    println!("Midprice is: {}", ob.midprice());
}
