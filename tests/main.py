from __future__ import annotations

from tests.data import BenchmarkResult, generate_db
from tests.cases import CASES
from tests.output import format_results

DATA_SIZES: list[int] = [50, 200, 800]


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
            results.append(case.to_result(size, runs, data))

    return format_results(results, update_readme)


if __name__ == "__main__":
    runs = int(input("Enter the number of runs: "))
    update_readme = input("Update README? (y/n): ").strip().lower() == "y"
    main(runs, update_readme)
