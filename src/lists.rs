use crate::exprs::Expr;
use crate::nodes::{into_node, into_node_lit, ListOp, Node};
use pyo3::prelude::*;

#[pyclass(module = "dictexprs", name = "ExprListNameSpace")]
pub struct ExprListNameSpace {
    pub(crate) expr: Expr,
}

#[pymethods]
impl ExprListNameSpace {
    pub fn get(&self, i: isize) -> Expr {
        Expr {
            node: Node::List(self.expr.node.clone().into(), ListOp::Index(i)),
        }
    }

    #[pyo3(signature = (start=None, end=None, step=None))]
    pub fn slice(&self, start: Option<isize>, end: Option<isize>, step: Option<isize>) -> Expr {
        Expr {
            node: Node::List(
                self.expr.node.clone().into(),
                ListOp::Slice { start, end, step },
            ),
        }
    }

    pub fn flatten(&self) -> Expr {
        Expr {
            node: Node::List(self.expr.node.clone().into(), ListOp::Flatten),
        }
    }

    pub fn reverse(&self) -> Expr {
        Expr {
            node: Node::List(self.expr.node.clone().into(), ListOp::Reverse),
        }
    }

    pub fn sort(&self) -> Expr {
        Expr {
            node: Node::List(self.expr.node.clone().into(), ListOp::Sort),
        }
    }

    pub fn sum(&self) -> Expr {
        Expr {
            node: Node::List(self.expr.node.clone().into(), ListOp::Sum),
        }
    }

    pub fn min(&self) -> Expr {
        Expr {
            node: Node::List(self.expr.node.clone().into(), ListOp::Min),
        }
    }

    pub fn max(&self) -> Expr {
        Expr {
            node: Node::List(self.expr.node.clone().into(), ListOp::Max),
        }
    }

    pub fn avg(&self) -> Expr {
        Expr {
            node: Node::List(self.expr.node.clone().into(), ListOp::Avg),
        }
    }

    pub fn length(&self) -> Expr {
        Expr {
            node: Node::Length(self.expr.node.clone().into()),
        }
    }

    pub fn join(&self, py: Python<'_>, array: &Bound<'_, PyAny>) -> PyResult<Expr> {
        Ok(Expr {
            node: Node::List(
                self.expr.node.clone().into(),
                ListOp::Join(into_node_lit(py, array)?.into()),
            ),
        })
    }

    pub fn map(&self, py: Python<'_>, expr: &Bound<'_, PyAny>) -> PyResult<Expr> {
        Ok(Expr {
            node: Node::List(
                self.expr.node.clone().into(),
                ListOp::Map(into_node(py, expr)?.into()),
            ),
        })
    }

    pub fn contains(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Expr> {
        Ok(Expr {
            node: Node::List(
                self.expr.node.clone().into(),
                ListOp::Contains(into_node_lit(py, other)?.into()),
            ),
        })
    }

    pub fn filter(&self, py: Python<'_>, cond: &Bound<'_, PyAny>) -> PyResult<Expr> {
        Ok(Expr {
            node: Node::List(
                self.expr.node.clone().into(),
                ListOp::Filter(into_node(py, cond)?.into()),
            ),
        })
    }

    pub fn sort_by(&self, py: Python<'_>, key: &Bound<'_, PyAny>) -> PyResult<Expr> {
        Ok(Expr {
            node: Node::List(
                self.expr.node.clone().into(),
                ListOp::SortBy(into_node(py, key)?.into()),
            ),
        })
    }

    pub fn min_by(&self, py: Python<'_>, key: &Bound<'_, PyAny>) -> PyResult<Expr> {
        Ok(Expr {
            node: Node::List(
                self.expr.node.clone().into(),
                ListOp::MinBy(into_node(py, key)?.into()),
            ),
        })
    }

    pub fn max_by(&self, py: Python<'_>, key: &Bound<'_, PyAny>) -> PyResult<Expr> {
        Ok(Expr {
            node: Node::List(
                self.expr.node.clone().into(),
                ListOp::MaxBy(into_node(py, key)?.into()),
            ),
        })
    }
}
