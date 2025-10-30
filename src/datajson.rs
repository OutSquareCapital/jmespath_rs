use crate::eval;
use crate::querybuilder as qb;
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

    pub fn search(&self, q: &qb::QueryBuilder) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            let root = self.data.bind(py);
            Ok(eval::eval_any(py, &q.node, root.clone())?.unbind().into())
        })
    }
}
