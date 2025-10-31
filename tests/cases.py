from tests.data import DataBase, BenchmarkResult
import time
import jmespath
from typing import Any
import statistics
import math
import jmespath_rs as qd
from dataclasses import dataclass, field
from typing import Self


@dataclass(slots=True)
class CheckResult:
    got: Any
    want: Any

    def _check_equal(self) -> bool:
        if isinstance(self.got, float) and isinstance(self.want, float):
            return math.isclose(self.got, self.want)
        else:
            return self.got == self.want

    def _on_error(self, jmes_query: str) -> str:
        return f"Query: {jmes_query!r}\n  Got:   {self.got!r}\n  Want:  {self.want!r}"

    def assert_equal(self, jmes_query: str) -> None:
        assert self._check_equal(), self._on_error(jmes_query)
        print(f"✔ {jmes_query}")


@dataclass(slots=True, frozen=True)
class Case:
    qd_query: qd.Expr
    jmes_query: str

    def check(self, data: DataBase) -> None:
        """Checks the query against the provided data."""
        CheckResult(
            qd.DataJson(data).collect(self.qd_query),
            jmespath.search(self.jmes_query, data),
        ).assert_equal(self.jmes_query)

    def to_result(self, size: int, runs: int, data: DataBase) -> BenchmarkResult:
        df = qd.DataJson(data)
        compiled = jmespath.compile(self.jmes_query)

        timings_qd: list[float] = []
        for _ in range(runs):
            start = time.perf_counter()
            df.collect(self.qd_query)
            end = time.perf_counter()
            timings_qd.append(end - start)

        timings_jp: list[float] = []
        for _ in range(runs):
            start = time.perf_counter()
            compiled.search(data)
            end = time.perf_counter()
            timings_jp.append(end - start)

        return BenchmarkResult(
            size=size,
            query=self.jmes_query,
            qrydict=statistics.median(timings_qd),
            jmespth=statistics.median(timings_jp),
        )


@dataclass(slots=True)
class CasesBuilder:
    cases: list[Case] = field(default_factory=list)

    def add(self, qd_query: qd.Expr, jmes_query: str) -> Self:
        self.cases.append(Case(qd_query, jmes_query))
        return self

    def get(self) -> list[Case]:
        return self.cases


def build_cases() -> list[Case]:
    return (
        CasesBuilder()
        .add(
            qd.field("users").index(0).address.city,
            "users[0].address.city",
        )
        .add(
            qd.field("users").index(0).name,
            "users[0].name",
        )
        .add(
            qd.field("users").slice(1, 10, 2),
            "users[1:10:2]",
        )
        .add(
            qd.field("users").project("name"),
            "users[].name",
        )
        .add(
            qd.field("users").vproject("address"),
            "users[].*.address",
        )
        .add(
            qd.field("products").project("tags").flatten(),
            "products[].tags[]",
        )
        .add(
            qd.field("users").pipe(qd.identity().length()),
            "length(users)",
        )
        .add(
            qd.field("users")
            .filter(qd.field("age").ge(30).and_(qd.field("active").eq(True)))
            .then(qd.field("name")),
            "users[?age >= `30` && active == `true`].name",
        )
        .add(
            qd.select_dict(
                names=qd.field("users").project("name"),
                count=qd.field("users").length(),
            ),
            "{names: users[].name, count: length(users)}",
        )
        .add(
            qd.select_list(qd.field("users").index(0), qd.field("products").index(0)),
            "[users[0], products[0]]",
        )
        .add(
            qd.field("users").index(0).age.eq(30),
            "users[0].age == `30`",
        )
        .add(
            qd.field("users").index(0).active.eq(True),
            "users[0].active == `true`",
        )
        .add(
            qd.field("users")
            .index(0)
            .age.gt(1)
            .and_(qd.field("users").index(0).age.eq(5).not_())
            .or_(0),
            "(users[0].age > `1` && !(users[0].age == `5`)) || `0`",
        )
        .add(qd.field("users").index(0).keys(), "keys(users[0])")
        .add(qd.field("users").length(), "length(users)")
        .add(
            qd.field("users").project("name").map(qd.identity().length()),
            "map(&length(@), users[].name)",
        )
        .add(
            qd.field("products").max_by("price"),
            "max_by(products, &price)",
        )
        .add(
            qd.field("users").min_by("age"),
            "min_by(users, &age)",
        )
        .add(
            qd.field("products").project("tags").flatten().sort(),
            "sort(products[].tags[])",
        )
        .add(
            qd.field("users").sort_by("age").project("name"),
            "sort_by(users, &age)[].name",
        )
        .add(qd.field("users").index(0).to_array(), "to_array(users[0])")
        .add(qd.field("users").index(0).to_string(), "to_string(users[0])")
        .add(qd.lit("42").to_number(), "to_number(`42`)")
        .add(
            qd.field("users").index(0).values(),
            "values(users[0])",
        )
        .add(
            qd.field("products").project("price").map(qd.identity().abs()),
            "map(&abs(@), products[].price)",
        )
        .add(
            qd.field("products").project("price").avg(),
            "avg(products[].price)",
        )
        .add(
            qd.field("products").project("price").map(qd.identity().ceil()),
            "map(&ceil(@), products[].price)",
        )
        .add(
            qd.field("products").project("price").map(qd.identity().floor()),
            "map(&floor(@), products[].price)",
        )
        .add(
            qd.field("products").project("price").max(),
            "max(products[].price)",
        )
        .add(
            qd.field("products").project("price").min(),
            "min(products[].price)",
        )
        .add(
            qd.field("products").project("price").reverse(),
            "reverse(products[].price)",
        )
        .add(
            qd.field("products").project("price").sum(),
            "sum(products[].price)",
        )
        .add(
            qd.field("users").index(0).type_(),
            "type(users[0])",
        )
        .add(
            qd.field("products").index(0).tags.contains("electronics"),
            'contains(products[0].tags, `"electronics"`)',
        )
        .add(
            qd.lit("hello").ends_with("lo"),
            'ends_with(`"hello"`, `"lo"`)',
        )
        .add(
            qd.lit("hello").starts_with("he"),
            'starts_with(`"hello"`, `"he"`)',
        )
        .add(
            qd.field("products").index(0).tags.join(", "),
            "join(`, `, products[0].tags)",
        )
        .add(
            qd.merge(qd.field("users").index(0), qd.field("products").index(0)),
            "merge(users[0], products[0])",
        )
        .add(
            qd.not_null(qd.lit(None), qd.lit("a"), qd.lit("b")),
            'not_null(null, `"a"`, `"b"`)',
        )
        .get()
    )


CASES: list[Case] = build_cases()
