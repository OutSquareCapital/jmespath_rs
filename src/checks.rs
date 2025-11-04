use pyo3::prelude::*;
use pyo3::types::*;

#[inline]
pub fn is_sized(v: &Bound<'_, PyAny>) -> bool {
    v.len().is_ok()
}
#[inline]
pub fn is_number(v: &Bound<'_, PyAny>) -> bool {
    (v.is_instance_of::<PyFloat>() || v.is_instance_of::<PyLong>()) && !v.is_instance_of::<PyBool>()
}
#[inline]
pub fn is_string(v: &Bound<'_, PyAny>) -> bool {
    v.is_instance_of::<PyUnicode>()
}
#[inline]
pub fn is_eq(va: &Bound<'_, PyAny>, vb: &Bound<'_, PyAny>) -> PyResult<bool> {
    if (va.is_instance_of::<PyBool>() && is_number(vb))
        || (is_number(va) && vb.is_instance_of::<PyBool>())
    {
        return Ok(false);
    }
    va.eq(vb)
}
