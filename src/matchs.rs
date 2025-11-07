use crate::eval;
use crate::nodes::{ComparisonOp, ListOp, Node, StrOp, StructOp};
use serde_json as sd;

pub fn match_any(node: &Node, value: &sd::Value) -> sd::Value {
    match node {
        Node::This => value.clone(),
        Node::Literal(obj) => eval::literal(obj).clone(),
        Node::And(a, b) => {
            let left = match_any(a, value);
            if eval::is_truthy(&left) {
                match_any(b, value)
            } else {
                left
            }
        }
        Node::Or(a, b) => {
            let left = match_any(a, value);
            if eval::is_truthy(&left) {
                left
            } else {
                match_any(b, value)
            }
        }
        Node::Not(x) => sd::Value::Bool(!eval::is_truthy(&match_any(x, value))),
        Node::Coalesce(items) => eval::coalesce(items, value).unwrap_or(sd::Value::Null),
        Node::List(base, op) => op.eval(&match_any(base, value), value),
        Node::Str(base, op) => op.eval(&match_any(base, value)),
        Node::Struct(base, op) => op.eval(&match_any(base, value)),
        Node::Compare(base, op) => op.eval(value, &match_any(base, value)),
    }
}

impl ListOp {
    pub fn eval(&self, value: &sd::Value, context: &sd::Value) -> sd::Value {
        let list = value.as_array().unwrap();
        match self {
            Self::Length => sd::Value::Number(eval::list::length(list).into()),
            Self::Index(i) => eval::list::index(list, *i).clone(),
            Self::Slice { start, end, step } => sd::Value::Array(
                eval::list::slice(list, start, end, step)
                    .into_iter()
                    .cloned()
                    .collect(),
            ),
            Self::Reverse => {
                sd::Value::Array(eval::list::reverse(list).into_iter().cloned().collect())
            }
            Self::Flatten => {
                sd::Value::Array(eval::list::flatten(list).into_iter().cloned().collect())
            }
            Self::Contains(search_node) => {
                let search = match_any(search_node, context);
                sd::Value::Bool(eval::list::contains(list, &search))
            }
            Self::Join(glue) => sd::Value::String(eval::list::join(list, glue)),
            Self::Filter(cond) => {
                let filtered =
                    eval::list::filter(list, |item| eval::is_truthy(&match_any(cond, item)));
                sd::Value::Array(filtered.into_iter().cloned().collect())
            }
            Self::Map(key) => {
                let mapped = eval::list::map(list, |item| match_any(key, item));
                sd::Value::Array(mapped)
            }
        }
    }
}

impl StrOp {
    pub fn eval(&self, value: &sd::Value) -> sd::Value {
        let string = value.as_str().unwrap();
        match self {
            Self::Slice { start, end, step } => {
                sd::Value::String(eval::strs::slice(string, start, end, step))
            }
            Self::Reverse => sd::Value::String(eval::strs::reverse(string)),
            Self::Contains(search) => sd::Value::Bool(eval::strs::contains(string, search)),
            Self::StartsWith(prefix) => sd::Value::Bool(eval::strs::starts_with(string, prefix)),
            Self::EndsWith(suffix) => sd::Value::Bool(eval::strs::ends_with(string, suffix)),
            Self::Length => sd::Value::Number(eval::strs::length(string).into()),
        }
    }
}

impl StructOp {
    pub fn eval(&self, value: &sd::Value) -> sd::Value {
        let dict = value.as_object().unwrap();
        match self {
            Self::Field(name) => eval::structs::field(dict, name).clone(),
            Self::Keys => {
                let keys = eval::structs::keys(dict);
                sd::Value::Array(keys.into_iter().map(sd::Value::String).collect())
            }
            Self::Values => {
                let values = eval::structs::values(dict);
                sd::Value::Array(values.into_iter().cloned().collect())
            }
        }
    }
}

impl ComparisonOp {
    pub fn eval(&self, value: &sd::Value, base_evaluated: &sd::Value) -> sd::Value {
        match self {
            Self::Eq(other_node) => {
                sd::Value::Bool(eval::eq(base_evaluated, &match_any(other_node, value)))
            }
            Self::Ne(other_node) => {
                sd::Value::Bool(eval::ne(base_evaluated, &match_any(other_node, value)))
            }
            Self::Lt(other_node) => sd::Value::Bool(eval::cmp_bool(
                base_evaluated,
                &match_any(other_node, value),
                |l, r| l < r,
            )),
            Self::Le(other_node) => sd::Value::Bool(eval::cmp_bool(
                base_evaluated,
                &match_any(other_node, value),
                |l, r| l <= r,
            )),
            Self::Gt(other_node) => sd::Value::Bool(eval::cmp_bool(
                base_evaluated,
                &match_any(other_node, value),
                |l, r| l > r,
            )),
            Self::Ge(other_node) => sd::Value::Bool(eval::cmp_bool(
                base_evaluated,
                &match_any(other_node, value),
                |l, r| l >= r,
            )),
        }
    }
}
