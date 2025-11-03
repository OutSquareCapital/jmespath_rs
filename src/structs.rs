use crate::exprs::Expr;
use crate::nodes::{Node, StructOp};
use pyo3::prelude::*;

#[pyclass(module = "dictexprs", name = "ExprStructNameSpace")]
pub struct ExprStructNameSpace {
    pub(crate) expr: Expr,
}

#[pymethods]
impl ExprStructNameSpace {
    pub fn field(&self, name: String) -> Expr {
        Expr {
            node: Node::Struct(self.expr.node.clone().into(), StructOp::Field(name)),
        }
    }

    pub fn keys(&self) -> Expr {
        Expr {
            node: Node::Struct(self.expr.node.clone().into(), StructOp::Keys),
        }
    }

    pub fn values(&self) -> Expr {
        Expr {
            node: Node::Struct(self.expr.node.clone().into(), StructOp::Values),
        }
    }
}
