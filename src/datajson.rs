use crate::conversions::{new_error, py_to_variable, variable_to_py};
use crate::querybuilder as qb;
use jmespath::{Expression, Variable}; // Ajout de Expression
use pyo3::prelude::*;
use std::cell::RefCell; // Ajout de RefCell
use std::collections::HashMap; // Ajout de HashMap
use std::rc::Rc;

type QueryCache = Rc<RefCell<HashMap<String, Expression<'static>>>>;

#[pyclass(unsendable, name = "DataJson")]
pub struct DataJson {
    data: Rc<Variable>,
    cache: QueryCache,
}

#[pymethods]
impl DataJson {
    #[new]
    fn new(py: Python<'_>, data: Py<PyAny>) -> PyResult<Self> {
        let var = py_to_variable(py, data.bind(py))?;
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
        variable_to_py(py, self.data.as_ref())
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        let repr = self.collect(py)?.bind(py).repr()?.to_string();
        Ok(format!("DataJson({})", repr))
    }
}
