use crate::matchs::match_any;
use crate::nodes::{Bounded, EvalResult, Node, PyObjectWrapper};
use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::*;
pub mod pylibs {
    pub const BUILTINS: &str = "builtins";
    pub const SORTED: &str = "sorted";
    pub const JOIN: &str = "join";
    pub const JSON: &str = "json";
}
#[inline]
pub fn is_number(value: &Bound<'_, PyAny>) -> bool {
    (value.is_instance_of::<PyFloat>() || value.is_instance_of::<PyInt>())
        && !value.is_instance_of::<PyBool>()
}
#[inline]
fn is_string(value: &Bound<'_, PyAny>) -> bool {
    value.is_instance_of::<PyString>()
}
#[inline]
fn is_eq(left: &Bound<'_, PyAny>, right: &Bound<'_, PyAny>) -> PyResult<bool> {
    if (left.is_instance_of::<PyBool>() && is_number(right))
        || (is_number(left) && right.is_instance_of::<PyBool>())
    {
        return Ok(false);
    }
    left.eq(right)
}
#[inline]
fn not_eq(left: &Bound<'_, PyAny>, right: &Bound<'_, PyAny>) -> PyResult<bool> {
    Ok(!is_eq(left, right)?)
}

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
    pub fn index<'py>(py: Python<'py>, list: &Bound<'py, PyList>, i: isize) -> EvalResult<'py> {
        let len = list.len() as isize;
        let idx = if i < 0 { len + i } else { i };
        if idx < 0 || idx >= len {
            Ok(py.None().into_bound(py))
        } else {
            Ok(list.get_item(idx as usize)?)
        }
    }

    pub fn length<'py>(py: Python<'py>, list: &Bound<'py, PyList>) -> EvalResult<'py> {
        Ok(list.len().into_pyobject(py)?.into_any())
    }

    pub fn slice<'py>(
        py: Python<'py>,
        list: &Bound<'py, PyList>,
        start: &Option<isize>,
        end: &Option<isize>,
        step: &Option<isize>,
    ) -> EvalResult<'py> {
        Ok(list
            .as_any()
            .get_item(PySlice::new(
                py,
                start.unwrap_or(0),
                end.unwrap_or(isize::MAX),
                step.unwrap_or(1),
            ))?
            .into_any())
    }

    pub fn flatten<'py>(py: Python<'py>, list: &Bound<'py, PyList>) -> EvalResult<'py> {
        let output = PyList::empty(py);

        for element in list.iter() {
            if let Ok(inner_list) = element.cast::<PyList>() {
                for item in inner_list.iter() {
                    output.append(item)?;
                }
            } else {
                output.append(element)?;
            }
        }

        Ok(output.into_any())
    }

    pub fn filter<'py>(py: Python<'py>, list: &Bound<'py, PyList>, cond: &Node) -> EvalResult<'py> {
        let output = PyList::empty(py);

        for element in list.iter() {
            if match_any(py, cond, &element)?.is_truthy()? {
                output.append(element)?;
            }
        }

        Ok(output.into_any())
    }

    pub fn map<'py>(py: Python<'py>, list: &Bound<'py, PyList>, key: &Node) -> EvalResult<'py> {
        let output = PyList::empty(py);

        for element in list.iter() {
            output.append(match_any(py, key, &element)?)?;
        }

        Ok(output.into_any())
    }

    pub fn sort<'py>(py: Python<'py>, list: &Bound<'py, PyList>) -> EvalResult<'py> {
        pyo3::types::PyModule::import(py, pylibs::BUILTINS)?
            .getattr(pylibs::SORTED)?
            .call1((list,))
    }

    pub fn sort_like<'py>(
        py: Python<'py>,
        list: &Bound<'py, PyList>,
        key: &Node,
        kind: SortKind,
    ) -> EvalResult<'py> {
        type SortedVec = Vec<(u8, SortKey, Option<i64>, Option<String>, Py<PyAny>)>;
        let mut pairs: SortedVec = Vec::with_capacity(list.len());

        for element in list.iter() {
            let key_value = match_any(py, key, &element)?;
            let f = key_value.extract::<f64>().ok();
            let i = key_value.extract::<i64>().ok();
            let s = key_value.extract::<String>().ok();
            let has = if f.is_some() || i.is_some() || s.is_some() {
                0
            } else {
                1
            };
            pairs.push((has, SortKey(f), i, s, element.unbind().into_any()));
        }

        match kind {
            SortKind::SortBy => {
                pairs.sort_by(|a, b| {
                    (a.0, &a.1, a.2, a.3.as_deref()).cmp(&(b.0, &b.1, b.2, b.3.as_deref()))
                });
                let output = PyList::empty(py);
                for (_, _, _, _, element) in pairs {
                    output.append(element.bind(py))?;
                }
                Ok(output.into_any())
            }
            SortKind::MinBy => {
                if let Some(min) = pairs.iter().min_by(|a, b| {
                    (a.0, &a.1, a.2, a.3.as_deref()).cmp(&(b.0, &b.1, b.2, b.3.as_deref()))
                }) {
                    Ok(min.4.clone_ref(py).into_bound(py).into_any())
                } else {
                    Ok(py.None().into_bound(py))
                }
            }
            SortKind::MaxBy => {
                if let Some(max) = pairs.iter().max_by(|a, b| {
                    (a.0, &a.1, a.2, a.3.as_deref()).cmp(&(b.0, &b.1, b.2, b.3.as_deref()))
                }) {
                    Ok(max.4.clone_ref(py).into_bound(py).into_any())
                } else {
                    Ok(py.None().into_bound(py))
                }
            }
        }
    }

    pub fn sort_by<'py>(py: Python<'py>, list: &Bound<'py, PyList>, key: &Node) -> EvalResult<'py> {
        sort_like(py, list, key, SortKind::SortBy)
    }

    pub fn min_by<'py>(py: Python<'py>, list: &Bound<'py, PyList>, key: &Node) -> EvalResult<'py> {
        sort_like(py, list, key, SortKind::MinBy)
    }

    pub fn max_by<'py>(py: Python<'py>, list: &Bound<'py, PyList>, key: &Node) -> EvalResult<'py> {
        sort_like(py, list, key, SortKind::MaxBy)
    }

    pub fn sum<'py>(py: Python<'py>, list: &Bound<'py, PyList>) -> EvalResult<'py> {
        if list.len() == 0 {
            return Ok(PyFloat::new(py, 0.0).into_any());
        }

        let mut sum = 0.0;
        for element in list.iter() {
            if !is_number(&element) {
                return Ok(py.None().into_bound(py));
            }
            sum += element.extract::<f64>()?;
        }

        Ok(sum.into_pyobject(py)?.into_any())
    }

    pub fn reverse<'py>(py: Python<'py>, list: &Bound<'py, PyList>) -> EvalResult<'py> {
        list.as_any()
            .get_item(PySlice::new(py, isize::MAX, isize::MIN, -1isize))
            .map(|result| result.into_any())
    }
    pub fn min_max<'py>(
        py: Python<'py>,
        list: &Bound<'py, PyList>,
        is_max: bool,
    ) -> EvalResult<'py> {
        if list.len() == 0 {
            return Ok(py.None().into_bound(py));
        }

        let mut iter = list.iter();
        let first = iter.next().unwrap();
        let expect_number = is_number(&first);
        let expect_string = is_string(&first);

        if !expect_number && !expect_string {
            return Ok(py.None().into_bound(py));
        }

        let op = if is_max { CompareOp::Gt } else { CompareOp::Lt };
        let mut best = first;

        for current in iter {
            let is_num = is_number(&current);
            let is_str = is_string(&current);

            if expect_number && !is_num {
                return Ok(py.None().into_bound(py));
            }
            if expect_string && !is_str {
                return Ok(py.None().into_bound(py));
            }

            if current.rich_compare(&best, op)?.is_truthy()? {
                best = current;
            }
        }

        Ok(best.into_any())
    }
    pub fn join<'py>(py: Python<'py>, list: &Bound<'py, PyList>, glue: &str) -> EvalResult<'py> {
        for element in list.iter() {
            if !is_string(&element) {
                return Ok(py.None().into_bound(py));
            }
        }
        PyString::new(py, glue).call_method1(pylibs::JOIN, (list,))
    }
    pub fn avg<'py>(py: Python<'py>, list: &Bound<'py, PyList>) -> EvalResult<'py> {
        let length = list.len();

        if length == 0 {
            return Ok(py.None().into_bound(py));
        }

        let mut sum = 0.0;
        for element in list.iter() {
            if !is_number(&element) {
                return Ok(py.None().into_bound(py));
            }
            sum += element.extract::<f64>()?;
        }

        let average = sum / (length as f64);
        Ok(average.into_pyobject(py)?.into_any())
    }
    pub fn contains<'py>(
        py: Python<'py>,
        list: &Bound<'py, PyList>,
        search: &Bounded<'py>,
    ) -> EvalResult<'py> {
        let mut found = false;

        for element in list.iter() {
            if element.eq(search)? {
                found = true;
                break;
            }
        }

        Ok(PyBool::new(py, found).to_owned().into_any())
    }
}

