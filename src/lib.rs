use pyo3::prelude::*;

mod checks;
mod datajson;
mod eval;
mod exprs;
mod nodes;
use datajson::DataJson;
use exprs as xp;

#[pymodule]
fn jmespath_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<xp::Expr>()?;
    m.add_class::<DataJson>()?;
    m.add_function(wrap_pyfunction!(xp::field, m)?)?;
    m.add_function(wrap_pyfunction!(xp::select_list, m)?)?;
    m.add_function(wrap_pyfunction!(xp::select_dict, m)?)?;
    m.add_function(wrap_pyfunction!(xp::lit, m)?)?;
    m.add_function(wrap_pyfunction!(xp::identity, m)?)?;
    Ok(())
}
