use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::*;

#[inline]
pub fn is_list(v: &Bound<'_, PyAny>) -> PyResult<bool> {
    Ok(v.is_instance_of::<PyList>() || v.is_instance_of::<PyTuple>())
}

#[inline]
pub fn is_sized(v: &Bound<'_, PyAny>) -> bool {
    unsafe { pyo3::ffi::PySequence_Check(v.as_ptr()) == 1 }
}

#[inline]
pub fn is_number(v: &Bound<'_, PyAny>) -> PyResult<bool> {
    Ok(
        (v.is_instance_of::<PyFloat>() || v.is_instance_of::<PyLong>())
            && !v.is_instance_of::<PyBool>(),
    )
}

#[inline]
pub fn is_comparable(v: &Bound<'_, PyAny>) -> PyResult<bool> {
    Ok(is_number(v)? || v.is_instance_of::<PyUnicode>())
}

#[inline]
pub fn is_empty(v: &Bound<'_, PyAny>) -> PyResult<bool> {
    if v.is_none() {
        return Ok(true);
    }
    if v.is_instance_of::<PyBool>() && v.extract::<bool>()? == false {
        return Ok(true);
    }
    if v.is_instance_of::<PyUnicode>() && v.extract::<&str>()?.is_empty() {
        return Ok(true);
    }
    if is_list(v)? {
        let n = unsafe { pyo3::ffi::PySequence_Size(v.as_ptr()) };
        return Ok(n == 0);
    }
    if v.is_instance_of::<PyDict>() && v.downcast::<PyDict>()?.len() == 0 {
        return Ok(true);
    }
    Ok(false)
}

#[inline]
pub fn not_empty(v: &Bound<'_, PyAny>) -> PyResult<bool> {
    Ok(!is_empty(v)?)
}

#[inline]
pub fn eq_semantics(x: &Bound<'_, PyAny>, y: &Bound<'_, PyAny>) -> PyResult<bool> {
    let x_num = is_number(x)?;
    let y_num = is_number(y)?;
    if x_num || y_num {
        let x_bool = x.is_instance_of::<PyBool>();
        let y_bool = y.is_instance_of::<PyBool>();
        let x_is_01 = !x_bool
            && x.extract::<i64>()
                .ok()
                .map(|i| i == 0 || i == 1)
                .unwrap_or(false);
        let y_is_01 = !y_bool
            && y.extract::<i64>()
                .ok()
                .map(|i| i == 0 || i == 1)
                .unwrap_or(false);
        if (x_is_01 && y_bool) || (y_is_01 && x_bool) {
            return Ok(false);
        }
    }
    Ok(x.as_ref()
        .rich_compare(y.as_ref(), CompareOp::Eq)?
        .is_truthy()?)
}
