from __future__ import annotations
import statistics
import time
from typing import Any

import jmespath
import jmespath_rs as qd
import polars as pl
from tests.data import BenchmarkResult, generate_db, DataBase
from tests.cases import CASES, Case

BENCHMARKS: list[Case] = [case for case in CASES]

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
        .agg(
            pl.col("jmespth").mul(1000).round(2).alias("jmespth_ms"),
            pl.col("qrydict").mul(1000).round(2).alias("qrydict_ms"),
            "speedup_pct",
            pl.col("speedup_pct")
            .mean()
            .truediv(100)
            .round(2)
            .alias("avg_speedup_factor"),
        )
        .sort("avg_speedup_factor", descending=True)
        .collect()
    )


def _qd_func(df: qd.DataJson, qry: qd.Expr):
    return df.collect(qry)


def _jsem_func(data: dict[str, Any], compiled: Any) -> Any:
    return compiled.search(data)


def add_case(case: Case, size: int, runs: int, data: DataBase) -> BenchmarkResult:
    jp_expr = case.jmes_query
    jp_compiled = jmespath.compile(jp_expr)
    test_data = case.data if case.data is not None else data
    df = qd.DataJson(test_data)
    qd_query_obj = case.build()
    jp_search_func = jp_compiled.search

    timings_qd: list[float] = []
    for _ in range(runs):
        start = time.perf_counter()
        _qd_func(df, qd_query_obj)
        end = time.perf_counter()
        timings_qd.append(end - start)

    timings_jp: list[float] = []
    for _ in range(runs):
        start = time.perf_counter()
        jp_search_func(test_data)
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
    data = generate_db(10)
    print(f"Running {len(BENCHMARKS)} benchmarks on sample data...")
    for case in BENCHMARKS:
        case.check(data)
    print("All benchmark cases passed correctness checks.\n")
    for size in DATA_SIZES:
        data = generate_db(size)

        for case in BENCHMARKS:
            results.append(add_case(case, size, runs, data))

    return format_results(results)


if __name__ == "__main__":
    main(5).pipe(print)
