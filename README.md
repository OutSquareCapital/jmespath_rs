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

Each column besides query represent the data size.

200 runs are computed for each test.

Each value is the median Rust time divided by the median python time (median across runs)

A value of 2 meaning that Rust is twice as fast, a value of 0.5 meaning that Rust is half as fast.

<!-- BENCHMARK_RESULTS -->
| query | 50 | 200 | 800 |
|---|---|---|---|
| (users[0].age > `1` && !(users[0].age == `5`)) \|\| `0` | 30.14 | 20.5 | 26.25 |
| [users[0], products[0]] | 15.4 | 16.0 | 11.75 |
| avg(products[].price) | 13.09 | 13.09 | 13.09 |
| contains(products[0].tags, `"electronics"`) | 17.0 | 19.33 | 23.2 |
| ends_with(`"hello"`, `"lo"`) | 36.5 | 36.0 | 36.5 |
| join(`,`, products[0].tags) | 18.14 | 18.43 | 18.14 |
| keys(users[0]) | 20.25 | 15.67 | 20.5 |
| length(users) | 21.0 | 19.165 | 26.585 |
| map(&abs(@), products[].price) | 34.49 | 37.64 | 20.54 |
| map(&ceil(@), products[].price) | 34.13 | 17.91 | 18.44 |
| map(&floor(@), products[].price) | 33.97 | 36.24 | 34.92 |
| map(&length(@), users[].name) | 42.88 | 42.21 | 40.26 |
| max(products[].price) | 11.77 | 11.3 | 11.41 |
| max_by(products, &price) | 2.66 | 2.31 | 4.11 |
| merge(users[0], products[0]) | 17.83 | 17.67 | 11.67 |
| min(products[].price) | 11.3 | 11.3 | 11.81 |
| min_by(users, &age) | 7.94 | 9.24 | 7.84 |
| not_null(null, `"a"`, `"b"`) | 33.5 | 33.0 | 37.0 |
| products[].tags[] | 16.36 | 16.28 | 15.81 |
| reverse(products[].price) | 11.08 | 10.52 | 11.04 |
| sort(products[].tags[]) | 13.57 | 13.32 | 13.76 |
| sort_by(users, &age)[].name | 4.69 | 4.39 | 6.86 |
| starts_with(`"hello"`, `"he"`) | 20.5 | 36.0 | 36.0 |
| sum(products[].price) | 13.59 | 12.91 | 13.45 |
| to_array(users[0]) | 23.67 | 13.67 | 24.0 |
| to_number(`42`) | 24.0 | 13.5 | 24.5 |
| to_string(users[0]) | 1.94 | 1.76 | 1.95 |
| type(users[0]) | 10.25 | 18.0 | 18.0 |
| users[0].active == `true` | 18.2 | 18.33 | 23.25 |
| users[0].address.city | 11.0 | 18.0 | 14.8 |
| users[0].age == `30` | 25.25 | 13.56 | 34.0 |
| users[0].name | 15.0 | 15.75 | 16.0 |
| users[1:10:2] | 25.5 | 25.0 | 25.25 |
| users[?age >= `30` && active == `true`].name | 13.36 | 24.72 | 22.4 |
| users[].\*.address | 38.95 | 38.16 | 34.31 |
| users[].name | 11.05 | 5.08 | 5.05 |
| values(users[0]) | 20.0 | 15.33 | 20.5 |
| {names: users[].name, count: length(users)} | 11.96 | 5.58 | 10.03 |

<!-- END_BENCHMARK_RESULTS -->