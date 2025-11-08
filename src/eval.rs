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
