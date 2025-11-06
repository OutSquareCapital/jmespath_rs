from collections.abc import Iterator
from pathlib import Path

import doctester as dt
import polars as pl
import pyochain as pc

from tests.bench import (
    BenchmarkConfig,
    Cols,
    generate_markdown_table,
    write_markdown_table,
)
from tests.cases import BenchmarkResult, Case, build_cases

README = Path().joinpath("README").with_suffix(".md")
STUBS = Path().joinpath("dictexprs").with_suffix(".pyi")
CASES: pc.Seq[Case] = pc.Seq(build_cases())


def _update_readme() -> bool:
    return input("Update README? (y/n): ").strip().lower() == "y"


def run() -> None:
    config = BenchmarkConfig(data_sizes=pc.Seq.from_(10, 50, 250, 1250), runs=100)
    data = config.get_data()

    sample = data.iter_values().first()
    print(f"Running {CASES.count()} benchmarks on sample data...")
    CASES.iter().for_each(lambda case: case.check(sample))
    print("All benchmark cases passed correctness checks.\n")

    def _get_res(case: Case) -> Iterator[BenchmarkResult]:
        return (
            data.iter_items()
            .map(lambda kv: case.to_result(kv[0], kv[1], config.runs))
            .unwrap()
        )

    if _update_readme():
        (
            CASES.iter()
            .map(lambda case: case.warmup())
            .map(_get_res)
            .flatten()
            .into(pl.LazyFrame)
            .with_columns(
                pl.col(Cols.JMESPTH)
                .truediv(pl.col(Cols.QRYDICT))
                .round(1)
                .over(Cols.QUERY, Cols.SIZE)
                .alias(Cols.SPEEDUP)
            )
            .collect()
            .pivot(
                on=Cols.SIZE,
                index=Cols.QUERY,
                values=Cols.SPEEDUP,
                aggregate_function="median",
            )
            .lazy()
            .with_columns(
                pl.mean_horizontal(pl.all().exclude(Cols.QUERY))
                .cast(pl.UInt8)
                .alias(Cols.AVERAGE_SPEEDUP)
            )
            .sort(Cols.AVERAGE_SPEEDUP, descending=True)
            .collect()
            .pipe(generate_markdown_table, config)
            .into(write_markdown_table, README)
        )


if __name__ == "__main__":
    dt.run_on_file(STUBS)
    run()
