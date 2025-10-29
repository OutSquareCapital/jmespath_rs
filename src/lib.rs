use jmespath::Variable;
use pyo3::prelude::*;
use std::rc::Rc;
mod conversions;
mod querybuilder;
use crate::conversions::{new_error, py_to_variable, variable_to_py};
use crate::querybuilder as qb;

#[pyclass(unsendable, name = "DataJson")]
struct DataJson {
    data: Rc<Variable>,
}

#[pymethods]
impl DataJson {
    #[new]
    fn new(py: Python<'_>, data: Py<PyAny>) -> PyResult<Self> {
        let var = py_to_variable(py, data.bind(py))?;
        Ok(DataJson { data: Rc::new(var) })
    }

    #[pyo3(signature = (query))]
    fn query(&self, query: PyRef<'_, qb::QueryBuilder>) -> PyResult<DataJson> {
        let compiled = jmespath::compile(&query.expr).map_err(|e| new_error(&e.to_string()))?;
        let new_data: Rc<Variable> = compiled
            .search(self.data.as_ref())
            .map_err(|e| new_error(&e.to_string()))?;
        Ok(DataJson { data: new_data })
    }

    fn collect(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        variable_to_py(py, self.data.as_ref())
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        let py_val = self.collect(py)?;
        let py_val_bound = py_val.bind(py);
        let repr = py_val_bound.repr()?.to_string();
        Ok(format!("DataJson({})", repr))
    }
}
#[pymodule]
fn jmespath_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<DataJson>()?;
    m.add_class::<qb::QueryBuilder>()?;
    m.add_function(wrap_pyfunction!(qb::field, m)?)?;
    m.add_function(wrap_pyfunction!(qb::select_list, m)?)?;
    m.add_function(wrap_pyfunction!(qb::select_dict, m)?)?;
    m.add_function(wrap_pyfunction!(qb::lit, m)?)?;

    Ok(())
}
