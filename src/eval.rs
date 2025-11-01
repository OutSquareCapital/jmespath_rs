use crate::checks::*;
use crate::nodes::Node;
use crate::nodes::PyObjectWrapper;
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
        Node::Literal(obj) => eval_literal(py, obj),
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
        Node::Abs(x) => eval_abs(py, value, x),
        Node::Avg(x) => eval_avg(py, value, x),
        Node::Ceil(x) => eval_ceil_floor(py, value, x, true),
        Node::Contains(a, b) => eval_contains(py, value, a, b),
        Node::EndsWith(a, b) => eval_starts_ends_with(py, value, a, b, false),
        Node::Floor(x) => eval_ceil_floor(py, value, x, false),
        Node::Join(a, b) => eval_join(py, value, a, b),
        Node::Max(x) => eval_min_max(py, value, x, true),
        Node::Merge(items) => eval_merge(py, value, items),
        Node::Min(x) => eval_min_max(py, value, x, false),
        Node::NotNull(items) => eval_not_null(py, value, items),
        Node::Reverse(x) => eval_reverse(py, value, x),
        Node::StartsWith(a, b) => eval_starts_ends_with(py, value, a, b, true),
        Node::Sum(x) => eval_sum(py, value, x),
        Node::DType(x) => eval_dtype(py, value, x),
    }
}

fn eval_literal<'py>(py: Python<'py>, obj: &PyObjectWrapper) -> Result<'py> {
    Ok(obj.0.clone_ref(py).into_bound(py).into_any())
}

fn eval_field<'py>(py: Python<'py>, value: &Bounded<'py>, name: &str) -> Result<'py> {
    if is_object(value) {
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
    if !is_object(&basev) {
        return Ok(py.None().into_bound(py));
    }

    let out = PyList::empty_bound(py);

    for item_value in basev.downcast::<PyDict>()?.values() {
        let mapped = eval_any(py, rhs, &item_value)?;
        if !mapped.is_none() {
            out.append(mapped)?;
        }
    }
    Ok(out.into_any())
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
    if is_object(&xv) {
        Ok(xv.downcast::<PyDict>()?.keys().into_any())
    } else {
        Ok(py.None().into_bound(py))
    }
}

fn eval_values<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> Result<'py> {
    let xv = eval_any(py, x, value)?;
    if is_object(&xv) {
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
    if xv.is_instance_of::<PyBool>() || xv.is_none() || is_object(&xv) || is_list(&xv) {
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
fn eval_abs<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> Result<'py> {
    let xv = eval_any(py, x, value)?;
    if !is_number(&xv) {
        return Ok(py.None().into_bound(py));
    }
    let f = xv.extract::<f64>()?;
    Ok(f.abs().to_object(py).into_bound(py).into_any())
}

fn eval_avg<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> Result<'py> {
    let xv = eval_any(py, x, value)?;
    if !is_list(&xv) {
        return Ok(py.None().into_bound(py));
    }
    let seq = xv.downcast::<PySequence>()?;
    let len = seq.len()?;
    if len == 0 {
        return Ok(py.None().into_bound(py));
    }
    let mut sum = 0.0;
    for i in 0..len {
        let el = seq.get_item(i)?;
        if !is_number(&el) {
            return Ok(py.None().into_bound(py));
        }
        sum += el.extract::<f64>()?;
    }
    Ok((sum / (len as f64)).to_object(py).into_bound(py).into_any())
}

fn eval_ceil_floor<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    x: &Node,
    is_ceil: bool,
) -> Result<'py> {
    let xv = eval_any(py, x, value)?;
    if !is_number(&xv) {
        return Ok(py.None().into_bound(py));
    }
    let f = xv.extract::<f64>()?;
    let res = if is_ceil { f.ceil() } else { f.floor() };
    Ok(res.to_object(py).into_bound(py).into_any())
}

fn eval_contains<'py>(py: Python<'py>, value: &Bounded<'py>, a: &Node, b: &Node) -> Result<'py> {
    let subject = eval_any(py, a, value)?;
    let search = eval_any(py, b, value)?;

    let res = if let Ok(s) = subject.extract::<&str>() {
        if let Ok(needle) = search.extract::<&str>() {
            s.contains(needle)
        } else {
            false
        }
    } else if is_list(&subject) {
        let seq = subject.downcast::<PySequence>()?;
        let mut found = false;
        for i in 0..seq.len()? {
            if eq_semantics(&seq.get_item(i)?, &search)? {
                found = true;
                break;
            }
        }
        found
    } else {
        false
    };
    Ok(res.to_object(py).into_bound(py).into_any())
}

fn eval_starts_ends_with<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    a: &Node,
    b: &Node,
    is_starts_with: bool,
) -> Result<'py> {
    let subject_node = eval_any(py, a, value)?;
    let search_node = eval_any(py, b, value)?;

    let res = if let (Ok(subject), Ok(search)) = (
        subject_node.extract::<&str>(),
        search_node.extract::<&str>(),
    ) {
        if is_starts_with {
            subject.starts_with(search)
        } else {
            subject.ends_with(search)
        }
    } else {
        false
    };
    Ok(res.to_object(py).into_bound(py).into_any())
}

