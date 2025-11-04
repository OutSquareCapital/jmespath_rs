use crate::checks::*;
use crate::matchs::match_any;
use crate::nodes::{Bounded, EvalResult, Node, PyObjectWrapper};
use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::*;

const BUILTINS: &str = "builtins";
const SORTED: &str = "sorted";

pub mod list {
    use super::*;

    pub enum SortKind {
        SortBy,
        MinBy,
        MaxBy,
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

    pub fn slice<'py>(
        py: Python<'py>,
        list: &Bound<'py, PyList>,
        start: &Option<isize>,
        end: &Option<isize>,
        step: &Option<isize>,
    ) -> EvalResult<'py> {
        Ok(list
            .as_any()
            .get_item(PySlice::new_bound(
                py,
                start.unwrap_or(0),
                end.unwrap_or(isize::MAX),
                step.unwrap_or(1),
            ))?
            .into_any())
    }

    pub fn flatten<'py>(py: Python<'py>, list: &Bound<'py, PyList>) -> EvalResult<'py> {
        let output = PyList::empty_bound(py);

        for element in list.iter() {
            if let Ok(inner_list) = element.downcast::<PyList>() {
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
        let output = PyList::empty_bound(py);

        for element in list.iter() {
            if match_any(py, cond, &element)?.is_truthy()? {
                output.append(element)?;
            }
        }

        Ok(output.into_any())
    }

    pub fn map<'py>(py: Python<'py>, list: &Bound<'py, PyList>, key: &Node) -> EvalResult<'py> {
        let output = PyList::empty_bound(py);

        for element in list.iter() {
            output.append(match_any(py, key, &element)?)?;
        }

        Ok(output.into_any())
    }

    pub fn sort<'py>(py: Python<'py>, list: &Bound<'py, PyList>) -> EvalResult<'py> {
        Ok(py.import_bound(BUILTINS)?.getattr(SORTED)?.call1((list,))?)
    }

    pub fn sort_like<'py>(
        py: Python<'py>,
        list: &Bound<'py, PyList>,
        key: &Node,
        kind: SortKind,
    ) -> EvalResult<'py> {
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

        let mut pairs: Vec<(u8, SortKey, Option<i64>, Option<String>, PyObject)> =
            Vec::with_capacity(list.len());

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
            pairs.push((has, SortKey(f), i, s, element.to_object(py)));
        }

        match kind {
            SortKind::SortBy => {
                pairs.sort_by(|a, b| {
                    (a.0, &a.1, a.2, a.3.as_deref()).cmp(&(b.0, &b.1, b.2, b.3.as_deref()))
                });
                let output = PyList::empty_bound(py);
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
        let length = list.len();

        if length == 0 {
            return Ok(0.to_object(py).into_bound(py).into_any());
        }

        let mut sum = 0.0;
        for element in list.iter() {
            if !is_number(&element) {
                return Ok(py.None().into_bound(py));
            }
            sum += element.extract::<f64>()?;
        }

        Ok(sum.to_object(py).into_bound(py).into_any())
    }

    pub fn reverse<'py>(py: Python<'py>, list: &Bound<'py, PyList>) -> EvalResult<'py> {
        list.as_any()
            .get_item(PySlice::new_bound(py, isize::MAX, isize::MIN, -1isize))
            .map(|result| result.into_any())
    }
    pub fn min_max<'py>(
        py: Python<'py>,
        list: &Bound<'py, PyList>,
        is_max: bool,
    ) -> EvalResult<'py> {
        let length = list.len();

        if length == 0 {
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
    pub fn join<'py>(
        py: Python<'py>,
        glue: &Bounded<'py>,
        list: &Bound<'py, PyList>,
    ) -> EvalResult<'py> {
        let glue_str = glue.extract::<&str>()?;
        let length = list.len();
        let mut parts: Vec<String> = Vec::with_capacity(length);

        for element in list.iter() {
            if !is_string(&element) {
                return Ok(py.None().into_bound(py));
            }
            parts.push(element.extract::<String>()?);
        }

        Ok(PyString::new_bound(py, &parts.join(glue_str)).into_any())
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
        Ok(average.to_object(py).into_bound(py).into_any())
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

        Ok(found.to_object(py).into_bound(py).into_any())
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

    pub fn contains<'py>(
        py: Python<'py>,
        string: &Bound<'py, PyString>,
        search: &Bounded<'py>,
    ) -> EvalResult<'py> {
        if !is_string(search) {
            return Ok(py.None().into_bound(py));
        }

        Ok(string
            .to_str()?
            .contains(search.extract::<&str>()?)
            .to_object(py)
            .into_bound(py)
            .into_any())
    }

    pub fn starts_with<'py>(
        py: Python<'py>,
        string: &Bound<'py, PyString>,
        prefix: &Bounded<'py>,
    ) -> EvalResult<'py> {
        if !is_string(prefix) {
            return Ok(false.to_object(py).into_bound(py).into_any());
        }

        Ok(string
            .to_str()?
            .starts_with(prefix.extract::<&str>()?)
            .to_object(py)
            .into_bound(py)
            .into_any())
    }

    pub fn ends_with<'py>(
        py: Python<'py>,
        string: &Bound<'py, PyString>,
        suffix: &Bounded<'py>,
    ) -> EvalResult<'py> {
        if !is_string(suffix) {
            return Ok(false.to_object(py).into_bound(py).into_any());
        }

        Ok(string
            .to_str()?
            .ends_with(suffix.extract::<&str>()?)
            .to_object(py)
            .into_bound(py)
            .into_any())
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
            .get_item(PySlice::new_bound(
                py,
                start.unwrap_or(0),
                end.unwrap_or(isize::MAX),
                step.unwrap_or(1),
            ))?
            .into_any())
    }

    pub fn reverse<'py>(py: Python<'py>, string: &Bound<'py, PyString>) -> EvalResult<'py> {
        let reversed: String = string.to_str()?.chars().rev().collect();
        Ok(PyString::new_bound(py, &reversed).into_any())
    }
}

