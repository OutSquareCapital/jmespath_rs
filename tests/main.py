from tests.bench import BenchmarkConfig, run_benchs, run_checks
import doctester as dt
from pathlib import Path


def _update_readme() -> bool:
    return input("Update README? (y/n): ").strip().lower() == "y"


def run() -> None:
    config = BenchmarkConfig(data_sizes=[1, 50, 200, 800], runs=200)
    data = config.get_data()
    run_checks(data)
    if _update_readme():
        run_benchs(config, data)


if __name__ == "__main__":
    dt.run_on_file(Path().joinpath("dictexprs").with_suffix(".pyi"))
    run()
