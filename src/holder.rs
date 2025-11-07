use crate::matchs::match_any;
use crate::nodes;
use crate::queries::Expr;
use pyo3::prelude::*;
use pyo3::types::PyType;
use pythonize::{depythonize, pythonize};
use serde_json as sd;
use std::sync::Arc;

#[pyclass(module = "dictexprs", name = "LazyQuery")]
pub struct LazyQuery {
    data: Arc<sd::Value>,
    node: nodes::Node,
}

#[pymethods]
impl LazyQuery {
    pub fn collect(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let v = match_any(&self.node, &self.data);
        Ok(pythonize(py, &v).unwrap().into_any().unbind())
    }
    pub fn to_bytes(&self) -> PyResult<Vec<u8>> {
        let v = match_any(&self.node, &self.data);
        Ok(sd::to_vec(&v).unwrap())
    }
}

#[pyclass(module = "dictexprs", name = "DataJson")]
pub struct DataJson {
    data: Arc<sd::Value>,
}

#[pymethods]
impl DataJson {
    #[new]
    pub fn new(data: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(DataJson {
            data: Arc::new(depythonize(data)?),
        })
    }
    #[classmethod]
    #[pyo3(name = "from_bytes")]
    pub fn from_bytes(_cls: &Bound<'_, PyType>, bytes_data: &[u8]) -> PyResult<Self> {
        Ok(DataJson {
            data: Arc::new(
                sd::from_slice(bytes_data)
                    .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            ),
        })
    }
    pub fn query(&self, _py: Python<'_>, expr: &Expr) -> PyResult<LazyQuery> {
        Ok(LazyQuery {
            data: Arc::clone(&self.data),
            node: expr.node.clone(),
        })
    }
}
