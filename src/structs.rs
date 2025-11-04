use crate::exprs::{Expr, OpWrapper};
use crate::nodes::StructOp;
use pyo3::prelude::*;

#[pyclass(module = "dictexprs", name = "ExprStructNameSpace")]
pub struct ExprStructNameSpace {
    pub(crate) builder: OpWrapper<StructOp>,
}

#[pymethods]
impl ExprStructNameSpace {
    pub fn field(&self, name: String) -> Expr {
        self.builder.wrap(StructOp::Field(name))
    }

    pub fn keys(&self) -> Expr {
        self.builder.wrap(StructOp::Keys)
    }

    pub fn values(&self) -> Expr {
        self.builder.wrap(StructOp::Values)
    }
}