fn eval_join<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    glue_node: &Node,
    array_node: &Node,
) -> Result<'py> {
    let glue = eval_any(py, glue_node, value)?;
    let array = eval_any(py, array_node, value)?;

    if !is_list(&array) {
        return Ok(py.None().into_bound(py));
    }
    let glue_str = if let Ok(s) = glue.extract::<&str>() {
        s
    } else {
        return Ok(py.None().into_bound(py));
    };

    let seq = array.downcast::<PySequence>()?;
    let len = seq.len()?;
    let mut parts: Vec<String> = Vec::with_capacity(len);

    for i in 0..len {
        let el = seq.get_item(i)?;
        if let Ok(s) = el.extract::<String>() {
            parts.push(s);
        } else {
            return Ok(py.None().into_bound(py));
        }
    }
    Ok(PyString::new_bound(py, &parts.join(glue_str)).into_any())
}

fn eval_min_max<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node, is_max: bool) -> Result<'py> {
    let xv = eval_any(py, x, value)?;
    if !is_list(&xv) {
        return Ok(py.None().into_bound(py));
    }
    let seq = xv.downcast::<PySequence>()?;
    let len = seq.len()?;
    if len == 0 {
        return Ok(py.None().into_bound(py));
    }

    let mut items: Vec<PyObject> = Vec::with_capacity(len);
    let mut has_str = false;
    let mut has_num = false;

    for i in 0..len {
        let el = seq.get_item(i)?;
        if is_number(&el) {
            has_num = true;
        } else if el.is_instance_of::<PyUnicode>() {
            has_str = true;
        } else {
            return Ok(py.None().into_bound(py));
        }
        if has_str && has_num {
            return Ok(py.None().into_bound(py));
        }
        items.push(el.to_object(py));
    }

    let op = if is_max { CompareOp::Gt } else { CompareOp::Lt };
    let mut best = items[0].clone_ref(py).into_bound(py);

    for i in 1..len {
        let current = items[i].clone_ref(py).into_bound(py);
        if current.rich_compare(&best, op)?.is_truthy()? {
            best = current;
        }
    }
    Ok(best.into_any())
}

fn eval_merge<'py>(py: Python<'py>, value: &Bounded<'py>, items: &[Node]) -> Result<'py> {
    let out = PyDict::new_bound(py);
    for it in items {
        let obj = eval_any(py, it, value)?;
        if let Ok(dict) = obj.downcast::<PyDict>() {
            out.update(dict.as_mapping())?;
        } else {
            return Ok(py.None().into_bound(py));
        }
    }
    Ok(out.into_any())
}
fn eval_not_null<'py>(py: Python<'py>, value: &Bounded<'py>, items: &[Node]) -> Result<'py> {
    for it in items {
        let v = eval_any(py, it, value)?;
        if !v.is_none() {
            return Ok(v);
        }
    }
    Ok(py.None().into_bound(py))
}

fn eval_reverse<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> Result<'py> {
    let xv = eval_any(py, x, value)?;
    if is_list(&xv) {
        return xv
            .get_item(PySlice::new_bound(py, isize::MAX, isize::MIN, -1isize))
            .map(|any| any.into_any());
    }
    if let Ok(s) = xv.extract::<&str>() {
        let reversed: String = s.chars().rev().collect();
        return Ok(PyString::new_bound(py, &reversed).into_any());
    }
    Ok(py.None().into_bound(py))
}

fn eval_sum<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> Result<'py> {
    let xv = eval_any(py, x, value)?;
    if !is_list(&xv) {
        return Ok(py.None().into_bound(py));
    }
    let seq = xv.downcast::<PySequence>()?;
    let len = seq.len()?;
    if len == 0 {
        return Ok(py.None().into_bound(py));
    }
    let mut sum = 0.0;
    for i in 0..len {
        let el = seq.get_item(i)?;
        if !is_number(&el) {
            return Ok(py.None().into_bound(py));
        }
        sum += el.extract::<f64>()?;
    }
    Ok(sum.to_object(py).into_bound(py).into_any())
}

fn eval_dtype<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> Result<'py> {
    let xv = eval_any(py, x, value)?;
    let dtype_str = if is_number(&xv) {
        "number"
    } else if xv.is_instance_of::<PyUnicode>() {
        "string"
    } else if xv.is_instance_of::<PyBool>() {
        "boolean"
    } else if is_list(&xv) {
        "array"
    } else if is_object(&xv) {
        "object"
    } else if xv.is_none() {
        "null"
    } else {
        return Ok(py.None().into_bound(py));
    };
    Ok(PyString::new_bound(py, dtype_str).into_any())
}
