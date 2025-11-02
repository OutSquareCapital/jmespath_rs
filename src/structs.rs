use crate::exprs::Expr;
use crate::nodes::Node;
use pyo3::prelude::*;

#[pyclass(module = "dictexprs", name = "ExprStructNameSpace")]
pub struct ExprStructNameSpace {
    pub(crate) expr: Expr,
}

#[pymethods]
impl ExprStructNameSpace {
    pub fn field(&self, name: String) -> Expr {
        Expr {
            node: Node::SubExpr(self.expr.node.clone().into(), Node::Field(name).into()),
        }
    }

    pub fn keys(&self) -> Expr {
        Expr {
            node: Node::Keys(self.expr.node.clone().into()),
        }
    }

    pub fn values(&self) -> Expr {
        Expr {
            node: Node::Values(self.expr.node.clone().into()),
        }
    }
}
