use crate::eval;
use crate::nodes::{ComparisonOp, ListOp, Node, StrOp, StructOp};
use serde_json as sd;

pub fn match_any(node: &Node, value: &sd::Value) -> Result<sd::Value, String> {
    match node {
        Node::This => Ok(value.clone()),
        Node::Literal(obj) => eval::literal(obj),
        Node::And(a, b) => eval::and(value, a, b),
        Node::Or(a, b) => eval::or(value, a, b),
        Node::Not(x) => eval::not(value, x),
        Node::Coalesce(items) => eval::coalesce(value, items),
        Node::List(base, op) => op.eval(&match_any(base, value)?),
        Node::Str(base, op) => op.eval(&match_any(base, value)?),
        Node::Struct(base, op) => op.eval(&match_any(base, value)?),
        Node::Compare(base, op) => op.eval(value, &match_any(base, value)?),
    }
}
impl ListOp {
    pub fn eval(&self, value: &sd::Value) -> Result<sd::Value, String> {
        match value.as_array() {
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
            },
            None => Ok(sd::Value::Null),
        }
    }
}
impl StrOp {
    pub fn eval(&self, value: &sd::Value) -> Result<sd::Value, String> {
        match value.as_str() {
            Some(string) => match self {
                Self::Slice { start, end, step } => eval::strs::slice(string, start, end, step),
                Self::Reverse => eval::strs::reverse(string),
                Self::Contains(search) => eval::strs::contains(string, search),
                Self::StartsWith(prefix) => eval::strs::starts_with(string, prefix),
                Self::EndsWith(suffix) => eval::strs::ends_with(string, suffix),
                Self::Length => eval::strs::length(string),
            },
            None => Ok(sd::Value::Null),
        }
    }
}
impl StructOp {
    pub fn eval(&self, value: &sd::Value) -> Result<sd::Value, String> {
        match value.as_object() {
            Some(dict) => match self {
                Self::Field(name) => eval::structs::field(dict, name),
                Self::Keys => eval::structs::keys(dict),
                Self::Values => eval::structs::values(dict),
            },
            None => Ok(sd::Value::Null),
        }
    }
}
impl ComparisonOp {
    pub fn eval(&self, value: &sd::Value, base_evaluated: &sd::Value) -> Result<sd::Value, String> {
        match self {
            Self::Eq(other_node) => eval::eq(base_evaluated, &match_any(other_node, value)?),
            Self::Ne(other_node) => eval::ne(base_evaluated, &match_any(other_node, value)?),
            Self::Lt(other_node) => {
                eval::cmp_bool(base_evaluated, &match_any(other_node, value)?, |l, r| {
                    l.as_i64() < r.as_i64()
                })
            }
            Self::Le(other_node) => {
                eval::cmp_bool(base_evaluated, &match_any(other_node, value)?, |l, r| {
                    l.as_i64() <= r.as_i64()
                })
            }
            Self::Gt(other_node) => {
                eval::cmp_bool(base_evaluated, &match_any(other_node, value)?, |l, r| {
                    l.as_i64() > r.as_i64()
                })
            }
            Self::Ge(other_node) => {
                eval::cmp_bool(base_evaluated, &match_any(other_node, value)?, |l, r| {
                    l.as_i64() >= r.as_i64()
                })
            }
        }
    }
}
