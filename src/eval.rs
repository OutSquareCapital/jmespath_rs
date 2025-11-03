use crate::checks::*;
use crate::nodes::{ComparisonOp, ListOp, Node, PyObjectWrapper, ScalarOp, StrOp, StructOp};
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
        Node::MultiList(items) => eval_multi_list(py, value, items),
        Node::MultiDict(items) => eval_multi_dict(py, value, items),
        Node::And(a, b) => eval_and(py, value, a, b),
        Node::Or(a, b) => eval_or(py, value, a, b),
        Node::Not(x) => eval_not(py, value, x),
        Node::NotNull(items) => eval_not_null(py, value, items),
        Node::Length(x) => eval_length(py, value, x),
        Node::Merge(items) => eval_merge(py, value, items),
        Node::List(base, op) => {
            let base_evaluated = eval_any(py, base, value)?;
            if !is_list(&base_evaluated) {
                return Ok(py.None().into_bound(py));
            }
            eval_list_op(py, value, &base_evaluated, op)
        }
        Node::Str(base, op) => {
            let base_evaluated = eval_any(py, base, value)?;
            if !is_string(&base_evaluated) {
                return Ok(py.None().into_bound(py));
            }
            eval_str_op(py, value, &base_evaluated, op)
        }
        Node::Struct(base, op) => {
            let base_evaluated = eval_any(py, base, value)?;
            if !is_object(&base_evaluated) {
                return Ok(py.None().into_bound(py));
            }
            eval_struct_op(py, value, &base_evaluated, op)
        }
        Node::Scalar(base, op) => {
            let base_evaluated = eval_any(py, base, value)?;
            if !is_number(&base_evaluated) {
                return Ok(py.None().into_bound(py));
            }
            eval_scalar_op(py, &base_evaluated, op)
        }
        Node::Compare(base, op) => eval_comparison_op(py, value, base, op),
    }
}

fn eval_list_op<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    list: &Bounded<'py>,
    op: &ListOp,
) -> Result<'py> {
    match op {
        ListOp::Index(i) => eval_list_index(py, list, *i),
        ListOp::Slice { start, end, step } => eval_list_slice(py, list, start, end, step),
        ListOp::Reverse => eval_list_reverse(py, list),
        ListOp::Flatten => eval_list_flatten(py, list),
        ListOp::Contains(search_node) => {
            let search = eval_any(py, search_node, value)?;
            eval_list_contains(py, list, &search)
        }
        ListOp::Join(glue_node) => eval_list_join(py, &eval_any(py, glue_node, value)?, list),
        ListOp::Filter(cond) => eval_list_filter(py, list, cond),
        ListOp::Map(key) => eval_list_map(py, list, key),
        ListOp::Sort => eval_list_sort(py, list),
        ListOp::Max => eval_list_min_max(py, list, true),
        ListOp::Min => eval_list_min_max(py, list, false),
        ListOp::Sum => eval_list_sum(py, list),
        ListOp::Avg => eval_list_avg(py, list),
        ListOp::SortBy(key) => eval_list_sort_by(py, list, key),
        ListOp::MinBy(key) => eval_list_min_by(py, list, key),
        ListOp::MaxBy(key) => eval_list_max_by(py, list, key),
    }
}

fn eval_scalar_op<'py>(py: Python<'py>, number: &Bounded<'py>, op: &ScalarOp) -> Result<'py> {
    match op {
        ScalarOp::Abs => eval_abs(py, number),
        ScalarOp::Ceil => eval_ceil(py, number),
        ScalarOp::Floor => eval_floor(py, number),
    }
}

fn eval_comparison_op<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    base: &Node,
    op: &ComparisonOp,
) -> Result<'py> {
    let base_evaluated = eval_any(py, base, value)?;
    match op {
        ComparisonOp::Eq(other_node) => {
            eval_eq(py, &base_evaluated, &eval_any(py, other_node, value)?)
        }
        ComparisonOp::Ne(other_node) => {
            eval_ne(py, &base_evaluated, &eval_any(py, other_node, value)?)
        }
        ComparisonOp::Lt(other_node) => cmp_bool(
            py,
            &base_evaluated,
            &eval_any(py, other_node, value)?,
            CompareOp::Lt,
        ),
        ComparisonOp::Le(other_node) => cmp_bool(
            py,
            &base_evaluated,
            &eval_any(py, other_node, value)?,
            CompareOp::Le,
        ),
        ComparisonOp::Gt(other_node) => cmp_bool(
            py,
            &base_evaluated,
            &eval_any(py, other_node, value)?,
            CompareOp::Gt,
        ),
        ComparisonOp::Ge(other_node) => cmp_bool(
            py,
            &base_evaluated,
            &eval_any(py, other_node, value)?,
            CompareOp::Ge,
        ),
    }
}

