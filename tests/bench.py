from __future__ import annotations
import statistics
import time
from typing import Any

import jmespath
import jmespath_rs as qd
from tests.data import BenchmarkResult, generate_db, DataBase
from tests.cases import CASES, Case
from tests.output import format_results


DATA_SIZES: list[int] = [500, 2000, 8000]


def _qd_func(df: qd.DataJson, qry: qd.Expr):
    return df.collect(qry)


def _jsem_func(data: dict[str, Any], compiled: Any) -> Any:
    return compiled.search(data)


def add_case(case: Case, size: int, runs: int, data: DataBase) -> BenchmarkResult:
    df = qd.DataJson(data)
    qd_query_obj = case.build()
    jp_search_func = jmespath.compile(case.jmes_query).search

    timings_qd: list[float] = []
    for _ in range(runs):
        start = time.perf_counter()
        _qd_func(df, qd_query_obj)
        end = time.perf_counter()
        timings_qd.append(end - start)

    timings_jp: list[float] = []
    for _ in range(runs):
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


def main(runs: int = 1) -> None:
    print(f"Lancement des benchmarks (Runs par test: {runs})\n")
    results: list[BenchmarkResult] = []
    data = generate_db(10)
    print(f"Running {len(CASES)} benchmarks on sample data...")
    for case in CASES:
        case.check(data)
    print("All benchmark cases passed correctness checks.\n")
    for size in DATA_SIZES:
        data = generate_db(size)

        for case in CASES:
            results.append(add_case(case, size, runs, data))

    return format_results(results)


if __name__ == "__main__":
    main()
