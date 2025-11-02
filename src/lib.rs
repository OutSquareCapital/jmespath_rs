use pyo3::prelude::*;

mod checks;
mod eval;
mod exprs;
mod lists;
mod nodes;
mod strings;
mod structs;
use exprs as xp;

#[pymodule]
fn dictexprs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<xp::Expr>()?;
    m.add_function(wrap_pyfunction!(xp::key, m)?)?;
    m.add_function(wrap_pyfunction!(xp::select_list, m)?)?;
    m.add_function(wrap_pyfunction!(xp::select_dict, m)?)?;
    m.add_function(wrap_pyfunction!(xp::lit, m)?)?;
    m.add_function(wrap_pyfunction!(xp::element, m)?)?;
    m.add_function(wrap_pyfunction!(xp::merge, m)?)?;
    m.add_function(wrap_pyfunction!(xp::not_null, m)?)?;
    Ok(())
}