pub mod structs {
    use super::*;

    pub fn field<'py>(py: Python<'py>, dict: &Bound<'py, PyDict>, name: &str) -> EvalResult<'py> {
        Ok(dict
            .get_item(name)?
            .unwrap_or_else(|| py.None().into_bound(py)))
    }

    pub fn keys<'py>(dict: &Bound<'py, PyDict>) -> EvalResult<'py> {
        Ok(dict.keys().into_any())
    }

    pub fn values<'py>(dict: &Bound<'py, PyDict>) -> EvalResult<'py> {
        Ok(dict.values().into_any())
    }
}

pub mod strs {
    use super::*;

    pub fn length<'py>(py: Python<'py>, string: &Bound<'py, PyString>) -> EvalResult<'py> {
        Ok(string.len()?.into_pyobject(py)?.into_any())
    }

    pub fn contains<'py>(
        py: Python<'py>,
        string: &Bound<'py, PyString>,
        search: &str,
    ) -> EvalResult<'py> {
        let b = string.to_str()?.contains(search);
        Ok(PyBool::new(py, b).to_owned().into_any())
    }
    pub fn starts_with<'py>(
        py: Python<'py>,
        string: &Bound<'py, PyString>,
        prefix: &str,
    ) -> EvalResult<'py> {
        let b = string.to_str()?.starts_with(prefix);
        Ok(PyBool::new(py, b).to_owned().into_any())
    }

    pub fn ends_with<'py>(
        py: Python<'py>,
        string: &Bound<'py, PyString>,
        suffix: &str,
    ) -> EvalResult<'py> {
        let b = string.to_str()?.ends_with(suffix);
        Ok(PyBool::new(py, b).to_owned().into_any())
    }

    pub fn slice<'py>(
        py: Python<'py>,
        string: &Bound<'py, PyString>,
        start: &Option<isize>,
        end: &Option<isize>,
        step: &Option<isize>,
    ) -> EvalResult<'py> {
        Ok(string
            .as_any()
            .get_item(PySlice::new(
                py,
                start.unwrap_or(0),
                end.unwrap_or(isize::MAX),
                step.unwrap_or(1),
            ))?
            .into_any())
    }

    pub fn reverse<'py>(py: Python<'py>, string: &Bound<'py, PyString>) -> EvalResult<'py> {
        Ok(PyString::new(py, &string.to_str()?.chars().rev().collect::<String>()).into_any())
    }
}

