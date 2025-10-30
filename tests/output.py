import polars as pl
from tests.data import BenchmarkResult
from pathlib import Path

OUTPUT = Path(__file__).parent.joinpath("benchmark_results").with_suffix(".ndjson")


def _speedup():
    return (
        pl.col("jmespth")
        .truediv(pl.col("qrydict"))
        .mean()
        .round(2)
        .over("case_name", "size")
        .alias("avg_speedup_factor")
    )


def format_results(results: list[BenchmarkResult]) -> None:
    return (
        pl.LazyFrame(results)
        .with_columns(_speedup())
        .collect()
        .pivot(on="size", index="case_name", values="avg_speedup_factor")
        .write_ndjson(OUTPUT)
    )
