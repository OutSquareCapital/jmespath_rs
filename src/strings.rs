use crate::exprs::Expr;
use crate::nodes::{into_node_lit, Node};
use pyo3::prelude::*;

#[pyclass(module = "dictexprs", name = "ExprStrNameSpace")]
pub struct ExprStrNameSpace {
    pub(crate) expr: Expr,
}

#[pymethods]
impl ExprStrNameSpace {
    pub fn contains(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Expr> {
        Ok(Expr {
            node: Node::StrContains(
                self.expr.node.clone().into(),
                into_node_lit(py, other)?.into(),
            ),
        })
    }
    pub fn reverse(&self) -> Expr {
        Expr {
            node: Node::StrReverse(self.expr.node.clone().into()),
        }
    }

    pub fn starts_with(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Expr> {
        Ok(Expr {
            node: Node::StartsWith(
                self.expr.node.clone().into(),
                into_node_lit(py, other)?.into(),
            ),
        })
    }

    pub fn ends_with(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Expr> {
        Ok(Expr {
            node: Node::EndsWith(
                self.expr.node.clone().into(),
                into_node_lit(py, other)?.into(),
            ),
        })
    }
    #[pyo3(signature = (start=None, end=None, step=None))]
    pub fn slice(&self, start: Option<isize>, end: Option<isize>, step: Option<isize>) -> Expr {
        Expr {
            node: Node::StrSlice {
                base: Box::new(self.expr.node.clone()),
                start,
                end,
                step,
            },
        }
    }
}
