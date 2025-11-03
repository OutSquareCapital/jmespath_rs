from tests.data import DataBase, BenchmarkResult
import time
import jmespath
from typing import Any
import statistics
import math
import dictexprs as dx
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
    dx_query: dx.Expr
    jmes_query: str

    def check(self, data: DataBase) -> None:
        """Checks the query against the provided data."""
        CheckResult(
            self.dx_query.search(data),
            jmespath.search(self.jmes_query, data),
        ).assert_equal(self.jmes_query)

    def to_result(self, size: int, runs: int, data: DataBase) -> BenchmarkResult:
        compiled = jmespath.compile(self.jmes_query)

        timings_dx: list[float] = []
        for _ in range(runs):
            start = time.perf_counter()
            self.dx_query.search(data)
            end = time.perf_counter()
            timings_dx.append(end - start)

        timings_jp: list[float] = []
        for _ in range(runs):
            start = time.perf_counter()
            compiled.search(data)
            end = time.perf_counter()
            timings_jp.append(end - start)

        return BenchmarkResult(
            size=size,
            query=self.jmes_query,
            qrydict=statistics.median(timings_dx),
            jmespth=statistics.median(timings_jp),
        )


@dataclass(slots=True)
class CasesBuilder:
    cases: list[Case] = field(default_factory=list)

    def add(self, dx_query: dx.Expr, jmes_query: str) -> Self:
        self.cases.append(Case(dx_query, jmes_query))
        return self

    def get(self) -> list[Case]:
        return self.cases


def build_cases() -> list[Case]:
    users = dx.field("users")
    return (
        CasesBuilder()
        .add(
            users.list.map(dx.field("name")),
            "users[*].name",
        )
        .add(
            users.list.map(dx.field("address")),
            "users[*].address",
        )
        .add(dx.field("users").list.length(), "users | length(@)")
        .add(
            users.list.filter(
                dx.field("age").ge(30).and_(dx.field("active").eq(True))
            ).list.map(dx.field("name")),
            "users[?(age >= `30` && active == `true`)].name",
        )
        .add(users.list.length(), "length(users)")
        .add(
            users.list.map(dx.field("name")).list.map(dx.list().length()),
            "map(&length(@), users[*].name)",
        )
        .add(users.list.min_by("age"), "min_by(users, &age)")
        .add(
            users.list.sort_by("age").list.map(dx.field("name")),
            "sort_by(users, &age)[*].name",
        )
        .add(
            users.list.filter(
                dx.struct()
                .field("age")
                .gt(40)
                .and_(dx.field("active").eq(True))
                .and_(dx.field("category").list.contains("VIP"))
            )
            .list.map(dx.field("name"))
            .list.sort(),
            'sort(users[?((age > `40` && active == `true`) && contains(category, `"VIP"`))].name)',
        )
        .add(
            users.list.map(dx.field("category")).list.flatten(),
            "users[*].category[]",
        )
        .add(users.list.max_by("age"), "max_by(users, &age)")
        .add(
            users.list.map(dx.field("category").list.flatten())
            .list.flatten()
            .list.sort(),
            "sort(users[*].category[])",
        )
        .add(
            users.list.map(dx.field("age").abs()),
            "map(&abs(@), users[*].age)",
        )
        .add(users.list.map(dx.field("age")).list.avg(), "avg(users[*].age)")
        .add(
            users.list.map(dx.field("age").ceil()),
            "map(&ceil(@), users[*].age)",
        )
        .add(
            users.list.map(dx.field("age").floor()),
            "map(&floor(@), users[*].age)",
        )
        .add(users.list.map(dx.field("age")).list.max(), "max(users[*].age)")
        .add(users.list.map(dx.field("age")).list.min(), "min(users[*].age)")
        .add(
            users.list.map(dx.field("age")).list.reverse(),
            "reverse(users[*].age)",
        )
        .add(users.list.map(dx.field("age")).list.sum(), "sum(users[*].age)")
        .add(
            users.list.map(dx.field("address").struct.field("city")),
            "users[*].address.city",
        )
        .add(
            users.list.map(dx.field("name")).list.length(),
            "length(users[*].name)",
        )
        .add(
            users.list.map(dx.field("age").eq(30)).list.get(0),
            "users[*].age == `30`",
        )
        .add(
            users.list.map(dx.field("age"))
            .gt(1)
            .and_(users.list.map(dx.field("age")).eq(5).not_())
            .or_(0),
            "((users[*].age > `1` && !(users[*].age == `5`)) || `0`)",
        )
        .add(users.list.map(dx.struct().keys()), "users[*].keys(@)")
        .add(users.list.map(dx.struct().values()), "users[*].values(@)")
        .add(
            users.list.map(dx.field("category").list.contains("VIP")),
            'users[*].contains(category, `"VIP"`)',
        )
        .add(
            users.list.map(dx.field("category").list.join(", ")),
            'users[*].join(`", "`, category)',
        )
        .add(
            users.list.map(dx.merge(dx.element(), dx.lit({"extra_field": 1}))),
            'users[*].merge(@, `{"extra_field":1}`)',
        )
        .add(
            users.list.map(dx.field("name").str.ends_with("s")),
            'users[*].ends_with(name, `"s"`)',
        )
        .add(
            users.list.map(dx.field("name").str.starts_with("A")),
            'users[*].starts_with(name, `"A"`)',
        )
        .add(
            users.list.map(dx.not_null(dx.field("MISSING"), dx.field("name"))),
            "users[*].not_null(MISSING, name)",
        )
        .get()
    )


CASES: list[Case] = build_cases()
