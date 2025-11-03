use crate::checks::*;
use crate::nodes::Node;
use crate::nodes::PyObjectWrapper;
use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::*;

const BUILTINS: &str = "builtins";
const SORTED: &str = "sorted";

type Result<'py> = PyResult<Bound<'py, PyAny>>;
type Bounded<'py> = Bound<'py, PyAny>;

pub fn eval_any<'py>(py: Python<'py>, node: &Node, value: &Bounded<'py>) -> Result<'py> {
    match node {
        Node::This => Ok(value.clone()),
        Node::Literal(obj) => eval_literal(py, obj),
        Node::Field { base, name } => eval_field(py, value, base, name),
        Node::Index { base, index } => eval_index(py, value, base, index),
        Node::ListSlice {
            base,
            start,
            end,
            step,
        } => eval_list_slice(py, value, base, start, end, step),
        Node::StrSlice {
            base,
            start,
            end,
            step,
        } => eval_str_slice(py, value, base, start, end, step),
        Node::MultiList(items) => eval_multi_list(py, value, items),
        Node::MultiDict(items) => eval_multi_dict(py, value, items),
        Node::Flatten(inner) => eval_flatten(py, value, inner),
        Node::Filter { base, condition } => eval_filter(py, value, base, condition),
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
        Node::MapApply { base, key } => map_apply(py, value, base, key),
        Node::SortBy { base, key } => sort_like(py, value, base, key, SortKind::SortBy),
        Node::MinBy { base, key } => sort_like(py, value, base, key, SortKind::MinBy),
        Node::MaxBy { base, key } => sort_like(py, value, base, key, SortKind::MaxBy),
        Node::Abs(x) => eval_abs(py, value, x),
        Node::Avg(x) => eval_avg(py, value, x),
        Node::Ceil(x) => eval_ceil_floor(py, value, x, true),
        Node::ListContains(a, b) => eval_list_contains(py, value, a, b),
        Node::StrContains(a, b) => eval_str_contains(py, value, a, b),
        Node::EndsWith(a, b) => eval_starts_ends_with(py, value, a, b, false),
        Node::Floor(x) => eval_ceil_floor(py, value, x, false),
        Node::Join(a, b) => eval_join(py, value, a, b),
        Node::Max(x) => eval_min_max(py, value, x, true),
        Node::Merge(items) => eval_merge(py, value, items),
        Node::Min(x) => eval_min_max(py, value, x, false),
        Node::NotNull(items) => eval_not_null(py, value, items),
        Node::ListReverse(x) => eval_list_reverse(py, value, x),
        Node::StrReverse(x) => eval_str_reverse(py, value, x),
        Node::StartsWith(a, b) => eval_starts_ends_with(py, value, a, b, true),
        Node::Sum(x) => eval_sum(py, value, x),
    }
}

fn eval_literal<'py>(py: Python<'py>, obj: &PyObjectWrapper) -> Result<'py> {
    Ok(obj.0.clone_ref(py).into_bound(py).into_any())
}

fn eval_field<'py>(py: Python<'py>, value: &Bounded<'py>, base: &Node, name: &str) -> Result<'py> {
    let base_evaluated = eval_any(py, base, value)?;
    if !is_object(&base_evaluated) {
        return Ok(py.None().into_bound(py));
    }

    Ok(base_evaluated
        .downcast::<PyDict>()?
        .get_item(name)?
        .unwrap_or_else(|| py.None().into_bound(py)))
}

fn eval_index<'py>(py: Python<'py>, value: &Bounded<'py>, base: &Node, i: &isize) -> Result<'py> {
    if let Ok(seq) = eval_any(py, base, value)?.downcast::<PySequence>() {
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

fn eval_list_slice<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    base: &Node,
    start: &Option<isize>,
    end: &Option<isize>,
    step: &Option<isize>,
) -> Result<'py> {
    let base_evaluated = eval_any(py, base, value)?;
    if !is_list(&base_evaluated) {
        return Ok(py.None().into_bound(py));
    }

    Ok(base_evaluated
        .get_item(PySlice::new_bound(
            py,
            start.unwrap_or(0),
            end.unwrap_or(isize::MAX),
            step.unwrap_or(1),
        ))?
        .into_any())
}

