use crate::exprs::Expr;
use crate::nodes::{Node, StrOp};
use pyo3::prelude::*;

#[pyclass(module = "dictexprs", name = "ExprStrNameSpace")]
pub struct ExprStrNameSpace {
    pub(crate) expr: Expr,
}

#[pymethods]
impl ExprStrNameSpace {
    pub fn contains(&self, other: &str) -> Expr {
        Expr {
            node: Node::Str(
                self.expr.node.clone().into(),
                StrOp::Contains(other.to_string()),
            ),
        }
    }
    pub fn reverse(&self) -> Expr {
        Expr {
            node: Node::Str(self.expr.node.clone().into(), StrOp::Reverse),
        }
    }

    pub fn starts_with(&self, other: &str) -> Expr {
        Expr {
            node: Node::Str(
                self.expr.node.clone().into(),
                StrOp::StartsWith(other.to_string()),
            ),
        }
    }

    pub fn ends_with(&self, other: &str) -> Expr {
        Expr {
            node: Node::Str(
                self.expr.node.clone().into(),
                StrOp::EndsWith(other.to_string()),
            ),
        }
    }
    #[pyo3(signature = (start=None, end=None, step=None))]
    pub fn slice(&self, start: Option<isize>, end: Option<isize>, step: Option<isize>) -> Expr {
        Expr {
            node: Node::Str(
                self.expr.node.clone().into(),
                StrOp::Slice { start, end, step },
            ),
        }
    }
}
