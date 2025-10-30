use pyo3::prelude::*;

use crate::nodes::{into_node, Node};
#[pyclass(module = "jmespath_rs", name = "QueryBuilder")]
#[derive(Clone)]
pub struct QueryBuilder {
    pub node: Node,
}

#[pymethods]
impl QueryBuilder {
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
    pub fn pipe(&self, rhs: QueryBuilder) -> Self {
        Self {
            node: Node::Pipe(self.node.clone().into(), rhs.node.into()),
        }
    }
    pub fn project<'py>(&self, py: Python<'py>, rhs: &Bound<'py, PyAny>) -> PyResult<Self> {
        let rhs_node = into_node(py, rhs)?;
        Ok(Self {
            node: Node::ProjectArray {
                base: self.node.clone().into(),
                rhs: rhs_node.into(),
            },
        })
    }

    pub fn vproject<'py>(&self, py: Python<'py>, rhs: &Bound<'py, PyAny>) -> PyResult<Self> {
        let rhs_node = into_node(py, rhs)?;
        Ok(Self {
            node: Node::ProjectObject {
                base: self.node.clone().into(),
                rhs: rhs_node.into(),
            },
        })
    }

    pub fn filter<'py>(
        &self,
        py: Python<'py>,
        cond: QueryBuilder,
        then: &Bound<'py, PyAny>,
    ) -> PyResult<Self> {
        let then_node = into_node(py, then)?;
        Ok(Self {
            node: Node::FilterProjection {
                base: self.node.clone().into(),
                then: then_node.into(),
                cond: cond.node.into(),
            },
        })
    }

    pub fn flatten(&self) -> Self {
        Self {
            node: Node::Flatten(self.node.clone().into()),
        }
    }

    pub fn eq<'py>(&self, py: Python<'py>, rhs: &Bound<'py, PyAny>) -> PyResult<Self> {
        let r = into_node(py, rhs)?;
        Ok(Self {
            node: Node::CmpEq(self.node.clone().into(), r.into()),
        })
    }
    pub fn ne<'py>(&self, py: Python<'py>, rhs: &Bound<'py, PyAny>) -> PyResult<Self> {
        let r = into_node(py, rhs)?;
        Ok(Self {
            node: Node::CmpNe(self.node.clone().into(), r.into()),
        })
    }
    pub fn lt<'py>(&self, py: Python<'py>, rhs: &Bound<'py, PyAny>) -> PyResult<Self> {
        let r = into_node(py, rhs)?;
        Ok(Self {
            node: Node::CmpLt(self.node.clone().into(), r.into()),
        })
    }
    pub fn le<'py>(&self, py: Python<'py>, rhs: &Bound<'py, PyAny>) -> PyResult<Self> {
        let r = into_node(py, rhs)?;
        Ok(Self {
            node: Node::CmpLe(self.node.clone().into(), r.into()),
        })
    }
    pub fn gt<'py>(&self, py: Python<'py>, rhs: &Bound<'py, PyAny>) -> PyResult<Self> {
        let r = into_node(py, rhs)?;
        Ok(Self {
            node: Node::CmpGt(self.node.clone().into(), r.into()),
        })
    }
    pub fn ge<'py>(&self, py: Python<'py>, rhs: &Bound<'py, PyAny>) -> PyResult<Self> {
        let r = into_node(py, rhs)?;
        Ok(Self {
            node: Node::CmpGe(self.node.clone().into(), r.into()),
        })
    }

    pub fn and_<'py>(&self, py: Python<'py>, rhs: &Bound<'py, PyAny>) -> PyResult<Self> {
        let r = into_node(py, rhs)?;
        Ok(Self {
            node: Node::And(self.node.clone().into(), r.into()),
        })
    }
    pub fn or_<'py>(&self, py: Python<'py>, rhs: &Bound<'py, PyAny>) -> PyResult<Self> {
        let r = into_node(py, rhs)?;
        Ok(Self {
            node: Node::Or(self.node.clone().into(), r.into()),
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

    pub fn map_with(&self, build: QueryBuilder) -> Self {
        Self {
            node: Node::MapApply {
                base: self.node.clone().into(),
                key: build.node.into(),
            },
        }
    }
    pub fn sort_by(&self, key: QueryBuilder) -> Self {
        Self {
            node: Node::SortBy {
                base: self.node.clone().into(),
                key: key.node.into(),
            },
        }
    }
    pub fn min_by(&self, key: QueryBuilder) -> Self {
        Self {
            node: Node::MinBy {
                base: self.node.clone().into(),
                key: key.node.into(),
            },
        }
    }
    pub fn max_by(&self, key: QueryBuilder) -> Self {
        Self {
            node: Node::MaxBy {
                base: self.node.clone().into(),
                key: key.node.into(),
            },
        }
    }

    pub fn __repr__(&self) -> String {
        format!("QueryBuilder({:?})", self.node)
    }
}

#[pyfunction]
pub fn field(name: String) -> QueryBuilder {
    QueryBuilder {
        node: Node::Field(name).into(),
    }
}
#[pyfunction]
pub fn select_list(exprs: Vec<QueryBuilder>) -> QueryBuilder {
    QueryBuilder {
        node: Node::MultiList(exprs.into_iter().map(|q| q.node).collect()),
    }
}
#[pyfunction]
pub fn select_dict(items: Vec<(String, QueryBuilder)>) -> QueryBuilder {
    QueryBuilder {
        node: Node::MultiDict(items.into_iter().map(|(k, q)| (k, q.node)).collect()),
    }
}
#[pyfunction]
pub fn lit(value: &Bound<'_, PyAny>) -> PyResult<QueryBuilder> {
    Python::with_gil(|py| {
        let n = into_node(py, value)?;
        Ok(QueryBuilder { node: n })
    })
}
#[pyfunction]
pub fn identity() -> QueryBuilder {
    QueryBuilder::new()
}
