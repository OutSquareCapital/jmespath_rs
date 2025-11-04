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
                Ok(list) => match_list_op(py, value, list, op),
                Err(_) => Ok(py.None().into_bound(py)),
            }
        }
        Node::Str(base, op) => {
            let base_evaluated = match_any(py, base, value)?;
            match base_evaluated.downcast::<PyString>() {
                Ok(string) => match_str_op(py, string, op),
                Err(_) => Ok(py.None().into_bound(py)),
            }
        }
        Node::Struct(base, op) => {
            let base_evaluated = match_any(py, base, value)?;
            match base_evaluated.downcast::<PyDict>() {
                Ok(dict) => match_struct_op(py, value, dict, op),
                Err(_) => Ok(py.None().into_bound(py)),
            }
        }
        Node::Scalar(base, op) => {
            let base_evaluated = match_any(py, base, value)?;
            if !is_number(&base_evaluated) {
                return Ok(py.None().into_bound(py));
            }
            match_scalar_op(py, &base_evaluated, op)
        }
        Node::Compare(base, op) => match_comparison_op(py, value, base, op),
    }
}

fn match_list_op<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    list: &Bound<'py, PyList>,
    op: &ListOp,
) -> EvalResult<'py> {
    match op {
        ListOp::Index(i) => eval::list::index(py, list, *i),
        ListOp::Slice { start, end, step } => eval::list::slice(py, list, start, end, step),
        ListOp::Reverse => eval::list::reverse(py, list),
        ListOp::Flatten => eval::list::flatten(py, list),
        ListOp::Contains(search_node) => {
            eval::list::contains(py, list, &match_any(py, search_node, value)?)
        }
        ListOp::Join(glue) => eval::list::join(py, list, glue),
        ListOp::Filter(cond) => eval::list::filter(py, list, cond),
        ListOp::Map(key) => eval::list::map(py, list, key),
        ListOp::Sort => eval::list::sort(py, list),
        ListOp::Max => eval::list::min_max(py, list, true),
        ListOp::Min => eval::list::min_max(py, list, false),
        ListOp::Sum => eval::list::sum(py, list),
        ListOp::Avg => eval::list::avg(py, list),
        ListOp::SortBy(key) => eval::list::sort_by(py, list, key),
        ListOp::MinBy(key) => eval::list::min_by(py, list, key),
        ListOp::MaxBy(key) => eval::list::max_by(py, list, key),
    }
}

fn match_scalar_op<'py>(py: Python<'py>, number: &Bounded<'py>, op: &ScalarOp) -> EvalResult<'py> {
    match op {
        ScalarOp::Abs => eval::abs(py, number),
        ScalarOp::Ceil => eval::ceil(py, number),
        ScalarOp::Floor => eval::floor(py, number),
    }
}

fn match_comparison_op<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    base: &Node,
    op: &ComparisonOp,
) -> EvalResult<'py> {
    let base_evaluated = match_any(py, base, value)?;
    match op {
        ComparisonOp::Eq(other_node) => {
            eval::eq(py, &base_evaluated, &match_any(py, other_node, value)?)
        }
        ComparisonOp::Ne(other_node) => {
            eval::ne(py, &base_evaluated, &match_any(py, other_node, value)?)
        }
        ComparisonOp::Lt(other_node) => eval::cmp_bool(
            py,
            &base_evaluated,
            &match_any(py, other_node, value)?,
            CompareOp::Lt,
        ),
        ComparisonOp::Le(other_node) => eval::cmp_bool(
            py,
            &base_evaluated,
            &match_any(py, other_node, value)?,
            CompareOp::Le,
        ),
        ComparisonOp::Gt(other_node) => eval::cmp_bool(
            py,
            &base_evaluated,
            &match_any(py, other_node, value)?,
            CompareOp::Gt,
        ),
        ComparisonOp::Ge(other_node) => eval::cmp_bool(
            py,
            &base_evaluated,
            &match_any(py, other_node, value)?,
            CompareOp::Ge,
        ),
    }
}

fn match_str_op<'py>(
    py: Python<'py>,
    string: &Bound<'py, PyString>,
    op: &StrOp,
) -> EvalResult<'py> {
    match op {
        StrOp::Slice { start, end, step } => eval::strs::slice(py, string, start, end, step),
        StrOp::Reverse => eval::strs::reverse(py, string),
        StrOp::Contains(search) => eval::strs::contains(py, string, search),
        StrOp::StartsWith(prefix) => eval::strs::starts_with(py, string, prefix),
        StrOp::EndsWith(suffix) => eval::strs::ends_with(py, string, suffix),
    }
}
fn match_struct_op<'py>(
    py: Python<'py>,
    _value: &Bounded<'py>,
    dict: &Bound<'py, PyDict>,
    op: &StructOp,
) -> EvalResult<'py> {
    match op {
        StructOp::Field(name) => eval::structs::field(py, dict, name),
        StructOp::Keys => eval::structs::keys(dict),
        StructOp::Values => eval::structs::values(dict),
    }
}
