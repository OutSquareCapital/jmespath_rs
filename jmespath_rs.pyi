from __future__ import annotations
from typing import Any, Self

# --- Type alias for expression inputs ---
type IntoExpr = QueryBuilder | str | int | float | bool | None

# --- Factory functions ---

def field(name: str) -> QueryBuilder:
    """
    Entry point for a query starting with a field.
    """
    ...

def select_list(*exprs: IntoExpr) -> QueryBuilder:
    """
    Creates a list multiselect (e.g., [users, tags]).
    """
    ...

def select_dict(**items: IntoExpr) -> QueryBuilder:
    """
    Creates a dict multiselect (e.g., {u: users, t: tags}).
    """
    ...

def lit(value: Any) -> QueryBuilder:
    """
    Creates a JMESPath literal node (e.g., `18` becomes '`18`').
    """
    ...

# --- Data Context Class ---

class DataJson:
    """
    A data context that holds pre-parsed JSON data for querying.
    The data is converted to internal Rust structures once upon initialization.
    """

    def __init__(self, data: Any) -> None:
        """
        Initializes the context, converting the Python data.
        This is the main "cost" (serialization).
        """
        ...

    def query(self, query: QueryBuilder) -> Self:
        """
        Executes a QueryBuilder expression against the internal data.
        Updates the internal data with the result and returns self for chaining.
        """
        ...

    def collect(self) -> Any:
        """
        Converts the *current* internal data back to a Python object.
        Call this at the end of a query chain.
        """
        ...

    def __repr__(self) -> str: ...

# --- Query Builder Class ---

class QueryBuilder:
    """
    A chainable JMESPath query builder.
    This class *builds* a query string, it does not execute it.
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
        """
        ...

    def vproject(self, rhs: IntoExpr) -> Self:
        """
        Performs an object projection (e.g., *.name).
        """
        ...

    def flatten(self) -> Self:
        """
        Flattens a list of lists (e.g., [][]).
        """
        ...

    def filter(self, cond: IntoExpr, then: IntoExpr) -> Self:
        """
        Filters a list (e.g., [?age > `18`].name).
        """
        ...

    # --- Comparison Operators ---
    def eq(self, other: IntoExpr) -> Self: ...
    def ne(self, other: IntoExpr) -> Self: ...
    def gt(self, other: IntoExpr) -> Self: ...
    def ge(self, other: IntoExpr) -> Self: ...
    def lt(self, other: IntoExpr) -> Self: ...
    def le(self, other: IntoExpr) -> Self: ...

    # --- Logical Operators ---
    def and_(self, other: IntoExpr) -> Self: ...
    def or_(self, other: IntoExpr) -> Self: ...
    def not_(self) -> Self: ...
    def pipe(self, rhs: IntoExpr) -> Self:
        """
        Pipes the current output to another expression (e.g., ... | ...).
        """
        ...

    # --- Functions ---
    def length(self) -> Self: ...
    def sort(self) -> Self: ...
    def keys(self) -> Self: ...
    def values(self) -> Self: ...
    def to_string(self) -> Self: ...
    def to_number(self) -> Self: ...
    def to_array(self) -> Self: ...

    # --- Higher-Order Functions (JMESPath style) ---
    def map_with(self, expr: IntoExpr) -> Self:
        """
        Applies an expression to each element.
        (JMESPath syntax: map(&expr, @))
        """
        ...

    def sort_by(self, key: IntoExpr) -> Self:
        """
        Sorts a list using a key expression.
        (JMESPath syntax: sort_by(@, &key))
        """
        ...

    def min_by(self, key: IntoExpr) -> Self:
        """
        Finds the minimum element using a key expression.
        (JMESPath syntax: min_by(@, &key))
        """
        ...

    def max_by(self, key: IntoExpr) -> Self:
        """
        Finds the maximum element using a key expression.
        (JMESPath syntax: max_by(@, &key))
        """
        ...

    # --- Finalizer ---
    def to_jmespath(self) -> str:
        """
        Returns the constructed JMESPath query string.
        """
        ...