fn eval_str_slice<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    base: &Node,
    start: &Option<isize>,
    end: &Option<isize>,
    step: &Option<isize>,
) -> Result<'py> {
    let base_evaluated = eval_any(py, base, value)?;
    if !is_string(&base_evaluated) {
        return Ok(py.None().into_bound(py));
    }

    Ok(base_evaluated
        .get_item(PySlice::new_bound(
            py,
            start.unwrap_or(0),
            end.unwrap_or(isize::MAX),
            step.unwrap_or(1),
        ))?
        .into_any())
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

fn eval_flatten<'py>(py: Python<'py>, value: &Bounded<'py>, inner: &Node) -> Result<'py> {
    let base_evaluated = eval_any(py, inner, value)?;
    if !is_list(&base_evaluated) {
        return Ok(py.None().into_bound(py));
    }

    let list = base_evaluated.downcast::<PyList>()?;
    let output = PyList::empty_bound(py);

    for element in list.iter() {
        if is_list(&element) {
            let sequence = element.downcast::<PySequence>()?;
            for j in 0..sequence.len()? {
                output.append(sequence.get_item(j)?)?;
            }
        } else {
            output.append(element)?;
        }
    }

    Ok(output.into_any())
}

fn eval_filter<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    base: &Node,
    condition: &Node,
) -> Result<'py> {
    let base_evaluated = eval_any(py, base, value)?;
    if !is_list(&base_evaluated) {
        return Ok(py.None().into_bound(py));
    }

    let sequence = base_evaluated.downcast::<PySequence>()?;
    let output = PyList::empty_bound(py);

    for i in 0..sequence.len()? {
        let element = sequence.get_item(i)?;
        if eval_any(py, condition, &element)?.is_truthy()? {
            output.append(element)?;
        }
    }

    Ok(output.into_any())
}

fn eval_and<'py>(py: Python<'py>, value: &Bounded<'py>, a: &Node, b: &Node) -> Result<'py> {
    let left = eval_any(py, a, &value)?;
    if left.is_truthy()? {
        eval_any(py, b, value)
    } else {
        Ok(left)
    }
}

fn eval_or<'py>(py: Python<'py>, value: &Bounded<'py>, a: &Node, b: &Node) -> Result<'py> {
    let left = eval_any(py, a, &value)?;
    if left.is_truthy()? {
        Ok(left)
    } else {
        eval_any(py, b, value)
    }
}

fn eval_not<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> Result<'py> {
    let result = !eval_any(py, x, value)?.is_truthy()?;
    Ok(result.to_object(py).into_bound(py).into_any())
}

fn eval_length<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> Result<'py> {
    let evaluated = eval_any(py, x, value)?;
    if !is_sized(&evaluated) {
        return Ok(py.None().into_bound(py));
    }

    let length = evaluated.len()? as i64;
    Ok(length.to_object(py).into_bound(py).into_any())
}

fn eval_sort<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> Result<'py> {
    let evaluated = eval_any(py, x, value)?;
    if !is_list(&evaluated) {
        return Ok(py.None().into_bound(py));
    }

    Ok(py
        .import_bound(BUILTINS)?
        .getattr(SORTED)?
        .call1((evaluated,))?)
}

fn eval_keys<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> Result<'py> {
    let evaluated = eval_any(py, x, value)?;
    if !is_object(&evaluated) {
        return Ok(py.None().into_bound(py));
    }

    Ok(evaluated.downcast::<PyDict>()?.keys().into_any())
}

fn eval_values<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> Result<'py> {
    let evaluated = eval_any(py, x, value)?;
    if !is_object(&evaluated) {
        return Ok(py.None().into_bound(py));
    }

    Ok(evaluated.downcast::<PyDict>()?.values().into_any())
}

