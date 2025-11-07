use pyo3::prelude::*;
use pyo3::types::*;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    List(Vec<Value>),
    Dict(HashMap<String, Value>),
}

impl Value {
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(l) => !l.is_empty(),
            Value::Dict(d) => !d.is_empty(),
        }
    }
    pub fn from_serde_value(v: serde_json::Value) -> PyResult<Self> {
        match v {
            serde_json::Value::Null => Ok(Value::Null),
            serde_json::Value::Bool(b) => Ok(Value::Bool(b)),
            serde_json::Value::Number(n) => {
                // `n.as_f64()` est requis car serde_json::Number peut être i64, u64, ou f64
                let f = n.as_f64().ok_or_else(|| {
                    pyo3::exceptions::PyValueError::new_err("Nombre JSON non représentable en f64")
                })?;
                Ok(Value::Number(f))
            }
            serde_json::Value::String(s) => Ok(Value::String(s)),
            serde_json::Value::Array(arr) => {
                let mut vec = Vec::with_capacity(arr.len());
                for item in arr {
                    // Appel récursif
                    vec.push(Value::from_serde_value(item)?);
                }
                Ok(Value::List(vec))
            }
            serde_json::Value::Object(map) => {
                let mut out_map = HashMap::new();
                for (k, v) in map {
                    // Appel récursif
                    out_map.insert(k, Value::from_serde_value(v)?);
                }
                Ok(Value::Dict(out_map))
            }
        }
    }
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&Vec<Value>> {
        match self {
            Value::List(l) => Some(l),
            _ => None,
        }
    }

    pub fn as_dict(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Value::Dict(d) => Some(d),
            _ => None,
        }
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    pub fn eq_strict(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Bool(_), Value::Number(_)) | (Value::Number(_), Value::Bool(_)) => false,
            _ => self == other,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::List(l) => {
                write!(f, "[")?;
                for (i, v) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Dict(d) => {
                write!(f, "{{")?;
                for (i, (k, v)) in d.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", k, v)?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl Value {
    pub fn from_python(obj: &Bound<'_, PyAny>) -> PyResult<Self> {
        if obj.is_none() {
            Ok(Value::Null)
        } else if let Ok(b) = obj.extract::<bool>() {
            Ok(Value::Bool(b))
        } else if let Ok(n) = obj.extract::<f64>() {
            Ok(Value::Number(n))
        } else if let Ok(s) = obj.extract::<String>() {
            Ok(Value::String(s))
        } else if let Ok(list) = obj.cast::<PyList>() {
            let mut vec = Vec::with_capacity(list.len());
            for item in list.iter() {
                vec.push(Value::from_python(&item)?);
            }
            Ok(Value::List(vec))
        } else if let Ok(dict) = obj.cast::<PyDict>() {
            let mut map = HashMap::new();
            for (key, value) in dict.iter() {
                let key_str = key.extract::<String>()?;
                map.insert(key_str, Value::from_python(&value)?);
            }
            Ok(Value::Dict(map))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Unsupported Python type",
            ))
        }
    }

    pub fn to_python<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        match self {
            Value::Null => Ok(py.None().into_bound(py)),
            Value::Bool(b) => Ok(PyBool::new(py, *b).to_owned().into_any()),
            Value::Number(n) => Ok(PyFloat::new(py, *n).into_any()),
            Value::String(s) => Ok(PyString::new(py, s).into_any()),
            Value::List(vec) => {
                let list = PyList::empty(py);
                for item in vec {
                    list.append(item.to_python(py)?)?;
                }
                Ok(list.into_any())
            }
            Value::Dict(map) => {
                let dict = PyDict::new(py);
                for (k, v) in map {
                    dict.set_item(k, v.to_python(py)?)?;
                }
                Ok(dict.into_any())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Node {
    This,
    Literal(Value),
    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),
    Not(Box<Node>),
    Coalesce(Vec<Node>),
    Merge(Vec<Node>),
    List(Box<Node>, ListOp),
    Str(Box<Node>, StrOp),
    Struct(Box<Node>, StructOp),
    Scalar(Box<Node>, ScalarOp),
    Compare(Box<Node>, ComparisonOp),
}

#[derive(Debug, Clone)]
pub(crate) enum ListOp {
    Index(isize),
    Slice {
        start: Option<isize>,
        end: Option<isize>,
        step: Option<isize>,
    },
    Length,
    Reverse,
    Flatten,
    Contains(Box<Node>),
    Filter(Box<Node>),
    Map(Box<Node>),
    Join(String),
    Sort,
    Max,
    Min,
    Sum,
    Avg,
    SortBy(Box<Node>),
    MinBy(Box<Node>),
    MaxBy(Box<Node>),
}

#[derive(Debug, Clone)]
pub(crate) enum StrOp {
    Slice {
        start: Option<isize>,
        end: Option<isize>,
        step: Option<isize>,
    },
    Reverse,
    Contains(String),
    StartsWith(String),
    EndsWith(String),
    Length,
}

#[derive(Debug, Clone)]
pub(crate) enum StructOp {
    Field(String),
    Keys,
    Values,
}

#[derive(Debug, Clone)]
pub(crate) enum ScalarOp {
    Abs,
    Ceil,
    Floor,
}

#[derive(Debug, Clone)]
pub(crate) enum ComparisonOp {
    Eq(Box<Node>),
    Ne(Box<Node>),
    Lt(Box<Node>),
    Le(Box<Node>),
    Gt(Box<Node>),
    Ge(Box<Node>),
}