fn eval_str_op<'py>(
    py: Python<'py>,
    value: &Bounded<'py>,
    string: &Bounded<'py>,
    op: &StrOp,
) -> Result<'py> {
    match op {
        StrOp::Slice { start, end, step } => eval_str_slice(py, string, start, end, step),
        StrOp::Reverse => eval_str_reverse(py, string),
        StrOp::Contains(search_node) => {
            eval_str_contains(py, string, &eval_any(py, search_node, value)?)
        }
        StrOp::StartsWith(prefix_node) => {
            eval_starts_with(py, string, &eval_any(py, prefix_node, value)?)
        }
        StrOp::EndsWith(suffix_node) => {
            eval_ends_with(py, string, &eval_any(py, suffix_node, value)?)
        }
    }
}
fn eval_struct_op<'py>(
    py: Python<'py>,
    _value: &Bounded<'py>,
    dict: &Bounded<'py>,
    op: &StructOp,
) -> Result<'py> {
    match op {
        StructOp::Field(name) => eval_field(py, dict, name),
        StructOp::Keys => eval_keys(dict),
        StructOp::Values => eval_values(dict),
    }
}

fn eval_literal<'py>(py: Python<'py>, obj: &PyObjectWrapper) -> Result<'py> {
    Ok(obj.0.clone_ref(py).into_bound(py).into_any())
}

fn eval_field<'py>(py: Python<'py>, dict: &Bounded<'py>, name: &str) -> Result<'py> {
    Ok(dict
        .downcast::<PyDict>()?
        .get_item(name)?
        .unwrap_or_else(|| py.None().into_bound(py)))
}

fn eval_list_index<'py>(py: Python<'py>, list: &Bounded<'py>, i: isize) -> Result<'py> {
    let seq = list.downcast::<PySequence>()?;
    let len = seq.len()? as isize;
    let idx = if i < 0 { len + i } else { i };
    if idx < 0 || idx >= len {
        Ok(py.None().into_bound(py))
    } else {
        Ok(seq.get_item(idx as usize)?)
    }
}

fn eval_list_slice<'py>(
    py: Python<'py>,
    list: &Bounded<'py>,
    start: &Option<isize>,
    end: &Option<isize>,
    step: &Option<isize>,
) -> Result<'py> {
    Ok(list
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
    string: &Bounded<'py>,
    start: &Option<isize>,
    end: &Option<isize>,
    step: &Option<isize>,
) -> Result<'py> {
    Ok(string
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

fn eval_list_flatten<'py>(py: Python<'py>, list: &Bounded<'py>) -> Result<'py> {
    let list_py = list.downcast::<PyList>()?;
    let output = PyList::empty_bound(py);

    for element in list_py.iter() {
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

fn eval_list_filter<'py>(py: Python<'py>, list: &Bounded<'py>, cond: &Node) -> Result<'py> {
    let sequence = list.downcast::<PySequence>()?;
    let output = PyList::empty_bound(py);

    for i in 0..sequence.len()? {
        let element = sequence.get_item(i)?;
        if eval_any(py, cond, &element)?.is_truthy()? {
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

fn eval_list_sort<'py>(py: Python<'py>, list: &Bounded<'py>) -> Result<'py> {
    Ok(py.import_bound(BUILTINS)?.getattr(SORTED)?.call1((list,))?)
}
fn eval_keys<'py>(dict: &Bounded<'py>) -> Result<'py> {
    Ok(dict.downcast::<PyDict>()?.keys().into_any())
}

fn eval_values<'py>(dict: &Bounded<'py>) -> Result<'py> {
    Ok(dict.downcast::<PyDict>()?.values().into_any())
}

fn eval_list_map<'py>(py: Python<'py>, list: &Bounded<'py>, key: &Node) -> Result<'py> {
    let sequence = list.downcast::<PySequence>()?;
    let output = PyList::empty_bound(py);

    for i in 0..sequence.len()? {
        let element = sequence.get_item(i)?;
        output.append(eval_any(py, key, &element)?)?;
    }

    Ok(output.into_any())
}

fn cmp_bool<'py>(
    py: Python<'py>,
    left: &Bounded<'py>,
    right: &Bounded<'py>,
    op: CompareOp,
) -> Result<'py> {
    let result = if is_number(left) && is_number(right) {
        left.rich_compare(right, op)?.is_truthy()?
    } else {
        false
    };
    Ok(result.to_object(py).into_bound(py).into_any())
}

enum SortKind {
    SortBy,
    MinBy,
    MaxBy,
}

