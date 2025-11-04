from __future__ import annotations
from tests.data import generate_db, DataBase
from typing import Any
from pathlib import Path
from enum import StrEnum, auto
from dataclasses import dataclass
import pyochain as pc
import polars as pl


class Cols(StrEnum):
    QUERY = auto()
    SIZE = auto()
    JMESPTH = auto()
    QRYDICT = auto()
    SPEEDUP = auto()
    AVERAGE_SPEEDUP = auto()


class Markers(StrEnum):
    START = "<!-- BENCHMARK_RESULTS -->"
    END = "<!-- END_BENCHMARK_RESULTS -->"


@dataclass(slots=True)
class BenchmarkConfig:
    data_sizes: pc.Seq[int]
    runs: int

    def get_data(self) -> pc.Dict[int, DataBase]:
        return (
            self.data_sizes.iter()
            .map(lambda size: (size, generate_db(size, 5)))
            .into(lambda x: pc.Dict(dict(x)))
        )

    def header(self) -> str:
        cols = self.data_sizes.iter().map(str).into(" | ".join)
        return f"| query | {cols} | {Cols.AVERAGE_SPEEDUP} |\n"

    def separator(self) -> str:
        return "|---" * (self.data_sizes.count() + 2) + "|\n"

    def add_row(self, row: dict[str, Any]) -> str:
        query = row["query"].replace("|", "\\|").replace("*", r"\*")
        cols = (
            self.data_sizes.iter()
            .map(lambda size: str(row[str(size)]))
            .into(" | ".join)
        )
        return f"| {query} | {cols} | {row[Cols.AVERAGE_SPEEDUP]} |\n"


def generate_markdown_table(
    df: pl.DataFrame, config: BenchmarkConfig
) -> pc.Wrapper[str]:
    return pc.Wrapper(
        config.header()
        + config.separator()
        + "".join((config.add_row(row) for row in df.iter_rows(named=True)))
    )


def write_markdown_table(md: str, readme_path: Path) -> None:
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
