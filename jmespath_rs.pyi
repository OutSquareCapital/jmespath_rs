from __future__ import annotations
from typing import Any, Self

type IntoExpr = Expr | str | int | float | bool | None

def field(name: str) -> Expr:
    """
    Entry point for a query starting with a field.

    Equivalent to JMESPath: `name`

    Args:
        name: The name of the field to access.

    Example:
    ```python
    >>> import jmespath_rs as jp
    >>> data = {"foo": "bar"}
    >>> jp.field("foo").search(data)
    'bar'

    ```
    """
    ...

def select_list(*exprs: Expr) -> Expr:
    """
    Creates a list multiselect.

    Equivalent to JMESPath: `[expr1, expr2, ...]`

    Args:
        *exprs: A variable number of Expr arguments.

    Example:
    ```python
    >>> import jmespath_rs as jp
    >>> data = {"foo": 1, "bar": 2}
    >>> query = jp.select_list(jp.identity().field("foo"), jp.identity().field("bar"), jp.lit(3))
    >>> query.search(data)
    [1, 2, 3]

    ```
    """
    ...

def select_dict(**items: IntoExpr) -> Expr:
    """
    Creates a dict multiselect.

    Equivalent to JMESPath: `{key1: expr1, key2: expr2, ...}`

    Args:
        **items: Key-value pairs where keys are strings and values
            are either an `Expr` or a literal value.

    Example:
    ```python
    >>> import jmespath_rs as jp
    >>> data = {"foo": "bar", "baz": "qux"}
    >>> query = jp.select_dict(a=jp.identity().field("foo"), b="literal_string", c=jp.lit(10))
    >>> query.search(data)
    {'a': 'bar', 'b': 'literal_string', 'c': 10}

    ```
    """
    ...

def lit(value: Any) -> Expr:
    """
    Creates a JMESPath literal node.

    Equivalent to JMESPath: `` `value` ``

    Args:
        value: The literal value to wrap.

    Example:
    ```python
    >>> import jmespath_rs as jp
    >>> data = {"age": 20}
    >>> # Compare the 'age' field to the literal value 18
    >>> query = jp.identity().field("age").gt(jp.lit(18))
    >>> query.search(data)
    True

    ```
    """
    ...

def identity() -> Expr:
    """
    Creates an identity expression.

    Equivalent to JMESPath: `@`

    Example:
    ```python
    >>> import jmespath_rs as jp
    >>> data = [1, 2, 3]
    >>> # The identity expression returns the current data
    >>> jp.identity().search(data)
    [1, 2, 3]

    ```
    """
    ...

def merge(*exprs: Expr) -> Expr:
    """
    Merges multiple objects into one.

    Equivalent to JMESPath: `merge(expr1, expr2, ...)`

    Args:
        *exprs: A variable number of Expr arguments to merge.

    Example:
    ```python
    >>> import jmespath_rs as jp
    >>> data = {"a": 1, "b": {"c": 2}}
    >>> query = jp.merge(jp.identity().field("b"), jp.select_dict(d=jp.lit(3)))
    >>> query.search(data)
    {'c': 2, 'd': 3}

    ```
    """
    ...

def not_null(*exprs: Expr) -> Expr:
    """
    Returns the first argument that is not null.

    Equivalent to JMESPath: `not_null(expr1, expr2, ...)`

    Args:
        *exprs: A variable number of Expr arguments to check.

    Example:
    ```python
    >>> import jmespath_rs as jp
    >>> data = {"a": None, "b": "hello", "c": "world"}
    >>> query = jp.not_null(jp.field("a"), jp.field("b"), jp.field("c"))
    >>> query.search(data)
    'hello'

    ```
    """
    ...

