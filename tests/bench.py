from __future__ import annotations
import statistics
import time
from typing import Any
from dataclasses import dataclass
import jmespath
import jmespath_rs as qd
from tests.data import BenchmarkResult, DataBase


@dataclass(slots=True, frozen=True)
class Case:
    qd_query: qd.Expr
    jmes_query: str

    def check(self, data: dict[str, Any]) -> None:
        """Checks the query against the provided data."""

        got = qd.DataJson(data).collect(self.qd_query)
        want = jmespath.search(self.jmes_query, data)

        assert got == want, (
            f"{self.jmes_query}: \n  Query: {self.jmes_query!r}\n  Got:   {got!r}\n  Want:  {want!r}"
        )
        print(f"✔ {self.jmes_query}")

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
