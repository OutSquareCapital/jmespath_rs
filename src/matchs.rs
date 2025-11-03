use crate::checks::*;
use crate::eval;
use crate::nodes::{Bounded, ComparisonOp, EvalResult, ListOp, Node, ScalarOp, StrOp, StructOp};
use pyo3::basic::CompareOp;
use pyo3::prelude::*;

pub fn match_any<'py>(py: Python<'py>, node: &Node, value: &Bounded<'py>) -> EvalResult<'py> {
    match node {
        Node::This => Ok(value.clone()),
        Node::Literal(obj) => eval::literal(py, obj),
        Node::And(a, b) => eval::and(py, value, a, b),
        Node::Or(a, b) => eval::or(py, value, a, b),
        Node::Not(x) => eval::not(py, value, x),
        Node::NotNull(items) => eval::not_null(py, value, items),
        Node::Length(x) => eval::length(py, value, x),
        Node::Merge(items) => eval::merge(py, value, items),
        Node::List(base, op) => {
            let base_evaluated = match_any(py, base, value)?;
            if !is_list(&base_evaluated) {
                return Ok(py.None().into_bound(py));
            }
            match_list_op(py, value, &base_evaluated, op)
        }
        Node::Str(base, op) => {
            let base_evaluated = match_any(py, base, value)?;
            if !is_string(&base_evaluated) {
                return Ok(py.None().into_bound(py));
            }
            match_str_op(py, value, &base_evaluated, op)
        }
        Node::Struct(base, op) => {
            let base_evaluated = match_any(py, base, value)?;
            if !is_object(&base_evaluated) {
                return Ok(py.None().into_bound(py));
            }
            match_struct_op(py, value, &base_evaluated, op)
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
    list: &Bounded<'py>,
    op: &ListOp,
) -> EvalResult<'py> {
    match op {
        ListOp::Index(i) => eval::list_index(py, list, *i),
        ListOp::Slice { start, end, step } => eval::list_slice(py, list, start, end, step),
        ListOp::Reverse => eval::list_reverse(py, list),
        ListOp::Flatten => eval::list_flatten(py, list),
        ListOp::Contains(search_node) => {
            eval::list_contains(py, list, &match_any(py, search_node, value)?)
        }
        ListOp::Join(glue_node) => eval::list_join(py, &match_any(py, glue_node, value)?, list),
        ListOp::Filter(cond) => eval::list_filter(py, list, cond),
        ListOp::Map(key) => eval::list_map(py, list, key),
        ListOp::Sort => eval::list_sort(py, list),
        ListOp::Max => eval::list_min_max(py, list, true),
        ListOp::Min => eval::list_min_max(py, list, false),
        ListOp::Sum => eval::list_sum(py, list),
        ListOp::Avg => eval::list_avg(py, list),
        ListOp::SortBy(key) => eval::list_sort_by(py, list, key),
        ListOp::MinBy(key) => eval::list_min_by(py, list, key),
        ListOp::MaxBy(key) => eval::list_max_by(py, list, key),
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
    value: &Bounded<'py>,
    string: &Bounded<'py>,
    op: &StrOp,
) -> EvalResult<'py> {
    match op {
        StrOp::Slice { start, end, step } => eval::str_slice(py, string, start, end, step),
        StrOp::Reverse => eval::str_reverse(py, string),
        StrOp::Contains(search_node) => {
            eval::str_contains(py, string, &match_any(py, search_node, value)?)
        }
        StrOp::StartsWith(prefix_node) => {
            eval::starts_with(py, string, &match_any(py, prefix_node, value)?)
        }
        StrOp::EndsWith(suffix_node) => {
            eval::ends_with(py, string, &match_any(py, suffix_node, value)?)
        }
    }
}
fn match_struct_op<'py>(
    py: Python<'py>,
    _value: &Bounded<'py>,
    dict: &Bounded<'py>,
    op: &StructOp,
) -> EvalResult<'py> {
    match op {
        StructOp::Field(name) => eval::field(py, dict, name),
        StructOp::Keys => eval::keys(dict),
        StructOp::Values => eval::values(dict),
    }
}
