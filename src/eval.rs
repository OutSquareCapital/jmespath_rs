use crate::checks::*;
use crate::nodes::Node;
use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::*;

const BUILTINS: &str = "builtins";
const SORTED: &str = "sorted";
const JSON: &str = "json";

type Result<'py> = PyResult<Bound<'py, PyAny>>;
type Bounded<'py> = Bound<'py, PyAny>;

pub fn eval_any<'py>(py: Python<'py>, node: &Node, value: &Bounded<'py>) -> Result<'py> {
    match node {
        Node::This => Ok(value.clone()),
        Node::Literal(obj) => Ok(obj.0.clone_ref(py).into_bound(py).into_any()),
        Node::Field(name) => eval_field(py, value, name),
        Node::Index(i) => eval_index(py, value, i),
        Node::Slice(start, end, step) => eval_slice(py, value, start, end, step),
        Node::Pipe(lhs, rhs) | Node::SubExpr(lhs, rhs) => eval_pipe(py, value, lhs, rhs),
        Node::MultiList(items) => eval_multi_list(py, value, items),
        Node::MultiDict(items) => eval_multi_dict(py, value, items),
        Node::ProjectArray { base, rhs } => eval_project_array(py, value, base, rhs),
        Node::ProjectObject { base, rhs } => eval_project_object(py, value, base, rhs),
        Node::Flatten(inner) => eval_flatten(py, value, inner),
        Node::FilterProjection { base, then, cond } => {
            eval_filter_projection(py, value, base, then, cond)
        }
        Node::And(a, b) => eval_and(py, value, a, b),
        Node::Or(a, b) => eval_or(py, value, a, b),
        Node::Not(x) => eval_not(py, value, x),
        Node::CmpEq(a, b) => cmp_bool(py, value, a, b, CompareOp::Eq),
        Node::CmpNe(a, b) => cmp_bool(py, value, a, b, CompareOp::Ne),
        Node::CmpLt(a, b) => cmp_bool(py, value, a, b, CompareOp::Lt),
        Node::CmpLe(a, b) => cmp_bool(py, value, a, b, CompareOp::Le),
        Node::CmpGt(a, b) => cmp_bool(py, value, a, b, CompareOp::Gt),
        Node::CmpGe(a, b) => cmp_bool(py, value, a, b, CompareOp::Ge),
        Node::Length(x) => eval_length(py, value, x),
        Node::Sort(x) => eval_sort(py, value, x),
        Node::Keys(x) => eval_keys(py, value, x),
        Node::Values(x) => eval_values(py, value, x),
        Node::ToArray(x) => eval_to_array(py, value, x),
        Node::ToString(x) => eval_to_string(py, value, x),
        Node::ToNumber(x) => to_number(py, value, x),
        Node::MapApply { base, key } => map_apply(py, value, base, key),
        Node::SortBy { base, key } => sort_like(py, value, base, key, SortKind::SortBy),
        Node::MinBy { base, key } => sort_like(py, value, base, key, SortKind::MinBy),
        Node::MaxBy { base, key } => sort_like(py, value, base, key, SortKind::MaxBy),
    }
}
fn eval_field<'py>(py: Python<'py>, value: &Bounded<'py>, name: &str) -> Result<'py> {
    if value.is_instance_of::<PyDict>() {
        let d = value.downcast::<PyDict>()?;
        Ok(d.get_item(name)?
            .unwrap_or_else(|| py.None().into_bound(py)))
    } else {
        Ok(py.None().into_bound(py))
    }
}

fn eval_index<'py>(py: Python<'py>, value: &Bounded<'py>, i: &isize) -> Result<'py> {
    if let Ok(seq) = value.downcast::<PySequence>() {
        let len = seq.len()? as isize;
        let idx = if *i < 0 { len + *i } else { *i };
        if idx < 0 || idx >= len {
            Ok(py.None().into_bound(py))
        } else {
            Ok(seq.get_item(idx as usize)?)
        }
    } else {
        Ok(py.None().into_bound(py))
    }
}

