use crate::querybuilder::QueryBuilder;
use pyo3::prelude::*;
use pyo3::types::PyDict;

pub const KWORD_CURRENT: &str = "@";
pub const KWORD_DOT: &str = ".";
pub const KWORD_ARRAY_PROJECT: &str = "[*]";
pub const KWORD_OBJECT_PROJECT: &str = "*";
pub const KWORD_FLATTEN: &str = "[]";

pub fn from_py_arg<'py>(py: Python<'py>, obj: Py<PyAny>) -> PyArgConverter<'py> {
    PyArgConverter { py, obj }
}

pub struct PyArgConverter<'py> {
    py: Python<'py>,
    obj: Py<PyAny>,
}

impl<'py> PyArgConverter<'py> {
    pub fn to_string_expr(self) -> PyResult<StringExpr> {
        let ob_bound = self.obj.bind(self.py);
        if let Ok(q) = ob_bound.extract::<PyRef<QueryBuilder>>() {
            return Ok(StringExpr {
                value: q.expr.clone(),
            });
        }
        if let Ok(s) = ob_bound.extract::<String>() {
            return Ok(StringExpr { value: s });
        }
        self.to_literal_expr()
    }

    pub fn to_literal_expr(self) -> PyResult<StringExpr> {
        let ob_bound = self.obj.bind(self.py);
        let default = self.py.import_bound("builtins")?.getattr("str")?;
        let kwargs = PyDict::new_bound(self.py);
        kwargs.set_item("default", default)?;

        let s = self
            .py
            .import_bound("json")?
            .call_method("dumps", (ob_bound,), Some(&kwargs))?
            .extract::<String>()?;

        Ok(StringExpr {
            value: format!("`{}`", s),
        })
    }
}
pub struct StringFormatter {
    expr: StringExpr,
}

impl StringFormatter {
    fn new_with_value(&self, new_value: String) -> Self {
        Self {
            expr: StringExpr::new(new_value),
        }
    }
    pub fn inner(self) -> StringExpr {
        self.expr
    }

    pub fn as_binary_op(self, op: &str, left_expr: &str) -> Self {
        self.new_with_value(format!("({}) {} ({})", left_expr, op, self.expr.value))
    }

    pub fn as_by_func(self, name: &str, left_expr: &str) -> Self {
        self.new_with_value(format!("{}({}, &{})", name, left_expr, self.expr.value))
    }

    pub fn as_project(self, left_expr: &str, project_keyword: &str) -> Self {
        self.new_with_value(format!(
            "{}{}{}",
            left_expr, project_keyword, self.expr.value
        ))
    }

    pub fn as_filter(self, left_expr: &str, cond_expr: &StringExpr) -> Self {
        self.new_with_value(format!(
            "{}[?{}]{}",
            left_expr, cond_expr.value, self.expr.value
        ))
    }

    pub fn as_pipe(self, left_expr: &str) -> Self {
        self.new_with_value(format!("{} | {}", left_expr, self.expr.value))
    }

    pub fn as_map(self, left_expr: &str) -> Self {
        self.new_with_value(format!("map(&{}, {})", self.expr.value, left_expr))
    }
}

pub struct StringExpr {
    value: String,
}

impl StringExpr {
    pub fn new(value: String) -> Self {
        Self { value }
    }
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
    pub fn into_formatter(self) -> StringFormatter {
        StringFormatter { expr: self }
    }

    pub fn into_string(self) -> String {
        self.value
    }
    pub fn into_py_query(self) -> PyResult<QueryBuilder> {
        Ok(QueryBuilder { expr: self.value })
    }
}
