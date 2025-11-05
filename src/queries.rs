use crate::matchs::match_any;
use crate::nodes;
use pyo3::prelude::*;
use std::marker::PhantomData;

fn into_lit(py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<nodes::Node> {
    if let Ok(expr) = obj.extract::<PyRef<Expr>>() {
        return Ok(expr.node.clone());
    }
    Ok(nodes::Node::Literal(nodes::PyObjectWrapper(
        obj.to_object(py),
    )))
}

type OpWrapper<Op> = NameSpaceBuilder<Op, fn(Box<nodes::Node>, Op) -> nodes::Node>;

struct NameSpaceBuilder<Op, WrapperFn> {
    expr: Expr,
    wrapper: WrapperFn,
    _phantom: PhantomData<Op>,
}

impl<Op, WrapperFn> NameSpaceBuilder<Op, WrapperFn> {
    fn new(expr: Expr, wrapper: WrapperFn) -> Self {
        Self {
            expr,
            wrapper,
            _phantom: PhantomData,
        }
    }
    fn wrap(&self, op: Op) -> Expr
    where
        WrapperFn: Fn(Box<nodes::Node>, Op) -> nodes::Node,
    {
        Expr {
            node: (self.wrapper)(self.expr.node.clone().into(), op),
        }
    }
}

#[pyclass(module = "dictexprs", name = "Expr")]
#[derive(Clone)]
pub struct Expr {
    pub(crate) node: nodes::Node,
}

#[pymethods]
impl Expr {
    #[new]
    pub fn new() -> Self {
        Self {
            node: nodes::Node::This,
        }
    }

    #[getter]
    pub fn list(&self) -> ExprListNameSpace {
        ExprListNameSpace {
            builder: NameSpaceBuilder::new(self.clone(), nodes::Node::List),
        }
    }

    #[getter]
    pub fn str(&self) -> ExprStrNameSpace {
        ExprStrNameSpace {
            builder: NameSpaceBuilder::new(self.clone(), nodes::Node::Str),
        }
    }

    #[getter]
    #[pyo3(name = "struct")]
    pub fn struct_(&self) -> ExprStructNameSpace {
        ExprStructNameSpace {
            builder: NameSpaceBuilder::new(self.clone(), nodes::Node::Struct),
        }
    }
    pub fn eq(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: nodes::Node::Compare(
                self.node.clone().into(),
                nodes::ComparisonOp::Eq(into_lit(py, other)?.into()),
            ),
        })
    }

    pub fn ne(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: nodes::Node::Compare(
                self.node.clone().into(),
                nodes::ComparisonOp::Ne(into_lit(py, other)?.into()),
            ),
        })
    }

    pub fn lt(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: nodes::Node::Compare(
                self.node.clone().into(),
                nodes::ComparisonOp::Lt(into_lit(py, other)?.into()),
            ),
        })
    }

    pub fn le(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: nodes::Node::Compare(
                self.node.clone().into(),
                nodes::ComparisonOp::Le(into_lit(py, other)?.into()),
            ),
        })
    }

    pub fn gt(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: nodes::Node::Compare(
                self.node.clone().into(),
                nodes::ComparisonOp::Gt(into_lit(py, other)?.into()),
            ),
        })
    }

    pub fn ge(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            node: nodes::Node::Compare(
                self.node.clone().into(),
                nodes::ComparisonOp::Ge(into_lit(py, other)?.into()),
            ),
        })
    }
    pub fn and_(&self, other: &Expr) -> Self {
        Self {
            node: nodes::Node::And(self.node.clone().into(), other.node.clone().into()),
        }
    }

    pub fn or_(&self, other: &Expr) -> Self {
        Self {
            node: nodes::Node::Or(self.node.clone().into(), other.node.clone().into()),
        }
    }

    pub fn not_(&self) -> Self {
        Self {
            node: nodes::Node::Not(self.node.clone().into()),
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            node: nodes::Node::Scalar(self.node.clone().into(), nodes::ScalarOp::Abs),
        }
    }

    pub fn ceil(&self) -> Self {
        Self {
            node: nodes::Node::Scalar(self.node.clone().into(), nodes::ScalarOp::Ceil),
        }
    }

    pub fn floor(&self) -> Self {
        Self {
            node: nodes::Node::Scalar(self.node.clone().into(), nodes::ScalarOp::Floor),
        }
    }
}
#[pyclass(module = "dictexprs", name = "ExprStructNameSpace")]
pub struct ExprStructNameSpace {
    builder: OpWrapper<nodes::StructOp>,
}

#[pymethods]
impl ExprStructNameSpace {
    pub fn field(&self, name: &str) -> Expr {
        self.builder.wrap(nodes::StructOp::Field(name.to_string()))
    }

    pub fn keys(&self) -> Expr {
        self.builder.wrap(nodes::StructOp::Keys)
    }

    pub fn values(&self) -> Expr {
        self.builder.wrap(nodes::StructOp::Values)
    }
}

#[pyclass(module = "dictexprs", name = "ExprStrNameSpace")]
pub struct ExprStrNameSpace {
    builder: OpWrapper<nodes::StrOp>,
}

