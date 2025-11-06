use pyo3::{
    prelude::*,
    types::{PyDict, PyString, PyTuple},
};
use std::fmt;

use crate::eval;

pub type EvalResult<'py> = PyResult<Bound<'py, PyAny>>;
pub type Bounded<'py> = Bound<'py, PyAny>;

pub(crate) struct PyObjectWrapper(pub Py<PyAny>);

impl Clone for PyObjectWrapper {
    fn clone(&self) -> Self {
        Python::attach(|py| Self(self.0.clone_ref(py)))
    }
}

impl fmt::Debug for PyObjectWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Python::attach(|py| {
            let obj = self.0.bind(py);
            let json_result: Result<Bound<PyAny>, PyErr> = (|| {
                let json = py.import(eval::pylibs::JSON)?;
                let kwargs = PyDict::new(py);
                let seps = PyTuple::new(
                    py,
                    &[
                        PyString::new(py, ",").into_any(),
                        PyString::new(py, ":").into_any(),
                    ],
                )?;
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
