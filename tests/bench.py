from __future__ import annotations

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
]

DATA_SIZES: list[int] = [500, 1000, 10_000, 100_000]


def format_results(results: list[BenchmarkResult]) -> pl.DataFrame:
    return (
        pl.LazyFrame(results)
        .unpivot(index=["size", "case_name"], variable_name="lib", value_name="time")
        .group_by("size", "case_name")
        .agg(
            pl.all().exclude("time"),
            pl.col("time").mul(1000).round(3).alias("time"),
            pl.col("time").rank().alias("rank"),
        )
        .sort("case_name", "size")
        .collect()
    )


def _qd_func(df: qd.DataJson, qry: qd.QueryBuilder) -> qd.DataJson:
    return df.query(qry)


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
    assert jp_search_func(data) == _qd_func(df, qd_query_obj).collect()

    start = time.perf_counter()
    timings_qd = []
    for i in range(runs):
        _qd_func(df, qd_query_obj)
        end = time.perf_counter()
        timings_qd.append(end - start)
    start = time.perf_counter()
    timings_jp = []
    for i in range(runs):
        jp_search_func(data)
        end = time.perf_counter()
        timings_jp.append(end - start)

    return BenchmarkResult(
        size=size,
        case_name=case.name,
        qrydict=sum(timings_qd) / runs,
        jmespth=sum(timings_jp) / runs,
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
    main(5).pipe(print)