fn eval_slice<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    start: &Option<isize>,
    end: &Option<isize>,
    step: &Option<isize>,
) -> Result<'py> {
    if is_list(value) {
        let s = PySlice::new_bound(
            py,
            start.unwrap_or(0),
            end.unwrap_or(isize::MAX),
            step.unwrap_or(1),
        );
        Ok(value.get_item(s)?.into_any())
    } else {
        Ok(py.None().into_bound(py))
    }
}

fn eval_pipe<'py>(py: Python<'py>, value: &Bounded<'py>, lhs: &Node, rhs: &Node) -> Result<'py> {
    let mid = eval_any(py, lhs, value)?;
    eval_any(py, rhs, &mid)
}

fn eval_multi_list<'py>(py: Python<'py>, value: &Bounded<'py>, items: &[Node]) -> Result<'py> {
    let out = PyList::empty_bound(py);
    for it in items {
        out.append(eval_any(py, it, value)?)?;
    }
    Ok(out.into_any())
}

fn eval_multi_dict<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    items: &[(String, Node)],
) -> Result<'py> {
    let out = PyDict::new_bound(py);
    for (k, expr) in items {
        out.set_item(k, eval_any(py, expr, value)?)?;
    }
    Ok(out.into_any())
}

fn eval_project_array<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    base: &Node,
    rhs: &Node,
) -> Result<'py> {
    let basev = eval_any(py, base, value)?;
    if !is_list(&basev) {
        return Ok(py.None().into_bound(py));
    }
    let seq = basev.downcast::<PySequence>()?;
    let out = PyList::empty_bound(py);
    for i in 0..seq.len()? {
        let el = seq.get_item(i)?;
        let mapped = eval_any(py, rhs, &el)?;
        if !mapped.is_none() {
            out.append(mapped)?;
        }
    }
    Ok(out.into_any())
}

fn eval_project_object<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    base: &Node,
    rhs: &Node,
) -> Result<'py> {
    let basev = eval_any(py, base, value)?;
    if basev.is_instance_of::<PyDict>() {
        let d = basev.downcast::<PyDict>()?;
        let out = PyList::empty_bound(py);
        for (_, val) in d.iter() {
            let mapped = eval_any(py, rhs, &val)?;
            if !mapped.is_none() {
                out.append(mapped)?;
            }
        }
        Ok(out.into_any())
    } else {
        Ok(py.None().into_bound(py))
    }
}

fn eval_flatten<'py>(py: Python<'py>, value: &Bounded<'py>, inner: &Node) -> Result<'py> {
    let base = eval_any(py, inner, value)?;
    if !is_list(&base) {
        return Ok(py.None().into_bound(py));
    }
    let lst = base.downcast::<PyList>()?;
    let out = PyList::empty_bound(py);
    for el in lst.iter() {
        if is_list(&el) {
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

fn eval_filter_projection<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    base: &Node,
    then: &Node,
    cond: &Node,
) -> Result<'py> {
    let basev = eval_any(py, base, value)?;
    if !is_list(&basev) {
        return Ok(py.None().into_bound(py));
    }
    let seq = basev.downcast::<PySequence>()?;
    let out = PyList::empty_bound(py);
    for i in 0..seq.len()? {
        let el = seq.get_item(i)?;
        let c = eval_any(py, cond, &el)?;
        if not_empty(&c)? {
            out.append(eval_any(py, then, &el)?)?;
        }
    }
    Ok(out.into_any())
}

fn eval_and<'py>(py: Python<'py>, value: &Bounded<'py>, a: &Node, b: &Node) -> Result<'py> {
    let av = eval_any(py, a, &value)?;
    if not_empty(&av)? {
        eval_any(py, b, value)
    } else {
        Ok(av)
    }
}

fn eval_or<'py>(py: Python<'py>, value: &Bounded<'py>, a: &Node, b: &Node) -> Result<'py> {
    let av = eval_any(py, a, &value)?;
    if not_empty(&av)? {
        Ok(av)
    } else {
        eval_any(py, b, value)
    }
}

fn eval_not<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> Result<'py> {
    let xv = eval_any(py, x, value)?;
    let res = (is_number(&xv) && xv.extract::<i64>().unwrap_or(1) == 0) || is_empty(&xv)?;
    Ok(res.to_object(py).into_bound(py).into_any())
}

