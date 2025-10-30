use crate::nodes::Node;
use crate::util::*;
use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::*;

fn none<'py>(py: Python<'py>) -> Bound<'py, PyAny> {
    py.None().into_bound(py)
}

pub fn eval_any<'py>(
    py: Python<'py>,
    node: &Node,
    v: Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyAny>> {
    match node {
        Node::This => Ok(v.clone()),
        Node::Literal(obj) => Ok(obj.0.clone_ref(py).into_bound(py).into_any()),
        Node::Field(name) => {
            if v.is_instance_of::<PyDict>() {
                let d = v.downcast::<PyDict>()?;
                Ok(d.get_item(name)?.unwrap_or_else(|| none(py)))
            } else {
                Ok(none(py))
            }
        }

        Node::Index(i) => {
            if let Ok(seq) = v.downcast::<PySequence>() {
                let len = seq.len()? as isize;
                let idx = if *i < 0 { len + *i } else { *i };
                if idx < 0 || idx >= len {
                    Ok(none(py))
                } else {
                    Ok(seq.get_item(idx as usize)?)
                }
            } else {
                Ok(none(py))
            }
        }

        Node::Slice(start, end, step) => {
            if v.is_instance_of::<PyList>() || v.is_instance_of::<PyTuple>() {
                let s = PySlice::new_bound(
                    py,
                    start.unwrap_or(0),
                    end.unwrap_or(isize::MAX),
                    step.unwrap_or(1),
                );
                Ok(v.get_item(s)?.into_any())
            } else {
                Ok(none(py))
            }
        }

        Node::Pipe(lhs, rhs) | Node::SubExpr(lhs, rhs) => {
            let mid = eval_any(py, lhs, v)?;
            eval_any(py, rhs, mid)
        }

        Node::MultiList(items) => {
            let out = PyList::empty_bound(py);
            for it in items {
                out.append(eval_any(py, it, v.clone())?)?;
            }
            Ok(out.into_any())
        }

        Node::MultiDict(items) => {
            let out = PyDict::new_bound(py);
            for (k, expr) in items {
                out.set_item(k, eval_any(py, expr, v.clone())?)?;
            }
            Ok(out.into_any())
        }

        Node::ProjectArray { base, rhs } => {
            let basev = eval_any(py, base, v)?;
            if !is_list(&basev)? {
                return Ok(none(py));
            }
            let seq = basev.downcast::<PySequence>()?;
            let out = PyList::empty_bound(py);
            for i in 0..seq.len()? {
                let el = seq.get_item(i)?;
                let mapped = eval_any(py, rhs, el)?;
                if !mapped.is_none() {
                    out.append(mapped)?;
                }
            }
            Ok(out.into_any())
        }

        Node::ProjectObject { base, rhs } => {
            let basev = eval_any(py, base, v)?;
            if basev.is_instance_of::<PyDict>() {
                let d = basev.downcast::<PyDict>()?;
                let out = PyList::empty_bound(py);
                for (_, val) in d.iter() {
                    let mapped = eval_any(py, rhs, val)?;
                    if !mapped.is_none() {
                        out.append(mapped)?;
                    }
                }
                Ok(out.into_any())
            } else {
                Ok(none(py))
            }
        }

        Node::Flatten(inner) => {
            let base = eval_any(py, inner, v)?;
            if !is_list(&base)? {
                return Ok(none(py));
            }
            let lst = base.downcast::<PyList>()?;
            let out = PyList::empty_bound(py);
            for el in lst.iter() {
                if is_list(&el)? {
                    let seq = el.downcast::<PySequence>()?;
                    for j in 0..seq.len()? {
                        out.append(seq.get_item(j)?)?;
                    }
                } else {
                    out.append(el)?;
                }
            }
            Ok(out.into_any())
        }

        Node::FilterProjection { base, then, cond } => {
            let basev = eval_any(py, base, v)?;
            if !is_list(&basev)? {
                return Ok(none(py));
            }
            let seq = basev.downcast::<PySequence>()?;
            let out = PyList::empty_bound(py);
            for i in 0..seq.len()? {
                let el = seq.get_item(i)?;
                let c = eval_any(py, cond, el.clone())?;
                if not_empty(&c)? {
                    out.append(eval_any(py, then, el)?)?;
                }
            }
            Ok(out.into_any())
        }

        Node::And(a, b) => {
            let av = eval_any(py, a, v.clone())?;
            if not_empty(&av)? {
                eval_any(py, b, v)
            } else {
                Ok(av)
            }
        }

        Node::Or(a, b) => {
            let av = eval_any(py, a, v.clone())?;
            if not_empty(&av)? {
                Ok(av)
            } else {
                eval_any(py, b, v)
            }
        }

        Node::Not(x) => {
            let xv = eval_any(py, x, v)?;
            let res = (is_number(&xv)? && xv.extract::<i64>().unwrap_or(1) == 0) || is_empty(&xv)?;
            Ok(res.to_object(py).into_bound(py).into_any())
        }

        Node::CmpEq(a, b) => cmp_bool(py, v, a, b, CompareOp::Eq),
        Node::CmpNe(a, b) => cmp_bool(py, v, a, b, CompareOp::Ne),
        Node::CmpLt(a, b) => cmp_bool(py, v, a, b, CompareOp::Lt),
        Node::CmpLe(a, b) => cmp_bool(py, v, a, b, CompareOp::Le),
        Node::CmpGt(a, b) => cmp_bool(py, v, a, b, CompareOp::Gt),
        Node::CmpGe(a, b) => cmp_bool(py, v, a, b, CompareOp::Ge),

        Node::Length(x) => {
            let xv = eval_any(py, x, v)?;
            if is_sized(&xv) {
                let n = xv.len()?;
                Ok((n as i64).to_object(py).into_bound(py).into_any())
            } else {
                Ok(none(py))
            }
        }

        Node::Sort(x) => {
            let xv = eval_any(py, x, v)?;
            if !is_list(&xv)? {
                return Ok(none(py));
            }
            let builtins = py.import_bound("builtins")?;
            let sorted = builtins.getattr("sorted")?.call1((xv,))?;
            Ok(sorted)
        }

        Node::Keys(x) => {
            let xv = eval_any(py, x, v)?;
            if xv.is_instance_of::<PyDict>() {
                Ok(xv.downcast::<PyDict>()?.keys().into_any())
            } else {
                Ok(none(py))
            }
        }

        Node::Values(x) => {
            let xv = eval_any(py, x, v)?;
            if xv.is_instance_of::<PyDict>() {
                Ok(xv.downcast::<PyDict>()?.values().into_any())
            } else {
                Ok(none(py))
            }
        }

        Node::ToArray(x) => {
            let xv = eval_any(py, x, v)?;
            if is_list(&xv)? {
                Ok(xv)
            } else {
                let out = PyList::empty_bound(py);
                out.append(xv)?;
                Ok(out.into_any())
            }
        }

        Node::ToString(x) => {
            let xv = eval_any(py, x, v)?;
            if xv.is_instance_of::<PyUnicode>() {
                Ok(xv)
            } else {
                let json = py.import_bound("json")?;
                let kwargs = PyDict::new_bound(py);
                let seps = PyTuple::new_bound(
                    py,
                    &[
                        PyString::new_bound(py, ",").into_any(),
                        PyString::new_bound(py, ":").into_any(),
                    ],
                );
                kwargs.set_item("separators", seps)?;
                let s = json.getattr("dumps")?.call((xv,), Some(&kwargs))?;
                Ok(s)
            }
        }

        Node::ToNumber(x) => {
            let xv = eval_any(py, x, v)?;
            if xv.is_instance_of::<PyBool>()
                || xv.is_none()
                || xv.is_instance_of::<PyDict>()
                || is_list(&xv)?
            {
                return Ok(none(py));
            }
            if is_number(&xv)? {
                return Ok(xv);
            }
            if let Ok(i) = xv.extract::<i64>() {
                return Ok(i.to_object(py).into_bound(py).into_any());
            }
            if let Ok(f) = xv.extract::<f64>() {
                return Ok(f.to_object(py).into_bound(py).into_any());
            }
            Ok(none(py))
        }

        Node::MapApply { base, key } => {
            let basev = eval_any(py, base, v)?;
            if !is_list(&basev)? {
                return Ok(none(py));
            }
            let seq = basev.downcast::<PySequence>()?;
            let out = PyList::empty_bound(py);
            for i in 0..seq.len()? {
                let el = seq.get_item(i)?;
                out.append(eval_any(py, key, el)?)?;
            }
            Ok(out.into_any())
        }

        Node::SortBy { base, key } => sort_like(py, v, base, key, SortKind::SortBy),
        Node::MinBy { base, key } => sort_like(py, v, base, key, SortKind::MinBy),
        Node::MaxBy { base, key } => sort_like(py, v, base, key, SortKind::MaxBy),
    }
}

