// lib.rs : Final script where to place wrapped functionality

// use pyo3::prelude::*;
// pub mod structs;
// use structs::marketdata::orderbook::Orderbook;

/* #[pyfunction]
fn midprice(orderbook: &data_types::Orderbook) ->  PyResult<f32>{
    Ok(orderbook.midprice()?)
}
*/

// Definition of the exposed MODULE
// should be the same as
// pyproject.toml/tool.setuptools-rust.ext-modules[target]
// #[pymodule]
//pub fn _rs_atelier(m: &Bound<'_, PyModule>) -> PyResult<()> {
//    // m.add_function(wrap_pyfunction!(, m)?)?;
//    // m.add_function(wrap_pyfunction!(midprice, m)?)?;
//    // m.add_class::<Orderbook>()?;
//    Ok(())
//}
