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
| query | 10 | 50 | 250 | 500 | average_speedup |
|---|---|---|---|---|---|
| sales[][] \| map(&join(`, `, sort(keys(@))), @) | 4.4 | 5.1 | 4.2 | 3.6 | 4 |
| users[\*].contains(nested_scores[], `50`) | 1.2 | 1.2 | 1.2 | 0.9 | 1 |
| users[\*].name | 0.1 | 0.1 | 0.0 | 0.0 | 0 |
| users[\*].address | 0.1 | 0.1 | 0.1 | 0.0 | 0 |
| users \| length(@) | 0.1 | 0.0 | 0.0 | 0.0 | 0 |
| users[?(age >= `30` && active == `true`)].name | 0.6 | 0.6 | 0.5 | 0.4 | 0 |
| length(users) | 0.1 | 0.0 | 0.0 | 0.0 | 0 |
| map(&length(@), users[\*].name) | 0.3 | 0.5 | 0.5 | 0.4 | 0 |
| min_by(users, &age) | 0.2 | 0.1 | 0.1 | 0.1 | 0 |
| sort_by(users, &age)[\*].name | 0.3 | 0.1 | 0.2 | 0.1 | 0 |
| sort(users[?((age > `40` && active == `true`) && contains(category, `"VIP"`))].name) | 0.8 | 0.8 | 0.5 | 0.6 | 0 |
| users[\*].category[] | 0.3 | 0.2 | 0.2 | 0.2 | 0 |
| users[\*].nested_scores[] | 0.3 | 0.3 | 0.3 | 0.1 | 0 |
| users[\*].nested_scores[][] | 1.0 | 0.5 | 0.9 | 0.7 | 0 |
| max_by(users, &age) | 0.2 | 0.2 | 0.1 | 0.1 | 0 |
| sort(users[\*].nested_scores[][]) | 1.1 | 1.0 | 1.0 | 0.8 | 0 |
| map(&abs(@), users[\*].age) | 0.5 | 0.5 | 0.4 | 0.3 | 0 |
| avg(users[\*].age) | 0.1 | 0.1 | 0.1 | 0.1 | 0 |
| map(&ceil(@), users[\*].age) | 0.3 | 0.5 | 0.5 | 0.4 | 0 |
| map(&floor(@), users[\*].age) | 0.3 | 0.5 | 0.5 | 0.3 | 0 |
| max(users[\*].age) | 0.2 | 0.1 | 0.1 | 0.1 | 0 |
| min(users[\*].age) | 0.2 | 0.1 | 0.1 | 0.1 | 0 |
| reverse(users[\*].age) | 0.1 | 0.1 | 0.1 | 0.1 | 0 |
| sum(users[\*].age) | 0.1 | 0.1 | 0.1 | 0.1 | 0 |
| users[\*].address.city | 0.1 | 0.2 | 0.2 | 0.1 | 0 |
| length(users[\*].name) | 0.1 | 0.1 | 0.0 | 0.1 | 0 |
| users[\*].age == `30` | 0.1 | 0.1 | 0.0 | 0.0 | 0 |
| users[\*] \| map(&(age > `1` && !(age == `5`) \|\| age == `0`), @) | 0.6 | 1.0 | 0.6 | 0.7 | 0 |
| sort(users[\*].keys(@)[]) | 0.9 | 0.9 | 0.8 | 0.6 | 0 |
| sort(users[\*].address.values(@)[]) | 0.4 | 0.4 | 0.6 | 0.5 | 0 |
| users[\*].merge(@, `{"extra_field":1}`) | 0.5 | 0.4 | 0.4 | 0.3 | 0 |
| users[\*].ends_with(name, `"s"`) | 0.3 | 0.3 | 0.5 | 0.3 | 0 |
| users[\*].starts_with(name, `"A"`) | 0.5 | 0.3 | 0.5 | 0.4 | 0 |
| users[\*].not_null(MISSING, name) | 0.3 | 0.2 | 0.3 | 0.1 | 0 |
| abs(sum(users[\*].age)) | 0.2 | 0.1 | 0.1 | 0.1 | 0 |

<!-- END_BENCHMARK_RESULTS -->