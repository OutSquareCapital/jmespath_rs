use crate::matchs::match_any;
use crate::nodes;
use crate::queries::Expr;
use pyo3::prelude::*;

#[pyclass(module = "dictexprs", name = "LazyQuery")]
pub struct LazyQuery {
    data: PyObject,
    node: nodes::Node,
}

#[pymethods]
impl LazyQuery {
    pub fn collect(&self, py: Python<'_>) -> PyResult<PyObject> {
        match_any(py, &self.node, self.data.bind(py)).map(|result| result.unbind())
    }
}

#[pyclass(module = "dictexprs", name = "DataJson")]
pub struct DataJson {
    data: PyObject,
}

#[pymethods]
impl DataJson {
    #[new]
    pub fn new(data: &Bound<'_, PyAny>) -> Self {
        DataJson {
            data: data.to_object(data.py()),
        }
    }
    pub fn query(&self, py: Python<'_>, expr: &Expr) -> PyResult<LazyQuery> {
        Ok(LazyQuery {
            data: self.data.clone_ref(py),
            node: expr.node.clone(),
        })
    }
}
