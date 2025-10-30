from __future__ import annotations
from typing import Any, Self

type IntoExpr = Expr | str | int | float | bool | None

def field(name: str) -> Expr:
    """
    Entry point for a query starting with a field.
    """
    ...

def select_list(*exprs: Expr) -> Expr:
    """
    Creates a list multiselect (e.g., [users, tags]).
    """
    ...

def select_dict(**items: IntoExpr) -> Expr:
    """
    Creates a dict multiselect (e.g., {u: users, t: tags}).
    Values can be Expr or literal values.
    """
    ...

def lit(value: Any) -> Expr:
    """
    Creates a JMESPath literal node (e.g., `18` becomes '`18`').
    """
    ...

def identity() -> Expr:
    """
    Creates an identity expression (i.e., '@').
    """
    ...

class DataJson:
    """
    A data context that holds pre-parsed JSON data for querying.
    """

    def __init__(self, data: Any) -> None:
        """
        Initializes the context with Python data.
        """
        ...
    def query(self, query: Expr) -> Self:
        """
        Executes a Expr expression against the internal data.
        Returns the result as a Python object.
        """
        ...

    def collect(self, query: Expr) -> Any:
        """
        Executes a Expr expression against the internal data.
        Returns the result as a Python object.
        """
        ...

    def __repr__(self) -> str: ...

class Expr:
    """
    A chainable JMESPath query builder.
    This class *builds* a query, it does not execute it.
    """

    def __init__(self) -> None:
        """
        Creates a new query pointing to the current node ('@').
        """
        ...

    def field(self, name: str) -> Self:
        """
        Accesses an object field (e.g., .name).
        """
        ...

    def __getattr__(self, name: str) -> Self:
        """
        Alias for .field(name). Allows attribute-style access.
        """
        ...

    def index(self, i: int) -> Self:
        """
        Accesses a list index (e.g., [0]).
        """
        ...

    def slice(
        self,
        start: int | None = None,
        end: int | None = None,
        step: int | None = None,
    ) -> Self:
        """
        Slices a list (e.g., [0:2]).
        """
        ...

    def project(self, rhs: IntoExpr) -> Self:
        """
        Performs a list projection (e.g., [*].name).
        Accepts Expr, string (for field), or literal values.
        """
        ...

    def vproject(self, rhs: IntoExpr) -> Self:
        """
        Performs an object projection (e.g., *.name).
        Accepts Expr, string (for field), or literal values.
        """
        ...

    def flatten(self) -> Self:
        """
        Flattens a list of lists (e.g., [][]).
        """
        ...

    def filter(self, cond: Expr) -> FilteredExpr:
        """
        Filters a list (e.g., [?age > `18`]).
        """
        ...

    def eq(self, other: IntoExpr) -> Self:
        """
        Equality comparison.
        """
        ...

    def ne(self, other: IntoExpr) -> Self:
        """
        Inequality comparison.
        """
        ...

    def gt(self, other: IntoExpr) -> Self:
        """
        Greater than comparison.
        """
        ...

    def ge(self, other: IntoExpr) -> Self:
        """
        Greater than or equal comparison.
        """
        ...

    def lt(self, other: IntoExpr) -> Self:
        """
        Less than comparison.
        """
        ...

    def le(self, other: IntoExpr) -> Self:
        """
        Less than or equal comparison.
        """
        ...

    def and_(self, other: IntoExpr) -> Self:
        """
        Logical AND.
        """
        ...

    def or_(self, other: IntoExpr) -> Self:
        """
        Logical OR.
        """
        ...

    def not_(self) -> Self:
        """
        Logical NOT.
        """
        ...

    def pipe(self, rhs: Expr) -> Self:
        """
        Pipes the current output to another expression (e.g., ... | ...).
        """
        ...

    def length(self) -> Self: ...
    def sort(self) -> Self: ...
    def keys(self) -> Self: ...
    def values(self) -> Self: ...
    def to_string(self) -> Self: ...
    def to_number(self) -> Self: ...
    def to_array(self) -> Self: ...
    def map_with(self, expr: Expr) -> Self:
        """
        Applies an expression to each element.
        (JMESPath syntax: map(&expr, @))
        """
        ...

    def sort_by(self, key: Expr) -> Self:
        """
        Sorts a list using a key expression.
        (JMESPath syntax: sort_by(@, &key))
        """
        ...

    def min_by(self, key: Expr) -> Self:
        """
        Finds the minimum element using a key expression.
        (JMESPath syntax: min_by(@, &key))
        """
        ...

    def max_by(self, key: Expr) -> Self:
        """
        Finds the maximum element using a key expression.
        (JMESPath syntax: max_by(@, &key))
        """
        ...

class FilteredExpr:
    """
    A JMESPath expression representing a filtered projection.
    """
    def then(self, then: Expr) -> Expr:
        """
        Completes the filtered expression by specifying the 'then' part.
        """
        ...