fn map_apply<'py>(py: Python<'py>, value: &Bounded<'py>, base: &Node, key: &Node) -> Result<'py> {
    let base_evaluated = eval_any(py, base, value)?;
    if !is_list(&base_evaluated) {
        return Ok(py.None().into_bound(py));
    }

    let sequence = base_evaluated.downcast::<PySequence>()?;
    let output = PyList::empty_bound(py);

    for i in 0..sequence.len()? {
        let element = sequence.get_item(i)?;
        output.append(eval_any(py, key, &element)?)?;
    }

    Ok(output.into_any())
}

fn cmp_bool<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    a: &Node,
    b: &Node,
    op: CompareOp,
) -> Result<'py> {
    let left = eval_any(py, a, value)?;
    let right = eval_any(py, b, value)?;

    let result = match op {
        CompareOp::Eq => is_eq(&left, &right)?,
        CompareOp::Ne => !is_eq(&left, &right)?,
        _ => {
            if !(is_number(&left) && is_number(&right)) {
                false
            } else {
                left.rich_compare(&right, op)?.is_truthy()?
            }
        }
    };

    Ok(result.to_object(py).into_bound(py).into_any())
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
    let base_evaluated = eval_any(py, base, value)?;
    if !is_list(&base_evaluated) {
        return Ok(py.None().into_bound(py));
    }

    let list = base_evaluated.downcast::<PyList>()?;

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
        let key_value = eval_any(py, key, &element)?;
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
fn eval_abs<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> Result<'py> {
    let evaluated = eval_any(py, x, value)?;
    if !is_number(&evaluated) {
        return Ok(py.None().into_bound(py));
    }
    Ok(evaluated
        .extract::<f64>()?
        .abs()
        .to_object(py)
        .into_bound(py)
        .into_any())
}

fn eval_avg<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> Result<'py> {
    let evaluated = eval_any(py, x, value)?;
    if !is_list(&evaluated) {
        return Ok(py.None().into_bound(py));
    }

    let sequence = evaluated.downcast::<PySequence>()?;
    let length = sequence.len()?;

    if length == 0 {
        return Ok(py.None().into_bound(py));
    }

    let mut sum = 0.0;
    for i in 0..length {
        let element = sequence.get_item(i)?;
        if !is_number(&element) {
            return Ok(py.None().into_bound(py));
        }
        sum += element.extract::<f64>()?;
    }

    let average = sum / (length as f64);
    Ok(average.to_object(py).into_bound(py).into_any())
}

fn eval_ceil_floor<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    x: &Node,
    is_ceil: bool,
) -> Result<'py> {
    let evaluated = eval_any(py, x, value)?;
    if !is_number(&evaluated) {
        return Ok(py.None().into_bound(py));
    }

    let f = evaluated.extract::<f64>()?;
    let result = if is_ceil { f.ceil() } else { f.floor() };
    Ok(result.to_object(py).into_bound(py).into_any())
}

fn eval_list_contains<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    a: &Node,
    b: &Node,
) -> Result<'py> {
    let subject = eval_any(py, a, value)?;
    let search = eval_any(py, b, value)?;

    if !is_list(&subject) {
        return Ok(py.None().into_bound(py));
    }

    let sequence = subject.downcast::<PySequence>()?;
    let mut found = false;

    for i in 0..sequence.len()? {
        if sequence.get_item(i)?.eq(&search)? {
            found = true;
            break;
        }
    }

    Ok(found.to_object(py).into_bound(py).into_any())
}

fn eval_str_contains<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    a: &Node,
    b: &Node,
) -> Result<'py> {
    let subject = eval_any(py, a, value)?;
    let search = eval_any(py, b, value)?;

    if !is_string(&subject) || !is_string(&search) {
        return Ok(py.None().into_bound(py));
    }

    Ok(subject
        .extract::<&str>()?
        .contains(search.extract::<&str>()?)
        .to_object(py)
        .into_bound(py)
        .into_any())
}

fn eval_starts_ends_with<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    a: &Node,
    b: &Node,
    is_starts_with: bool,
) -> Result<'py> {
    let subject = eval_any(py, a, value)?;
    let search = eval_any(py, b, value)?;

    let result = if is_string(&subject) && is_string(&search) {
        let subject_str = subject.extract::<&str>()?;
        let search_str = search.extract::<&str>()?;
        if is_starts_with {
            subject_str.starts_with(search_str)
        } else {
            subject_str.ends_with(search_str)
        }
    } else {
        false
    };

    Ok(result.to_object(py).into_bound(py).into_any())
}

