import polars as pl
from tests.data import BenchmarkResult
from pathlib import Path

CURRENT = Path(__file__).parent
OUTPUT = CURRENT.joinpath("benchmark_results").with_suffix(".ndjson")
README = CURRENT.parent.joinpath("README").with_suffix(".md")


def _speedup():
    return (
        pl.col("jmespth")
        .truediv(pl.col("qrydict"))
        .round(2)
        .over("case_name", "size")
        .alias("avg_speedup_factor")
    )


def _write_markdown_table(df: pl.DataFrame, readme_path: Path):
    md = "| case_name | 500 | 2000 | 8000 |\n|---|---|---|---|\n"
    for row in df.iter_rows(named=True):
        md += f"| {row['case_name']} | {row['500']} | {row['2000']} | {row['8000']} |\n"
    marker = "<!-- BENCHMARK_RESULTS -->"
    marker_end = "<!-- END_BENCHMARK_RESULTS -->"

    with open(readme_path, "r", encoding="utf-8") as f:
        content = f.read()

    if marker in content:
        before = content.split(marker, 1)[0]
        after = content.split(marker_end, 1)[-1] if marker_end in content else ""
        content = before + marker + "\n" + md + "\n" + marker_end + after
    else:
        content += "\n" + marker + "\n" + md + "\n" + marker_end + "\n"

    with open(readme_path, "w", encoding="utf-8") as f:
        f.write(content)


def format_results(results: list[BenchmarkResult], update_readme: bool) -> None:
    df = (
        pl.LazyFrame(results)
        .with_columns(_speedup())
        .collect()
        .pivot(on="size", index="case_name", values="avg_speedup_factor")
        .sort("case_name")
    )
    df.write_ndjson(OUTPUT)
    if update_readme:
        _write_markdown_table(df, README)
