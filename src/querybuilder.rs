use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

const KWORD_CURRENT: &str = "@";
const KWORD_DOT: &str = ".";
const KWORD_ARRAY_PROJECT: &str = "[*]";
const KWORD_OBJECT_PROJECT: &str = "*";
const KWORD_FLATTEN: &str = "[]";

fn clean_rhs_expr_for_by_func(expr_str: &str) -> &str {
    let cleaned = expr_str.strip_prefix(KWORD_CURRENT).unwrap_or(expr_str);
    cleaned.strip_prefix(KWORD_DOT).unwrap_or(cleaned)
}

fn obj_to_jmespath_literal_string(py: Python<'_>, ob: &Bound<'_, PyAny>) -> PyResult<String> {
    let default = py.import_bound("builtins")?.getattr("str")?;
    let kwargs = PyDict::new_bound(py);
    kwargs.set_item("default", default)?;

    let s = py
        .import_bound("json")?
        .call_method("dumps", (ob,), Some(&kwargs))?
        .extract::<String>()?;
    Ok(format!("`{}`", s))
}

fn obj_to_jmespath_string(py: Python<'_>, ob: Py<PyAny>) -> PyResult<String> {
    let ob_bound = ob.bind(py);
    if let Ok(q) = ob_bound.extract::<PyRef<QueryBuilder>>() {
        return Ok(q.expr.clone());
    }
    if let Ok(s) = ob_bound.extract::<String>() {
        return Ok(s);
    }
    obj_to_jmespath_literal_string(py, ob_bound)
}

fn ensure_leading_dot(text: &str) -> String {
    if text.starts_with(KWORD_DOT) || text.starts_with('[') || text.is_empty() {
        text.to_string()
    } else {
        format!("{}{}", KWORD_DOT, text)
    }
}

#[pyclass(frozen, unsendable, name = "QueryBuilder")]
#[derive(Clone)]
pub struct QueryBuilder {
    pub expr: String,
}

impl QueryBuilder {
    fn new_expr(&self, expr: String) -> Self {
        Self { expr }
    }

    fn binary_op(&self, py: Python<'_>, other: Py<PyAny>, op: &str) -> PyResult<Self> {
        let right_expr = obj_to_jmespath_string(py, other)?;
        let s = format!("({}) {} ({})", self.expr, op, right_expr);
        Ok(self.new_expr(s))
    }
    fn by_func(&self, py: Python<'_>, name: &str, rhs: Py<PyAny>) -> PyResult<Self> {
        let rhs_expr = obj_to_jmespath_string(py, rhs)?;
        let rhs_cleaned = clean_rhs_expr_for_by_func(&rhs_expr);
        let s = format!("{}({}, &{})", name, self.expr, rhs_cleaned);
        Ok(self.new_expr(s))
    }
}

#[pymethods]
impl QueryBuilder {
    #[new]
    fn new() -> Self {
        Self {
            expr: KWORD_CURRENT.to_string(),
        }
    }

    #[pyo3(signature = (start = None, end = None, step = None))]
    fn slice(&self, start: Option<i64>, end: Option<i64>, step: Option<i64>) -> Self {
        let start_s = start.map_or("".to_string(), |s| s.to_string());
        let end_s = end.map_or("".to_string(), |e| e.to_string());

        let slice_s = if let Some(step_val) = step {
            format!("{}:{}:{}", start_s, end_s, step_val)
        } else {
            format!("{}:{}", start_s, end_s)
        };
        self.new_expr(format!("{}[{}]", self.expr, slice_s))
    }

    #[pyo3(name = "field")]
    fn field_(&self, name: String) -> Self {
        self.new_expr(format!("{}{}{}", self.expr, KWORD_DOT, name))
    }
    fn __getattr__(&self, name: String) -> Self {
        self.field_(name)
    }
    fn index(&self, i: i64) -> Self {
        self.new_expr(format!("{}[{}]", self.expr, i))
    }
    fn project(&self, py: Python<'_>, rhs: Py<PyAny>) -> PyResult<Self> {
        let rhs_expr = obj_to_jmespath_string(py, rhs)?;
        let rhs_dotted = ensure_leading_dot(&rhs_expr);
        let s = format!("{}{}{}", self.expr, KWORD_ARRAY_PROJECT, rhs_dotted);
        Ok(self.new_expr(s))
    }
    fn vproject(&self, py: Python<'_>, rhs: Py<PyAny>) -> PyResult<Self> {
        let rhs_expr = obj_to_jmespath_string(py, rhs)?;
        let rhs_dotted = ensure_leading_dot(&rhs_expr);
        let s = format!("{}{}{}", self.expr, KWORD_OBJECT_PROJECT, rhs_dotted);
        Ok(self.new_expr(s))
    }
    fn flatten(&self) -> Self {
        self.new_expr(format!("{}{}", self.expr, KWORD_FLATTEN))
    }

    fn filter(&self, py: Python<'_>, cond: Py<PyAny>, then: Py<PyAny>) -> PyResult<Self> {
        let cond_expr = obj_to_jmespath_string(py, cond)?;
        let then_expr = obj_to_jmespath_string(py, then)?;
        let then_dotted = ensure_leading_dot(&then_expr);
        let cond_cleaned = cond_expr.strip_prefix(KWORD_CURRENT).unwrap_or(&cond_expr);

        let s = format!("{}[?{}]{}", self.expr, cond_cleaned, then_dotted);
        Ok(self.new_expr(s))
    }