#[pymethods]
impl ExprStrNameSpace {
    pub fn contains(&self, other: &str) -> Expr {
        self.builder.wrap(nodes::StrOp::Contains(other.to_string()))
    }

    pub fn reverse(&self) -> Expr {
        self.builder.wrap(nodes::StrOp::Reverse)
    }

    pub fn starts_with(&self, other: &str) -> Expr {
        self.builder
            .wrap(nodes::StrOp::StartsWith(other.to_string()))
    }

    pub fn ends_with(&self, other: &str) -> Expr {
        self.builder.wrap(nodes::StrOp::EndsWith(other.to_string()))
    }

    #[pyo3(signature = (start=None, end=None, step=None))]
    pub fn slice(&self, start: Option<isize>, end: Option<isize>, step: Option<isize>) -> Expr {
        self.builder.wrap(nodes::StrOp::Slice { start, end, step })
    }
    pub fn length(&self) -> Expr {
        self.builder.wrap(nodes::StrOp::Length)
    }
}

#[pyclass(module = "dictexprs", name = "ExprListNameSpace")]
pub struct ExprListNameSpace {
    builder: OpWrapper<nodes::ListOp>,
}

#[pymethods]
impl ExprListNameSpace {
    pub fn get(&self, i: isize) -> Expr {
        self.builder.wrap(nodes::ListOp::Index(i))
    }

    #[pyo3(signature = (start=None, end=None, step=None))]
    pub fn slice(&self, start: Option<isize>, end: Option<isize>, step: Option<isize>) -> Expr {
        self.builder.wrap(nodes::ListOp::Slice { start, end, step })
    }

    pub fn flatten(&self) -> Expr {
        self.builder.wrap(nodes::ListOp::Flatten)
    }

    pub fn reverse(&self) -> Expr {
        self.builder.wrap(nodes::ListOp::Reverse)
    }
    pub fn sort(&self) -> Expr {
        self.builder.wrap(nodes::ListOp::Sort)
    }

    pub fn sum(&self) -> Expr {
        self.builder.wrap(nodes::ListOp::Sum)
    }

    pub fn min(&self) -> Expr {
        self.builder.wrap(nodes::ListOp::Min)
    }

    pub fn max(&self) -> Expr {
        self.builder.wrap(nodes::ListOp::Max)
    }

    pub fn avg(&self) -> Expr {
        self.builder.wrap(nodes::ListOp::Avg)
    }

    pub fn length(&self) -> Expr {
        self.builder.wrap(nodes::ListOp::Length)
    }

    pub fn join(&self, glue: &str) -> Expr {
        self.builder.wrap(nodes::ListOp::Join(glue.to_string()))
    }

    pub fn map(&self, expr: &Expr) -> Expr {
        self.builder
            .wrap(nodes::ListOp::Map(expr.node.clone().into()))
    }

    pub fn contains(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Expr> {
        Ok(self
            .builder
            .wrap(nodes::ListOp::Contains(into_lit(py, other)?.into())))
    }
    pub fn filter(&self, cond: &Expr) -> Expr {
        self.builder
            .wrap(nodes::ListOp::Filter(cond.node.clone().into()))
    }
    pub fn sort_by(&self, key: &Expr) -> Expr {
        self.builder
            .wrap(nodes::ListOp::SortBy(key.node.clone().into()))
    }

    pub fn min_by(&self, key: &Expr) -> Expr {
        self.builder
            .wrap(nodes::ListOp::MinBy(key.node.clone().into()))
    }

    pub fn max_by(&self, key: &Expr) -> Expr {
        self.builder
            .wrap(nodes::ListOp::MaxBy(key.node.clone().into()))
    }
}
pub mod entryfuncs {
    use super::*;

    #[pyfunction]
    pub fn element() -> Expr {
        Expr::new()
    }
    #[pyfunction]
    pub fn field(name: &str) -> Expr {
        Expr::new().struct_().field(name)
    }
    #[pyfunction]
    #[pyo3(name = "struct")]
    pub fn struct_() -> ExprStructNameSpace {
        Expr::new().struct_()
    }
    #[pyfunction]
    #[pyo3(name = "list")]
    pub fn list() -> ExprListNameSpace {
        Expr::new().list()
    }

    #[pyfunction(signature = (*args))]
    pub fn merge(args: Vec<Expr>) -> Expr {
        Expr {
            node: nodes::Node::Merge(args.into_iter().map(|q| q.node).collect()),
        }
    }

    #[pyfunction(signature = (*args))]
    pub fn coalesce(args: Vec<Expr>) -> Expr {
        Expr {
            node: nodes::Node::Coalesce(args.into_iter().map(|q| q.node).collect()),
        }
    }
    #[pyfunction]
    pub fn lit(value: &Bound<'_, PyAny>) -> Expr {
        Python::with_gil(|py| Expr {
            node: nodes::Node::Literal(nodes::PyObjectWrapper(value.to_object(py))),
        })
    }
}
#[pyclass(module = "dictexprs", name = "DataJson")]
pub struct DataJson {
    data: PyObject,
}

#[pymethods]
impl DataJson {
    pub fn query(&self, py: Python<'_>, expr: &Expr) -> PyResult<PyObject> {
        match_any(py, &expr.node, self.data.bind(py)).map(|result| result.unbind())
    }
}
