from tests.data import DataBase
import time
import jmespath
from typing import Any, NamedTuple

from collections.abc import Callable
import statistics
import math
import dictexprs as dx
from dataclasses import dataclass, field
from typing import Self


class BenchmarkResult(NamedTuple):
    size: int
    query: str
    qrydict: float
    jmespth: float


def _check_equal(got: Any, want: Any) -> bool:
    if isinstance(got, float) and isinstance(want, float):
        return math.isclose(got, want)
    else:
        return got == want


def _add_time(func: Callable[[Any], Any], data: Any) -> float:
    start = time.perf_counter()
    func(data)
    return time.perf_counter() - start


def _get_perf(func: Callable[[Any], Any], data: Any, runs: int) -> float:
    return statistics.median([_add_time(func, data) for _ in range(runs)])


@dataclass(slots=True, frozen=True)
class Case:
    dx_query: dx.Expr
    jmes_query: str

    def check(self, data: DataBase) -> None:
        """Checks the query against the provided data."""
        try:
            dx_result = self.dx_query.search(data)
        except Exception as dx_exc:
            print(f"[dx] Exception: {dx_exc!r}")
            try:
                jmes_result = jmespath.search(self.jmes_query, data)
                print(f"[jmes] {jmes_result!r}")
            except Exception as jmes_exc:
                print(f"[jmes] Exception: {jmes_exc!r}")
            raise
        try:
            jmes_result = jmespath.search(self.jmes_query, data)
        except Exception as jmes_exc:
            print(f"[dx] {dx_result!r}")
            print(f"[jmes] Exception: {jmes_exc!r}")
            raise

        assert _check_equal(dx_result, jmes_result), print(
            f"Query: {self.jmes_query!r}\n  Got:   {dx_result!r}\n  Want:  {jmes_result!r}"
        )
        print(f"✔ {self.jmes_query}")

    def warmup(self, data: DataBase, compiled: Any) -> None:
        for _ in range(20):
            self.dx_query.search(data)
        for _ in range(20):
            compiled.search(data)

    def to_result(self, size: int, data: DataBase, runs: int) -> BenchmarkResult:
        compiled = jmespath.compile(self.jmes_query)
        self.warmup(data, compiled)
        return BenchmarkResult(
            size=size,
            query=self.jmes_query,
            qrydict=_get_perf(self.dx_query.search, data, runs),
            jmespth=_get_perf(compiled.search, data, runs),
        )


@dataclass(slots=True)
class CasesBuilder:
    cases: list[Case] = field(default_factory=list[Case])

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
        .add(
            users.list.map(dx.field("nested_scores")).list.flatten(),
            "users[*].nested_scores[]",
        )
        .add(
            users.list.map(dx.field("nested_scores")).list.flatten().list.flatten(),
            "users[*].nested_scores[][]",
        )
        .add(users.list.max_by("age"), "max_by(users, &age)")
        .add(
            users.list.map(dx.field("nested_scores"))
            .list.flatten()
            .list.flatten()
            .list.sort(),
            "sort(users[*].nested_scores[][])",
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
            users.list.map(dx.field("nested_scores").list.flatten().list.contains(50)),
            "users[*].contains(nested_scores[], `50`)",
        )
        .add(
            dx.field("sales")
            .list.flatten()
            .list.map(dx.struct().keys().list.join(", ")),
            """sales[][] | map(&join(`, `, keys(@)), @)""",
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
        .add(
            users.list.map(dx.field("age")).list.sum().abs(),
            "abs(sum(users[*].age))",
        )
        .get()
    )
