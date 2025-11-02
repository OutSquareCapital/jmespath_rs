use crate::eval;
use crate::lists::ExprListNameSpace;
use crate::nodes::{into_node, into_node_lit, Node, PyObjectWrapper};
use crate::strings::ExprStrNameSpace;
use crate::structs::ExprStructNameSpace;
use pyo3::prelude::*;
use pyo3::types::PyDict;
#[pyclass(module = "dictexprs", name = "FilteredExpr")]
pub struct FilteredExpr {
    base: Node,
    cond: Node,
}

#[pymethods]
impl FilteredExpr {
    pub fn then(&self, py: Python<'_>, then: &Bound<'_, PyAny>) -> PyResult<Expr> {
        Ok(Expr {
            node: Node::FilterProjection {
                base: self.base.clone().into(),
                then: into_node(py, then)?.into(),
                cond: self.cond.clone().into(),
            },
        })
    }
}
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

    fn __getattr__(&self, name: String) -> Self {
        self.field(name)
    }

    pub fn field(&self, name: String) -> Self {
        Self {
            node: Node::SubExpr(self.node.clone().into(), Node::Field(name).into()),
        }
    }

    pub fn pipe(&self, rhs: Expr) -> Self {
        Self {
            node: Node::Pipe(self.node.clone().into(), rhs.node.into()),
        }
    }

    pub fn project(&self, py: Python<'_>, rhs: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::ProjectArray {
                base: self.node.clone().into(),
                rhs: into_node(py, rhs)?.into(),
            },
        })
    }

    pub fn vproject(&self, py: Python<'_>, rhs: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::ProjectObject {
                base: self.node.clone().into(),
                rhs: into_node(py, rhs)?.into(),
            },
        })
    }

    pub fn filter(&self, cond: &Expr) -> FilteredExpr {
        FilteredExpr {
            base: self.node.clone(),
            cond: cond.node.clone(),
        }
    }

    pub fn eq(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::CmpEq(self.node.clone().into(), into_node_lit(py, other)?.into()),
        })
    }

    pub fn ne(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::CmpNe(self.node.clone().into(), into_node_lit(py, other)?.into()),
        })
    }

    pub fn lt(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::CmpLt(self.node.clone().into(), into_node_lit(py, other)?.into()),
        })
    }

    pub fn le(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::CmpLe(self.node.clone().into(), into_node_lit(py, other)?.into()),
        })
    }

    pub fn gt(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::CmpGt(self.node.clone().into(), into_node_lit(py, other)?.into()),
        })
    }

    pub fn ge(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::CmpGe(self.node.clone().into(), into_node_lit(py, other)?.into()),
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

    pub fn map(&self, py: Python<'_>, expr: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::MapApply {
                base: self.node.clone().into(),
                key: into_node(py, expr)?.into(),
            },
        })
    }

    pub fn sort_by(&self, py: Python<'_>, key: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::SortBy {
                base: self.node.clone().into(),
                key: into_node(py, key)?.into(),
            },
        })
    }

    pub fn min_by(&self, py: Python<'_>, key: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::MinBy {
                base: self.node.clone().into(),
                key: into_node(py, key)?.into(),
            },
        })
    }

    pub fn max_by(&self, py: Python<'_>, key: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::MaxBy {
                base: self.node.clone().into(),
                key: into_node(py, key)?.into(),
            },
        })
    }
    pub fn abs(&self) -> Self {
        Self {
            node: Node::Abs(self.node.clone().into()),
        }
    }

    pub fn avg(&self) -> Self {
        Self {
            node: Node::Avg(self.node.clone().into()),
        }
    }

    pub fn ceil(&self) -> Self {
        Self {
            node: Node::Ceil(self.node.clone().into()),
        }
    }

    pub fn floor(&self) -> Self {
        Self {
            node: Node::Floor(self.node.clone().into()),
        }
    }

    #[pyo3(name = "dtype")]
    pub fn dtype(&self) -> Self {
        Self {
            node: Node::DType(self.node.clone().into()),
        }
    }

    pub fn to_number(&self) -> Self {
        Self {
            node: Node::ToNumber(self.node.clone().into()),
        }
    }

    pub fn to_string(&self) -> Expr {
        Expr {
            node: Node::ToString(self.node.clone().into()),
        }
    }
    pub fn search(&self, py: Python<'_>, data: PyObject) -> PyResult<PyObject> {
        eval::eval_any(py, &self.node, data.bind(py)).map(|result| result.unbind())
    }
}

#[pyfunction]
pub fn key(name: String) -> Expr {
    Expr {
        node: Node::Field(name).into(),
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
