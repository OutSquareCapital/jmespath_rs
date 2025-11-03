use crate::exprs::Expr;
use crate::nodes::{into_node_lit, Node, StrOp};
use pyo3::prelude::*;

#[pyclass(module = "dictexprs", name = "ExprStrNameSpace")]
pub struct ExprStrNameSpace {
    pub(crate) expr: Expr,
}

#[pymethods]
impl ExprStrNameSpace {
    pub fn contains(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Expr> {
        Ok(Expr {
            node: Node::Str(
                self.expr.node.clone().into(),
                StrOp::Contains(into_node_lit(py, other)?.into()),
            ),
        })
    }
    pub fn reverse(&self) -> Expr {
        Expr {
            node: Node::Str(self.expr.node.clone().into(), StrOp::Reverse),
        }
    }

    pub fn starts_with(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Expr> {
        Ok(Expr {
            node: Node::Str(
                self.expr.node.clone().into(),
                StrOp::StartsWith(into_node_lit(py, other)?.into()),
            ),
        })
    }

    pub fn ends_with(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Expr> {
        Ok(Expr {
            node: Node::Str(
                self.expr.node.clone().into(),
                StrOp::EndsWith(into_node_lit(py, other)?.into()),
            ),
        })
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
