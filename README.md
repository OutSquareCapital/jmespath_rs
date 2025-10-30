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
| and-or-not-logic | 14.93 | 14.84 | 11.14 |
| boolean-comparison-eq | 14.67 | 14.13 | 10.56 |
| field.subfield.index.field | 4.76 | 4.84 | 5.66 |
| filter-age-and-active | 20.77 | 21.43 | 19.7 |
| flatten-nested-list | 15.0 | 13.0 | 11.2 |
| keys-of-object | 14.75 | 11.4 | 11.9 |
| length-of-array | 12.2 | 10.57 | 9.71 |
| list-projection | 4.69 | 4.74 | 4.18 |
| map-string-lengths | 20.5 | 36.01 | 25.03 |
| max_by-price | 1.98 | 3.36 | 2.42 |
| min_by-age | 4.77 | 7.12 | 5.6 |
| multiselect-dict | 4.6 | 8.14 | 3.95 |
| multiselect-list | 8.7 | 6.93 | 5.76 |
| numeric-comparison-eq | 12.67 | 12.18 | 8.07 |
| object-projection | 9.56 | 8.17 | 8.05 |
| pipe-to-length | 14.0 | 11.64 | 11.45 |
| simple-field-access | 6.32 | 7.44 | 6.64 |
| slice-array | 7.78 | 7.27 | 9.31 |
| sort | 2.41 | 2.61 | 2.17 |
| sort-by-age | 3.32 | 6.18 | 3.45 |
| to_array | 8.19 | 17.73 | 8.37 |
| to_number | 13.0 | 12.5 | 9.86 |
| to_string | 0.83 | 0.76 | 0.66 |
| values-of-object | 12.0 | 10.9 | 10.5 |

<!-- END_BENCHMARK_RESULTS -->