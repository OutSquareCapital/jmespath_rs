# dictexprs

A high-performance Python library for querying and transforming JSON data with a Polars-like API, built in Rust.

## Overview

**dictexprs** combines the best of two worlds:

- **Polars-like API**: Chainable, expressive methods for data transformation (`.map()`, `.filter()`, `.sort_by()`)
- **JMESPath semantics**: Robust type handling and JSON-native operations
- **Rust performance**: 5-40× faster than pure Python JMESPath implementations

### Philosophy

Unlike traditional JMESPath libraries that use string-based queries, dictexprs provides a **programmatic, type-safe API**:

```python
import dictexprs as dx

# Traditional JMESPath (string-based)
jmespath.search("users[?age >= `30`].name", data)

# dictexprs (programmatic)
(
    dx.field("users")
    .list.filter(dx.field("age").ge(30))
    .list.map(dx.field("name"))
    .search(data)
)
```

**Key benefits:**

- ✅ **IDE autocomplete** and type hints
- ✅ **Compile-time validation** (no runtime query parsing errors)
- ✅ **Composable expressions** (reuse query fragments)
- ✅ **Zero-cost abstractions** (Rust evaluates directly, no intermediate representation)

### Type System

dictexprs uses JMESPath's type system with explicit conversions:

- **`IntoExpr`** (`Expr | str | int | float | bool | None`): For literals and comparisons
  - `.contains(42)` → search for literal value
  - `.eq("Alice")` → compare with string literal

This explicit typing prevents ambiguous operations while maintaining ergonomics.

## Developpement

```bash
uv run maturin build --release
uv pip install -e .
uv run  -m tests.bench
```

## Benchmark Results

Each column besides query represent the data size.

100 runs are computed for each test.

Each value is the median Python time divided by the median Rust time (median across runs)

A value of 2 meaning that Rust is twice as fast, a value of 0.5 meaning that Rust is half as fast.

<!-- BENCHMARK_RESULTS -->
| query | 10 | 50 | 250 | 1250 | average_speedup |
|---|---|---|---|---|---|
| users[\*].ends_with(name, `"s"`) | 24.9 | 45.7 | 45.5 | 44.7 | 40 |
| users[\*].starts_with(name, `"A"`) | 24.4 | 27.1 | 51.4 | 45.7 | 37 |
| map(&length(@), users[\*].name) | 36.6 | 24.1 | 37.4 | 41.0 | 34 |
| users[\*] \| map(&(age > `1` && !(age == `5`) \|\| age == `0`), @) | 40.1 | 31.7 | 39.9 | 26.6 | 34 |
| users[\*].keys(@) | 21.7 | 43.6 | 37.0 | 35.6 | 34 |
| users[\*].nested_scores[][] | 29.7 | 30.0 | 32.0 | 33.4 | 31 |
| users[\*].values(@) | 31.2 | 34.9 | 27.9 | 25.8 | 29 |
| map(&abs(@), users[\*].age) | 29.4 | 29.0 | 27.1 | 27.7 | 28 |
| map(&ceil(@), users[\*].age) | 21.1 | 29.2 | 36.9 | 27.8 | 28 |
| users \| length(@) | 30.7 | 25.0 | 24.5 | 25.0 | 26 |
| sort(users[?((age > `40` && active == `true`) && contains(category, `"VIP"`))].name) | 22.9 | 26.8 | 27.7 | 26.6 | 26 |
| sales[][] \| map(&join(`,`, keys(@)), @) | 27.6 | 27.9 | 27.9 | 22.2 | 26 |
| map(&floor(@), users[\*].age) | 20.9 | 20.3 | 28.7 | 27.6 | 24 |
| users[\*].merge(@, `{"extra_field":1}`) | 17.3 | 27.7 | 23.8 | 23.4 | 23 |
| users[?(age >= `30` && active == `true`)].name | 14.9 | 27.5 | 21.8 | 23.4 | 21 |
| length(users) | 22.0 | 21.7 | 21.7 | 21.0 | 21 |
| users[\*].contains(nested_scores[], `50`) | 16.3 | 26.2 | 25.4 | 20.0 | 21 |
| users[\*].not_null(MISSING, name) | 11.2 | 23.8 | 12.1 | 20.4 | 16 |
| users[\*].nested_scores[] | 20.6 | 9.3 | 13.2 | 17.0 | 15 |
| users[\*].category[] | 19.4 | 9.3 | 9.6 | 17.1 | 13 |
| sort(users[\*].nested_scores[][]) | 11.9 | 17.4 | 11.9 | 12.1 | 13 |
| users[\*].address.city | 11.9 | 11.8 | 11.3 | 10.8 | 11 |
| max(users[\*].age) | 12.1 | 9.9 | 9.0 | 8.3 | 9 |
| min(users[\*].age) | 11.9 | 9.8 | 8.7 | 8.4 | 9 |
| length(users[\*].name) | 12.8 | 9.6 | 8.4 | 7.8 | 9 |
| abs(sum(users[\*].age)) | 13.4 | 9.9 | 8.6 | 8.0 | 9 |
| users[\*].address | 8.4 | 8.5 | 7.8 | 7.4 | 8 |
| min_by(users, &age) | 10.0 | 7.6 | 7.2 | 7.7 | 8 |
| avg(users[\*].age) | 11.5 | 9.3 | 4.3 | 8.1 | 8 |
| reverse(users[\*].age) | 11.1 | 9.4 | 4.3 | 8.0 | 8 |
| sum(users[\*].age) | 11.3 | 9.2 | 8.4 | 4.2 | 8 |
| sort_by[users, &age](\*).name | 9.1 | 7.2 | 6.3 | 5.8 | 7 |
| max_by(users, &age) | 9.9 | 7.7 | 4.0 | 6.9 | 7 |
| users[\*].name | 6.2 | 5.0 | 4.8 | 4.4 | 5 |
| users[\*].age == `30` | 5.7 | 3.9 | 3.7 | 5.2 | 4 |

<!-- END_BENCHMARK_RESULTS -->