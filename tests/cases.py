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
            self.qd_query.search(data),
            jmespath.search(self.qd_query.to_jmespath(), data),
        ).assert_equal(self.qd_query.to_jmespath())

    def to_result(self, size: int, runs: int, data: DataBase) -> BenchmarkResult:
        compiled = jmespath.compile(self.qd_query.to_jmespath())

        timings_qd: list[float] = []
        for _ in range(runs):
            start = time.perf_counter()
            self.qd_query.search(data)
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
        .add(qd.field("users").project("name"))
        .add(qd.field("users").vproject("address"))
        .add(qd.field("users").pipe(qd.identity().length()))
        .add(
            qd.field("users")
            .filter(qd.identity().age.ge(30).and_(qd.identity().active.eq(True)))
            .then(qd.identity().name)
        )
        .add(
            qd.select_dict(
                names=qd.field("users").project("name"),
                count=qd.field("users").length(),
            )
        )
        .add(qd.field("users").length())
        .add(qd.field("users").project("name").map(qd.identity().length()))
        .add(qd.field("users").min_by("age"))
        .add(qd.field("users").sort_by("age").project("name"))
        .add(
            qd.field("users")
            .filter(
                qd.identity()
                .age.gt(40)
                .and_(qd.identity().active.eq(True))
                .and_(qd.identity().category.contains("VIP"))
            )
            .then("name")
            .sort()
        )
        .add(qd.field("users").project("category").flatten())
        .add(qd.field("users").max_by("age"))
        .add(qd.field("users").project("category").flatten().sort())
        .add(qd.field("users").project("age").map(qd.identity().abs()))
        .add(qd.field("users").project("age").avg())
        .add(qd.field("users").project("age").map(qd.identity().ceil()))
        .add(qd.field("users").project("age").map(qd.identity().floor()))
        .add(qd.field("users").project("age").max())
        .add(qd.field("users").project("age").min())
        .add(qd.field("users").project("age").reverse())
        .add(qd.field("users").project("age").sum())
        .add(qd.field("users").project(qd.identity().address.city))
        .add(qd.field("users").project("name").length())
        .add(
            qd.select_list(
                qd.field("users").slice(0, 10), qd.field("users").slice(-10, None)
            )
        )
        .add(qd.field("users").project("age").eq(30))
        .add(qd.field("users").project("active").eq(True))
        .add(
            qd.field("users")
            .project("age")
            .gt(1)
            .and_(qd.field("users").project("age").eq(5).not_())
            .or_(0)
        )
        .add(qd.field("users").project(qd.identity().keys()))
        .add(qd.field("users").project(qd.identity().to_array()))
        .add(qd.field("users").project(qd.identity().to_string()))
        .add(qd.field("users").project(qd.identity().values()))
        .add(qd.field("users").project(qd.identity().dtype()))
        .add(qd.field("users").project(qd.identity().category.contains("VIP")))
        .add(qd.field("users").project(qd.identity().category.join(", ")))
        .add(
            qd.field("users").project(
                qd.merge(qd.identity(), qd.lit({"extra_field": 1}))
            )
        )
        .add(qd.field("users").project(qd.identity().age.to_string().to_number()))
        .add(qd.field("users").project(qd.identity().name.ends_with("s")))
        .add(qd.field("users").project(qd.identity().name.starts_with("A")))
        .add(
            qd.field("users").project(
                qd.not_null(qd.identity().field("MISSING"), qd.identity().name)
            )
        )
        .get()
    )


CASES: list[Case] = build_cases()
