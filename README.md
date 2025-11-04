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
| users[\*].ends_with(name, `"s"`) | 41.1 | 46.2 | 37.1 | 48.2 | 43 |
| users[\*].starts_with(name, `"A"`) | 40.8 | 27.3 | 46.8 | 41.4 | 39 |
| map(&length(@), users[\*].name) | 38.9 | 26.0 | 41.7 | 37.0 | 35 |
| users[\*].keys(@) | 24.0 | 42.8 | 38.0 | 35.8 | 35 |
| users[\*].nested_scores[][] | 35.1 | 34.7 | 33.3 | 33.0 | 34 |
| map(&floor(@), users[\*].age) | 31.2 | 22.7 | 53.0 | 27.5 | 33 |
| map(&ceil(@), users[\*].age) | 22.6 | 43.7 | 20.6 | 39.4 | 31 |
| users[\*].values(@) | 33.8 | 37.0 | 28.5 | 19.9 | 29 |
| sales[][] \| map(&join(`, `, keys(@)), @) | 29.7 | 37.3 | 25.9 | 23.7 | 29 |
| users[\*] \| map(&(age > `1` && !(age == `5`) \|\| age == `0`), @) | 25.4 | 19.7 | 35.8 | 33.1 | 28 |
| users \| length(@) | 29.8 | 29.0 | 28.3 | 24.0 | 27 |
| map(&abs(@), users[\*].age) | 30.5 | 16.1 | 38.3 | 26.8 | 27 |
| users[\*].contains(nested_scores[], `50`) | 28.8 | 23.7 | 28.8 | 23.3 | 26 |
| length(users) | 21.7 | 21.7 | 20.7 | 20.7 | 21 |
| sort(users[?((age > `40` && active == `true`) && contains(category, `"VIP"`))].name) | 13.5 | 21.4 | 27.9 | 21.9 | 21 |
| users[\*].not_null(MISSING, name) | 20.0 | 21.3 | 18.6 | 16.2 | 19 |
| users[\*].merge(@, `{"extra_field":1}`) | 15.7 | 15.5 | 23.0 | 20.0 | 18 |
| users[?(age >= `30` && active == `true`)].name | 21.4 | 15.7 | 13.9 | 18.6 | 17 |
| users[\*].category[] | 19.4 | 20.2 | 9.9 | 18.0 | 16 |
| sort(users[\*].nested_scores[][]) | 25.3 | 17.9 | 7.9 | 12.2 | 15 |
| users[\*].nested_scores[] | 12.5 | 10.9 | 18.4 | 17.4 | 14 |
| max(users[\*].age) | 18.9 | 8.6 | 7.3 | 6.0 | 10 |
| users[\*].address.city | 12.5 | 11.9 | 7.0 | 12.3 | 10 |
| length(users[\*].name) | 12.6 | 9.8 | 8.3 | 8.1 | 9 |
| max_by(users, &age) | 10.5 | 8.2 | 7.7 | 6.3 | 8 |
| avg(users[\*].age) | 8.4 | 5.7 | 9.5 | 8.4 | 8 |
| min(users[\*].age) | 13.5 | 9.2 | 7.2 | 4.9 | 8 |
| abs(sum(users[\*].age)) | 9.6 | 10.1 | 8.9 | 4.5 | 8 |
| users[\*].address | 9.2 | 7.9 | 4.1 | 4.0 | 6 |
| min_by(users, &age) | 7.2 | 5.6 | 7.9 | 5.3 | 6 |
| sort_by(users, &age)[\*].name | 6.6 | 4.6 | 7.3 | 6.2 | 6 |
| reverse(users[\*].age) | 7.3 | 5.6 | 9.3 | 4.4 | 6 |
| users[\*].age == `30` | 9.1 | 6.4 | 3.6 | 5.5 | 6 |
| users[\*].name | 8.5 | 5.7 | 5.0 | 4.1 | 5 |
| sum(users[\*].age) | 7.2 | 5.5 | 4.6 | 4.5 | 5 |

<!-- END_BENCHMARK_RESULTS -->