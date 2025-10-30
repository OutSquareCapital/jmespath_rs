use crate::parsing as ps;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

fn py_obj_to_selection_string(py: Python<'_>, obj: Py<PyAny>) -> PyResult<String> {
    let expr_str = ps::from_py_arg(py, obj).to_string_expr()?.into_string();
    Ok(if expr_str.is_empty() {
        ps::KWORD_CURRENT.to_string()
    } else {
        expr_str
    })
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
        ps::from_py_arg(py, other)
            .to_string_expr()?
            .into_formatter()
            .as_binary_op(op, &self.expr)
            .inner()
            .into_py_query()
    }
    fn by_func(&self, py: Python<'_>, name: &str, rhs: Py<PyAny>) -> PyResult<Self> {
        ps::from_py_arg(py, rhs)
            .to_string_expr()?
            .strip_current()
            .strip_dot()
            .into_formatter()
            .as_by_func(name, &self.expr)
            .inner()
            .into_py_query()
    }
}
#[pymethods]
impl QueryBuilder {
    #[new]
    fn new() -> Self {
        Self {
            expr: ps::KWORD_CURRENT.to_string(),
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
        self.new_expr(format!("{}{}{}", self.expr, ps::KWORD_DOT, name))
    }
    fn __getattr__(&self, name: String) -> Self {
        self.field_(name)
    }
    fn index(&self, i: i64) -> Self {
        self.new_expr(format!("{}[{}]", self.expr, i))
    }
    fn project(&self, py: Python<'_>, rhs: Py<PyAny>) -> PyResult<Self> {
        ps::from_py_arg(py, rhs)
            .to_string_expr()?
            .ensure_leading_dot()
            .into_formatter()
            .as_project(&self.expr, ps::KWORD_ARRAY_PROJECT)
            .inner()
            .into_py_query()
    }
    fn vproject(&self, py: Python<'_>, rhs: Py<PyAny>) -> PyResult<Self> {
        ps::from_py_arg(py, rhs)
            .to_string_expr()?
            .ensure_leading_dot()
            .into_formatter()
            .as_project(&self.expr, ps::KWORD_OBJECT_PROJECT)
            .inner()
            .into_py_query()
    }
    fn flatten(&self) -> Self {
        self.new_expr(format!("{}{}", self.expr, ps::KWORD_FLATTEN))
    }

    fn filter(&self, py: Python<'_>, cond: Py<PyAny>, then: Py<PyAny>) -> PyResult<Self> {
        let cond_expr = ps::from_py_arg(py, cond).to_string_expr()?.strip_current();

        ps::from_py_arg(py, then)
            .to_string_expr()?
            .ensure_leading_dot()
            .into_formatter()
            .as_filter(&self.expr, &cond_expr)
            .inner()
            .into_py_query()
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
        ps::from_py_arg(py, rhs)
            .to_string_expr()?
            .into_formatter()
            .as_pipe(&self.expr)
            .inner()
            .into_py_query()
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
        ps::from_py_arg(py, rhs)
            .to_string_expr()?
            .strip_current()
            .strip_dot()
            .into_formatter()
            .as_map(&self.expr)
            .inner()
            .into_py_query()
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
    ps::from_py_arg(py, value)
        .to_literal_expr()?
        .into_py_query()
}
#[pyfunction(signature = (*args))]
pub fn select_list(py: Python<'_>, args: &Bound<'_, PyList>) -> PyResult<QueryBuilder> {
    let inner = args
        .iter()
        .map(|item| py_obj_to_selection_string(py, item.to_object(py)))
        .collect::<PyResult<Vec<String>>>()?
        .join(", ");

    ps::StringExpr::new(format!("[{}]", inner)).into_py_query()
}

#[pyfunction(signature = (**kwargs))]
pub fn select_dict(py: Python<'_>, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<QueryBuilder> {
    let inner = kwargs
        .map_or(Ok(Vec::new()), |items| {
            items
                .iter()
                .map(|(key, value)| {
                    let key_str = key.extract::<String>()?;
                    let value_str = py_obj_to_selection_string(py, value.to_object(py))?;
                    Ok(format!("{}: {}", key_str, value_str))
                })
                .collect::<PyResult<Vec<String>>>()
        })?
        .join(", ");

    ps::StringExpr::new(format!("{{{}}}", inner)).into_py_query()
}