fn eval_join<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    glue_node: &Node,
    array_node: &Node,
) -> Result<'py> {
    let glue = eval_any(py, glue_node, value)?;
    let array = eval_any(py, array_node, value)?;

    if !is_string(&glue) || !is_list(&array) {
        return Ok(py.None().into_bound(py));
    }

    let glue_str = glue.extract::<&str>()?;
    let sequence = array.downcast::<PySequence>()?;
    let length = sequence.len()?;
    let mut parts: Vec<String> = Vec::with_capacity(length);

    for i in 0..length {
        let element = sequence.get_item(i)?;
        if !is_string(&element) {
            return Ok(py.None().into_bound(py));
        }
        parts.push(element.extract::<String>()?);
    }

    Ok(PyString::new_bound(py, &parts.join(glue_str)).into_any())
}

fn eval_min_max<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node, is_max: bool) -> Result<'py> {
    let evaluated = eval_any(py, x, value)?;
    if !is_list(&evaluated) {
        return Ok(py.None().into_bound(py));
    }

    let sequence = evaluated.downcast::<PySequence>()?;
    let length = sequence.len()?;

    if length == 0 {
        return Ok(py.None().into_bound(py));
    }

    let first = sequence.get_item(0)?;
    let expect_number = is_number(&first);
    let expect_string = is_string(&first);

    if !expect_number && !expect_string {
        return Ok(py.None().into_bound(py));
    }

    let op = if is_max { CompareOp::Gt } else { CompareOp::Lt };
    let mut best = first;

    for i in 1..length {
        let current = sequence.get_item(i)?;
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

fn eval_merge<'py>(py: Python<'py>, value: &Bounded<'py>, items: &[Node]) -> Result<'py> {
    let output = PyDict::new_bound(py);

    for item in items {
        let evaluated = eval_any(py, item, value)?;
        if let Ok(dict) = evaluated.downcast::<PyDict>() {
            output.update(dict.as_mapping())?;
        } else {
            return Ok(py.None().into_bound(py));
        }
    }

    Ok(output.into_any())
}

fn eval_not_null<'py>(py: Python<'py>, value: &Bounded<'py>, items: &[Node]) -> Result<'py> {
    for item in items {
        let evaluated = eval_any(py, item, value)?;
        if !evaluated.is_none() {
            return Ok(evaluated);
        }
    }
    Ok(py.None().into_bound(py))
}

fn eval_list_reverse<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> Result<'py> {
    let evaluated = eval_any(py, x, value)?;
    if !is_list(&evaluated) {
        return Ok(py.None().into_bound(py));
    }

    evaluated
        .get_item(PySlice::new_bound(py, isize::MAX, isize::MIN, -1isize))
        .map(|any| any.into_any())
}

fn eval_str_reverse<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> Result<'py> {
    let evaluated = eval_any(py, x, value)?;
    if !is_string(&evaluated) {
        return Ok(py.None().into_bound(py));
    }

    let reversed: String = evaluated.extract::<&str>()?.chars().rev().collect();
    Ok(PyString::new_bound(py, &reversed).into_any())
}

fn eval_sum<'py>(py: Python<'py>, value: &Bounded<'py>, x: &Node) -> Result<'py> {
    let evaluated = eval_any(py, x, value)?;
    if !is_list(&evaluated) {
        return Ok(py.None().into_bound(py));
    }

    let sequence = evaluated.downcast::<PySequence>()?;
    let length = sequence.len()?;

    if length == 0 {
        return Ok(0.to_object(py).into_bound(py).into_any());
    }

    let mut sum = 0.0;
    for i in 0..length {
        let element = sequence.get_item(i)?;
        if !is_number(&element) {
            return Ok(py.None().into_bound(py));
        }
        sum += element.extract::<f64>()?;
    }

    Ok(sum.to_object(py).into_bound(py).into_any())
}
