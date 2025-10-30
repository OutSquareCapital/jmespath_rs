use crate::querybuilder::QueryBuilder;
use pyo3::types::*;
use pyo3::{prelude::*, PyObject};
#[derive(Debug)]
pub struct PyObjectWrapper(pub PyObject);

impl Clone for PyObjectWrapper {
    fn clone(&self) -> Self {
        Python::with_gil(|py| Self(self.0.clone_ref(py)))
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    This,
    Field(String),
    Index(isize),
    Slice(Option<isize>, Option<isize>, Option<isize>),
    Literal(PyObjectWrapper),

    Pipe(Box<Node>, Box<Node>),
    SubExpr(Box<Node>, Box<Node>),
    MultiList(Vec<Node>),
    MultiDict(Vec<(String, Node)>),
    ProjectArray {
        base: Box<Node>,
        rhs: Box<Node>,
    },
    ProjectObject {
        base: Box<Node>,
        rhs: Box<Node>,
    },
    Flatten(Box<Node>),
    FilterProjection {
        base: Box<Node>,
        then: Box<Node>,
        cond: Box<Node>,
    },
    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),
    Not(Box<Node>),
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
    ToArray(Box<Node>),
    ToString(Box<Node>),
    ToNumber(Box<Node>),
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

fn is_instance_of<'py, T: pyo3::PyTypeInfo>(obj: &Bound<'py, PyAny>) -> bool {
    obj.is_instance_of::<T>()
}

pub fn into_node(py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<Node> {
    if let Some(n) = try_as_builder(obj) {
        return Ok(n);
    }
    if is_instance_of::<PyString>(obj) {
        let s: String = obj.extract()?;
        return Ok(Node::Field(s));
    }
    if is_instance_of::<PyNone>(obj)
        || is_instance_of::<PyInt>(obj)
        || is_instance_of::<PyFloat>(obj)
        || is_instance_of::<PyBool>(obj)
    {
        return Ok(Node::Literal(PyObjectWrapper(obj.to_object(py))));
    }
    Ok(Node::Literal(PyObjectWrapper(obj.to_object(py))))
}

fn try_as_builder(obj: &Bound<'_, PyAny>) -> Option<Node> {
    if let Ok(qb) = obj.extract::<PyRef<QueryBuilder>>() {
        Some(qb.node.clone())
    } else {
        None
    }
}
