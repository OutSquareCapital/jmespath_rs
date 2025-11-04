use crate::exprs::{Expr, OpWrapper};
use crate::nodes::{into_lit, ListOp};
use pyo3::prelude::*;

#[pyclass(module = "dictexprs", name = "ExprListNameSpace")]
pub struct ExprListNameSpace {
    pub(crate) builder: OpWrapper<ListOp>,
}

#[pymethods]
impl ExprListNameSpace {
    pub fn get(&self, i: isize) -> Expr {
        self.builder.wrap(ListOp::Index(i))
    }

    #[pyo3(signature = (start=None, end=None, step=None))]
    pub fn slice(&self, start: Option<isize>, end: Option<isize>, step: Option<isize>) -> Expr {
        self.builder.wrap(ListOp::Slice { start, end, step })
    }

    pub fn flatten(&self) -> Expr {
        self.builder.wrap(ListOp::Flatten)
    }

    pub fn reverse(&self) -> Expr {
        self.builder.wrap(ListOp::Reverse)
    }
    pub fn sort(&self) -> Expr {
        self.builder.wrap(ListOp::Sort)
    }

    pub fn sum(&self) -> Expr {
        self.builder.wrap(ListOp::Sum)
    }

    pub fn min(&self) -> Expr {
        self.builder.wrap(ListOp::Min)
    }

    pub fn max(&self) -> Expr {
        self.builder.wrap(ListOp::Max)
    }

    pub fn avg(&self) -> Expr {
        self.builder.wrap(ListOp::Avg)
    }

    pub fn length(&self) -> Expr {
        self.builder.wrap(ListOp::Length)
    }

    pub fn join(&self, glue: &str) -> Expr {
        self.builder.wrap(ListOp::Join(glue.to_string()))
    }

    pub fn map(&self, expr: &Expr) -> Expr {
        self.builder.wrap(ListOp::Map(expr.node.clone().into()))
    }

    pub fn contains(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Expr> {
        Ok(self
            .builder
            .wrap(ListOp::Contains(into_lit(py, other)?.into())))
    }
    pub fn filter(&self, cond: &Expr) -> Expr {
        self.builder.wrap(ListOp::Filter(cond.node.clone().into()))
    }
    pub fn sort_by(&self, key: &Expr) -> Expr {
        self.builder.wrap(ListOp::SortBy(key.node.clone().into()))
    }

    pub fn min_by(&self, key: &Expr) -> Expr {
        self.builder.wrap(ListOp::MinBy(key.node.clone().into()))
    }

    pub fn max_by(&self, key: &Expr) -> Expr {
        self.builder.wrap(ListOp::MaxBy(key.node.clone().into()))
    }
}
