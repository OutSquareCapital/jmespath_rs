use crate::exprs::Expr;
use pyo3::{
    prelude::*,
    types::{PyDict, PyString, PyTuple},
    PyObject,
};
use std::fmt;
pub struct PyObjectWrapper(pub PyObject);

impl Clone for PyObjectWrapper {
    fn clone(&self) -> Self {
        Python::with_gil(|py| Self(self.0.clone_ref(py)))
    }
}

impl fmt::Debug for PyObjectWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Python::with_gil(|py| {
            let obj = self.0.bind(py);
            let json_result: Result<Bound<PyAny>, PyErr> = (|| {
                let json = py.import_bound("json")?;
                let kwargs = PyDict::new_bound(py);
                let seps = PyTuple::new_bound(
                    py,
                    &[
                        PyString::new_bound(py, ",").into_any(),
                        PyString::new_bound(py, ":").into_any(),
                    ],
                );
                kwargs.set_item("separators", seps)?;
                json.getattr("dumps")?.call((obj,), Some(&kwargs))
            })();

            match json_result {
                Ok(json_string) => write!(f, "`{}`", json_string),
                Err(_) => match obj.repr() {
                    Ok(repr) => write!(f, "`{}`", repr),
                    Err(_) => write!(f, "PyObject(repr_failed)"),
                },
            }
        })
    }
}

pub fn into_node(py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<Node> {
    if let Ok(expr) = obj.extract::<PyRef<Expr>>() {
        return Ok(expr.node.clone());
    }
    if let Ok(s) = obj.extract::<String>() {
        return Ok(Node::Field(s));
    }
    Ok(Node::Literal(PyObjectWrapper(obj.to_object(py))))
}
pub fn into_node_lit(py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<Node> {
    if let Ok(expr) = obj.extract::<PyRef<Expr>>() {
        return Ok(expr.node.clone());
    }
    Ok(Node::Literal(PyObjectWrapper(obj.to_object(py))))
}

#[derive(Debug, Clone)]
pub enum Node {
    This,
    Field(String),
    Index(isize),
    Slice(Option<isize>, Option<isize>, Option<isize>),
    Literal(PyObjectWrapper),

    SubExpr(Box<Node>, Box<Node>),
    MultiList(Vec<Node>),
    MultiDict(Vec<(String, Node)>),
    Flatten(Box<Node>),
    FilterProjection {
        base: Box<Node>,
        then: Box<Node>,
        cond: Box<Node>,
    },
    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),
    Not(Box<Node>),
    Abs(Box<Node>),
    Avg(Box<Node>),
    Ceil(Box<Node>),
    Contains(Box<Node>, Box<Node>),
    EndsWith(Box<Node>, Box<Node>),
    Floor(Box<Node>),
    Join(Box<Node>, Box<Node>),
    Max(Box<Node>),
    Merge(Vec<Node>),
    Min(Box<Node>),
    NotNull(Vec<Node>),
    Reverse(Box<Node>),
    StartsWith(Box<Node>, Box<Node>),
    Sum(Box<Node>),
    CmpEq(Box<Node>, Box<Node>),
    CmpNe(Box<Node>, Box<Node>),
    CmpLt(Box<Node>, Box<Node>),
    CmpLe(Box<Node>, Box<Node>),
    CmpGt(Box<Node>, Box<Node>),
    CmpGe(Box<Node>, Box<Node>),
    Length(Box<Node>),
    Sort(Box<Node>),
    Keys(Box<Node>),
    Values(Box<Node>),
    MapApply {
        base: Box<Node>,
        key: Box<Node>,
    },
    SortBy {
        base: Box<Node>,
        key: Box<Node>,
    },
    MinBy {
        base: Box<Node>,
        key: Box<Node>,
    },
    MaxBy {
        base: Box<Node>,
        key: Box<Node>,
    },
}
