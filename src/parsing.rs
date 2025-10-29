use crate::querybuilder::QueryBuilder;
use pyo3::prelude::*;
use pyo3::types::PyDict;

pub const KWORD_CURRENT: &str = "@";
pub const KWORD_DOT: &str = ".";
pub const KWORD_ARRAY_PROJECT: &str = "[*]";
pub const KWORD_OBJECT_PROJECT: &str = "*";
pub const KWORD_FLATTEN: &str = "[]";

pub fn clean_rhs_expr_for_by_func(expr_str: &str) -> &str {
    let cleaned = expr_str.strip_prefix(KWORD_CURRENT).unwrap_or(expr_str);
    cleaned.strip_prefix(KWORD_DOT).unwrap_or(cleaned)
}

pub fn obj_to_jmespath_literal_string(py: Python<'_>, ob: &Bound<'_, PyAny>) -> PyResult<String> {
    let default = py.import_bound("builtins")?.getattr("str")?;
    let kwargs = PyDict::new_bound(py);
    kwargs.set_item("default", default)?;

    let s = py
        .import_bound("json")?
        .call_method("dumps", (ob,), Some(&kwargs))?
        .extract::<String>()?;
    Ok(format!("`{}`", s))
}

pub fn obj_to_jmespath_string(py: Python<'_>, ob: Py<PyAny>) -> PyResult<String> {
    let ob_bound = ob.bind(py);
    if let Ok(q) = ob_bound.extract::<PyRef<QueryBuilder>>() {
        return Ok(q.expr.clone());
    }
    if let Ok(s) = ob_bound.extract::<String>() {
        return Ok(s);
    }
    obj_to_jmespath_literal_string(py, ob_bound)
}

pub fn ensure_leading_dot(text: &str) -> String {
    if text.starts_with(KWORD_DOT) || text.starts_with('[') || text.is_empty() {
        text.to_string()
    } else {
        format!("{}{}", KWORD_DOT, text)
    }
}
