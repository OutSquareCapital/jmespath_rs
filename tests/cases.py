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
    elem = dx.Expr()
    return (
        CasesBuilder()
        .add(dx.key("users").list.map(dx.key("name")), "users[*].name")
        .add(dx.key("users").list.map(dx.key("address")), "users[*].address")
        .add(elem.struct.field("users").list.lengths(), "users | length(@)")
        .add(
            dx.key("users")
            .list.filter(
                elem.struct.field("age")
                .ge(30)
                .and_(elem.struct.field("active").eq(True))
            )
            .list.map(dx.key("name")),
            "users[?(age >= `30` && active == `true`)].name",
        )
        .add(
            dx.select_dict(
                names=dx.key("users").list.map(dx.key("name")),
                count=dx.key("users").list.lengths(),
            ),
            '{"names": users[*].name, "count": length(users)}',
        )
        .add(dx.key("users").list.lengths(), "length(users)")
        .add(
            dx.key("users").list.map(dx.key("name")).list.map(elem.list.lengths()),
            "map(&length(@), users[*].name)",
        )
        .add(dx.key("users").list.min_by(dx.key("age")), "min_by(users, &age)")
        .add(
            dx.key("users").list.sort_by(dx.key("age")).list.map(dx.key("name")),
            "sort_by(users, &age)[*].name",
        )
        .add(
            dx.key("users")
            .list.filter(
                elem.struct.field("age")
                .gt(40)
                .and_(elem.struct.field("active").eq(True))
                .and_(elem.struct.field("category").list.contains(dx.lit("VIP")))
            )
            .list.map(dx.key("name"))
            .list.sort(),
            'sort(users[?((age > `40` && active == `true`) && contains(category, `"VIP"`))].name)',
        )
        .add(
            dx.key("users").list.map(dx.key("category")).list.flatten(),
            "users[*].category[]",
        )
        .add(dx.key("users").list.max_by(dx.key("age")), "max_by(users, &age)")
        .add(
            dx.key("users")
            .list.map(elem.struct.field("category").list.flatten())
            .list.flatten()
            .list.sort(),
            "sort(users[*].category[])",
        )
        .add(
            dx.key("users").list.map(elem.struct.field("age").abs()),
            "map(&abs(@), users[*].age)",
        )
        .add(dx.key("users").list.map(dx.key("age")).list.avg(), "avg(users[*].age)")
        .add(
            dx.key("users").list.map(elem.struct.field("age").ceil()),
            "map(&ceil(@), users[*].age)",
        )
        .add(
            dx.key("users").list.map(elem.struct.field("age").floor()),
            "map(&floor(@), users[*].age)",
        )
        .add(dx.key("users").list.map(dx.key("age")).list.max(), "max(users[*].age)")
        .add(dx.key("users").list.map(dx.key("age")).list.min(), "min(users[*].age)")
        .add(
            dx.key("users").list.map(dx.key("age")).list.reverse(),
            "reverse(users[*].age)",
        )
        .add(dx.key("users").list.map(dx.key("age")).list.sum(), "sum(users[*].age)")
        .add(
            dx.key("users").list.map(elem.struct.field("address").struct.field("city")),
            "users[*].address.city",
        )
        .add(
            dx.key("users").list.map(dx.key("name")).list.lengths(),
            "length(users[*].name)",
        )
        .add(
            dx.select_list(
                dx.key("users").list.slice(0, 10), dx.key("users").list.slice(-10, None)
            ),
            "[users[0:10], users[-10:]]",
        )
        .add(
            dx.key("users").list.map(elem.struct.field("age").eq(30)).list.get(0),
            "users[*].age == `30`",
        )
        .add(
            dx.key("users")
            .list.map(dx.key("age"))
            .gt(1)
            .and_(dx.key("users").list.map(dx.key("age")).eq(5).not_())
            .or_(0),
            "((users[*].age > `1` && !(users[*].age == `5`)) || `0`)",
        )
        .add(dx.key("users").list.map(elem.struct.keys()), "users[*].keys(@)")
        .add(dx.key("users").list.map(elem.struct.values()), "users[*].values(@)")
        .add(
            dx.key("users").list.map(
                elem.struct.field("category").list.contains(dx.lit("VIP"))
            ),
            'users[*].contains(category, `"VIP"`)',
        )
        .add(
            dx.key("users").list.map(
                elem.struct.field("category").list.join(dx.lit(", "))
            ),
            'users[*].join(`", "`, category)',
        )
        .add(
            dx.key("users").list.map(dx.merge(elem, dx.lit({"extra_field": 1}))),
            'users[*].merge(@, `{"extra_field":1}`)',
        )
        .add(
            dx.key("users").list.map(
                elem.struct.field("name").str.ends_with(dx.lit("s"))
            ),
            'users[*].ends_with(name, `"s"`)',
        )
        .add(
            dx.key("users").list.map(
                elem.struct.field("name").str.starts_with(dx.lit("A"))
            ),
            'users[*].starts_with(name, `"A"`)',
        )
        .add(
            dx.key("users").list.map(
                dx.not_null(elem.struct.field("MISSING"), elem.struct.field("name"))
            ),
            "users[*].not_null(MISSING, name)",
        )
        .get()
    )


CASES: list[Case] = build_cases()
