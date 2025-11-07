use serde_json::Value;

#[derive(Debug, Clone)]
pub(crate) enum Node {
    This,
    Literal(Value),
    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),
    Not(Box<Node>),
    Coalesce(Vec<Node>),
    List(Box<Node>, ListOp),
    Str(Box<Node>, StrOp),
    Struct(Box<Node>, StructOp),
    Compare(Box<Node>, ComparisonOp),
}

#[derive(Debug, Clone)]
pub(crate) enum ListOp {
    Index(isize),
    Slice {
        start: Option<isize>,
        end: Option<isize>,
        step: Option<isize>,
    },
    Length,
    Reverse,
    Flatten,
    Contains(Box<Node>),
    Filter(Box<Node>),
    Map(Box<Node>),
    Join(String),
}

#[derive(Debug, Clone)]
pub(crate) enum StrOp {
    Slice {
        start: Option<isize>,
        end: Option<isize>,
        step: Option<isize>,
    },
    Reverse,
    Contains(String),
    StartsWith(String),
    EndsWith(String),
    Length,
}

#[derive(Debug, Clone)]
pub(crate) enum StructOp {
    Field(String),
    Keys,
    Values,
}

#[derive(Debug, Clone)]
pub(crate) enum ComparisonOp {
    Eq(Box<Node>),
    Ne(Box<Node>),
    Lt(Box<Node>),
    Le(Box<Node>),
    Gt(Box<Node>),
    Ge(Box<Node>),
}
