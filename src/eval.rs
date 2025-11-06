use crate::matchs::match_any;
use crate::nodes::{EvalResult, Node, Value};
pub mod list {
    use super::*;

    pub enum SortKind {
        SortBy,
        MinBy,
        MaxBy,
    }

    #[derive(PartialEq, PartialOrd)]
    struct SortKey(Option<f64>);

    impl Eq for SortKey {}
    impl Ord for SortKey {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0
                .partial_cmp(&other.0)
                .unwrap_or(std::cmp::Ordering::Equal)
        }
    }

    pub fn index(list: &[Value], i: isize) -> EvalResult {
        let len = list.len() as isize;
        let idx = if i < 0 { len + i } else { i };
        Ok(if idx < 0 || idx >= len {
            Value::Null
        } else {
            list[idx as usize].clone()
        })
    }

    pub fn length(list: &[Value]) -> EvalResult {
        Ok(Value::Number(list.len() as f64))
    }

    pub fn slice(
        list: &[Value],
        start: &Option<isize>,
        end: &Option<isize>,
        step: &Option<isize>,
    ) -> EvalResult {
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

        let result: Vec<Value> = if step > 0 {
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

        Ok(Value::List(result))
    }

    pub fn flatten(list: &[Value]) -> EvalResult {
        Ok(Value::List(list.iter().fold(Vec::new(), |mut acc, v| {
            match v {
                Value::List(inner) => acc.extend(inner.iter().cloned()),
                other => acc.push(other.clone()),
            }
            acc
        })))
    }

    pub fn filter(list: &[Value], cond: &Node) -> EvalResult {
        Ok(Value::List(
            list.iter()
                .filter_map(|v| match match_any(cond, v) {
                    Ok(res) if res.is_truthy() => Some(v.clone()),
                    _ => None,
                })
                .collect(),
        ))
    }

    pub fn map(list: &[Value], key: &Node) -> EvalResult {
        list.iter()
            .map(|v| match_any(key, v))
            .collect::<Result<Vec<_>, _>>()
            .map(Value::List)
    }

    pub fn sort(list: &[Value]) -> EvalResult {
        let mut sorted = list.to_vec();
        sorted.sort_by(|a, b| match (a, b) {
            (Value::Number(x), Value::Number(y)) => {
                x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
            }
            (Value::String(x), Value::String(y)) => x.cmp(y),
            _ => std::cmp::Ordering::Equal,
        });
        Ok(Value::List(sorted))
    }

    pub fn sort_like(list: &[Value], key: &Node, kind: SortKind) -> EvalResult {
        let mut pairs: Vec<_> = list
            .iter()
            .map(|v| {
                let key_val = match_any(key, v).unwrap_or(Value::Null);
                let (f, i, s) = (
                    key_val.as_number(),
                    key_val.as_number().map(|n| n as i64),
                    key_val.as_string().map(|s| s.to_string()),
                );
                let has = if f.is_some() || i.is_some() || s.is_some() {
                    0u8
                } else {
                    1
                };
                (has, SortKey(f), i, s, v.clone())
            })
            .collect();

        match kind {
            SortKind::SortBy => {
                pairs.sort_by(|a, b| {
                    (a.0, &a.1, a.2, a.3.as_deref()).cmp(&(b.0, &b.1, b.2, b.3.as_deref()))
                });
                Ok(Value::List(
                    pairs.into_iter().map(|(_, _, _, _, v)| v).collect(),
                ))
            }
            SortKind::MinBy => Ok(pairs
                .iter()
                .min_by(|a, b| {
                    (a.0, &a.1, a.2, a.3.as_deref()).cmp(&(b.0, &b.1, b.2, b.3.as_deref()))
                })
                .map(|m| m.4.clone())
                .unwrap_or(Value::Null)),
            SortKind::MaxBy => Ok(pairs
                .iter()
                .max_by(|a, b| {
                    (a.0, &a.1, a.2, a.3.as_deref()).cmp(&(b.0, &b.1, b.2, b.3.as_deref()))
                })
                .map(|m| m.4.clone())
                .unwrap_or(Value::Null)),
        }
    }

    pub fn sort_by(list: &[Value], key: &Node) -> EvalResult {
        sort_like(list, key, SortKind::SortBy)
    }

    pub fn min_by(list: &[Value], key: &Node) -> EvalResult {
        sort_like(list, key, SortKind::MinBy)
    }

    pub fn max_by(list: &[Value], key: &Node) -> EvalResult {
        sort_like(list, key, SortKind::MaxBy)
    }

    pub fn sum(list: &[Value]) -> EvalResult {
        if list.is_empty() {
            return Ok(Value::Number(0.0));
        }
        list.iter()
            .try_fold(0.0, |acc, v| {
                v.as_number()
                    .map(|n| acc + n)
                    .ok_or_else(|| "not a number".to_string())
            })
            .map(Value::Number)
            .or(Ok(Value::Null))
    }

    pub fn reverse(list: &[Value]) -> EvalResult {
        Ok(Value::List(list.iter().rev().cloned().collect()))
    }

    pub fn min_max(list: &[Value], is_max: bool) -> EvalResult {
        let mut iter = list.iter();
        let first = iter.next().ok_or_else(|| "empty list".to_string())?;
        let expect_num = first.is_number();
        let expect_str = first.is_string();

        if !expect_num && !expect_str {
            return Ok(Value::Null);
        }

        iter.try_fold(first, |best, cur| {
            if (expect_num && !cur.is_number()) || (expect_str && !cur.is_string()) {
                return Err("type mismatch".to_string());
            }
            Ok(match (best, cur) {
                (Value::Number(b), Value::Number(c)) => {
                    if (is_max && c > b) || (!is_max && c < b) {
                        cur
                    } else {
                        best
                    }
                }
                (Value::String(b), Value::String(c)) => {
                    if (is_max && c > b) || (!is_max && c < b) {
                        cur
                    } else {
                        best
                    }
                }
                _ => best,
            })
        })
        .map(|v| v.clone())
        .or(Ok(Value::Null))
    }

    pub fn join(list: &[Value], glue: &str) -> EvalResult {
        if list.iter().any(|v| !v.is_string()) {
            return Ok(Value::Null);
        }
        Ok(Value::String(
            list.iter()
                .filter_map(|v| v.as_string())
                .collect::<Vec<_>>()
                .join(glue),
        ))
    }

    pub fn avg(list: &[Value]) -> EvalResult {
        if list.is_empty() {
            return Ok(Value::Null);
        }
        sum(list).and_then(|s| match s {
            Value::Number(total) => Ok(Value::Number(total / list.len() as f64)),
            _ => Ok(Value::Null),
        })
    }

    pub fn contains(list: &[Value], search: &Value) -> EvalResult {
        Ok(Value::Bool(list.iter().any(|v| v.eq_strict(search))))
    }
}

