from dataclasses import dataclass
from collections.abc import Callable
from typing import Any
import jmespath
import jmespath_rs as qd


@dataclass(slots=True, frozen=True)
class Case:
    name: str
    build: Callable[[], qd.Expr]
    jmes_query: str
    data: dict[str, Any] | None = None

    def check(self, data: dict[str, Any]) -> None:
        """Checks the query against the provided data."""
        test_data = self.data if self.data is not None else data
        q = self.build()
        expr = self.jmes_query

        got = qd.DataJson(test_data).collect(q)
        want = jmespath.search(expr, test_data)

        assert got == want, (
            f"{self.name}: \n  Query: {expr!r}\n  Got:   {got!r}\n  Want:  {want!r}"
        )
        print(f"✔ {self.name}")


CASES: list[Case] = [
    # region: Core Expressions
    Case(
        "field.subfield.index.field",
        lambda: qd.field("metadata").a.b.index(0).c,
        "metadata.a.b[0].c",
    ),
    Case(
        "simple-field-access",
        lambda: qd.field("users").index(0).name,
        "users[0].name",
    ),
    Case(
        "slice-array",
        lambda: qd.field("users").slice(1, 10, 2),
        "users[1:10:2]",
    ),
    Case(
        "list-projection",
        lambda: qd.field("users").project("name"),
        "users[].name",
    ),
    Case(
        "object-projection",
        lambda: qd.field("metadata").d.vproject("e"),
        "metadata.d.*.e",
    ),
    Case(
        "flatten-nested-list",
        lambda: qd.field("metadata").i.flatten(),
        "metadata.i[][]",
    ),
    Case(
        "pipe-to-length",
        lambda: qd.field("users").pipe(qd.identity().length()),
        "length(users)",
    ),
    # endregion
    # region: Filters
    Case(
        "filter-age-and-active",
        lambda: qd.field("users")
        .filter(qd.field("age").ge(30).and_(qd.field("active").eq(True)))
        .then(qd.field("name")),
        "users[?age >= `30` && active == `true`].name",
    ),
    # endregion
    # region: Multiselect
    Case(
        "multiselect-dict",
        lambda: qd.select_dict(
            names=qd.field("users").project("name"), count=qd.field("users").length()
        ),
        "{names: users[].name, count: length(users)}",
    ),
    Case(
        "multiselect-list",
        lambda: qd.select_list(
            qd.field("users").index(0), qd.field("products").index(0)
        ),
        "[users[0], products[0]]",
    ),
    # endregion
    # region: Comparisons and Logic
    Case(
        "numeric-comparison-eq",
        lambda: qd.field("metadata").j.index(1).eq(1),
        "metadata.j[1] == `1`",
    ),
    Case(
        "boolean-comparison-eq",
        lambda: qd.field("metadata").k.index(0).eq(True),
        "metadata.k[0] == `true`",
    ),
    Case(
        "and-or-not-logic",
        lambda: qd.field("metadata")
        .l.m.n.o.gt(1)
        .and_(qd.field("metadata").l.m.n.o.eq(5).not_())
        .or_(0),
        "(metadata.l.m.n.o > `1` && !(metadata.l.m.n.o == `5`)) || `0`",
    ),
    # endregion
    # region: Available Built-in Functions
    Case("keys-of-object", lambda: qd.field("metadata").d.keys(), "keys(metadata.d)"),
    Case("length-of-array", lambda: qd.field("users").length(), "length(users)"),
    Case(
        "map-string-lengths",
        lambda: qd.field("users").project("name").map_with(qd.identity().length()),
        "map(&length(@), users[].name)",
    ),
    Case(
        "max_by-price",
        lambda: qd.field("products").max_by(qd.field("price")),
        "max_by(products, &price)",
    ),
    Case(
        "min_by-age",
        lambda: qd.field("users").min_by(qd.field("age")),
        "min_by(users, &age)",
    ),
    Case("sort", lambda: qd.field("metadata").h.sort(), "sort(metadata.h)"),
    Case(
        "sort-by-age",
        lambda: qd.field("users").sort_by(qd.field("age")).project("name"),
        "sort_by(users, &age)[].name",
    ),
    Case("to_array", lambda: qd.field("metadata").a.to_array(), "to_array(metadata.a)"),
    Case(
        "to_string", lambda: qd.field("metadata").d.to_string(), "to_string(metadata.d)"
    ),
    Case("to_number", lambda: qd.lit("42").to_number(), "to_number(`42`)", data={}),
    Case(
        "values-of-object",
        lambda: qd.field("metadata").d.values(),
        "values(metadata.d)",
    ),
    # endregion
]
