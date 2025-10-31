# jmespath_rs

A simple package to test Rust interaction with dictionaries.

Meant to be a new version of querydict which hopefully can be more performant

## Developpement

```bash
maturin build --release
uv pip install -e .
uv run  -m tests.bench
```

## Benchmark Results

Each value is the median Rust time divided by the median python time (median across runs)

A value of 2 meaning that Rust is twice as fast, a value of 0.5 meaning that Rust is half as fast.

<!-- BENCHMARK_RESULTS -->
| case_name | 500 | 2000 | 8000 |
|---|---|---|---|
| abs-of-number | 35.06 | 32.63 | 32.19 |
| and-or-not-logic | 24.22 | 18.07 | 24.33 |
| avg-of-numbers | 16.89 | 15.0 | 15.0 |
| boolean-comparison-eq | 19.0 | 19.33 | 19.0 |
| ceil-of-numbers | 12.46 | 19.25 | 37.57 |
| contains-in-array | 18.4 | 13.0 | 18.2 |
| ends-with-string | 20.0 | 20.0 | 24.33 |
| field.subfield.index.field | 16.0 | 11.75 | 16.0 |
| filter-age-and-active | 16.48 | 20.43 | 20.11 |
| flatten-nested-list | 30.6 | 17.2 | 30.0 |
| floor-of-numbers | 26.6 | 25.0 | 37.86 |
| join-strings | 9.14 | 8.86 | 11.78 |
| keys-of-object | 19.5 | 14.67 | 19.0 |
| length-of-array | 16.25 | 12.0 | 16.0 |
| list-projection | 10.18 | 9.55 | 9.84 |
| map-string-lengths | 40.92 | 37.53 | 36.62 |
| max-of-numbers | 16.25 | 11.2 | 16.0 |
| max_by-price | 3.95 | 3.98 | 2.64 |
| merge-objects | 11.4 | 11.4 | 16.33 |
| min-of-numbers | 19.8 | 11.2 | 15.83 |
| min_by-age | 6.87 | 4.67 | 7.5 |
| multiselect-dict | 5.48 | 9.82 | 9.98 |
| multiselect-list | 11.5 | 13.25 | 16.8 |
| not-null-values | 28.57 | 19.0 | 22.0 |
| numeric-comparison-eq | 19.67 | 19.67 | 19.4 |
| object-projection | 15.33 | 21.0 | 24.5 |
| pipe-to-length | 8.75 | 12.33 | 16.0 |
| reverse-array | 14.17 | 12.0 | 16.4 |
| simple-field-access | 12.6 | 13.0 | 12.8 |
| slice-array | 19.6 | 20.0 | 19.8 |
| sort | 12.38 | 10.3 | 10.3 |
| sort-by-age | 7.27 | 6.68 | 5.98 |
| starts-with-string | 20.0 | 20.0 | 23.83 |
| sum-of-numbers | 15.17 | 12.5 | 14.67 |
| to_array | 7.5 | 16.75 | 16.75 |
| to_number | 18.33 | 17.0 | 17.0 |
| to_string | 2.66 | 2.02 | 2.0 |
| type-of-value | 17.25 | 13.0 | 16.75 |
| values-of-object | 20.25 | 19.0 | 19.0 |

<!-- END_BENCHMARK_RESULTS -->