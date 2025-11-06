use crate::matchs::match_any;
use crate::nodes;
use crate::queries::Expr;
use pyo3::prelude::*;
use pyo3::types::PyType;
use std::sync::Arc;

#[pyclass(module = "dictexprs", name = "LazyQuery")]
pub struct LazyQuery {
    data: Arc<nodes::Value>,
    node: nodes::Node,
}

#[pymethods]
impl LazyQuery {
    pub fn collect(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(match_any(&self.node, &self.data)
            .map_err(pyo3::exceptions::PyRuntimeError::new_err)?
            .to_python(py)?
            .unbind())
    }
}

#[pyclass(module = "dictexprs", name = "DataJson")]
pub struct DataJson {
    data: Arc<nodes::Value>,
}

#[pymethods]
impl DataJson {
    #[new]
    pub fn new(data: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(DataJson {
            data: Arc::new(nodes::Value::from_python(data)?),
        })
    }
    #[classmethod]
    #[pyo3(name = "from_bytes")]
    pub fn from_bytes(_cls: &Bound<'_, PyType>, bytes_data: &[u8]) -> PyResult<Self> {
        Ok(DataJson {
            data: Arc::new(nodes::Value::from_serde_value(
                serde_json::from_slice(bytes_data)
                    .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            )?),
        })
    }
    pub fn query(&self, _py: Python<'_>, expr: &Expr) -> PyResult<LazyQuery> {
        Ok(LazyQuery {
            data: Arc::clone(&self.data),
            node: expr.node.clone(),
        })
    }
}
