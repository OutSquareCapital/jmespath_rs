use crate::querybuilder as qb;
use jmespath::{Expression, Variable};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pythonize::{depythonize, pythonize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type QueryCache = Rc<RefCell<HashMap<String, Expression<'static>>>>;

fn new_error(msg: &str) -> PyErr {
    PyValueError::new_err(msg.to_string())
}

#[pyclass(unsendable, name = "DataJson")]
pub struct DataJson {
    data: Rc<Variable>,
    cache: QueryCache,
}

#[pymethods]
impl DataJson {
    #[new]
    fn new(py: Python<'_>, data: Py<PyAny>) -> PyResult<Self> {
        let var = depythonize(data.bind(py))?;
        Ok(DataJson {
            data: Rc::new(var),
            cache: Rc::new(RefCell::new(HashMap::new())),
        })
    }

    #[pyo3(signature = (query))]
    fn query(&self, query: PyRef<'_, qb::QueryBuilder>) -> PyResult<DataJson> {
        let mut cache = self.cache.borrow_mut();
        if !cache.contains_key(&query.expr) {
            let static_expr: &'static str = query.expr.clone().leak();
            let compiled = jmespath::compile(static_expr).map_err(|e| new_error(&e.to_string()))?;
            cache.insert(query.expr.clone(), compiled);
        }
        drop(cache);
        let new_data: Rc<Variable> = self
            .cache
            .borrow()
            .get(&query.expr)
            .ok_or_else(|| new_error("Erreur interne du cache"))?
            .search(self.data.as_ref())
            .map_err(|e| new_error(&e.to_string()))?;
        Ok(DataJson {
            data: new_data,
            cache: self.cache.clone(),
        })
    }

    fn collect(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(pythonize(py, self.data.as_ref())?.to_object(py))
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        let repr = self.collect(py)?.bind(py).repr()?.to_string();
        Ok(format!("DataJson({})", repr))
    }
}
