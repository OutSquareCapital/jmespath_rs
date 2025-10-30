# jmespath_rs

A simple package to test Rust interaction with dictionaries.

Meant to be a new version of querydict which hopefully can be more performant (not the case yet except on complex queries)

## Developpement

```bash
maturin build --release
uv pip install -e .
uv run tests/checks.py
uv run tests/bench.py
```
