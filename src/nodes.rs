use crate::exprs::Expr;
use pyo3::{
    prelude::*,
    types::{PyDict, PyString, PyTuple},
    PyObject,
};
use std::fmt;

pub type EvalResult<'py> = PyResult<Bound<'py, PyAny>>;
pub type Bounded<'py> = Bound<'py, PyAny>;

pub(crate) struct PyObjectWrapper(pub PyObject);

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

pub(crate) fn into_lit(py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<Node> {
    if let Ok(expr) = obj.extract::<PyRef<Expr>>() {
        return Ok(expr.node.clone());
    }
    Ok(Node::Literal(PyObjectWrapper(obj.to_object(py))))
}

#[derive(Debug, Clone)]
pub(crate) enum Node {
    This,
    Literal(PyObjectWrapper),
    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),
    Not(Box<Node>),
    Coalesce(Vec<Node>),
    Merge(Vec<Node>),
    List(Box<Node>, ListOp),
    Str(Box<Node>, StrOp),
    Struct(Box<Node>, StructOp),
    Scalar(Box<Node>, ScalarOp),
    Compare(Box<Node>, ComparisonOp),
}

#[derive(Debug, Clone)]
pub(crate) enum ListOp {
    Index(isize),
    Slice {
        start: Option<isize>,
        end: Option<isize>,
        step: Option<isize>,
    },
    Length,
    Reverse,
    Flatten,
    Contains(Box<Node>),
    Filter(Box<Node>),
    Map(Box<Node>),
    Join(String),
    Sort,
    Max,
    Min,
    Sum,
    Avg,
    SortBy(Box<Node>),
    MinBy(Box<Node>),
    MaxBy(Box<Node>),
}

#[derive(Debug, Clone)]
pub(crate) enum StrOp {
    Slice {
        start: Option<isize>,
        end: Option<isize>,
        step: Option<isize>,
    },
    Reverse,
    Contains(String),
    StartsWith(String),
    EndsWith(String),
    Length,
}

#[derive(Debug, Clone)]
pub(crate) enum StructOp {
    Field(String),
    Keys,
    Values,
}

#[derive(Debug, Clone)]
pub(crate) enum ScalarOp {
    Abs,
    Ceil,
    Floor,
}

#[derive(Debug, Clone)]
pub(crate) enum ComparisonOp {
    Eq(Box<Node>),
    Ne(Box<Node>),
    Lt(Box<Node>),
    Le(Box<Node>),
    Gt(Box<Node>),
    Ge(Box<Node>),
}
