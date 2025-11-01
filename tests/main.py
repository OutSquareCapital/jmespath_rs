from tests.bench import run_benchmarks
import doctester as dt
from pathlib import Path

if __name__ == "__main__":
    dt.run_on_file(Path().joinpath("jmespath_rs").with_suffix(".pyi"))
    run_benchmarks()
