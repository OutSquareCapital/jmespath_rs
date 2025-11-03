use pyo3::prelude::*;

mod checks;
mod eval;
mod exprs;
mod lists;
mod matchs;
mod nodes;
mod strings;
mod structs;
use exprs as xp;

#[pymodule]
fn dictexprs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<xp::Expr>()?;
    m.add_function(wrap_pyfunction!(xp::lit, m)?)?;
    m.add_function(wrap_pyfunction!(xp::element, m)?)?;
    m.add_function(wrap_pyfunction!(xp::merge, m)?)?;
    m.add_function(wrap_pyfunction!(xp::not_null, m)?)?;
    m.add_function(wrap_pyfunction!(xp::struct_, m)?)?;
    m.add_function(wrap_pyfunction!(xp::list, m)?)?;
    m.add_function(wrap_pyfunction!(xp::field, m)?)?;
    Ok(())
}
