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
    pub fn index<'a>(list: &'a [sd::Value], i: isize) -> &'a sd::Value {
        let len = list.len() as isize;
        let idx = if i < 0 { len + i } else { i };
        &list[idx as usize]
    }

    #[inline]
    pub fn length(list: &[sd::Value]) -> usize {
        list.len()
    }

    #[inline]
    pub fn slice<'a>(
        list: &'a [sd::Value],
        start: &Option<isize>,
        end: &Option<isize>,
        step: &Option<isize>,
    ) -> Vec<&'a sd::Value> {
        let len = list.len() as isize;
        let step = step.unwrap_or(1);

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

        let result: Vec<&sd::Value> = if step > 0 {
            (start..end)
                .step_by(step as usize)
                .map(|i| &list[i as usize])
                .collect()
        } else {
            (end + 1..=start)
                .rev()
                .step_by((-step) as usize)
                .map(|i| &list[i as usize])
                .collect()
        };

        result
    }

    #[inline]
    pub fn flatten<'a>(list: &'a [sd::Value]) -> Vec<&'a sd::Value> {
        let mut refs = Vec::new();
        for v in list {
            match v {
                sd::Value::Array(inner) => refs.extend(inner.iter()),
                other => refs.push(other),
            }
        }
        refs
    }

    #[inline]
    pub fn filter<'a, F>(list: &'a [sd::Value], predicate: F) -> Vec<&'a sd::Value>
    where
        F: Fn(&sd::Value) -> bool,
    {
        list.iter().filter(|v| predicate(v)).collect()
    }

    #[inline]
    pub fn reverse<'a>(list: &'a [sd::Value]) -> Vec<&'a sd::Value> {
        list.iter().rev().collect()
    }

    #[inline]
    pub fn map<F>(list: &[sd::Value], transform: F) -> Vec<sd::Value>
    where
        F: Fn(&sd::Value) -> sd::Value,
    {
        list.iter().map(transform).collect()
    }

    #[inline]
    pub fn join(list: &[sd::Value], glue: &str) -> String {
        list.iter()
            .map(|v| v.as_str().unwrap())
            .collect::<Vec<_>>()
            .join(glue)
    }

    #[inline]
    pub fn contains(list: &[sd::Value], search: &sd::Value) -> bool {
        list.contains(search)
    }
}

pub mod structs {
    use super::*;

    #[inline]
    pub fn field<'a>(dict: &'a sd::Map<String, sd::Value>, name: &str) -> &'a sd::Value {
        &dict[name]
    }

    #[inline]
    pub fn keys(dict: &sd::Map<String, sd::Value>) -> Vec<String> {
        dict.keys().cloned().collect()
    }

    #[inline]
    pub fn values<'a>(dict: &'a sd::Map<String, sd::Value>) -> Vec<&'a sd::Value> {
        dict.values().collect()
    }
}

pub mod strs {

    #[inline]
    pub fn length(string: &str) -> usize {
        string.chars().count()
    }

    #[inline]
    pub fn contains(string: &str, search: &str) -> bool {
        string.contains(search)
    }

    #[inline]
    pub fn starts_with(string: &str, prefix: &str) -> bool {
        string.starts_with(prefix)
    }

    #[inline]
    pub fn ends_with(string: &str, suffix: &str) -> bool {
        string.ends_with(suffix)
    }

    #[inline]
    pub fn reverse(string: &str) -> String {
        string.chars().rev().collect()
    }

    #[inline]
    pub fn slice(
        string: &str,
        start: &Option<isize>,
        end: &Option<isize>,
        step: &Option<isize>,
    ) -> String {
        let chars: Vec<char> = string.chars().collect();
        let len = chars.len() as isize;
        let step = step.unwrap_or(1);

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

        if step > 0 {
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
        }
    }
}

#[inline]
pub fn literal<'a>(value: &'a sd::Value) -> &'a sd::Value {
    value
}

#[inline]
pub fn eq(left: &sd::Value, right: &sd::Value) -> bool {
    left == right
}

#[inline]
pub fn ne(left: &sd::Value, right: &sd::Value) -> bool {
    left != right
}

#[inline]
pub fn cmp_bool(left: &sd::Value, right: &sd::Value, op: fn(f64, f64) -> bool) -> bool {
    let l = left.as_f64().unwrap();
    let r = right.as_f64().unwrap();
    op(l, r)
}

#[inline]
pub fn coalesce(items: &[Node], value: &sd::Value) -> Option<sd::Value> {
    for item in items {
        let result = match_any(item, value);
        if !result.is_null() {
            return Some(result);
        }
    }
    None
}
