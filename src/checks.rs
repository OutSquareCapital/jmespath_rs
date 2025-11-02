use pyo3::prelude::*;
use pyo3::types::*;

#[inline]
pub fn is_list(v: &Bound<'_, PyAny>) -> bool {
    v.is_instance_of::<PyList>() || v.is_instance_of::<PyTuple>()
}
#[inline]
pub fn is_sized(v: &Bound<'_, PyAny>) -> bool {
    v.len().is_ok()
}
#[inline]
pub fn is_object(v: &Bound<'_, PyAny>) -> bool {
    v.is_instance_of::<PyDict>()
}
#[inline]
pub fn is_number(v: &Bound<'_, PyAny>) -> bool {
    (v.is_instance_of::<PyFloat>() || v.is_instance_of::<PyLong>()) && !v.is_instance_of::<PyBool>()
}

#[inline]
pub fn is_comparable(v: &Bound<'_, PyAny>) -> bool {
    is_number(v) || v.is_instance_of::<PyUnicode>()
}
