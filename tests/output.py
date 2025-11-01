import polars as pl
from enum import StrEnum, auto
from tests.data import BenchmarkResult


class Cols(StrEnum):
    QUERY = auto()
    SIZE = auto()
    JMESPTH = auto()
    QRYDICT = auto()
    SPEEDUP = auto()
    AVERAGE_SPEEDUP = auto()


def _speedup():
    return (
        pl.col(Cols.JMESPTH)
        .truediv(pl.col(Cols.QRYDICT))
        .round(1)
        .over(Cols.QUERY, Cols.SIZE)
        .alias(Cols.SPEEDUP)
    )


def format_results(results: list[BenchmarkResult]) -> pl.DataFrame:
    return (
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
            pl.mean_horizontal(pl.all().exclude(Cols.QUERY))
            .round(1)
            .alias(Cols.AVERAGE_SPEEDUP)
        )
        .sort(Cols.AVERAGE_SPEEDUP, descending=True)
        .collect()
    )
