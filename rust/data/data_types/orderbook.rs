use super::Order;
use pyo3::prelude::*;
use pyo3::types::PyString;

#[pyclass]
#[derive(Debug)]
pub struct Level {
    pub id: u32,
    pub price: f64,
    pub orders: Vec<Order>,
}

#[pyclass]
#[derive(Debug)]
pub struct Bids {
    pub levels: Vec<Level>,
}

#[pyclass]
#[derive(Debug)]
pub struct Asks {
    pub levels: Vec<Level>,
}

#[pyclass]
#[derive(Debug)]
pub struct Orderbook{
    pub id: u32,
    pub ts: f64,
    pub symbol: String,
    pub bids: Bids,
    pub asks: Asks,
}

#[pymethods]
impl Orderbook {
    

    fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        
        let class_name: Bound<'_, PyString> = slf.get_type().qualname()?;
        
        Ok(format!("{}('id':{})", class_name, slf.borrow().id))
    }

    
}

