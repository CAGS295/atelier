
use pyo3::pyclass;

#[derive(Debug)]
#[pyclass]
pub struct Order {
    pub id: u32,
    pub ts: f64,
    pub price: f64,
    pub amount: f64,
    pub is_buy: bool,
}

