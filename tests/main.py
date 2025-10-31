from __future__ import annotations

from tests.data import BenchmarkResult, generate_db
from tests.cases import CASES
from tests.output import format_results
from tests.bench import DATA_SIZES, add_case


def main(runs: int, update_readme: bool) -> None:
    print(f"Lancement des benchmarks (Runs par test: {runs})\n")
    results: list[BenchmarkResult] = []
    data = generate_db(10)
    print(f"Running {len(CASES)} benchmarks on sample data...")
    for case in CASES:
        case.check(data)
    print("All benchmark cases passed correctness checks.\n")
    for size in DATA_SIZES:
        data = generate_db(size)

        for case in CASES:
            results.append(add_case(case, size, runs, data))

    return format_results(results, update_readme)


if __name__ == "__main__":
    main(100, True)
