use pyo3::prelude::*;
mod eval;
mod holder;
mod matchs;
mod nodes;
mod queries;
use holder as hld;
use queries as qry;

#[pymodule]
fn dictexprs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<qry::Expr>()?;
    m.add_class::<hld::DataJson>()?;
    m.add_class::<hld::LazyQuery>()?;
    m.add_function(wrap_pyfunction!(qry::entryfuncs::lit, m)?)?;
    m.add_function(wrap_pyfunction!(qry::entryfuncs::element, m)?)?;
    m.add_function(wrap_pyfunction!(qry::entryfuncs::struct_, m)?)?;
    m.add_function(wrap_pyfunction!(qry::entryfuncs::list, m)?)?;
    m.add_function(wrap_pyfunction!(qry::entryfuncs::field, m)?)?;
    Ok(())
}
