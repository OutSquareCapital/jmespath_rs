use jmespath::{Expression, Variable};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use serde_json::Value;

#[pyclass(frozen, unsendable)]
struct Compiled {
    expr: Expression<'static>,
}

fn new_error(msg: &str) -> PyErr {
    PyValueError::new_err(msg.to_string())
}

#[pymethods]
impl Compiled {
    fn search(&self, py: Python<'_>, data: Py<PyAny>) -> PyResult<Py<PyAny>> {
        let any = data.bind(py);
        let val: Value = pythonize::depythonize(&any).map_err(|e| new_error(&e.to_string()))?;
        let json = serde_json::to_string(&val).map_err(|e| new_error(&e.to_string()))?;
        let var = Variable::from_json(&json).map_err(|e| new_error(&e.to_string()))?;

        let out = self
            .expr
            .search(var)
            .map_err(|e| new_error(&e.to_string()))?;

        let out_val: Value = serde_json::to_value(&out).map_err(|e| new_error(&e.to_string()))?;
        let obj = pythonize::pythonize(py, &out_val).map_err(|e| new_error(&e.to_string()))?;
        Ok(obj.into())
    }

    fn as_str(&self) -> &str {
        self.expr.as_str()
    }
}

#[pyfunction]
fn compile_expr(query: &str) -> PyResult<Compiled> {
    let expr = jmespath::compile(query).map_err(|e| new_error(&e.to_string()))?;
    Ok(Compiled { expr })
}

#[pymodule]
fn jmespath_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Compiled>()?;
    m.add_function(wrap_pyfunction!(compile_expr, m)?)?;
    Ok(())
}
