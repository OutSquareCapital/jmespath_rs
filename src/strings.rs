use crate::exprs::{Expr, OpWrapper};
use crate::nodes::StrOp;
use pyo3::prelude::*;

#[pyclass(module = "dictexprs", name = "ExprStrNameSpace")]
pub struct ExprStrNameSpace {
    pub(crate) builder: OpWrapper<StrOp>,
}

#[pymethods]
impl ExprStrNameSpace {
    pub fn contains(&self, other: &str) -> Expr {
        self.builder.wrap(StrOp::Contains(other.to_string()))
    }

    pub fn reverse(&self) -> Expr {
        self.builder.wrap(StrOp::Reverse)
    }

    pub fn starts_with(&self, other: &str) -> Expr {
        self.builder.wrap(StrOp::StartsWith(other.to_string()))
    }

    pub fn ends_with(&self, other: &str) -> Expr {
        self.builder.wrap(StrOp::EndsWith(other.to_string()))
    }

    #[pyo3(signature = (start=None, end=None, step=None))]
    pub fn slice(&self, start: Option<isize>, end: Option<isize>, step: Option<isize>) -> Expr {
        self.builder.wrap(StrOp::Slice { start, end, step })
    }
}
