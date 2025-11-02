use crate::exprs::Expr;
use crate::nodes::Node;
use pyo3::prelude::*;

#[pyclass(module = "dictexprs", name = "ExprStrNameSpace")]
pub struct ExprStrNameSpace {
    pub(crate) expr: Expr,
}

#[pymethods]
impl ExprStrNameSpace {
    pub fn contains(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Expr> {
        use crate::nodes::into_node_lit;
        Ok(Expr {
            node: Node::Contains(
                self.expr.node.clone().into(),
                into_node_lit(py, other)?.into(),
            ),
        })
    }

    pub fn starts_with(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Expr> {
        use crate::nodes::into_node_lit;
        Ok(Expr {
            node: Node::StartsWith(
                self.expr.node.clone().into(),
                into_node_lit(py, other)?.into(),
            ),
        })
    }

    pub fn ends_with(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Expr> {
        use crate::nodes::into_node_lit;
        Ok(Expr {
            node: Node::EndsWith(
                self.expr.node.clone().into(),
                into_node_lit(py, other)?.into(),
            ),
        })
    }
}