fn eval_list_sort_like<'py>(
    py: Python<'py>,
    list: &Bounded<'py>,
    key: &Node,
    kind: SortKind,
) -> Result<'py> {
    let list_py = list.downcast::<PyList>()?;

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
        Vec::with_capacity(list_py.len());

    for element in list_py.iter() {
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
fn eval_abs<'py>(py: Python<'py>, number: &Bounded<'py>) -> Result<'py> {
    Ok(number
        .extract::<f64>()?
        .abs()
        .to_object(py)
        .into_bound(py)
        .into_any())
}

fn eval_list_avg<'py>(py: Python<'py>, list: &Bounded<'py>) -> Result<'py> {
    let sequence = list.downcast::<PySequence>()?;
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
fn eval_ceil<'py>(py: Python<'py>, number: &Bounded<'py>) -> Result<'py> {
    let result = number.extract::<f64>()?.ceil();
    Ok(result.to_object(py).into_bound(py).into_any())
}

fn eval_floor<'py>(py: Python<'py>, number: &Bounded<'py>) -> Result<'py> {
    let result = number.extract::<f64>()?.floor();
    Ok(result.to_object(py).into_bound(py).into_any())
}

fn eval_list_contains<'py>(
    py: Python<'py>,
    list: &Bounded<'py>,
    search: &Bounded<'py>,
) -> Result<'py> {
    let sequence = list.downcast::<PySequence>()?;
    let mut found = false;

    for i in 0..sequence.len()? {
        if sequence.get_item(i)?.eq(search)? {
            found = true;
            break;
        }
    }

    Ok(found.to_object(py).into_bound(py).into_any())
}

fn eval_str_contains<'py>(
    py: Python<'py>,
    string: &Bounded<'py>,
    search: &Bounded<'py>,
) -> Result<'py> {
    if !is_string(search) {
        return Ok(py.None().into_bound(py));
    }

    Ok(string
        .extract::<&str>()?
        .contains(search.extract::<&str>()?)
        .to_object(py)
        .into_bound(py)
        .into_any())
}

fn eval_starts_with<'py>(
    py: Python<'py>,
    string: &Bounded<'py>,
    prefix: &Bounded<'py>,
) -> Result<'py> {
    if !is_string(prefix) {
        return Ok(false.to_object(py).into_bound(py).into_any());
    }

    let result = string
        .extract::<&str>()?
        .starts_with(prefix.extract::<&str>()?);
    Ok(result.to_object(py).into_bound(py).into_any())
}

fn eval_ends_with<'py>(
    py: Python<'py>,
    string: &Bounded<'py>,
    suffix: &Bounded<'py>,
) -> Result<'py> {
    if !is_string(suffix) {
        return Ok(false.to_object(py).into_bound(py).into_any());
    }

    let result = string
        .extract::<&str>()?
        .ends_with(suffix.extract::<&str>()?);
    Ok(result.to_object(py).into_bound(py).into_any())
}
fn eval_list_join<'py>(py: Python<'py>, glue: &Bounded<'py>, list: &Bounded<'py>) -> Result<'py> {
    let glue_str = glue.extract::<&str>()?;
    let sequence = list.downcast::<PySequence>()?;
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
fn eval_list_min_max<'py>(py: Python<'py>, list: &Bounded<'py>, is_max: bool) -> Result<'py> {
    let sequence = list.downcast::<PySequence>()?;
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

fn eval_list_reverse<'py>(py: Python<'py>, list: &Bounded<'py>) -> Result<'py> {
    list.get_item(PySlice::new_bound(py, isize::MAX, isize::MIN, -1isize))
        .map(|any| any.into_any())
}
fn eval_str_reverse<'py>(py: Python<'py>, string: &Bounded<'py>) -> Result<'py> {
    let reversed: String = string.extract::<&str>()?.chars().rev().collect();
    Ok(PyString::new_bound(py, &reversed).into_any())
}

fn eval_list_sum<'py>(py: Python<'py>, list: &Bounded<'py>) -> Result<'py> {
    let sequence = list.downcast::<PySequence>()?;
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

fn eval_list_sort_by<'py>(py: Python<'py>, list: &Bounded<'py>, key: &Node) -> Result<'py> {
    eval_list_sort_like(py, list, key, SortKind::SortBy)
}

fn eval_list_min_by<'py>(py: Python<'py>, list: &Bounded<'py>, key: &Node) -> Result<'py> {
    eval_list_sort_like(py, list, key, SortKind::MinBy)
}

fn eval_list_max_by<'py>(py: Python<'py>, list: &Bounded<'py>, key: &Node) -> Result<'py> {
    eval_list_sort_like(py, list, key, SortKind::MaxBy)
}
fn eval_eq<'py>(py: Python<'py>, left: &Bounded<'py>, right: &Bounded<'py>) -> Result<'py> {
    Ok(is_eq(left, right)?.to_object(py).into_bound(py).into_any())
}

fn eval_ne<'py>(py: Python<'py>, left: &Bounded<'py>, right: &Bounded<'py>) -> Result<'py> {
    let result = !is_eq(left, right)?;
    Ok(result.to_object(py).into_bound(py).into_any())
}
