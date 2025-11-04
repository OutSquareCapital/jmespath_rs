use crate::checks::*;
use crate::eval;
use crate::nodes::{Bounded, ComparisonOp, EvalResult, ListOp, Node, ScalarOp, StrOp, StructOp};
use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::*;

pub fn match_any<'py>(py: Python<'py>, node: &Node, value: &Bounded<'py>) -> EvalResult<'py> {
    match node {
        Node::This => Ok(value.clone()),
        Node::Literal(obj) => eval::literal(py, obj),
        Node::And(a, b) => eval::and(py, value, a, b),
        Node::Or(a, b) => eval::or(py, value, a, b),
        Node::Not(x) => eval::not(py, value, x),
        Node::Coalesce(items) => eval::coalesce(py, value, items),
        Node::Length(x) => eval::length(py, value, x),
        Node::Merge(items) => eval::merge(py, value, items),
        Node::List(base, op) => {
            let base_evaluated = match_any(py, base, value)?;
            match base_evaluated.downcast::<PyList>() {
                Ok(list) => op.eval(py, value, list),
                Err(_) => Ok(py.None().into_bound(py)),
            }
        }
        Node::Str(base, op) => {
            let base_evaluated = match_any(py, base, value)?;
            match base_evaluated.downcast::<PyString>() {
                Ok(string) => op.eval(py, string),
                Err(_) => Ok(py.None().into_bound(py)),
            }
        }
        Node::Struct(base, op) => {
            let base_evaluated = match_any(py, base, value)?;
            match base_evaluated.downcast::<PyDict>() {
                Ok(dict) => op.eval(py, value, dict),
                Err(_) => Ok(py.None().into_bound(py)),
            }
        }
        Node::Scalar(base, op) => {
            let base_evaluated = match_any(py, base, value)?;
            if !is_number(&base_evaluated) {
                return Ok(py.None().into_bound(py));
            }
            op.eval(py, &base_evaluated)
        }
        Node::Compare(base, op) => {
            let base_evaluated = match_any(py, base, value)?;
            op.eval(py, value, &base_evaluated)
        }
    }
}
impl ScalarOp {
    pub fn eval<'py>(&self, py: Python<'py>, number: &Bounded<'py>) -> EvalResult<'py> {
        match self {
            Self::Abs => eval::abs(py, number),
            Self::Ceil => eval::ceil(py, number),
            Self::Floor => eval::floor(py, number),
        }
    }
}
impl ListOp {
    pub fn eval<'py>(
        &self,
        py: Python<'py>,
        value: &Bounded<'py>,
        list: &Bound<'py, PyList>,
    ) -> EvalResult<'py> {
        match self {
            Self::Index(i) => eval::list::index(py, list, *i),
            Self::Slice { start, end, step } => eval::list::slice(py, list, start, end, step),
            Self::Reverse => eval::list::reverse(py, list),
            Self::Flatten => eval::list::flatten(py, list),
            Self::Contains(search_node) => {
                eval::list::contains(py, list, &match_any(py, search_node, value)?)
            }
            Self::Join(glue) => eval::list::join(py, list, glue),
            Self::Filter(cond) => eval::list::filter(py, list, cond),
            Self::Map(key) => eval::list::map(py, list, key),
            Self::Sort => eval::list::sort(py, list),
            Self::Max => eval::list::min_max(py, list, true),
            Self::Min => eval::list::min_max(py, list, false),
            Self::Sum => eval::list::sum(py, list),
            Self::Avg => eval::list::avg(py, list),
            Self::SortBy(key) => eval::list::sort_by(py, list, key),
            Self::MinBy(key) => eval::list::min_by(py, list, key),
            Self::MaxBy(key) => eval::list::max_by(py, list, key),
        }
    }
}
impl StrOp {
    pub fn eval<'py>(&self, py: Python<'py>, string: &Bound<'py, PyString>) -> EvalResult<'py> {
        match self {
            Self::Slice { start, end, step } => eval::strs::slice(py, string, start, end, step),
            Self::Reverse => eval::strs::reverse(py, string),
            Self::Contains(search) => eval::strs::contains(py, string, search),
            Self::StartsWith(prefix) => eval::strs::starts_with(py, string, prefix),
            Self::EndsWith(suffix) => eval::strs::ends_with(py, string, suffix),
        }
    }
}
impl StructOp {
    pub fn eval<'py>(
        &self,
        py: Python<'py>,
        _value: &Bounded<'py>,
        dict: &Bound<'py, PyDict>,
    ) -> EvalResult<'py> {
        match self {
            Self::Field(name) => eval::structs::field(py, dict, name),
            Self::Keys => eval::structs::keys(dict),
            Self::Values => eval::structs::values(dict),
        }
    }
}
impl ComparisonOp {
    pub fn eval<'py>(
        &self,
        py: Python<'py>,
        value: &Bounded<'py>,
        base_evaluated: &Bounded<'py>,
    ) -> EvalResult<'py> {
        match self {
            Self::Eq(other_node) => {
                eval::eq(py, base_evaluated, &match_any(py, other_node, value)?)
            }
            Self::Ne(other_node) => {
                eval::ne(py, base_evaluated, &match_any(py, other_node, value)?)
            }
            Self::Lt(other_node) => eval::cmp_bool(
                py,
                base_evaluated,
                &match_any(py, other_node, value)?,
                CompareOp::Lt,
            ),
            Self::Le(other_node) => eval::cmp_bool(
                py,
                base_evaluated,
                &match_any(py, other_node, value)?,
                CompareOp::Le,
            ),
            Self::Gt(other_node) => eval::cmp_bool(
                py,
                base_evaluated,
                &match_any(py, other_node, value)?,
                CompareOp::Gt,
            ),
            Self::Ge(other_node) => eval::cmp_bool(
                py,
                base_evaluated,
                &match_any(py, other_node, value)?,
                CompareOp::Ge,
            ),
        }
    }
}
