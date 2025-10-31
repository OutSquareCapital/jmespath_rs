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

fn format_slice(start: &Option<isize>, end: &Option<isize>, step: &Option<isize>) -> String {
    let s = start.map(|v| v.to_string()).unwrap_or_default();
    let e = end.map(|v| v.to_string()).unwrap_or_default();

    if let Some(st) = step {
        format!("{}:{}:{}", s, e, st)
    } else {
        format!("{}:{}", s, e)
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::This => write!(f, "@"),
            Node::Field(name) => write!(f, "{}", name),
            Node::Index(i) => write!(f, "[{}]", i),
            Node::Slice(start, end, step) => write!(f, "[{}]", format_slice(start, end, step)),
            Node::Literal(obj) => write!(f, "{:?}", obj),

            Node::SubExpr(lhs, rhs) => {
                if matches!(**lhs, Node::This) {
                    write!(f, "{}", rhs)
                } else {
                    match **rhs {
                        Node::Field(ref name) => write!(f, "{}.{}", lhs, name),
                        Node::Index(_) | Node::Slice(_, _, _) => write!(f, "{}{}", lhs, rhs),
                        _ => write!(f, "{}.({})", lhs, rhs),
                    }
                }
            }

            Node::Pipe(lhs, rhs) => write!(f, "{} | {}", lhs, rhs),
            Node::MultiList(items) => {
                let inner: Vec<String> = items.iter().map(|n| n.to_string()).collect();
                write!(f, "[{}]", inner.join(", "))
            }
            Node::MultiDict(items) => {
                let inner: Vec<String> = items
                    .iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, v))
                    .collect();
                write!(f, "{{{}}}", inner.join(", "))
            }
            Node::ProjectArray { base, rhs } => write!(f, "{}[].{}", base, rhs),
            Node::ProjectObject { base, rhs } => write!(f, "{}[].*.{}", base, rhs),

            Node::Flatten(inner) => write!(f, "{}[]", inner),
            Node::FilterProjection { base, then, cond } => {
                write!(f, "{}[?{}]", base, cond)?;
                if !matches!(**then, Node::This) {
                    write!(f, ".{}", then)?;
                }
                Ok(())
            }

            Node::And(a, b) => write!(f, "({} && {})", a, b),
            Node::Or(a, b) => write!(f, "({} || {})", a, b),
            Node::Not(x) => write!(f, "!({})", x),

            Node::CmpEq(a, b) => write!(f, "{} == {}", a, b),
            Node::CmpNe(a, b) => write!(f, "{} != {}", a, b),
            Node::CmpLt(a, b) => write!(f, "{} < {}", a, b),
            Node::CmpLe(a, b) => write!(f, "{} <= {}", a, b),
            Node::CmpGt(a, b) => write!(f, "{} > {}", a, b),
            Node::CmpGe(a, b) => write!(f, "{} >= {}", a, b),

            Node::Length(x) => write!(f, "length({})", x),
            Node::Sort(x) => write!(f, "sort({})", x),
            Node::Keys(x) => write!(f, "keys({})", x),
            Node::Values(x) => write!(f, "values({})", x),
            Node::ToArray(x) => write!(f, "to_array({})", x),
            Node::ToString(x) => write!(f, "to_string({})", x),
            Node::ToNumber(x) => write!(f, "to_number({})", x),
            Node::Abs(x) => write!(f, "abs({})", x),
            Node::Avg(x) => write!(f, "avg({})", x),
            Node::Ceil(x) => write!(f, "ceil({})", x),
            Node::Floor(x) => write!(f, "floor({})", x),
            Node::Max(x) => write!(f, "max({})", x),
            Node::Min(x) => write!(f, "min({})", x),
            Node::Reverse(x) => write!(f, "reverse({})", x),
            Node::Sum(x) => write!(f, "sum({})", x),
            Node::Type(x) => write!(f, "type({})", x),

            Node::Contains(a, b) => write!(f, "contains({}, {})", a, b),
            Node::EndsWith(a, b) => write!(f, "ends_with({}, {})", a, b),
            Node::StartsWith(a, b) => write!(f, "starts_with({}, {})", a, b),
            Node::Join(a, b) => write!(f, "join({}, {})", a, b),

            Node::Merge(items) => {
                let inner: Vec<String> = items.iter().map(|n| n.to_string()).collect();
                write!(f, "merge({})", inner.join(", "))
            }
            Node::NotNull(items) => {
                let inner: Vec<String> = items.iter().map(|n| n.to_string()).collect();
                write!(f, "not_null({})", inner.join(", "))
            }

            Node::MapApply { base, key } => write!(f, "map(&{}, {})", key, base),
            Node::SortBy { base, key } => write!(f, "sort_by({}, &{})", base, key),
            Node::MinBy { base, key } => write!(f, "min_by({}, &{})", base, key),
            Node::MaxBy { base, key } => write!(f, "max_by({}, &{})", base, key),
        }
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
