use crate::{matchs::match_any, nodes::Node};
use serde_json as sd;

#[inline]
pub fn is_truthy(value: &sd::Value) -> bool {
    match value {
        sd::Value::Null => false,
        sd::Value::Bool(b) => *b,
        sd::Value::Number(n) => n.as_f64().unwrap() != 0.0,
        sd::Value::String(s) => !s.is_empty(),
        sd::Value::Array(a) => !a.is_empty(),
        sd::Value::Object(o) => !o.is_empty(),
    }
}

pub mod list {
    use super::*;

    #[inline]
    pub fn index(list: &[sd::Value], i: isize) -> sd::Value {
        let len = list.len() as isize;
        let idx = if i < 0 { len + i } else { i };
        list[idx as usize].clone()
    }

    #[inline]
    pub fn length(list: &[sd::Value]) -> sd::Value {
        sd::Value::Number(list.len().into())
    }

    #[inline]
    pub fn slice(
        list: &[sd::Value],
        start: &Option<isize>,
        end: &Option<isize>,
        step: &Option<isize>,
    ) -> sd::Value {
        let len = list.len() as isize;
        let step = step.unwrap_or(1);
        assert!(step != 0, "slice step cannot be zero");

        let start = start.unwrap_or(if step > 0 { 0 } else { len - 1 });
        let end = end.unwrap_or(if step > 0 { len } else { -len - 1 });

        let start = if start < 0 {
            (len + start).max(0)
        } else {
            start.min(len)
        };
        let end = if end < 0 {
            (len + end).max(-1)
        } else {
            end.min(len)
        };

        let result: Vec<sd::Value> = if step > 0 {
            (start..end)
                .step_by(step as usize)
                .map(|i| list[i as usize].clone())
                .collect()
        } else {
            (end + 1..=start)
                .rev()
                .step_by((-step) as usize)
                .map(|i| list[i as usize].clone())
                .collect()
        };

        sd::Value::Array(result)
    }

    #[inline]
    pub fn flatten(list: &[sd::Value]) -> sd::Value {
        sd::Value::Array(list.iter().fold(Vec::new(), |mut acc, v| {
            match v {
                sd::Value::Array(inner) => acc.extend(inner.iter().cloned()),
                other => acc.push(other.clone()),
            }
            acc
        }))
    }

    #[inline]
    pub fn filter<F>(list: &[sd::Value], predicate: F) -> sd::Value
    where
        F: Fn(&sd::Value) -> bool,
    {
        sd::Value::Array(list.iter().filter(|v| predicate(v)).cloned().collect())
    }

    #[inline]
    pub fn map<F>(list: &[sd::Value], transform: F) -> sd::Value
    where
        F: Fn(&sd::Value) -> sd::Value,
    {
        sd::Value::Array(list.iter().map(transform).collect())
    }

    #[inline]
    pub fn reverse(list: &[sd::Value]) -> sd::Value {
        sd::Value::Array(list.iter().rev().cloned().collect())
    }

    #[inline]
    pub fn join(list: &[sd::Value], glue: &str) -> sd::Value {
        sd::Value::String(
            list.iter()
                .map(|v| v.as_str().unwrap())
                .collect::<Vec<_>>()
                .join(glue),
        )
    }

    #[inline]
    pub fn contains(list: &[sd::Value], search: &sd::Value) -> sd::Value {
        sd::Value::Bool(list.contains(search))
    }
}

pub mod structs {
    use super::*;

    #[inline]
    pub fn field(dict: &sd::Map<String, sd::Value>, name: &str) -> sd::Value {
        dict[name].clone()
    }

    #[inline]
    pub fn keys(dict: &sd::Map<String, sd::Value>) -> sd::Value {
        sd::Value::Array(dict.keys().map(|k| sd::Value::String(k.clone())).collect())
    }

    #[inline]
    pub fn values(dict: &sd::Map<String, sd::Value>) -> sd::Value {
        sd::Value::Array(dict.values().cloned().collect())
    }
}

pub mod strs {
    use super::*;

    #[inline]
    pub fn length(string: &str) -> sd::Value {
        sd::Value::Number(string.chars().count().into())
    }

    #[inline]
    pub fn contains(string: &str, search: &str) -> sd::Value {
        sd::Value::Bool(string.contains(search))
    }

    #[inline]
    pub fn starts_with(string: &str, prefix: &str) -> sd::Value {
        sd::Value::Bool(string.starts_with(prefix))
    }

    #[inline]
    pub fn ends_with(string: &str, suffix: &str) -> sd::Value {
        sd::Value::Bool(string.ends_with(suffix))
    }

    #[inline]
    pub fn slice(
        string: &str,
        start: &Option<isize>,
        end: &Option<isize>,
        step: &Option<isize>,
    ) -> sd::Value {
        let chars: Vec<char> = string.chars().collect();
        let len = chars.len() as isize;
        let step = step.unwrap_or(1);
        assert!(step != 0, "slice step cannot be zero");

        let start = start.unwrap_or(if step > 0 { 0 } else { len - 1 });
        let end = end.unwrap_or(if step > 0 { len } else { -len - 1 });

        let start = if start < 0 {
            (len + start).max(0)
        } else {
            start.min(len)
        };
        let end = if end < 0 {
            (len + end).max(-1)
        } else {
            end.min(len)
        };

        let result: String = if step > 0 {
            (start..end)
                .step_by(step as usize)
                .map(|i| chars[i as usize])
                .collect()
        } else {
            (end + 1..=start)
                .rev()
                .step_by((-step) as usize)
                .map(|i| chars[i as usize])
                .collect()
        };

        sd::Value::String(result)
    }

    #[inline]
    pub fn reverse(string: &str) -> sd::Value {
        sd::Value::String(string.chars().rev().collect())
    }
}

#[inline]
pub fn literal(value: &sd::Value) -> sd::Value {
    value.clone()
}

#[inline]
pub fn eq(left: &sd::Value, right: &sd::Value) -> sd::Value {
    sd::Value::Bool(left == right)
}

#[inline]
pub fn ne(left: &sd::Value, right: &sd::Value) -> sd::Value {
    sd::Value::Bool(left != right)
}

#[inline]
pub fn cmp_bool(left: &sd::Value, right: &sd::Value, op: fn(f64, f64) -> bool) -> sd::Value {
    let l = left.as_f64().unwrap();
    let r = right.as_f64().unwrap();
    sd::Value::Bool(op(l, r))
}

#[inline]
pub fn coalesce(items: &[Node], value: &sd::Value) -> sd::Value {
    items
        .iter()
        .find_map(|item| match match_any(item, value) {
            Ok(v) if !v.is_null() => Some(v),
            _ => None,
        })
        .unwrap_or(sd::Value::Null)
}
