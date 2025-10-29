use jmespath::Variable;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyFloat, PyList, PyLong, PyString};
use serde_json::Number;
use std::rc::Rc;

pub fn new_error(msg: &str) -> PyErr {
    PyValueError::new_err(msg.to_string())
}
enum PyJsonType {
    None,
    String,
    Bool,
    Float,
    Long,
    List,
    Dict,
    Unknown,
}
fn get_py_json_type(data: &Bound<'_, PyAny>) -> PyJsonType {
    if data.is_none() {
        PyJsonType::None
    } else if data.is_instance_of::<PyString>() {
        PyJsonType::String
    } else if data.is_instance_of::<PyBool>() {
        PyJsonType::Bool
    } else if data.is_instance_of::<PyFloat>() {
        PyJsonType::Float
    } else if data.is_instance_of::<PyLong>() {
        PyJsonType::Long
    } else if data.is_instance_of::<PyList>() {
        PyJsonType::List
    } else if data.is_instance_of::<PyDict>() {
        PyJsonType::Dict
    } else {
        PyJsonType::Unknown
    }
}
pub fn py_to_variable(py: Python<'_>, data: &Bound<'_, PyAny>) -> PyResult<Variable> {
    match get_py_json_type(data) {
        PyJsonType::None => Ok(Variable::Null),
        PyJsonType::String => {
            let s = data.downcast::<PyString>()?;
            Ok(Variable::String(s.to_string()))
        }
        PyJsonType::Bool => {
            let b = data.downcast::<PyBool>()?;
            Ok(Variable::Bool(b.is_true()))
        }
        PyJsonType::Float => {
            let f = data.downcast::<PyFloat>()?;
            let num = Number::from_f64(f.value())
                .ok_or_else(|| new_error("Impossible de convertir f64 en serde_json::Number"))?;
            Ok(Variable::Number(num))
        }
        PyJsonType::Long => {
            let i = data.downcast::<PyLong>()?;
            let num = Number::from(i.extract::<i64>()?);
            Ok(Variable::Number(num))
        }
        PyJsonType::List => {
            let list = data.downcast::<PyList>()?;
            let mut vec: Vec<Rc<Variable>> = Vec::with_capacity(list.len());
            for item in list {
                vec.push(Rc::new(py_to_variable(py, &item)?));
            }
            Ok(Variable::Array(vec))
        }
        PyJsonType::Dict => {
            let dict = data.downcast::<PyDict>()?;
            let mut map: std::collections::BTreeMap<String, Rc<Variable>> =
                std::collections::BTreeMap::new();
            for (key, value) in dict {
                let key_str = key.extract::<String>()?;
                map.insert(key_str, Rc::new(py_to_variable(py, &value)?));
            }
            Ok(Variable::Object(map))
        }
        PyJsonType::Unknown => Ok(Variable::Null),
    }
}
pub fn variable_to_py(py: Python<'_>, var: &Variable) -> PyResult<Py<PyAny>> {
    match var {
        Variable::Null => Ok(py.None()),
        Variable::String(s) => Ok(s.to_object(py)),
        Variable::Bool(b) => Ok(b.to_object(py)),
        Variable::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.to_object(py))
            } else if let Some(f) = n.as_f64() {
                Ok(f.to_object(py))
            } else {
                Ok(n.to_string().to_object(py))
            }
        }
        Variable::Array(arr) => {
            let list = PyList::empty_bound(py);
            for item in arr {
                list.append(variable_to_py(py, item.as_ref())?)?;
            }
            Ok(list.to_object(py))
        }
        Variable::Object(map) => {
            let dict = PyDict::new_bound(py);
            for (key, value) in map {
                dict.set_item(key, variable_to_py(py, value.as_ref())?)?;
            }
            Ok(dict.to_object(py))
        }
        _ => Ok(py.None()),
    }
}
