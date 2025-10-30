from __future__ import annotations

from collections.abc import Callable
from dataclasses import dataclass
from typing import Any

import jmespath

import jmespath_rs as qd


@dataclass(slots=True, frozen=True)
class Case:
    name: str
    # MODIFIÉ: qd.Query -> qd.QueryBuilder
    build: Callable[[], qd.QueryBuilder]
    jmes_query: str
    data: dict[str, Any]

    def check(self) -> None:
        q = self.build()
        expr = self.jmes_query
        got = qd.DataJson(self.data).search(q)
        want = jmespath.search(expr, self.data)
        assert got == want, f"{self.name}: \n{got=!r} != \n{want=!r}  \nexpr={expr!r}"
        print(f"✔ {self.name}, \nexpr: \n  {expr}, \nresult: \n  {got!r}")


DATA_USER: dict[str, Any] = {
    "users": [
        {"name": "Ada", "age": 36},
        {"name": "Bob", "age": 17},
        {"name": "Cy", "age": 20},
    ]
}

DATA_MIXED: dict[str, Any] = {
    "foo": {"bar": [{"baz": 1}, {"baz": 2}]},
    "stats": {"a": 3, "b": 1, "c": 2},
    "arr": [3, 1, 2, 2],
    "nested": [[1, 2], [3], 4],
}

DATA_EDGE: dict[str, Any] = {
    "numbers": [0, 1, 2],
    "truth": [True, False],
    "obj": {"x": {"y": {"z": 5}}},
}

CASES: list[Case] = [
    Case(
        name="dot-field-index-dot",
        build=lambda: qd.field("foo").bar.index(0).baz,
        jmes_query="foo.bar[0].baz",
        data=DATA_MIXED,
    ),
    Case(
        name="simple-field",
        build=lambda: qd.field("users").index(0).name,
        jmes_query="users[0].name",
        data=DATA_USER,
    ),
    Case(
        name="slice",
        build=lambda: qd.field("arr").slice(1, 3),
        jmes_query="arr[1:3]",
        data=DATA_MIXED,
    ),
    Case(
        name="projection",
        build=lambda: qd.field("foo").bar.project("baz"),
        jmes_query="foo.bar[].baz",
        data=DATA_MIXED,
    ),
    Case(
        name="value-projection-sort",
        build=lambda: qd.field("stats").values().sort(),
        jmes_query="sort(values(stats))",
        data=DATA_MIXED,
    ),
    Case(
        name="filter-then-name",
        build=lambda: (
            qd.field("users").filter(
                qd.field("age").ge(18),
                then="name",
            )
        ),
        jmes_query="users[?age >= `18`].name",
        data=DATA_USER,
    ),
    Case(
        name="multi-select-dict",
        build=lambda: qd.select_dict(
            a=qd.field("stats").a,
            b=qd.field("stats").b,
        ),
        jmes_query="{a: stats.a, b: stats.b}",
        data=DATA_MIXED,
    ),
    Case(
        name="pipe-length",
        build=lambda: qd.field("foo").bar.length(),
        jmes_query="length(foo.bar)",
        data=DATA_MIXED,
    ),
    Case(
        name="numbers-vs-bool-eq",
        build=lambda: qd.field("numbers").index(0).eq(False),
        jmes_query="numbers[0] == `false`",
        data=DATA_EDGE,
    ),
    Case(
        name="and-or-not",
        build=lambda: (
            qd.field("obj").x.y.z.gt(1).and_(qd.field("obj").x.y.z.eq(5).not_()).or_(0)
        ),
        jmes_query="(obj.x.y.z > `1` && !(obj.x.y.z == `5`)) || `0`",
        data=DATA_EDGE,
    ),
    Case(
        name="map_with-length",
        build=lambda: (
            qd.field("users").project("name").map_with(qd.QueryBuilder().length())
        ),
        jmes_query="map(&length(@), users[].name)",
        data=DATA_USER,
    ),
    Case(
        name="sort_by-age",
        build=lambda: qd.field("users").sort_by(qd.field("age")),
        jmes_query="sort_by(users, &age)",
        data=DATA_USER,
    ),
    Case(
        name="min_by-age",
        # MODIFIÉ: On passe l'expression "age"
        build=lambda: qd.field("users").min_by(qd.field("age")),
        jmes_query="min_by(users, &age)",
        data=DATA_USER,
    ),
    Case(
        name="max_by-age",
        # MODIFIÉ: On passe l'expression "age"
        build=lambda: qd.field("users").max_by(qd.field("age")),
        jmes_query="max_by(users, &age)",
        data=DATA_USER,
    ),
    # conversions
    Case(
        name="to_array-wrap",
        build=lambda: qd.field("stats").a.to_array(),
        jmes_query="to_array(stats.a)",
        data=DATA_MIXED,
    ),
    Case(
        name="to_string-json",
        build=lambda: qd.field("stats").to_string(),
        jmes_query="to_string(stats)",
        data=DATA_MIXED,
    ),
    Case(
        name="to_number-valid",
        build=lambda: qd.lit("42").to_number(),
        jmes_query="to_number(`42`)",
        data=DATA_MIXED,
    ),
    Case(
        name="flatten-nested",
        build=lambda: qd.field("nested").flatten(),
        jmes_query="nested[][]",
        data=DATA_MIXED,
    ),
]


def main() -> None:
    print(f"Running {len(CASES)} cases…\n")
    for c in CASES:
        c.check()
    print("\nAll good.")


if __name__ == "__main__":
    main()
