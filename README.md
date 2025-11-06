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
| users[\*].ends_with(name, `"s"`) | 37.5 | 41.1 | 40.3 | 38.6 | 39 |
| users[\*].starts_with(name, `"A"`) | 37.4 | 42.0 | 39.4 | 40.2 | 39 |
| users[\*] \| map(&(age > `1` && !(age == `5`) \|\| age == `0`), @) | 33.0 | 31.7 | 31.6 | 30.0 | 31 |
| map(&length(@), users[\*].name) | 22.2 | 22.7 | 39.0 | 37.6 | 30 |
| users[\*].nested_scores[][] | 36.1 | 17.4 | 34.0 | 35.3 | 30 |
| map(&abs(@), users[\*].age) | 29.2 | 31.1 | 28.7 | 27.7 | 29 |
| map(&floor(@), users[\*].age) | 30.2 | 30.5 | 28.9 | 27.6 | 29 |
| users \| length(@) | 30.0 | 28.7 | 28.3 | 28.0 | 28 |
| users[\*].keys(@) | 19.6 | 22.9 | 37.3 | 32.3 | 28 |
| users[\*].contains(nested_scores[], `50`) | 27.1 | 31.9 | 30.4 | 26.3 | 28 |
| map(&ceil(@), users[\*].age) | 20.9 | 20.2 | 29.2 | 27.7 | 24 |
| users[\*].merge(@, `{"extra_field":1}`) | 25.8 | 26.3 | 23.0 | 22.6 | 24 |
| sales[][] \| map(&join(`,`, keys(@)), @) | 15.9 | 28.9 | 25.6 | 24.9 | 23 |
| sort(users[?((age > `40` && active == `true`) && contains(category, `"VIP"`))].name) | 16.4 | 20.2 | 27.8 | 26.9 | 22 |
| users[\*].values(@) | 18.1 | 19.1 | 27.7 | 25.3 | 22 |
| users[\*].not_null(MISSING, name) | 17.6 | 19.0 | 21.3 | 19.5 | 19 |
| users[?(age >= `30` && active == `true`)].name | 20.4 | 11.5 | 21.8 | 19.5 | 18 |
| length(users) | 19.5 | 18.0 | 17.5 | 17.5 | 18 |
| users[\*].category[] | 18.5 | 19.2 | 18.9 | 17.1 | 18 |
| users[\*].nested_scores[] | 19.7 | 19.1 | 17.9 | 16.9 | 18 |
| sort(users[\*].nested_scores[][]) | 23.7 | 17.9 | 12.6 | 12.3 | 16 |
| min(users[\*].age) | 12.0 | 10.0 | 8.8 | 8.3 | 9 |
| sum(users[\*].age) | 11.3 | 9.1 | 8.6 | 7.9 | 9 |
| users[\*].address.city | 11.9 | 6.6 | 9.2 | 11.0 | 9 |
| abs(sum(users[\*].age)) | 13.9 | 9.4 | 8.6 | 7.9 | 9 |
| min_by(users, &age) | 10.3 | 8.1 | 7.5 | 7.4 | 8 |
| max_by(users, &age) | 6.7 | 9.1 | 8.9 | 7.3 | 8 |
| avg(users[\*].age) | 11.3 | 5.0 | 8.9 | 7.9 | 8 |
| max(users[\*].age) | 12.8 | 9.8 | 5.0 | 8.3 | 8 |
| reverse(users[\*].age) | 11.1 | 9.0 | 4.3 | 7.7 | 8 |
| length(users[\*].name) | 11.8 | 8.8 | 7.8 | 7.3 | 8 |
| sort_by[users, &age](\*).name | 9.5 | 7.2 | 4.0 | 4.4 | 6 |
| users[\*].name | 7.8 | 4.3 | 4.1 | 7.3 | 5 |
| users[\*].address | 5.1 | 4.5 | 4.2 | 7.2 | 5 |
| users[\*].age == `30` | 6.0 | 3.2 | 6.4 | 5.0 | 5 |

<!-- END_BENCHMARK_RESULTS -->