use crate::exprs::Expr;
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
    Type(Box<Node>),
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