pub fn literal<'py>(py: Python<'py>, obj: &PyObjectWrapper) -> EvalResult<'py> {
    Ok(obj.0.clone_ref(py).into_bound(py).into_any())
}

pub fn and<'py>(py: Python<'py>, value: &Bounded<'py>, a: &Node, b: &Node) -> EvalResult<'py> {
    let left = match_any(py, a, &value)?;
    if left.is_truthy()? {
        match_any(py, b, value)
    } else {
        Ok(left)
    }
}

pub fn or<'py>(py: Python<'py>, value: &Bounded<'py>, a: &Node, b: &Node) -> EvalResult<'py> {
    let left = match_any(py, a, &value)?;
    if left.is_truthy()? {
        Ok(left)
    } else {
        match_any(py, b, value)
    }
}

pub fn not<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> EvalResult<'py> {
    let result = !match_any(py, x, value)?.is_truthy()?;
    Ok(result.to_object(py).into_bound(py).into_any())
}

pub fn length<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> EvalResult<'py> {
    let evaluated = match_any(py, x, value)?;
    if !is_sized(&evaluated) {
        return Ok(py.None().into_bound(py));
    }

    let length = evaluated.len()? as i64;
    Ok(length.to_object(py).into_bound(py).into_any())
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
    Ok(result.to_object(py).into_bound(py).into_any())
}

pub fn abs<'py>(py: Python<'py>, number: &Bounded<'py>) -> EvalResult<'py> {
    Ok(number
        .extract::<f64>()?
        .abs()
        .to_object(py)
        .into_bound(py)
        .into_any())
}

pub fn ceil<'py>(py: Python<'py>, number: &Bounded<'py>) -> EvalResult<'py> {
    let result = number.extract::<f64>()?.ceil();
    Ok(result.to_object(py).into_bound(py).into_any())
}

pub fn floor<'py>(py: Python<'py>, number: &Bounded<'py>) -> EvalResult<'py> {
    let result = number.extract::<f64>()?.floor();
    Ok(result.to_object(py).into_bound(py).into_any())
}

pub fn merge<'py>(py: Python<'py>, value: &Bounded<'py>, items: &[Node]) -> EvalResult<'py> {
    let output = PyDict::new_bound(py);

    for item in items {
        let evaluated = match_any(py, item, value)?;
        if let Ok(dict) = evaluated.downcast::<PyDict>() {
            output.update(dict.as_mapping())?;
        } else {
            return Ok(py.None().into_bound(py));
        }
    }

    Ok(output.into_any())
}

pub fn not_null<'py>(py: Python<'py>, value: &Bounded<'py>, items: &[Node]) -> EvalResult<'py> {
    for item in items {
        let evaluated = match_any(py, item, value)?;
        if !evaluated.is_none() {
            return Ok(evaluated);
        }
    }
    Ok(py.None().into_bound(py))
}

pub fn eq<'py>(py: Python<'py>, left: &Bounded<'py>, right: &Bounded<'py>) -> EvalResult<'py> {
    Ok(is_eq(left, right)?.to_object(py).into_bound(py).into_any())
}

pub fn ne<'py>(py: Python<'py>, left: &Bounded<'py>, right: &Bounded<'py>) -> EvalResult<'py> {
    let result = !is_eq(left, right)?;
    Ok(result.to_object(py).into_bound(py).into_any())
}
