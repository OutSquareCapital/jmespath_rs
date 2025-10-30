use crate::eval;
use crate::exprs::Expr;
use pyo3::prelude::*;
#[pyclass(module = "jmespath_rs", name = "DataJson")]
pub struct DataJson {
    data: PyObject,
}

#[pymethods]
impl DataJson {
    #[new]
    pub fn new(obj: PyObject) -> Self {
        Self { data: obj }
    }

    pub fn collect(&self, py: Python<'_>, q: &Expr) -> PyResult<PyObject> {
        let root = self.data.bind(py);
        eval::eval_any(py, &q.node, root).map(|result| result.unbind())
    }

    pub fn query(&self, py: Python<'_>, q: &Expr) -> PyResult<Self> {
        let result = self.collect(py, q)?;
        Ok(Self::new(result))
    }
}
