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
        return Ok(Node::Struct(Box::new(Node::This), StructOp::Field(s)));
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
    Literal(PyObjectWrapper),
    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),
    Not(Box<Node>),
    NotNull(Vec<Node>),
    Length(Box<Node>),
    MultiList(Vec<Node>),
    MultiDict(Vec<(String, Node)>),
    Merge(Vec<Node>),
    List(Box<Node>, ListOp),
    Str(Box<Node>, StrOp),
    Struct(Box<Node>, StructOp),
    Scalar(Box<Node>, ScalarOp),
    Compare(Box<Node>, ComparisonOp),
}

#[derive(Debug, Clone)]
pub enum ListOp {
    Index(isize),
    Slice {
        start: Option<isize>,
        end: Option<isize>,
        step: Option<isize>,
    },
    Reverse,
    Flatten,
    Contains(Box<Node>),
    Filter(Box<Node>),
    Map(Box<Node>),
    Join(Box<Node>),
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
pub enum StrOp {
    Slice {
        start: Option<isize>,
        end: Option<isize>,
        step: Option<isize>,
    },
    Reverse,
    Contains(Box<Node>),
    StartsWith(Box<Node>),
    EndsWith(Box<Node>),
}

#[derive(Debug, Clone)]
pub enum StructOp {
    Field(String),
    Keys,
    Values,
}

#[derive(Debug, Clone)]
pub enum ScalarOp {
    Abs,
    Ceil,
    Floor,
}

#[derive(Debug, Clone)]
pub enum ComparisonOp {
    Eq(Box<Node>),
    Ne(Box<Node>),
    Lt(Box<Node>),
    Le(Box<Node>),
    Gt(Box<Node>),
    Ge(Box<Node>),
}
