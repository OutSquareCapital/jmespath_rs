use crate::matchs::match_any;
use crate::nodes::Node;
use serde_json as sd;

pub mod list {
    use super::*;

    pub fn index(list: &[sd::Value], i: isize) -> Result<sd::Value, String> {
        let len = list.len() as isize;
        let idx = if i < 0 { len + i } else { i };
        Ok(if idx < 0 || idx >= len {
            sd::Value::Null
        } else {
            list[idx as usize].clone()
        })
    }

    pub fn length(list: &[sd::Value]) -> Result<sd::Value, String> {
        Ok(sd::Value::Number(list.len().into()))
    }

    pub fn slice(
        list: &[sd::Value],
        start: &Option<isize>,
        end: &Option<isize>,
        step: &Option<isize>,
    ) -> Result<sd::Value, String> {
        let len = list.len() as isize;
        let step = step.unwrap_or(1);
        if step == 0 {
            return Err("slice step cannot be zero".to_string());
        }

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
                .filter_map(|i| list.get(i as usize).cloned())
                .collect()
        } else {
            (end + 1..=start)
                .rev()
                .step_by((-step) as usize)
                .filter_map(|i| list.get(i as usize).cloned())
                .collect()
        };

        Ok(sd::Value::Array(result))
    }

    pub fn flatten(list: &[sd::Value]) -> Result<sd::Value, String> {
        Ok(sd::Value::Array(list.iter().fold(
            Vec::new(),
            |mut acc, v| {
                match v {
                    sd::Value::Array(inner) => acc.extend(inner.iter().cloned()),
                    other => acc.push(other.clone()),
                }
                acc
            },
        )))
    }

    pub fn filter(list: &[sd::Value], cond: &Node) -> Result<sd::Value, String> {
        Ok(sd::Value::Array(
            list.iter()
                .filter_map(|v| match match_any(cond, v) {
                    Ok(res) if !res.is_null() => Some(v.clone()),
                    _ => None,
                })
                .collect(),
        ))
    }

    pub fn map(list: &[sd::Value], key: &Node) -> Result<sd::Value, String> {
        list.iter()
            .map(|v| match_any(key, v))
            .collect::<Result<Vec<_>, _>>()
            .map(sd::Value::Array)
    }

    pub fn reverse(list: &[sd::Value]) -> Result<sd::Value, String> {
        Ok(sd::Value::Array(list.iter().rev().cloned().collect()))
    }

    pub fn join(list: &[sd::Value], glue: &str) -> Result<sd::Value, String> {
        if list.iter().any(|v| !v.is_string()) {
            return Ok(sd::Value::Null);
        }
        Ok(sd::Value::String(
            list.iter()
                .filter_map(|v| v.as_str())
                .collect::<Vec<_>>()
                .join(glue),
        ))
    }

    pub fn contains(list: &[sd::Value], search: &sd::Value) -> Result<sd::Value, String> {
        Ok(sd::Value::Bool(list.iter().any(|v| v.eq(search))))
    }
}

pub mod structs {
    use super::*;

    pub fn field(dict: &sd::Map<String, sd::Value>, name: &str) -> Result<sd::Value, String> {
        Ok(dict.get(name).cloned().unwrap_or(sd::Value::Null))
    }

    pub fn keys(dict: &sd::Map<String, sd::Value>) -> Result<sd::Value, String> {
        Ok(sd::Value::Array(
            dict.keys().map(|k| sd::Value::String(k.clone())).collect(),
        ))
    }

    pub fn values(dict: &sd::Map<String, sd::Value>) -> Result<sd::Value, String> {
        Ok(sd::Value::Array(dict.values().cloned().collect()))
    }
}

pub mod strs {
    use super::*;

    pub fn length(string: &str) -> Result<sd::Value, String> {
        Ok(sd::Value::Number(string.chars().count().into()))
    }

    pub fn contains(string: &str, search: &str) -> Result<sd::Value, String> {
        Ok(sd::Value::Bool(string.contains(search)))
    }

    pub fn starts_with(string: &str, prefix: &str) -> Result<sd::Value, String> {
        Ok(sd::Value::Bool(string.starts_with(prefix)))
    }

    pub fn ends_with(string: &str, suffix: &str) -> Result<sd::Value, String> {
        Ok(sd::Value::Bool(string.ends_with(suffix)))
    }

    pub fn slice(
        string: &str,
        start: &Option<isize>,
        end: &Option<isize>,
        step: &Option<isize>,
    ) -> Result<sd::Value, String> {
        let chars: Vec<char> = string.chars().collect();
        let len = chars.len() as isize;
        let step = step.unwrap_or(1);
        if step == 0 {
            return Err("slice step cannot be zero".to_string());
        }

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
                .filter_map(|i| chars.get(i as usize))
                .collect()
        } else {
            (end + 1..=start)
                .rev()
                .step_by((-step) as usize)
                .filter_map(|i| chars.get(i as usize))
                .collect()
        };

        Ok(sd::Value::String(result))
    }

    pub fn reverse(string: &str) -> Result<sd::Value, String> {
        Ok(sd::Value::String(string.chars().rev().collect()))
    }
}

pub fn literal(value: &sd::Value) -> Result<sd::Value, String> {
    Ok(value.clone())
}

pub fn and(value: &sd::Value, a: &Node, b: &Node) -> Result<sd::Value, String> {
    let left = match_any(a, value)?;
    if !left.is_null() {
        match_any(b, value)
    } else {
        Ok(left)
    }
}

pub fn or(value: &sd::Value, a: &Node, b: &Node) -> Result<sd::Value, String> {
    let left = match_any(a, value)?;
    if !left.is_null() {
        Ok(left)
    } else {
        match_any(b, value)
    }
}

pub fn not(value: &sd::Value, x: &Node) -> Result<sd::Value, String> {
    Ok(sd::Value::Bool(match_any(x, value)?.is_null()))
}

pub fn cmp_bool(
    left: &sd::Value,
    right: &sd::Value,
    op: fn(&sd::Number, &sd::Number) -> bool,
) -> Result<sd::Value, String> {
    match (left.as_number(), right.as_number()) {
        (Some(l), Some(r)) => Ok(sd::Value::Bool(op(&l, &r))),
        _ => Ok(sd::Value::Bool(false)),
    }
}

pub fn coalesce(value: &sd::Value, items: &[Node]) -> Result<sd::Value, String> {
    items
        .iter()
        .find_map(|item| match match_any(item, value) {
            Ok(v) if !v.is_null() => Some(Ok(v)),
            _ => None,
        })
        .unwrap_or(Ok(sd::Value::Null))
}

pub fn eq(left: &sd::Value, right: &sd::Value) -> Result<sd::Value, String> {
    Ok(sd::Value::Bool(left.eq(right)))
}

pub fn ne(left: &sd::Value, right: &sd::Value) -> Result<sd::Value, String> {
    Ok(sd::Value::Bool(!left.eq(right)))
}
