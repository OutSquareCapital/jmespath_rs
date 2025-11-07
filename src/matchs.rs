use crate::eval;
use crate::nodes::{ComparisonOp, ListOp, Node, StrOp, StructOp};
use serde_json as sd;

pub fn match_any(node: &Node, value: &sd::Value) -> Result<sd::Value, String> {
    Ok(match node {
        Node::This => value.clone(),
        Node::Literal(obj) => eval::literal(obj),
        Node::And(a, b) => {
            let left = match_any(a, value)?;
            if eval::is_truthy(&left) {
                match_any(b, value)?
            } else {
                left
            }
        }
        Node::Or(a, b) => {
            let left = match_any(a, value)?;
            if eval::is_truthy(&left) {
                left
            } else {
                match_any(b, value)?
            }
        }
        Node::Not(x) => sd::Value::Bool(!eval::is_truthy(&match_any(x, value)?)),
        Node::Coalesce(items) => eval::coalesce(items, value),
        Node::List(base, op) => op.eval(&match_any(base, value)?, value)?,
        Node::Str(base, op) => op.eval(&match_any(base, value)?)?,
        Node::Struct(base, op) => op.eval(&match_any(base, value)?)?,
        Node::Compare(base, op) => op.eval(value, &match_any(base, value)?)?,
    })
}

impl ListOp {
    pub fn eval(&self, value: &sd::Value, context: &sd::Value) -> Result<sd::Value, String> {
        let list = value.as_array().unwrap();
        Ok(match self {
            Self::Length => eval::list::length(list),
            Self::Index(i) => eval::list::index(list, *i),
            Self::Slice { start, end, step } => eval::list::slice(list, start, end, step),
            Self::Reverse => eval::list::reverse(list),
            Self::Flatten => eval::list::flatten(list),
            Self::Contains(search_node) => {
                let search = match_any(search_node, context)?;
                eval::list::contains(list, &search)
            }
            Self::Join(glue) => eval::list::join(list, glue),
            Self::Filter(cond) => eval::list::filter(list, |item| {
                match_any(cond, item)
                    .ok()
                    .map(|res| eval::is_truthy(&res))
                    .unwrap_or(false)
            }),
            Self::Map(key) => eval::list::map(list, |item| match_any(key, item).unwrap()),
        })
    }
}

impl StrOp {
    pub fn eval(&self, value: &sd::Value) -> Result<sd::Value, String> {
        let string = value.as_str().unwrap();
        Ok(match self {
            Self::Slice { start, end, step } => eval::strs::slice(string, start, end, step),
            Self::Reverse => eval::strs::reverse(string),
            Self::Contains(search) => eval::strs::contains(string, search),
            Self::StartsWith(prefix) => eval::strs::starts_with(string, prefix),
            Self::EndsWith(suffix) => eval::strs::ends_with(string, suffix),
            Self::Length => eval::strs::length(string),
        })
    }
}

impl StructOp {
    pub fn eval(&self, value: &sd::Value) -> Result<sd::Value, String> {
        let dict = value.as_object().unwrap();
        Ok(match self {
            Self::Field(name) => eval::structs::field(dict, name),
            Self::Keys => eval::structs::keys(dict),
            Self::Values => eval::structs::values(dict),
        })
    }
}

impl ComparisonOp {
    pub fn eval(&self, value: &sd::Value, base_evaluated: &sd::Value) -> Result<sd::Value, String> {
        Ok(match self {
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
        })
    }
}
