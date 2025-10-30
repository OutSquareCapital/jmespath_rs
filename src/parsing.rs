use crate::querybuilder::QueryBuilder;
use pyo3::prelude::*;
use pyo3::types::PyDict;

pub const KWORD_CURRENT: &str = "@";
pub const KWORD_DOT: &str = ".";
pub const KWORD_ARRAY_PROJECT: &str = "[*]";
pub const KWORD_OBJECT_PROJECT: &str = "*";
pub const KWORD_FLATTEN: &str = "[]";

pub struct Preprocessing;
pub struct Formatting;
pub struct Finalized;

pub struct StringExpr<State> {
    value: String,
    _state: std::marker::PhantomData<State>,
}

impl<State> StringExpr<State> {
    fn new_with_state(value: String) -> Self {
        Self {
            value,
            _state: std::marker::PhantomData,
        }
    }
    pub fn into_string(self) -> String {
        self.value
    }

    pub fn into_py_query(self) -> PyResult<QueryBuilder> {
        Ok(QueryBuilder { expr: self.value })
    }
}

pub struct PyArgConverter<'py> {
    py: Python<'py>,
    obj: Py<PyAny>,
}

impl<'py> PyArgConverter<'py> {
    pub fn new(py: Python<'py>, obj: Py<PyAny>) -> Self {
        PyArgConverter { py, obj }
    }
    pub fn to_string_expr(self) -> PyResult<StringExpr<Preprocessing>> {
        let ob_bound = self.obj.bind(self.py);
        if let Ok(q) = ob_bound.extract::<PyRef<QueryBuilder>>() {
            return Ok(StringExpr::new_with_state(q.expr.clone()));
        }
        if let Ok(s) = ob_bound.extract::<String>() {
            return Ok(StringExpr::new_with_state(s));
        }
        self.to_literal_expr()
    }

    pub fn to_literal_expr(self) -> PyResult<StringExpr<Preprocessing>> {
        let ob_bound = self.obj.bind(self.py);
        let default = self.py.import_bound("builtins")?.getattr("str")?;
        let kwargs = PyDict::new_bound(self.py);
        kwargs.set_item("default", default)?;

        let s = self
            .py
            .import_bound("json")?
            .call_method("dumps", (ob_bound,), Some(&kwargs))?
            .extract::<String>()?;

        Ok(StringExpr::new_with_state(format!("`{}`", s)))
    }
}

impl StringExpr<Preprocessing> {
    pub fn strip_current(mut self) -> Self {
        if let Some(s) = self.value.strip_prefix(KWORD_CURRENT) {
            self.value = s.to_string();
        }
        self
    }

    pub fn strip_dot(mut self) -> Self {
        if let Some(s) = self.value.strip_prefix(KWORD_DOT) {
            self.value = s.to_string();
        }
        self
    }
    pub fn ensure_leading_dot(mut self) -> Self {
        if !self.value.starts_with(KWORD_DOT)
            && !self.value.starts_with('[')
            && !self.value.is_empty()
        {
            self.value = format!("{}{}", KWORD_DOT, self.value);
        }
        self
    }
    pub fn into_formatter(self) -> StringExpr<Formatting> {
        StringExpr::new_with_state(self.value)
    }
}

impl StringExpr<Formatting> {
    fn new_with_value(&self, new_value: String) -> Self {
        Self::new_with_state(new_value)
    }

    pub fn as_binary_op(self, op: &str, left_expr: &str) -> Self {
        self.new_with_value(format!("({}) {} ({})", left_expr, op, self.value))
    }

    pub fn as_by_func(self, name: &str, left_expr: &str) -> Self {
        self.new_with_value(format!("{}({}, &{})", name, left_expr, self.value))
    }

    pub fn as_project(self, left_expr: &str, project_keyword: &str) -> Self {
        self.new_with_value(format!("{}{}{}", left_expr, project_keyword, self.value))
    }

    pub fn as_filter(self, left_expr: &str, cond_expr: &StringExpr<Preprocessing>) -> Self {
        self.new_with_value(format!("{}[?{}]{}", left_expr, cond_expr.value, self.value))
    }

    pub fn as_pipe(self, left_expr: &str) -> Self {
        self.new_with_value(format!("{} | {}", left_expr, self.value))
    }

    pub fn as_map(self, left_expr: &str) -> Self {
        self.new_with_value(format!("map(&{}, {})", self.value, left_expr))
    }
}

impl StringExpr<Finalized> {
    pub fn new(value: String) -> Self {
        Self::new_with_state(value)
    }
}
