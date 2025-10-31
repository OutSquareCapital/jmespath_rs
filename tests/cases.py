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

    def check(self, data: DataBase) -> None:
        """Checks the query against the provided data."""
        CheckResult(
            qd.DataJson(data).collect(self.qd_query),
            jmespath.search(self.qd_query.to_jmespath(), data),
        ).assert_equal(self.qd_query.to_jmespath())

    def to_result(self, size: int, runs: int, data: DataBase) -> BenchmarkResult:
        df = qd.DataJson(data)
        compiled = jmespath.compile(self.qd_query.to_jmespath())

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
            query=self.qd_query.to_jmespath(),
            qrydict=statistics.median(timings_qd),
            jmespth=statistics.median(timings_jp),
        )


@dataclass(slots=True)
class CasesBuilder:
    cases: list[Case] = field(default_factory=list)

    def add(self, qd_query: qd.Expr) -> Self:
        self.cases.append(Case(qd_query))
        return self

    def get(self) -> list[Case]:
        return self.cases


def build_cases() -> list[Case]:
    return (
        CasesBuilder()
        .add(qd.field("users").index(0).address.city)
        .add(qd.field("users").index(0).name)
        .add(qd.field("users").slice(1, 10, 2))
        .add(qd.field("users").project("name"))
        .add(qd.field("users").vproject("address"))
        .add(qd.field("products").project("tags").flatten())
        .add(qd.field("users").pipe(qd.identity().length()))
        .add(
            qd.field("users")
            .filter(qd.field("age").ge(30).and_(qd.field("active").eq(True)))
            .then(qd.field("name"))
        )
        .add(
            qd.select_dict(
                names=qd.field("users").project("name"),
                count=qd.field("users").length(),
            )
        )
        .add(qd.select_list(qd.field("users").index(0), qd.field("products").index(0)))
        .add(qd.field("users").index(0).age.eq(30))
        .add(qd.field("users").index(0).active.eq(True))
        .add(
            qd.field("users")
            .index(0)
            .age.gt(1)
            .and_(qd.field("users").index(0).age.eq(5).not_())
            .or_(0)
        )
        .add(qd.field("users").index(0).keys())
        .add(qd.field("users").length())
        .add(qd.field("users").project("name").map(qd.identity().length()))
        .add(qd.field("products").max_by("price"))
        .add(qd.field("users").min_by("age"))
        .add(qd.field("products").project("tags").flatten().sort())
        .add(qd.field("users").sort_by("age").project("name"))
        .add(qd.field("users").index(0).to_array())
        .add(qd.field("users").index(0).to_string())
        .add(qd.lit("42").to_number())
        .add(qd.field("users").index(0).values())
        .add(qd.field("products").project("price").map(qd.identity().abs()))
        .add(qd.field("products").project("price").avg())
        .add(qd.field("products").project("price").map(qd.identity().ceil()))
        .add(qd.field("products").project("price").map(qd.identity().floor()))
        .add(qd.field("products").project("price").max())
        .add(qd.field("products").project("price").min())
        .add(qd.field("products").project("price").reverse())
        .add(qd.field("products").project("price").sum())
        .add(qd.field("users").index(0).type_())
        .add(qd.field("products").index(0).tags.contains("electronics"))
        .add(qd.lit("hello").ends_with("lo"))
        .add(qd.lit("hello").starts_with("he"))
        .add(qd.field("products").index(0).tags.join(", "))
        .add(qd.merge(qd.field("users").index(0), qd.field("products").index(0)))
        .add(qd.not_null(qd.lit(None), qd.lit("a"), qd.lit("b")))
        .get()
    )


CASES: list[Case] = build_cases()
