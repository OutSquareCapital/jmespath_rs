from tests.bench import BenchmarkConfig, run_benchs, run_checks
import doctester as dt
from pathlib import Path

CONFIG = BenchmarkConfig(data_sizes=[50, 200, 800], runs=200)


def _update_readme() -> bool:
    return input("Update README? (y/n): ").strip().lower() == "y"


def run(config: BenchmarkConfig = CONFIG) -> None:
    run_checks()
    if _update_readme():
        run_benchs(config)


if __name__ == "__main__":
    dt.run_on_file(Path().joinpath("jmespath_rs").with_suffix(".pyi"))
    run()
