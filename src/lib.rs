use pyo3::prelude::*;

mod eval;
mod matchs;
mod nodes;
mod queries;
use queries as qry;

#[pymodule]
fn dictexprs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<qry::Expr>()?;
    m.add_function(wrap_pyfunction!(qry::entryfuncs::lit, m)?)?;
    m.add_function(wrap_pyfunction!(qry::entryfuncs::element, m)?)?;
    m.add_function(wrap_pyfunction!(qry::entryfuncs::merge, m)?)?;
    m.add_function(wrap_pyfunction!(qry::entryfuncs::coalesce, m)?)?;
    m.add_function(wrap_pyfunction!(qry::entryfuncs::struct_, m)?)?;
    m.add_function(wrap_pyfunction!(qry::entryfuncs::list, m)?)?;
    m.add_function(wrap_pyfunction!(qry::entryfuncs::field, m)?)?;
    Ok(())
}