pub mod structs {
    use super::*;
    use std::collections::HashMap;

    pub fn field(dict: &HashMap<String, Value>, name: &str) -> EvalResult {
        Ok(dict.get(name).cloned().unwrap_or(Value::Null))
    }

    pub fn keys(dict: &HashMap<String, Value>) -> EvalResult {
        Ok(Value::List(
            dict.keys().map(|k| Value::String(k.clone())).collect(),
        ))
    }

    pub fn values(dict: &HashMap<String, Value>) -> EvalResult {
        Ok(Value::List(dict.values().cloned().collect()))
    }
}

pub mod strs {
    use super::*;

    pub fn length(string: &str) -> EvalResult {
        Ok(Value::Number(string.chars().count() as f64))
    }

    pub fn contains(string: &str, search: &str) -> EvalResult {
        Ok(Value::Bool(string.contains(search)))
    }

    pub fn starts_with(string: &str, prefix: &str) -> EvalResult {
        Ok(Value::Bool(string.starts_with(prefix)))
    }

    pub fn ends_with(string: &str, suffix: &str) -> EvalResult {
        Ok(Value::Bool(string.ends_with(suffix)))
    }

    pub fn slice(
        string: &str,
        start: &Option<isize>,
        end: &Option<isize>,
        step: &Option<isize>,
    ) -> EvalResult {
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

        Ok(Value::String(result))
    }

    pub fn reverse(string: &str) -> EvalResult {
        Ok(Value::String(string.chars().rev().collect()))
    }
}

pub fn literal(value: &Value) -> EvalResult {
    Ok(value.clone())
}

pub fn and(value: &Value, a: &Node, b: &Node) -> EvalResult {
    let left = match_any(a, value)?;
    if left.is_truthy() {
        match_any(b, value)
    } else {
        Ok(left)
    }
}

pub fn or(value: &Value, a: &Node, b: &Node) -> EvalResult {
    let left = match_any(a, value)?;
    if left.is_truthy() {
        Ok(left)
    } else {
        match_any(b, value)
    }
}

pub fn not(value: &Value, x: &Node) -> EvalResult {
    Ok(Value::Bool(!match_any(x, value)?.is_truthy()))
}

pub fn cmp_bool(left: &Value, right: &Value, op: fn(f64, f64) -> bool) -> EvalResult {
    match (left.as_number(), right.as_number()) {
        (Some(l), Some(r)) => Ok(Value::Bool(op(l, r))),
        _ => Ok(Value::Bool(false)),
    }
}

pub fn abs(number: &Value) -> EvalResult {
    number
        .as_number()
        .map(|n| Value::Number(n.abs()))
        .ok_or_else(|| "not a number".to_string())
}

pub fn ceil(number: &Value) -> EvalResult {
    number
        .as_number()
        .map(|n| Value::Number(n.ceil()))
        .ok_or_else(|| "not a number".to_string())
}

pub fn floor(number: &Value) -> EvalResult {
    number
        .as_number()
        .map(|n| Value::Number(n.floor()))
        .ok_or_else(|| "not a number".to_string())
}

pub fn merge(value: &Value, items: &[Node]) -> EvalResult {
    use std::collections::HashMap;
    let mut output = HashMap::new();

    for item in items {
        match match_any(item, value)? {
            Value::Dict(dict) => output.extend(dict),
            _ => return Ok(Value::Null),
        }
    }

    Ok(Value::Dict(output))
}

pub fn coalesce(value: &Value, items: &[Node]) -> EvalResult {
    items
        .iter()
        .find_map(|item| match match_any(item, value) {
            Ok(v) if !v.is_null() => Some(Ok(v)),
            _ => None,
        })
        .unwrap_or(Ok(Value::Null))
}

pub fn eq(left: &Value, right: &Value) -> EvalResult {
    Ok(Value::Bool(left.eq_strict(right)))
}

pub fn ne(left: &Value, right: &Value) -> EvalResult {
    Ok(Value::Bool(!left.eq_strict(right)))
}