fn eval_length<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> Result<'py> {
    let xv = eval_any(py, x, value)?;
    if is_sized(&xv) {
        let n = xv.len()?;
        Ok((n as i64).to_object(py).into_bound(py).into_any())
    } else {
        Ok(py.None().into_bound(py))
    }
}

fn eval_sort<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> Result<'py> {
    let xv = eval_any(py, x, value)?;
    if !is_list(&xv) {
        return Ok(py.None().into_bound(py));
    }
    let builtins = py.import_bound(BUILTINS)?;
    let sorted = builtins.getattr(SORTED)?.call1((xv,))?;
    Ok(sorted)
}

fn eval_keys<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> Result<'py> {
    let xv = eval_any(py, x, value)?;
    if xv.is_instance_of::<PyDict>() {
        Ok(xv.downcast::<PyDict>()?.keys().into_any())
    } else {
        Ok(py.None().into_bound(py))
    }
}

fn eval_values<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> Result<'py> {
    let xv = eval_any(py, x, value)?;
    if xv.is_instance_of::<PyDict>() {
        Ok(xv.downcast::<PyDict>()?.values().into_any())
    } else {
        Ok(py.None().into_bound(py))
    }
}

fn eval_to_array<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> Result<'py> {
    let xv = eval_any(py, x, value)?;
    if is_list(&xv) {
        Ok(xv)
    } else {
        let out = PyList::empty_bound(py);
        out.append(xv)?;
        Ok(out.into_any())
    }
}

fn eval_to_string<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> Result<'py> {
    let xv = eval_any(py, x, value)?;
    if xv.is_instance_of::<PyUnicode>() {
        Ok(xv)
    } else {
        let json = py.import_bound(JSON)?;
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

fn to_number<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> Result<'py> {
    let xv = eval_any(py, x, value)?;
    if let Ok(s) = xv.extract::<&str>() {
        if let Ok(i) = s.parse::<i64>() {
            return Ok(i.to_object(py).into_bound(py).into_any());
        }
        if let Ok(f) = s.parse::<f64>() {
            return Ok(f.to_object(py).into_bound(py).into_any());
        }
        return Ok(py.None().into_bound(py));
    }
    if xv.is_instance_of::<PyBool>()
        || xv.is_none()
        || xv.is_instance_of::<PyDict>()
        || is_list(&xv)
    {
        return Ok(py.None().into_bound(py));
    }
    if is_number(&xv) {
        return Ok(xv);
    }

    if let Ok(i) = xv.extract::<i64>() {
        return Ok(i.to_object(py).into_bound(py).into_any());
    }
    if let Ok(f) = xv.extract::<f64>() {
        return Ok(f.to_object(py).into_bound(py).into_any());
    }
    Ok(py.None().into_bound(py))
}
fn map_apply<'py>(py: Python<'py>, value: &Bounded<'py>, base: &Node, key: &Node) -> Result<'py> {
    let basev = eval_any(py, base, value)?;
    if !is_list(&basev) {
        return Ok(py.None().into_bound(py));
    }
    let seq = basev.downcast::<PySequence>()?;
    let out = PyList::empty_bound(py);
    for i in 0..seq.len()? {
        let el = seq.get_item(i)?;
        out.append(eval_any(py, key, &el)?)?;
    }
    Ok(out.into_any())
}

fn cmp_bool<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    a: &Node,
    b: &Node,
    op: CompareOp,
) -> Result<'py> {
    let va = eval_any(py, a, value)?;
    let vb = eval_any(py, b, value)?;
    let res = match op {
        CompareOp::Eq => eq_semantics(&va, &vb)?,
        CompareOp::Ne => !eq_semantics(&va, &vb)?,
        _ => {
            if !(is_comparable(&va) && is_comparable(&vb)) {
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
    value: &Bounded<'py>,
    base: &Node,
    key: &Node,
    kind: SortKind,
) -> Result<'py> {
    let basev = eval_any(py, base, value)?;
    if !is_list(&basev) {
        return Ok(py.None().into_bound(py));
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
        let kv = eval_any(py, key, &el)?;
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
