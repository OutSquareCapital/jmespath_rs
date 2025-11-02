use crate::exprs::Expr;
use crate::nodes::{into_node, Node};
use pyo3::prelude::*;

#[pyclass(module = "dictexprs", name = "ExprListNameSpace")]
pub struct ExprListNameSpace {
    pub(crate) expr: Expr,
}

#[pymethods]
impl ExprListNameSpace {
    pub fn get(&self, i: isize) -> Expr {
        Expr {
            node: Node::SubExpr(self.expr.node.clone().into(), Node::Index(i).into()),
        }
    }

    #[pyo3(signature = (start=None, end=None, step=None))]
    pub fn slice(&self, start: Option<isize>, end: Option<isize>, step: Option<isize>) -> Expr {
        Expr {
            node: Node::SubExpr(
                self.expr.node.clone().into(),
                Node::Slice(start, end, step).into(),
            ),
        }
    }

    pub fn flatten(&self) -> Expr {
        Expr {
            node: Node::Flatten(self.expr.node.clone().into()),
        }
    }

    pub fn reverse(&self) -> Expr {
        Expr {
            node: Node::Reverse(self.expr.node.clone().into()),
        }
    }

    pub fn sort(&self) -> Expr {
        Expr {
            node: Node::Sort(self.expr.node.clone().into()),
        }
    }

    pub fn sum(&self) -> Expr {
        Expr {
            node: Node::Sum(self.expr.node.clone().into()),
        }
    }

    pub fn min(&self) -> Expr {
        Expr {
            node: Node::Min(self.expr.node.clone().into()),
        }
    }

    pub fn max(&self) -> Expr {
        Expr {
            node: Node::Max(self.expr.node.clone().into()),
        }
    }

    pub fn avg(&self) -> Expr {
        Expr {
            node: Node::Avg(self.expr.node.clone().into()),
        }
    }

    pub fn lengths(&self) -> Expr {
        Expr {
            node: Node::Length(self.expr.node.clone().into()),
        }
    }

    pub fn join(&self, py: Python<'_>, glue: &Bound<'_, PyAny>) -> PyResult<Expr> {
        use crate::nodes::into_node_lit;
        Ok(Expr {
            node: Node::Join(
                into_node_lit(py, glue)?.into(),
                self.expr.node.clone().into(),
            ),
        })
    }

    pub fn eval(&self, py: Python<'_>, expr: &Bound<'_, PyAny>) -> PyResult<Expr> {
        Ok(Expr {
            node: Node::MapApply {
                base: self.expr.node.clone().into(),
                key: into_node(py, expr)?.into(),
            },
        })
    }

    pub fn contains(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Expr> {
        use crate::nodes::into_node_lit;
        Ok(Expr {
            node: Node::Contains(
                self.expr.node.clone().into(),
                into_node_lit(py, other)?.into(),
            ),
        })
    }

    pub fn filter(&self, py: Python<'_>, cond: &Bound<'_, PyAny>) -> PyResult<Expr> {
        Ok(Expr {
            node: Node::FilterProjection {
                base: self.expr.node.clone().into(),
                then: Node::This.into(),
                cond: into_node(py, cond)?.into(),
            },
        })
    }
}