    fn eq(&self, py: Python<'_>, other: Py<PyAny>) -> PyResult<Self> {
        self.binary_op(py, other, "==")
    }
    fn ne(&self, py: Python<'_>, other: Py<PyAny>) -> PyResult<Self> {
        self.binary_op(py, other, "!=")
    }
    fn gt(&self, py: Python<'_>, other: Py<PyAny>) -> PyResult<Self> {
        self.binary_op(py, other, ">")
    }
    fn ge(&self, py: Python<'_>, other: Py<PyAny>) -> PyResult<Self> {
        self.binary_op(py, other, ">=")
    }
    fn lt(&self, py: Python<'_>, other: Py<PyAny>) -> PyResult<Self> {
        self.binary_op(py, other, "<")
    }
    fn le(&self, py: Python<'_>, other: Py<PyAny>) -> PyResult<Self> {
        self.binary_op(py, other, "<=")
    }

    #[pyo3(name = "and_")]
    fn and_(&self, py: Python<'_>, other: Py<PyAny>) -> PyResult<Self> {
        self.binary_op(py, other, "&&")
    }
    #[pyo3(name = "or_")]
    fn or_(&self, py: Python<'_>, other: Py<PyAny>) -> PyResult<Self> {
        self.binary_op(py, other, "||")
    }

    #[pyo3(name = "not_")]
    fn not_(&self) -> Self {
        self.new_expr(format!("!({})", self.expr))
    }
    fn pipe(&self, py: Python<'_>, rhs: Py<PyAny>) -> PyResult<Self> {
        let rhs_expr = obj_to_jmespath_string(py, rhs)?;
        Ok(self.new_expr(format!("{} | {}", self.expr, rhs_expr)))
    }

    fn length(&self) -> Self {
        self.new_expr(format!("length({})", self.expr))
    }
    fn sort(&self) -> Self {
        self.new_expr(format!("sort({})", self.expr))
    }
    fn keys(&self) -> Self {
        self.new_expr(format!("keys({})", self.expr))
    }
    fn values(&self) -> Self {
        self.new_expr(format!("values({})", self.expr))
    }
    fn to_string(&self) -> Self {
        self.new_expr(format!("to_string({})", self.expr))
    }
    fn to_number(&self) -> Self {
        self.new_expr(format!("to_number({})", self.expr))
    }
    fn to_array(&self) -> Self {
        self.new_expr(format!("to_array({})", self.expr))
    }

    #[pyo3(name = "map_with")]
    fn map(&self, py: Python<'_>, rhs: Py<PyAny>) -> PyResult<Self> {
        let rhs_expr = obj_to_jmespath_string(py, rhs)?;
        let rhs_cleaned = clean_rhs_expr_for_by_func(&rhs_expr);
        let s = format!("map(&{}, {})", rhs_cleaned, self.expr);
        Ok(self.new_expr(s))
    }

    fn sort_by(&self, py: Python<'_>, rhs: Py<PyAny>) -> PyResult<Self> {
        self.by_func(py, "sort_by", rhs)
    }

    fn min_by(&self, py: Python<'_>, rhs: Py<PyAny>) -> PyResult<Self> {
        self.by_func(py, "min_by", rhs)
    }

    fn max_by(&self, py: Python<'_>, rhs: Py<PyAny>) -> PyResult<Self> {
        self.by_func(py, "max_by", rhs)
    }

    fn to_jmespath(&self) -> String {
        self.expr.clone()
    }
}

#[pyfunction]
pub fn field(name: String) -> QueryBuilder {
    QueryBuilder { expr: name }
}

#[pyfunction]
pub fn lit(py: Python<'_>, value: Py<PyAny>) -> PyResult<QueryBuilder> {
    let value_bound = value.bind(py);
    Ok(QueryBuilder {
        expr: obj_to_jmespath_literal_string(py, value_bound)?,
    })
}

#[pyfunction(signature = (*args))]
pub fn select_list(py: Python<'_>, args: &Bound<'_, PyList>) -> PyResult<QueryBuilder> {
    let mut parts: Vec<String> = Vec::new();
    for item in args {
        let expr_str = obj_to_jmespath_string(py, item.to_object(py))?;
        parts.push(if expr_str.is_empty() {
            KWORD_CURRENT.to_string()
        } else {
            expr_str
        });
    }
    let inner = parts.join(", ");
    Ok(QueryBuilder {
        expr: format!("[{}]", inner),
    })
}

#[pyfunction(signature = (**kwargs))]
pub fn select_dict(py: Python<'_>, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<QueryBuilder> {
    let mut parts: Vec<String> = Vec::new();
    if let Some(items) = kwargs {
        for (key, value) in items {
            let key_str = key.extract::<String>()?;
            let value_str = obj_to_jmespath_string(py, value.to_object(py))?;
            let value_clean = if value_str.is_empty() {
                KWORD_CURRENT.to_string()
            } else {
                value_str
            };
            parts.push(format!("{}: {}", key_str, value_clean));
        }
    }
    let inner = parts.join(", ");
    Ok(QueryBuilder {
        expr: format!("{{{}}}", inner),
    })
}