pub fn literal<'py>(py: Python<'py>, obj: &PyObjectWrapper) -> EvalResult<'py> {
    Ok(obj.0.clone_ref(py).into_bound(py).into_any())
}

pub fn and<'py>(py: Python<'py>, value: &Bounded<'py>, a: &Node, b: &Node) -> EvalResult<'py> {
    let left = match_any(py, a, value)?;
    if left.is_truthy()? {
        match_any(py, b, value)
    } else {
        Ok(left)
    }
}

pub fn or<'py>(py: Python<'py>, value: &Bounded<'py>, a: &Node, b: &Node) -> EvalResult<'py> {
    let left = match_any(py, a, value)?;
    if left.is_truthy()? {
        Ok(left)
    } else {
        match_any(py, b, value)
    }
}

pub fn not<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> EvalResult<'py> {
    let result = !match_any(py, x, value)?.is_truthy()?;
    Ok(PyBool::new(py, result).to_owned().into_any())
}

pub fn cmp_bool<'py>(
    py: Python<'py>,
    left: &Bounded<'py>,
    right: &Bounded<'py>,
    op: CompareOp,
) -> EvalResult<'py> {
    let result = if is_number(left) && is_number(right) {
        left.rich_compare(right, op)?.is_truthy()?
    } else {
        false
    };
    Ok(PyBool::new(py, result).to_owned().into_any())
}

pub fn abs<'py>(py: Python<'py>, number: &Bounded<'py>) -> EvalResult<'py> {
    Ok(number.extract::<f64>()?.abs().into_pyobject(py)?.into_any())
}

pub fn ceil<'py>(py: Python<'py>, number: &Bounded<'py>) -> EvalResult<'py> {
    Ok(number
        .extract::<f64>()?
        .ceil()
        .into_pyobject(py)?
        .into_any())
}

pub fn floor<'py>(py: Python<'py>, number: &Bounded<'py>) -> EvalResult<'py> {
    Ok(number
        .extract::<f64>()?
        .floor()
        .into_pyobject(py)?
        .into_any())
}

pub fn merge<'py>(py: Python<'py>, value: &Bounded<'py>, items: &[Node]) -> EvalResult<'py> {
    let output = PyDict::new(py);

    for item in items {
        let evaluated = match_any(py, item, value)?;
        if let Ok(dict) = evaluated.cast::<PyDict>() {
            output.update(dict.as_mapping())?;
        } else {
            return Ok(py.None().into_bound(py));
        }
    }

    Ok(output.into_any())
}

pub fn coalesce<'py>(py: Python<'py>, value: &Bounded<'py>, items: &[Node]) -> EvalResult<'py> {
    for item in items {
        let evaluated = match_any(py, item, value)?;
        if !evaluated.is_none() {
            return Ok(evaluated);
        }
    }
    Ok(py.None().into_bound(py))
}

pub fn eq<'py>(py: Python<'py>, left: &Bounded<'py>, right: &Bounded<'py>) -> EvalResult<'py> {
    Ok(PyBool::new(py, is_eq(left, right)?).to_owned().into_any())
}

pub fn ne<'py>(py: Python<'py>, left: &Bounded<'py>, right: &Bounded<'py>) -> EvalResult<'py> {
    Ok(PyBool::new(py, not_eq(left, right)?).to_owned().into_any())
}
