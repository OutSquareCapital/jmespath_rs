use pyo3::prelude::*;

mod datajson;
mod eval;
mod nodes;
mod querybuilder;
mod util;
use datajson::DataJson;
use querybuilder as qb;

#[pymodule]
fn jmespath_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<qb::QueryBuilder>()?;
    m.add_class::<DataJson>()?;
    m.add_function(wrap_pyfunction!(qb::field, m)?)?;
    m.add_function(wrap_pyfunction!(qb::select_list, m)?)?;
    m.add_function(wrap_pyfunction!(qb::select_dict, m)?)?;
    m.add_function(wrap_pyfunction!(qb::lit, m)?)?;
    m.add_function(wrap_pyfunction!(qb::identity, m)?)?;
    Ok(())
}
