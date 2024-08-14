import atelier 
sum_as_string = atelier.sum_as_string(1,2)
print(sum_as_string)


"""


    }

    pub fn random_case(&mut self, entries: u32) {
        
        for _ in 0..entries {
    
            let (ts, price, amount, is_buy) = randomize_order();
            let order = Order {order_id: 123, order_ts, side, price, amount,};

            if is_buy {
                self.bids.levels.push(
                    Level { 
                        id: 123,
                        price,
                        orders: VecDeque::from_iter(vec![order]).into(),
                    }
                );
            } else {
                self.asks.levels.push(
                    Level {
                        id: 123,
                        price,
                        orders: VecDeque::from_iter(vec![order]).into(),
                    }
                );
            }
        }

fn fill_orderbook_py() -> PyResult<Orderbook> {
    let mut ob = Orderbook {
        id: 1234,
        ts: 1234.1234,
        symbol: String::from("BTCUSDT"),
        bids: Bids { levels: Vec::new() },
        asks: Asks { levels: Vec::new() },
    };

    for _ in 0..10 {
        
        let (ts, price, amount, is_buy) = randomize_order();
        
        let order = Order {
            id: 123,
            ts,
            price,
            amount,
            is_buy,};

        if is_buy {
            ob.bids.levels.push(
                Level { 
                    id: 123,
                    price,
                    orders: VecDeque::from_iter(vec![order]).into(),
                }
            );
        } else {
            ob.asks.levels.push(
                Level {
                    id: 123,
                    price,
                    orders: VecDeque::from_iter(vec![order]).into(),
                }
            );
        }
    }

    Ok(ob)
}

#[pyfunction]
fn fill_orderbook_py() -> PyResult<Orderbook> {
    let mut ob = Orderbook {
        id: 1234,
        ts: 1234.1234,
        symbol: String::from("BTCUSDT"),
        bids: Bids { levels: Vec::new() },
        asks: Asks { levels: Vec::new() },
    };

    for _ in 0..10 {
        
        let (ts, price, amount, is_buy) = randomize_order();
        
        let order = Order {
            id: 123,
            ts,
            price,
            amount,
            is_buy,};

        if is_buy {
            ob.bids.levels.push(
                Level { 
                    id: 123,
                    price,
                    orders: VecDeque::from_iter(vec![order]).into(),
                }
            );
        } else {
            ob.asks.levels.push(
                Level {
                    id: 123,
                    price,
                    orders: VecDeque::from_iter(vec![order]).into(),
                }
            );
        }
    }

    Ok(ob)
}


"""