class Expr:
    """
    A chainable JMESPath query builder.
    This class *builds* a query, it does not execute it.
    """

    def search(self, data: Any) -> Any:
        """
        Executes the JMESPath query against the provided data.

        Args:
            data: The input data to query.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = {"foo": [1, 2, 3]}
        >>> query = jp.identity().field("foo").project(jp.identity().gt(jp.lit(1)))
        >>> query.search(data)
        [False, True, True]

        ```
        """
        ...

    def to_jmespath(self) -> str:
        """
        Converts the Expr to its JMESPath string representation.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> query = jp.identity().field("foo").project("bar")
        >>> query.to_jmespath()
        'foo[*].bar'

        ```
        """
        ...

    def field(self, name: str) -> Self:
        """
        Accesses an object field.

        Equivalent to JMESPath: `.name`

        Args:
            name: The name of the field to access.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = {"foo": "bar"}
        >>> jp.identity().field("foo").search(data)
        'bar'
        ```
        """
        ...

    def __getattr__(self, name: str) -> Self:
        """
        Alias for .field(name).

        Allows attribute-style access.

        Equivalent to JMESPath: `.name`

        Args:
            name: The name of the field to access.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = {"foo": "bar"}
        >>> jp.identity().foo.search(data)
        'bar'
        ```
        """
        ...

    def index(self, i: int) -> Self:
        """
        Accesses a list index.

        Equivalent to JMESPath: `[i]`

        Args:
            i: The integer index to access.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = ["a", "b", "c"]
        >>> jp.identity().index(1).search(data)
        'b'
        >>> jp.identity().index(-1).search(data)
        'c'
        ```
        """
        ...

    def slice(
        self,
        start: int | None = None,
        end: int | None = None,
        step: int | None = None,
    ) -> Self:
        """
        Slices a list.

        Equivalent to JMESPath: `[start:end:step]`

        Args:
            start: Start index. Defaults to None.
            end: End index. Defaults to None.
            step: Step size. Defaults to None.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = [0, 1, 2, 3, 4, 5]
        >>> jp.identity().slice(1, 4).search(data)
        [1, 2, 3]
        >>> jp.identity().slice(step=2).search(data)
        [0, 2, 4]
        ```
        """
        ...

    def project(self, rhs: IntoExpr) -> Self:
        """
        Performs a list projection.

        Equivalent to JMESPath: `[*].rhs`

        Args:
            rhs: `Expr` to apply, string for field, or literal.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = [{"a": 1}, {"a": 2}]
        >>> jp.identity().project("a").search(data)
        [1, 2]
        ```
        """
        ...

    def vproject(self, rhs: IntoExpr) -> Self:
        """
        Performs an object projection.

        Equivalent to JMESPath: `*.rhs`

        Args:
            rhs: `Expr` to apply, string for field, or literal.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = {"a": {"id": 1}, "b": {"id": 2}}
        >>> jp.identity().vproject("id").search(data)
        [1, 2]
        ```
        """
        ...

    def flatten(self) -> Self:
        """
        Flattens a list of lists.

        Equivalent to JMESPath: `[][]`

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = [[1, 2], [3, 4]]
        >>> jp.identity().flatten().search(data)
        [1, 2, 3, 4]
        ```
        """
        ...

    def filter(self, cond: Expr) -> FilteredExpr:
        """
        Filters a list based on a condition.

        Equivalent to JMESPath: `[?cond]`

        Args:
            cond: The `Expr` condition to apply.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = [{"a": 1}, {"a": 2}, {"a": 3}]
        >>> query = jp.identity().filter(jp.identity().a.gt(jp.lit(1))).then("a")
        >>> query.search(data)
        [2, 3]
        ```
        """
        ...

    def eq(self, other: IntoExpr) -> Self:
        """
        Equality comparison.

        Equivalent to JMESPath: `== other`

        Args:
            other: Value to compare against.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = "foo"
        >>> jp.identity().eq("foo").search(data)
        True
        ```
        """
        ...

    def ne(self, other: IntoExpr) -> Self:
        """
        Inequality comparison.

        Equivalent to JMESPath: `!= other`

        Args:
            other: Value to compare against.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = "foo"
        >>> jp.identity().ne("bar").search(data)
        True
        ```
        """
        ...

    def gt(self, other: IntoExpr) -> Self:
        """
        Greater than comparison.

        Equivalent to JMESPath: `> other`

        Args:
            other: Value to compare against.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = 10
        >>> jp.identity().gt(5).search(data)
        True
        ```
        """
        ...

    def ge(self, other: IntoExpr) -> Self:
        """
        Greater than or equal comparison.

        Equivalent to JMESPath: `>= other`

        Args:
            other: Value to compare against.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = 10
        >>> jp.identity().ge(10).search(data)
        True
        ```
        """
        ...

    def lt(self, other: IntoExpr) -> Self:
        """
        Less than comparison.

        Equivalent to JMESPath: `< other`

        Args:
            other: Value to compare against.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = 10
        >>> jp.identity().lt(20).search(data)
        True
        ```
        """
        ...

    def le(self, other: IntoExpr) -> Self:
        """
        Less than or equal comparison.

        Equivalent to JMESPath: `<= other`

        Args:
            other: Value to compare against.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = 10
        >>> jp.identity().le(10).search(data)
        True
        ```
        """
        ...

    def and_(self, other: IntoExpr) -> Self:
        """
        Logical AND.

        Equivalent to JMESPath: `&& other`

        Args:
            other: Right-hand side of the AND expression.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = True
        >>> jp.identity().and_(False).search(data)
        False
        ```
        """
        ...

    def or_(self, other: IntoExpr) -> Self:
        """
        Logical OR.

        Equivalent to JMESPath: `|| other`

        Args:
            other: Right-hand side of the OR expression.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = True
        >>> jp.identity().or_(False).search(data)
        True
        ```
        """
        ...

    def not_(self) -> Self:
        """
        Logical NOT.

        Equivalent to JMESPath: `!@`

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = False
        >>> jp.identity().not_().search(data)
        True
        ```
        """
        ...

    def pipe(self, rhs: Expr) -> Self:
        """
        Pipes the current output to another expression.

        Equivalent to JMESPath: `... | rhs`

        Args:
            rhs: The `Expr` to pipe the result into.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = {"foo": [3, 1, 2]}
        >>> query = jp.identity().foo.pipe(jp.identity().sort())
        >>> query.search(data)
        [1, 2, 3]
        ```
        """
        ...

    def abs(self) -> Self:
        """
        Calculates the absolute value of a number.

        Equivalent to JMESPath: `abs(@)`

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = -10.5
        >>> jp.identity().abs().search(data)
        10.5
        ```
        """
        ...

    def avg(self) -> Self:
        """
        Calculates the average of a list of numbers.

        Equivalent to JMESPath: `avg(@)`

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = [1, 2, 3, 4]
        >>> jp.identity().avg().search(data)
        2.5
        ```
        """
        ...

    def ceil(self) -> Self:
        """
        Calculates the ceiling of a number.

        Equivalent to JMESPath: `ceil(@)`

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = 1.2
        >>> jp.identity().ceil().search(data)
        2.0
        ```
        """
        ...

    def floor(self) -> Self:
        """
        Calculates the floor of a number.

        Equivalent to JMESPath: `floor(@)`

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = 1.8
        >>> jp.identity().floor().search(data)
        1.0
        ```
        """
        ...

    def max(self) -> Self:
        """
        Finds the maximum value in a list of numbers or strings.

        Equivalent to JMESPath: `max(@)`

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = [1, 5, 2, 4]
        >>> jp.identity().max().search(data)
        5
        ```
        """
        ...

    def min(self) -> Self:
        """
        Finds the minimum value in a list of numbers or strings.

        Equivalent to JMESPath: `min(@)`

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = [1, 5, 2, 4]
        >>> jp.identity().min().search(data)
        1
        ```
        """
        ...

    def reverse(self) -> Self:
        """
        Reverses a list or string.

        Equivalent to JMESPath: `reverse(@)`

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> jp.identity().reverse().search([1, 2, 3])
        [3, 2, 1]
        >>> jp.identity().reverse().search("abc")
        'cba'
        ```
        """
        ...

    def sum(self) -> Self:
        """
        Calculates the sum of a list of numbers.

        Equivalent to JMESPath: `sum(@)`

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> jp.identity().sum().search([1, 2, 3])
        6.0
        ```
        """
        ...

    def dtype(self) -> Self:
        """
        Returns the JMESPath type name of the data.

        Equivalent to JMESPath: `type(@)`

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> jp.identity().dtype().search({"a": 1})
        'object'
        >>> jp.identity().dtype().search(123)
        'number'
        ```
        """
        ...

    def contains(self, other: IntoExpr) -> Self:
        """
        Checks if a list or string contains the given value.

        Equivalent to JMESPath: `contains(@, other)`

        Args:
            other: The value to search for.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> jp.identity().contains("ell").search("hello")
        True
        >>> jp.identity().contains(2).search([1, 2, 3])
        True
        ```
        """
        ...

    def ends_with(self, other: IntoExpr) -> Self:
        """
        Checks if a string ends with the given suffix.

        Equivalent to JMESPath: `ends_with(@, other)`

        Args:
            other: The suffix string to check for.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = "hello"
        >>> jp.identity().ends_with("llo").search(data)
        True
        ```
        """
        ...

    def starts_with(self, other: IntoExpr) -> Self:
        """
        Checks if a string starts with the given prefix.

        Equivalent to JMESPath: `starts_with(@, other)`

        Args:
            other: The prefix string to check for.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = "hello"
        >>> jp.identity().starts_with("he").search(data)
        True
        ```
        """
        ...

    def join(self, glue: IntoExpr) -> Self:
        """
        Joins a list of strings with a glue string.

        Equivalent to JMESPath: `join(glue, @)`

        Args:
            glue: The string to use as a separator.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = ["a", "b", "c"]
        >>> jp.identity().join("-").search(data)
        'a-b-c'
        ```
        """
        ...

    def length(self) -> Self:
        """
        Returns the length of a list, string, or object.

        Equivalent to JMESPath: `length(@)`

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> jp.identity().length().search([1, 2, 3])
        3
        >>> jp.identity().length().search("hello")
        5
        ```
        """
        ...

    def sort(self) -> Self:
        """
        Sorts a list.

        Equivalent to JMESPath: `sort(@)`

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = [3, 1, 2]
        >>> jp.identity().sort().search(data)
        [1, 2, 3]
        ```
        """
        ...

    def keys(self) -> Self:
        """
        Returns the keys of an object.

        Equivalent to JMESPath: `keys(@)`

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = {"a": 1, "b": 2}
        >>> sorted(jp.identity().keys().search(data))
        ['a', 'b']
        ```
        """
        ...

    def values(self) -> Self:
        """
        Returns the values of an object.

        Equivalent to JMESPath: `values(@)`

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = {"a": 1, "b": 2}
        >>> sorted(jp.identity().values().search(data))
        [1, 2]
        ```
        """
        ...

    def to_string(self) -> Self:
        """
        Converts a value to its JSON string representation.

        Equivalent to JMESPath: `to_string(@)`

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = {"a": 1}
        >>> jp.identity().to_string().search(data)
        '{"a":1}'
        ```
        """
        ...

    def to_number(self) -> Self:
        """
        Converts a value to a number.

        Equivalent to JMESPath: `to_number(@)`

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = "1.23"
        >>> jp.identity().to_number().search(data)
        1.23
        ```
        """
        ...

    def to_array(self) -> Self:
        """
        Converts a value to an array. If already an array, returns it.

        Equivalent to JMESPath: `to_array(@)`

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> jp.identity().to_array().search("foo")
        ['foo']
        >>> jp.identity().to_array().search([1, 2])
        [1, 2]
        ```
        """
        ...

    def map(self, expr: IntoExpr) -> Self:
        """
        Applies an expression to each element in a list.

        Equivalent to JMESPath: `map(&expr, @)`

        Args:
            expr: The `Expr` to apply to each element.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = [{"a": 1}, {"a": 2}, {"a": 3}]
        >>> jp.identity().map("a").search(data)
        [1, 2, 3]
        ```
        """
        ...

    def sort_by(self, key: IntoExpr) -> Self:
        """
        Sorts a list using a key expression.

        Equivalent to JMESPath: `sort_by(@, &key)`

        Args:
            key: `Expr` or field name to use as the sorting key.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = [{"a": 3}, {"a": 1}, {"a": 2}]
        >>> jp.identity().sort_by("a").search(data)
        [{'a': 1}, {'a': 2}, {'a': 3}]
        ```
        """
        ...

    def min_by(self, key: IntoExpr) -> Self:
        """
        Finds the minimum element using a key expression.

        Equivalent to JMESPath: `min_by(@, &key)`

        Args:
            key: `Expr` or field name to use as the key.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = [{"a": 3}, {"a": 1}, {"a": 2}]
        >>> jp.identity().min_by("a").search(data)
        {'a': 1}
        ```
        """
        ...

    def max_by(self, key: IntoExpr) -> Self:
        """
        Finds the maximum element using a key expression.

        Equivalent to JMESPath: `max_by(@, &key)`

        Args:
            key: `Expr` or field name to use as the key.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = [{"a": 3}, {"a": 1}, {"a": 2}]
        >>> jp.identity().max_by("a").search(data)
        {'a': 3}
        ```
        """
        ...

class FilteredExpr:
    """
    A JMESPath expression representing a filtered projection.

    This is an intermediate object returned by `Expr.filter()`.
    Call `.then()` on it to complete the projection.
    """

    def then(self, then: IntoExpr) -> Expr:
        """
        Completes the filtered expression by specifying the projection.

        Equivalent to JMESPath: `[?cond].then`

        Args:
            then: The `Expr` to apply to items that pass the filter.

        Example:
        ```python
        >>> import jmespath_rs as jp
        >>> data = [
        ...     {"name": "Alice", "age": 30},
        ...     {"name": "Bob", "age": 20}
        ... ]
        >>> # Find names of people older than 25
        >>> cond = jp.identity().age.gt(jp.lit(25))
        >>> query = jp.identity().filter(cond).then("name")
        >>> query.search(data)
        ['Alice']
        ```
        """
        ...
