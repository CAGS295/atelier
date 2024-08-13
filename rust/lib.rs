// lib.rs : Final script where to place wrapped functionality

use pyo3::prelude::*;

pub mod data;
pub mod simulation;

use crate::data::data_types::{Orderbook, Level, Order, Bids, Asks};
use crate::simulation::randomizer::randomize_order;

use std::collections::VecDeque;
/* #[pyfunction]
fn midprice(orderbook: &data_types::Orderbook) ->  PyResult<f32>{
    Ok(orderbook.midprice()?)
}
*/

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


/// Definition of the exposed MODULE
/// should be the same as
/// pyproject.toml/tool.setuptools-rust.ext-modules[target]
#[pymodule]
pub fn rs_atelier(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fill_orderbook_py, m)?)?;
    // m.add_function(wrap_pyfunction!(midprice, m)?)?;
    // m.add_class::<data_types::Orderbook>()?;
    Ok(())
}
