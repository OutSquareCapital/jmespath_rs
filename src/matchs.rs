use crate::eval;
use crate::nodes::{ComparisonOp, EvalResult, ListOp, Node, ScalarOp, StrOp, StructOp, Value};

pub fn match_any(node: &Node, value: &Value) -> EvalResult {
    match node {
        Node::This => Ok(value.clone()),
        Node::Literal(obj) => eval::literal(obj),
        Node::And(a, b) => eval::and(value, a, b),
        Node::Or(a, b) => eval::or(value, a, b),
        Node::Not(x) => eval::not(value, x),
        Node::Coalesce(items) => eval::coalesce(value, items),
        Node::Merge(items) => eval::merge(value, items),
        Node::List(base, op) => op.eval(&match_any(base, value)?),
        Node::Str(base, op) => op.eval(&match_any(base, value)?),
        Node::Struct(base, op) => op.eval(&match_any(base, value)?),
        Node::Scalar(base, op) => op.eval(&match_any(base, value)?),
        Node::Compare(base, op) => op.eval(value, &match_any(base, value)?),
    }
}
impl ScalarOp {
    pub fn eval(&self, number: &Value) -> EvalResult {
        match self {
            Self::Abs => eval::abs(number),
            Self::Ceil => eval::ceil(number),
            Self::Floor => eval::floor(number),
        }
    }
}
impl ListOp {
    pub fn eval(&self, value: &Value) -> EvalResult {
        match value.as_list() {
            Some(list) => match self {
                Self::Length => eval::list::length(list),
                Self::Index(i) => eval::list::index(list, *i),
                Self::Slice { start, end, step } => eval::list::slice(list, start, end, step),
                Self::Reverse => eval::list::reverse(list),
                Self::Flatten => eval::list::flatten(list),
                Self::Contains(search_node) => {
                    eval::list::contains(list, &match_any(search_node, value)?)
                }
                Self::Join(glue) => eval::list::join(list, glue),
                Self::Filter(cond) => eval::list::filter(list, cond),
                Self::Map(key) => eval::list::map(list, key),
                Self::Sort => eval::list::sort(list),
                Self::Max => eval::list::min_max(list, true),
                Self::Min => eval::list::min_max(list, false),
                Self::Sum => eval::list::sum(list),
                Self::Avg => eval::list::avg(list),
                Self::SortBy(key) => eval::list::sort_by(list, key),
                Self::MinBy(key) => eval::list::min_by(list, key),
                Self::MaxBy(key) => eval::list::max_by(list, key),
            },
            None => Ok(Value::Null),
        }
    }
}
impl StrOp {
    pub fn eval(&self, value: &Value) -> EvalResult {
        match value.as_string() {
            Some(string) => match self {
                Self::Slice { start, end, step } => eval::strs::slice(string, start, end, step),
                Self::Reverse => eval::strs::reverse(string),
                Self::Contains(search) => eval::strs::contains(string, search),
                Self::StartsWith(prefix) => eval::strs::starts_with(string, prefix),
                Self::EndsWith(suffix) => eval::strs::ends_with(string, suffix),
                Self::Length => eval::strs::length(string),
            },
            None => Ok(Value::Null),
        }
    }
}
impl StructOp {
    pub fn eval(&self, value: &Value) -> EvalResult {
        match value.as_dict() {
            Some(dict) => match self {
                Self::Field(name) => eval::structs::field(dict, name),
                Self::Keys => eval::structs::keys(dict),
                Self::Values => eval::structs::values(dict),
            },
            None => Ok(Value::Null),
        }
    }
}
impl ComparisonOp {
    pub fn eval(&self, value: &Value, base_evaluated: &Value) -> EvalResult {
        match self {
            Self::Eq(other_node) => eval::eq(base_evaluated, &match_any(other_node, value)?),
            Self::Ne(other_node) => eval::ne(base_evaluated, &match_any(other_node, value)?),
            Self::Lt(other_node) => {
                eval::cmp_bool(base_evaluated, &match_any(other_node, value)?, |l, r| l < r)
            }
            Self::Le(other_node) => {
                eval::cmp_bool(base_evaluated, &match_any(other_node, value)?, |l, r| {
                    l <= r
                })
            }
            Self::Gt(other_node) => {
                eval::cmp_bool(base_evaluated, &match_any(other_node, value)?, |l, r| l > r)
            }
            Self::Ge(other_node) => {
                eval::cmp_bool(base_evaluated, &match_any(other_node, value)?, |l, r| {
                    l >= r
                })
            }
        }
    }
}
