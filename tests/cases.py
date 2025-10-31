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

    def check(self, data: dict[str, Any]) -> None:
        """Checks the query against the provided data."""

        got = qd.DataJson(data).collect(self.build())
        want = jmespath.search(self.jmes_query, data)

        assert got == want, (
            f"{self.name}: \n  Query: {self.jmes_query!r}\n  Got:   {got!r}\n  Want:  {want!r}"
        )
        print(f"✔ {self.name}")


CASES: list[Case] = [
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
    Case("to_number", lambda: qd.lit("42").to_number(), "to_number(`42`)"),
    Case(
        "values-of-object",
        lambda: qd.field("metadata").d.values(),
        "values(metadata.d)",
    ),
    Case(
        "abs-of-number",
        lambda: qd.field("metadata").h.map_with(qd.identity().abs()),
        "map(&abs(@), metadata.h)",
    ),
    Case(
        "avg-of-numbers",
        lambda: qd.field("metadata").h.avg(),
        "avg(metadata.h)",
    ),
    Case(
        "ceil-of-numbers",
        lambda: qd.field("metadata").h.map_with(qd.identity().ceil()),
        "map(&ceil(@), metadata.h)",
    ),
    Case(
        "floor-of-numbers",
        lambda: qd.field("metadata").h.map_with(qd.identity().floor()),
        "map(&floor(@), metadata.h)",
    ),
    Case(
        "max-of-numbers",
        lambda: qd.field("metadata").h.max(),
        "max(metadata.h)",
    ),
    Case(
        "min-of-numbers",
        lambda: qd.field("metadata").h.min(),
        "min(metadata.h)",
    ),
    Case(
        "reverse-array",
        lambda: qd.field("metadata").h.reverse(),
        "reverse(metadata.h)",
    ),
    Case(
        "sum-of-numbers",
        lambda: qd.field("metadata").h.sum(),
        "sum(metadata.h)",
    ),
    Case(
        "type-of-value",
        lambda: qd.field("metadata").a.type_(),
        "type(metadata.a)",
    ),
    Case(
        "contains-in-array",
        lambda: qd.field("metadata").h.contains(3),
        "contains(metadata.h, `3`)",
    ),
    Case(
        "ends-with-string",
        lambda: qd.lit("hello").ends_with("lo"),
        'ends_with(`"hello"`, `"lo"`)',
    ),
    Case(
        "starts-with-string",
        lambda: qd.lit("hello").starts_with("he"),
        # CORRECTION: Remplacer ' par \"
        'starts_with(`"hello"`, `"he"`)',
    ),
    Case(
        "join-strings",
        lambda: qd.field("metadata").m.join(", "),
        "join(`, `, metadata.m)",
    ),
    Case(
        "merge-objects",
        lambda: qd.merge(qd.field("metadata").d, qd.field("metadata").l),
        "merge(metadata.d, metadata.l)",
    ),
    Case(
        "not-null-values",
        lambda: qd.not_null(qd.lit(None), qd.lit("a"), qd.lit("b")),
        # CORRECTION: Remplacer ' par \"
        'not_null(null, `"a"`, `"b"`)',
    ),
]
