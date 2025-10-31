from __future__ import annotations
import statistics
import time
from typing import Any
from dataclasses import dataclass
import jmespath
import jmespath_rs as qd
import math
from tests.data import BenchmarkResult, DataBase


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
    jmes_query: str

    def check(self, data: DataBase) -> None:
        """Checks the query against the provided data."""
        CheckResult(
            qd.DataJson(data).collect(self.qd_query),
            jmespath.search(self.jmes_query, data),
        ).assert_equal(self.jmes_query)

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
