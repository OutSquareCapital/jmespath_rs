from __future__ import annotations
from tests.data import BenchmarkResult, generate_db
from collections.abc import Iterator
from tests.cases import CASES
from typing import Any
from pathlib import Path
from enum import StrEnum
from dataclasses import dataclass
from tests.output import Cols, format_results


class Markers(StrEnum):
    START = "<!-- BENCHMARK_RESULTS -->"
    END = "<!-- END_BENCHMARK_RESULTS -->"


README = Path().joinpath("README").with_suffix(".md")


@dataclass(slots=True)
class BenchmarkConfig:
    data_sizes: list[int]
    runs: int

    @property
    def _header(self) -> str:
        return f"| query | {' | '.join((str(size) for size in self.data_sizes))} | {Cols.AVERAGE_SPEEDUP} |\n"

    @property
    def _separator(self) -> str:
        return "|---" * (len(self.data_sizes) + 2) + "|\n"

    def _add_row(self, row: dict[str, Any]) -> str:
        query = row["query"].replace("|", "\\|").replace("*", r"\*")
        return f"| {query} | {' | '.join((str(row[str(size)]) for size in self.data_sizes))} | {row[Cols.AVERAGE_SPEEDUP]} |\n"

    def generate_markdown_table(self, data: Iterator[dict[str, Any]]) -> str:
        return (
            self._header
            + self._separator
            + "".join((self._add_row(row) for row in data))
        )


def _write_markdown_table(readme_path: Path, md: str):
    with open(readme_path, "r", encoding="utf-8") as f:
        content = f.read()

    if Markers.START in content:
        before = content.split(Markers.START, 1)[0]
        after = content.split(Markers.END, 1)[-1] if Markers.END in content else ""
        content = before + Markers.START + "\n" + md + "\n" + Markers.END + after
    else:
        content += "\n" + Markers.START + "\n" + md + "\n" + Markers.END + "\n"

    with open(readme_path, "w", encoding="utf-8") as f:
        f.write(content)


def run_checks():
    print(f"Running {len(CASES)} benchmarks on sample data...")
    data = generate_db(1)
    for case in CASES:
        case.check(data)
    print("All benchmark cases passed correctness checks.\n")


def run_benchs(config: BenchmarkConfig) -> None:
    print(f"Lancement des benchmarks (Runs par test: {config.runs})\n")

    results: list[BenchmarkResult] = []
    for size in config.data_sizes:
        data = generate_db(size)

        for case in CASES:
            results.append(case.to_result(size, config.runs, data))

    _write_markdown_table(
        README,
        config.generate_markdown_table(format_results(results).iter_rows(named=True)),
    )
