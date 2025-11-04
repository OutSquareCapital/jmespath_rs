use crate::exprs::Expr;
use crate::nodes::{into_lit, ListOp, Node};
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

    pub fn join(&self, glue: &str) -> Expr {
        Expr {
            node: Node::List(
                self.expr.node.clone().into(),
                ListOp::Join(glue.to_string()),
            ),
        }
    }

    pub fn map(&self, expr: &Expr) -> Expr {
        Expr {
            node: Node::List(
                self.expr.node.clone().into(),
                ListOp::Map(expr.node.clone().into()),
            ),
        }
    }

    pub fn contains(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Expr> {
        Ok(Expr {
            node: Node::List(
                self.expr.node.clone().into(),
                ListOp::Contains(into_lit(py, other)?.into()),
            ),
        })
    }

    pub fn filter(&self, cond: &Expr) -> Expr {
        Expr {
            node: Node::List(
                self.expr.node.clone().into(),
                ListOp::Filter(cond.node.clone().into()),
            ),
        }
    }

    pub fn sort_by(&self, key: &Expr) -> Expr {
        Expr {
            node: Node::List(
                self.expr.node.clone().into(),
                ListOp::SortBy(key.node.clone().into()),
            ),
        }
    }

    pub fn min_by(&self, key: &Expr) -> Expr {
        Expr {
            node: Node::List(
                self.expr.node.clone().into(),
                ListOp::MinBy(key.node.clone().into()),
            ),
        }
    }

    pub fn max_by(&self, key: &Expr) -> Expr {
        Expr {
            node: Node::List(
                self.expr.node.clone().into(),
                ListOp::MaxBy(key.node.clone().into()),
            ),
        }
    }
}
