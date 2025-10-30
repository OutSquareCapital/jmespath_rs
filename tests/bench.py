from __future__ import annotations
import statistics
import random
import string
import time
from collections.abc import Callable
from dataclasses import dataclass
from typing import Any, TypedDict

import jmespath
import polars as pl

import jmespath_rs as qd

type JsonData = dict[str, Any]
type QueryFunc = Callable[[JsonData], Any]


class BenchmarkResult(TypedDict):
    size: int
    case_name: str
    qrydict: float
    jmespth: float


@dataclass(slots=True, frozen=True)
class BenchmarkCase:
    name: str
    qd_query: qd.QueryBuilder


def rand_str(k: int) -> str:
    return "".join(random.choices(string.ascii_lowercase, k=k))


def generate_user(i: int) -> dict[str, Any]:
    return {
        "id": i,
        "name": rand_str(10),
        "age": random.randint(18, 65),
        "active": random.choice([True, False]),
        "tags": [
            random.choice(["tag1", "tag2", "tag3"]) for _ in range(random.randint(1, 5))
        ],
    }


def generate_data(n: int) -> JsonData:
    return {"users": [generate_user(i) for i in range(n)]}


BENCHMARKS: list[BenchmarkCase] = [
    BenchmarkCase(
        name="Projection simple (names)", qd_query=qd.field("users").project("name")
    ),
    BenchmarkCase(
        name="Filtre complexe (active & >30 & tag1)",
        qd_query=qd.field("users").filter(
            qd.field("age")
            .gt(30)
            .and_(qd.field("active").eq(True))
            .and_(qd.field("tags").eq("tag1").or_(qd.field("tags").eq("tag2"))),
            then="name",
        ),
    ),
    BenchmarkCase(
        name="Tri (sort_by age)",
        qd_query=qd.field("users").sort_by(qd.field("age")),
    ),
    BenchmarkCase(
        name="Tranche (slice 10:20)",
        qd_query=qd.field("users").slice(10, 20),
    ),
    BenchmarkCase(
        name="Accès champ (first user name)",
        qd_query=qd.field("users").index(0).field("name"),
    ),
    BenchmarkCase(
        name="Multi-sélection dico (id et age)",
        qd_query=qd.select_dict(
            id=qd.field("users").index(0).field("id"),
            age=qd.field("users").index(0).field("age"),
        ),
    ),
]

DATA_SIZES: list[int] = [500, 2000, 8000]


def _speedup():
    return (
        pl.col("jmespth")
        .truediv(pl.col("qrydict"))
        .mul(100)
        .round(2)
        .alias("speedup_pct")
    )


def format_results(results: list[BenchmarkResult]) -> pl.DataFrame:
    return (
        pl.LazyFrame(results)
        .with_columns(_speedup())
        .group_by("case_name")
        .agg("speedup_pct", pl.col("speedup_pct").mean().alias("avg_speedup_pct"))
        .sort("avg_speedup_pct", descending=True)
        .collect()
    )


def _qd_func(df: qd.DataJson, qry: qd.QueryBuilder):
    return df.query(qry).collect()


def _jsem_func(data: dict[str, Any], compiled: Any) -> Any:
    return compiled.search(data)


def add_case(
    case: BenchmarkCase, size: int, runs: int, data: JsonData
) -> BenchmarkResult:
    jp_expr = case.qd_query.to_jmespath()
    jp_compiled = jmespath.compile(jp_expr)
    df = qd.DataJson(data)
    qd_query_obj = case.qd_query
    jp_search_func = jp_compiled.search
    assert jp_search_func(data) == _qd_func(df, qd_query_obj)

    timings_qd = []
    for i in range(runs):
        start = time.perf_counter()
        _qd_func(df, qd_query_obj)
        end = time.perf_counter()
        timings_qd.append(end - start)
    timings_jp = []
    for i in range(runs):
        start = time.perf_counter()
        jp_search_func(data)
        end = time.perf_counter()
        timings_jp.append(end - start)

    return BenchmarkResult(
        size=size,
        case_name=case.name,
        qrydict=statistics.median(timings_qd),
        jmespth=statistics.median(timings_jp),
    )


def main(runs: int) -> pl.DataFrame:
    print(f"Lancement des benchmarks (Runs par test: {runs})\n")
    results: list[BenchmarkResult] = []
    for size in DATA_SIZES:
        data = generate_data(size)

        for case in BENCHMARKS:
            results.append(add_case(case, size, runs, data))

    return format_results(results)


if __name__ == "__main__":
    main(20).pipe(print)
