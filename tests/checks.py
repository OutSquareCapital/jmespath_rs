from __future__ import annotations

from tests.cases import CASES


def main() -> None:
    print(f"Running {len(CASES)} cases…\n")
    for c in CASES:
        c.check()
    print("\nAll good.")


if __name__ == "__main__":
    main()
