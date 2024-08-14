use crate::simulation::randomizer::randomize_order;
use std::collections::VecDeque;

use pyo3::PyResult;
use pyo3::prelude::{pyclass, pymethods, Bound};
use pyo3::types::PyString;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Order {
    
    #[pyo3(get, set)]
    pub order_id: u32,
    
    #[pyo3(get, set)]
    pub order_ts: f64,
    
    #[pyo3(get, set)]
    pub side: bool,
    
    #[pyo3(get, set)]
    pub price: f64,
    
    #[pyo3(get, set)]
    pub amount: f64,
}

#[pymethods]
impl Order {

    #[new]
    fn new(order_id: u32, order_ts: f64, side:bool, price: f64, amount: f64) -> PyResult<Self>{
        Ok(Order {
            order_id,
            order_ts,
            side,
            price,
            amount,
        })
    }
}

#[pyclass]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Level {
    
    #[pyo3(get, set)]
    pub level_id: u32,
    
    #[pyo3(get, set)]
    pub side: bool,
    
    #[pyo3(get, set)]
    pub price: f64,
    
    #[pyo3(get, set)]
    pub orders: Vec<Order>,
}

#[pymethods]
impl Level {
    #[new]
    fn new(level_id: u32, side: bool, price: f64, orders: Vec<Order>) -> PyResult<Self> {
        Ok(Level {
            level_id,
            side,
            price,
            orders,
        })
    }
}

#[pyclass]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Orderbook{
    
    #[pyo3(get)]
    pub orderbook_id: u32,
    
    #[pyo3(get)]
    pub orderbook_ts: f64,
    
    #[pyo3(get)]
    pub symbol: String,
    
    #[pyo3(get)]
    pub bids: Vec<Level>,
    
    #[pyo3(get)]
    pub asks: Vec<Level>,
}

#[pymethods]

impl Orderbook{
    fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        
        let class_name: Bound<'_, PyString> = slf.get_type().qualname()?;
        
        Ok(format!("{}('orderbook_id':{})", class_name, slf.borrow().orderbook_id))
    }

}
