use crate::nodes::Node;
use std::fmt;

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::This => write!(f, "@"),
            Node::Field(name) => write!(f, "{}", name),
            Node::Index(i) => write!(f, "[{}]", i),
            Node::Slice(start, end, step) => write!(f, "[{}]", display_slice(start, end, step)),
            Node::Literal(obj) => write!(f, "{:?}", obj),

            Node::SubExpr(lhs, rhs) => {
                if matches!(**lhs, Node::This) {
                    write!(f, "{}", rhs)
                } else {
                    match **rhs {
                        Node::Field(ref name) => write!(f, "{}.{}", lhs, name),
                        Node::Index(_) | Node::Slice(_, _, _) => write!(f, "{}{}", lhs, rhs),
                        _ => write!(f, "{}.({})", lhs, rhs),
                    }
                }
            }

            Node::Pipe(lhs, rhs) => write!(f, "{} | {}", lhs, rhs),
            Node::MultiList(items) => {
                write!(f, "[{}]", display_items(items))
            }
            Node::MultiDict(items) => {
                write!(f, "{{{}}}", display_multidict(items))
            }
            Node::ProjectArray { base, rhs } => write!(f, "{}[*].{}", base, rhs),
            Node::ProjectObject { base, rhs } => write!(f, "{}.*.{}", base, rhs),

            Node::Flatten(inner) => write!(f, "{}[]", inner),
            Node::FilterProjection { base, then, cond } => {
                write!(f, "{}[?{}]", base, cond)?;
                if !matches!(**then, Node::This) {
                    write!(f, ".{}", then)?;
                }
                Ok(())
            }

            Node::And(a, b) => write!(f, "({} && {})", a, b),
            Node::Or(a, b) => write!(f, "({} || {})", a, b),
            Node::Not(x) => write!(f, "!({})", x),

            Node::CmpEq(a, b) => write!(f, "{} == {}", a, b),
            Node::CmpNe(a, b) => write!(f, "{} != {}", a, b),
            Node::CmpLt(a, b) => write!(f, "{} < {}", a, b),
            Node::CmpLe(a, b) => write!(f, "{} <= {}", a, b),
            Node::CmpGt(a, b) => write!(f, "{} > {}", a, b),
            Node::CmpGe(a, b) => write!(f, "{} >= {}", a, b),

            Node::Length(x) => write!(f, "length({})", x),
            Node::Sort(x) => write!(f, "sort({})", x),
            Node::Keys(x) => write!(f, "keys({})", x),
            Node::Values(x) => write!(f, "values({})", x),
            Node::ToArray(x) => write!(f, "to_array({})", x),
            Node::ToString(x) => write!(f, "to_string({})", x),
            Node::ToNumber(x) => write!(f, "to_number({})", x),
            Node::Abs(x) => write!(f, "abs({})", x),
            Node::Avg(x) => write!(f, "avg({})", x),
            Node::Ceil(x) => write!(f, "ceil({})", x),
            Node::Floor(x) => write!(f, "floor({})", x),
            Node::Max(x) => write!(f, "max({})", x),
            Node::Min(x) => write!(f, "min({})", x),
            Node::Reverse(x) => write!(f, "reverse({})", x),
            Node::Sum(x) => write!(f, "sum({})", x),
            Node::Type(x) => write!(f, "type({})", x),

            Node::Contains(a, b) => write!(f, "contains({}, {})", a, b),
            Node::EndsWith(a, b) => write!(f, "ends_with({}, {})", a, b),
            Node::StartsWith(a, b) => write!(f, "starts_with({}, {})", a, b),
            Node::Join(a, b) => write!(f, "join({}, {})", a, b),

            Node::Merge(items) => {
                write!(f, "merge({})", display_items(items))
            }
            Node::NotNull(items) => {
                write!(f, "not_null({})", display_items(items))
            }

            Node::MapApply { base, key } => write!(f, "map(&{}, {})", key, base),
            Node::SortBy { base, key } => write!(f, "sort_by({}, &{})", base, key),
            Node::MinBy { base, key } => write!(f, "min_by({}, &{})", base, key),
            Node::MaxBy { base, key } => write!(f, "max_by({}, &{})", base, key),
        }
    }
}

fn display_items(items: &[Node]) -> String {
    items
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

fn display_multidict(items: &[(String, Node)]) -> String {
    items
        .iter()
        .map(|(k, v)| format!("\"{}\": {}", k, v))
        .collect::<Vec<_>>()
        .join(", ")
}

fn display_slice(start: &Option<isize>, end: &Option<isize>, step: &Option<isize>) -> String {
    let s = start.map(|v| v.to_string()).unwrap_or_default();
    let e = end.map(|v| v.to_string()).unwrap_or_default();

    if let Some(st) = step {
        format!("{}:{}:{}", s, e, st)
    } else {
        format!("{}:{}", s, e)
    }
}