fn cmp_bool<'py>(
    py: Python<'py>,
    v: Bound<'py, PyAny>,
    a: &Node,
    b: &Node,
    op: CompareOp,
) -> PyResult<Bound<'py, PyAny>> {
    let va = eval_any(py, a, v.clone())?;
    let vb = eval_any(py, b, v)?;
    let res = match op {
        CompareOp::Eq => eq_semantics(&va, &vb)?,
        CompareOp::Ne => !eq_semantics(&va, &vb)?,
        _ => {
            if !(is_comparable(&va)? && is_comparable(&vb)?) {
                false
            } else {
                va.rich_compare(&vb, op)?.is_truthy()?
            }
        }
    };
    Ok(res.to_object(py).into_bound(py).into_any())
}

enum SortKind {
    SortBy,
    MinBy,
    MaxBy,
}

fn sort_like<'py>(
    py: Python<'py>,
    v: Bound<'py, PyAny>,
    base: &Node,
    key: &Node,
    kind: SortKind,
) -> PyResult<Bound<'py, PyAny>> {
    let basev = eval_any(py, base, v)?;
    if !is_list(&basev)? {
        return Ok(none(py));
    }
    let lst = basev.downcast::<PyList>()?;
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
        Vec::with_capacity(lst.len());
    for el in lst.iter() {
        let kv = eval_any(py, key, el.clone())?;
        let f = kv.extract::<f64>().ok();
        let i = kv.extract::<i64>().ok();
        let s = kv.extract::<String>().ok();
        let has = if f.is_some() || i.is_some() || s.is_some() {
            0
        } else {
            1
        };
        pairs.push((has, SortKey(f), i, s, el.to_object(py)));
    }
    match kind {
        SortKind::SortBy => {
            pairs.sort_by(|a, b| {
                (a.0, &a.1, a.2, a.3.as_deref()).cmp(&(b.0, &b.1, b.2, b.3.as_deref()))
            });
            let out = PyList::empty_bound(py);
            for (_, _, _, _, el) in pairs {
                out.append(el.bind(py))?;
            }
            Ok(out.into_any())
        }
        SortKind::MinBy => {
            if let Some(min) = pairs.iter().min_by(|a, b| {
                (a.0, &a.1, a.2, a.3.as_deref()).cmp(&(b.0, &b.1, b.2, b.3.as_deref()))
            }) {
                Ok(min.4.clone_ref(py).into_bound(py).into_any())
            } else {
                Ok(none(py))
            }
        }
        SortKind::MaxBy => {
            if let Some(max) = pairs.iter().max_by(|a, b| {
                (a.0, &a.1, a.2, a.3.as_deref()).cmp(&(b.0, &b.1, b.2, b.3.as_deref()))
            }) {
                Ok(max.4.clone_ref(py).into_bound(py).into_any())
            } else {
                Ok(none(py))
            }
        }
    }
}
