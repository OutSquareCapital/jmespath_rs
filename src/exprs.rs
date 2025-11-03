use crate::eval;
use crate::lists::ExprListNameSpace;
use crate::nodes::{into_node_lit, ComparisonOp, Node, PyObjectWrapper, ScalarOp, StructOp};
use crate::strings::ExprStrNameSpace;
use crate::structs::ExprStructNameSpace;
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyclass(module = "dictexprs", name = "Expr")]
#[derive(Clone)]
pub struct Expr {
    pub node: Node,
}

#[pymethods]
impl Expr {
    #[new]
    pub fn new() -> Self {
        Self { node: Node::This }
    }

    #[getter]
    pub fn list(&self) -> ExprListNameSpace {
        ExprListNameSpace { expr: self.clone() }
    }

    #[getter]
    pub fn str(&self) -> ExprStrNameSpace {
        ExprStrNameSpace { expr: self.clone() }
    }

    #[getter]
    #[pyo3(name = "struct")]
    pub fn struct_(&self) -> ExprStructNameSpace {
        ExprStructNameSpace { expr: self.clone() }
    }

    pub fn eq(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::Compare(
                self.node.clone().into(),
                ComparisonOp::Eq(into_node_lit(py, other)?.into()),
            ),
        })
    }

    pub fn ne(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::Compare(
                self.node.clone().into(),
                ComparisonOp::Ne(into_node_lit(py, other)?.into()),
            ),
        })
    }

    pub fn lt(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::Compare(
                self.node.clone().into(),
                ComparisonOp::Lt(into_node_lit(py, other)?.into()),
            ),
        })
    }

    pub fn le(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::Compare(
                self.node.clone().into(),
                ComparisonOp::Le(into_node_lit(py, other)?.into()),
            ),
        })
    }

    pub fn gt(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::Compare(
                self.node.clone().into(),
                ComparisonOp::Gt(into_node_lit(py, other)?.into()),
            ),
        })
    }

    pub fn ge(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::Compare(
                self.node.clone().into(),
                ComparisonOp::Ge(into_node_lit(py, other)?.into()),
            ),
        })
    }
    pub fn and_(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::And(self.node.clone().into(), into_node_lit(py, other)?.into()),
        })
    }

    pub fn or_(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::Or(self.node.clone().into(), into_node_lit(py, other)?.into()),
        })
    }

    pub fn not_(&self) -> Self {
        Self {
            node: Node::Not(self.node.clone().into()),
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            node: Node::Scalar(self.node.clone().into(), ScalarOp::Abs),
        }
    }

    pub fn ceil(&self) -> Self {
        Self {
            node: Node::Scalar(self.node.clone().into(), ScalarOp::Ceil),
        }
    }

    pub fn floor(&self) -> Self {
        Self {
            node: Node::Scalar(self.node.clone().into(), ScalarOp::Floor),
        }
    }

    pub fn search(&self, py: Python<'_>, data: PyObject) -> PyResult<PyObject> {
        eval::eval_any(py, &self.node, data.bind(py)).map(|result| result.unbind())
    }
}

#[pyfunction]
pub fn key(name: String) -> Expr {
    Expr {
        node: Node::Struct(Box::new(Node::This), StructOp::Field(name)),
    }
}

#[pyfunction(signature = (*args))]
pub fn select_list(args: Vec<Expr>) -> Expr {
    Expr {
        node: Node::MultiList(args.into_iter().map(|q| q.node).collect()),
    }
}

#[pyfunction(signature = (**kwargs))]
pub fn select_dict(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Expr> {
    let mut items = Vec::new();

    if let Some(dict) = kwargs {
        for (key, value) in dict {
            items.push((key.extract::<String>()?, into_node_lit(key.py(), &value)?));
        }
    }

    Ok(Expr {
        node: Node::MultiDict(items),
    })
}
#[pyfunction(signature = (*args))]
pub fn merge(args: Vec<Expr>) -> Expr {
    Expr {
        node: Node::Merge(args.into_iter().map(|q| q.node).collect()),
    }
}

#[pyfunction(signature = (*args))]
pub fn not_null(args: Vec<Expr>) -> Expr {
    Expr {
        node: Node::NotNull(args.into_iter().map(|q| q.node).collect()),
    }
}
#[pyfunction]
pub fn lit(value: &Bound<'_, PyAny>) -> Expr {
    Python::with_gil(|py| Expr {
        node: Node::Literal(PyObjectWrapper(value.to_object(py))),
    })
}

#[pyfunction]
pub fn element() -> Expr {
    Expr::new()
}
