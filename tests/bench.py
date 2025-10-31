from __future__ import annotations
from tests.data import BenchmarkResult, generate_db
from tests.cases import CASES
import polars as pl
from pathlib import Path
from enum import StrEnum, auto
from typing import TypedDict

DATA_SIZES: list[int] = [50, 200, 800]


class ResultRow(TypedDict):
    query: str
    with_50_runs: float
    with_200_runs: float
    with_800_runs: float


class Output(StrEnum):
    MARKER = "<!-- BENCHMARK_RESULTS -->"
    MARKER_END = "<!-- END_BENCHMARK_RESULTS -->"


def header() -> str:
    return "| query | 50 | 200 | 800 |\n|---|---|---|---|\n"


class Cols(StrEnum):
    QUERY = auto()
    SIZE = auto()
    JMESPTH = auto()
    QRYDICT = auto()
    SPEEDUP = auto()


README = Path().joinpath("README").with_suffix(".md")


def _speedup():
    return (
        pl.col(Cols.JMESPTH)
        .truediv(pl.col(Cols.QRYDICT))
        .round(2)
        .over(Cols.QUERY, Cols.SIZE)
        .alias(Cols.SPEEDUP)
    )


def _add_col(nb_runs: int) -> str:
    return f" | with_{nb_runs}_runs |"


def _add_row(row: ResultRow) -> str:
    query = row["query"].replace("|", "\\|").replace("*", r"\*")
    return f"| {query} | {row['with_50_runs']} | {row['with_200_runs']} | {row['with_800_runs']} |\n"


def _write_markdown_table(df: pl.DataFrame, readme_path: Path):
    md = header()
    for row in df.iter_rows(named=True):
        md += _add_row(row)

    with open(readme_path, "r", encoding="utf-8") as f:
        content = f.read()

    if Output.MARKER in content:
        before = content.split(Output.MARKER, 1)[0]
        after = (
            content.split(Output.MARKER_END, 1)[-1]
            if Output.MARKER_END in content
            else ""
        )
        content = before + Output.MARKER + "\n" + md + "\n" + Output.MARKER_END + after
    else:
        content += "\n" + Output.MARKER + "\n" + md + "\n" + Output.MARKER_END + "\n"

    with open(readme_path, "w", encoding="utf-8") as f:
        f.write(content)


def format_results(results: list[BenchmarkResult], update_readme: bool) -> None:
    df = (
        pl.LazyFrame(results)
        .with_columns(_speedup())
        .collect()
        .pivot(
            on=Cols.SIZE,
            index=Cols.QUERY,
            values=Cols.SPEEDUP,
            aggregate_function="median",
        )
        .lazy()
        .with_columns(
            pl.all().exclude(Cols.QUERY).name.suffix("_runs").name.prefix("with_")
        )
        .sort(Cols.QUERY)
        .collect()
    )
    if update_readme:
        df.pipe(_write_markdown_table, README)


def _runs_nb(update_readme: bool) -> int:
    if update_readme:
        return 200
    else:
        return 1


def _update_readme() -> bool:
    return input("Update README? (y/n): ").strip().lower() == "y"


def run_benchmarks() -> None:
    update_readme = _update_readme()
    runs = _runs_nb(update_readme)
    print(f"Lancement des benchmarks (Runs par test: {runs})\n")
    results: list[BenchmarkResult] = []
    data = generate_db(1)
    print(f"Running {len(CASES)} benchmarks on sample data...")
    for case in CASES:
        case.check(data)
    print("All benchmark cases passed correctness checks.\n")
    for size in DATA_SIZES:
        data = generate_db(size)

        for case in CASES:
            results.append(case.to_result(size, runs, data))

    return format_results(results, update_readme)
