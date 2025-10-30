use crate::nodes::{into_node, Node, PyObjectWrapper};
use pyo3::prelude::*;
use pyo3::types::PyDict;
#[pyclass(module = "jmespath_rs", name = "FilteredExpr")]
pub struct FilteredExpr {
    base: Node,
    cond: Node,
}

#[pymethods]
impl FilteredExpr {
    pub fn then(&self, then: &Expr) -> Expr {
        Expr {
            node: Node::FilterProjection {
                base: self.base.clone().into(),
                then: then.node.clone().into(),
                cond: self.cond.clone().into(),
            },
        }
    }
}
#[pyclass(module = "jmespath_rs", name = "Expr")]
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

    fn __getattr__(&self, name: String) -> Self {
        self.field(name)
    }

    pub fn field(&self, name: String) -> Self {
        Self {
            node: Node::SubExpr(self.node.clone().into(), Node::Field(name).into()),
        }
    }

    pub fn index(&self, i: isize) -> Self {
        Self {
            node: Node::SubExpr(self.node.clone().into(), Node::Index(i).into()),
        }
    }

    #[pyo3(signature = (start=None, end=None, step=None))]
    pub fn slice(&self, start: Option<isize>, end: Option<isize>, step: Option<isize>) -> Self {
        Self {
            node: Node::SubExpr(
                self.node.clone().into(),
                Node::Slice(start, end, step).into(),
            ),
        }
    }

    pub fn pipe(&self, rhs: Expr) -> Self {
        Self {
            node: Node::Pipe(self.node.clone().into(), rhs.node.into()),
        }
    }

    pub fn project(&self, py: Python<'_>, rhs: &Bound<'_, PyAny>) -> PyResult<Self> {
        let rhs_node = into_node(py, rhs)?;
        Ok(Self {
            node: Node::ProjectArray {
                base: self.node.clone().into(),
                rhs: rhs_node.into(),
            },
        })
    }

    pub fn vproject(&self, py: Python<'_>, rhs: &Bound<'_, PyAny>) -> PyResult<Self> {
        let rhs_node = into_node(py, rhs)?;
        Ok(Self {
            node: Node::ProjectObject {
                base: self.node.clone().into(),
                rhs: rhs_node.into(),
            },
        })
    }

    pub fn filter(&self, cond: &Expr) -> FilteredExpr {
        FilteredExpr {
            base: self.node.clone(),
            cond: cond.node.clone(),
        }
    }
    pub fn flatten(&self) -> Self {
        Self {
            node: Node::Flatten(self.node.clone().into()),
        }
    }
    pub fn eq(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::CmpEq(self.node.clone().into(), into_node(py, other)?.into()),
        })
    }

    pub fn ne(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::CmpNe(self.node.clone().into(), into_node(py, other)?.into()),
        })
    }

    pub fn lt(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::CmpLt(self.node.clone().into(), into_node(py, other)?.into()),
        })
    }

    pub fn le(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::CmpLe(self.node.clone().into(), into_node(py, other)?.into()),
        })
    }

    pub fn gt(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::CmpGt(self.node.clone().into(), into_node(py, other)?.into()),
        })
    }

    pub fn ge(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::CmpGe(self.node.clone().into(), into_node(py, other)?.into()),
        })
    }
    pub fn and_(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::And(self.node.clone().into(), into_node(py, other)?.into()),
        })
    }

    pub fn or_(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: Node::Or(self.node.clone().into(), into_node(py, other)?.into()),
        })
    }

    pub fn not_(&self) -> Self {
        Self {
            node: Node::Not(self.node.clone().into()),
        }
    }

    pub fn length(&self) -> Self {
        Self {
            node: Node::Length(self.node.clone().into()),
        }
    }

    pub fn sort(&self) -> Self {
        Self {
            node: Node::Sort(self.node.clone().into()),
        }
    }

    pub fn keys(&self) -> Self {
        Self {
            node: Node::Keys(self.node.clone().into()),
        }
    }

    pub fn values(&self) -> Self {
        Self {
            node: Node::Values(self.node.clone().into()),
        }
    }

    pub fn to_array(&self) -> Self {
        Self {
            node: Node::ToArray(self.node.clone().into()),
        }
    }

    pub fn to_string(&self) -> Self {
        Self {
            node: Node::ToString(self.node.clone().into()),
        }
    }

    pub fn to_number(&self) -> Self {
        Self {
            node: Node::ToNumber(self.node.clone().into()),
        }
    }

    pub fn map_with(&self, build: Expr) -> Self {
        Self {
            node: Node::MapApply {
                base: self.node.clone().into(),
                key: build.node.into(),
            },
        }
    }

    pub fn sort_by(&self, key: Expr) -> Self {
        Self {
            node: Node::SortBy {
                base: self.node.clone().into(),
                key: key.node.into(),
            },
        }
    }

    pub fn min_by(&self, key: Expr) -> Self {
        Self {
            node: Node::MinBy {
                base: self.node.clone().into(),
                key: key.node.into(),
            },
        }
    }

    pub fn max_by(&self, key: Expr) -> Self {
        Self {
            node: Node::MaxBy {
                base: self.node.clone().into(),
                key: key.node.into(),
            },
        }
    }

    pub fn __repr__(&self) -> String {
        format!("Expr({:?})", self.node)
    }
}

#[pyfunction]
pub fn field(name: String) -> Expr {
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
            let key_str = key.extract::<String>()?;
            let value_expr = into_node(key.py(), &value)?;
            items.push((key_str, value_expr));
        }
    }

    Ok(Expr {
        node: Node::MultiDict(items),
    })
}
#[pyfunction]
pub fn lit(value: &Bound<'_, PyAny>) -> Expr {
    Python::with_gil(|py| Expr {
        node: Node::Literal(PyObjectWrapper(value.to_object(py))),
    })
}

#[pyfunction]
pub fn identity() -> Expr {
    Expr::new()
}
