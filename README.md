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
| query | 50 | 200 | 800 | average_speedup |
|---|---|---|---|---|
| map(&length(@), users[\*].name) | 40.23 | 39.04 | 39.42 | 39.56 |
| ends_with(`"hello"`, `"lo"`) | 36.0 | 35.5 | 35.5 | 35.67 |
| map(&abs(@), products[\*].price) | 30.67 | 35.07 | 32.63 | 32.79 |
| map(&floor(@), products[\*].price) | 30.91 | 31.78 | 31.0 | 31.23 |
| starts_with(`"hello"`, `"he"`) | 36.0 | 36.0 | 21.0 | 31.0 |
| not_null(`null`, `"a"`, `"b"`) | 30.75 | 30.25 | 30.25 | 30.42 |
| users \| length(@) | 28.67 | 28.67 | 28.67 | 28.67 |
| ((users[0].age > `1` && !(users[0].age == `5`)) \|\| `0`) | 25.75 | 34.33 | 25.5 | 28.53 |
| map(&ceil(@), products[\*].price) | 33.97 | 19.57 | 30.76 | 28.1 |
| to_number(`"42"`) | 24.5 | 28.0 | 25.0 | 25.83 |
| users[?(age >= `30` && active == `true`)].name | 24.44 | 24.82 | 24.14 | 24.47 |
| users[1:10:2] | 24.5 | 24.25 | 24.5 | 24.42 |
| contains(products[0].tags, `"electronics"`) | 23.2 | 23.0 | 23.0 | 23.07 |
| sort(users[?((age > `40` && active == `true`) && contains(category, `"VIP"`))].name) | 14.53 | 26.46 | 26.05 | 22.35 |
| to_array(users[0]) | 17.5 | 24.0 | 24.0 | 21.83 |
| users[0].age == `30` | 22.67 | 21.33 | 20.2 | 21.4 |
| length(users) | 21.0 | 21.33 | 21.0 | 21.11 |
| keys(users[0]) | 20.25 | 20.5 | 20.0 | 20.25 |
| users[0].active == `true` | 23.0 | 18.67 | 18.4 | 20.02 |
| type(users[0]) | 24.0 | 18.0 | 18.0 | 20.0 |
| values(users[0]) | 20.5 | 15.67 | 20.25 | 18.81 |
| join(`", "`, products[0].tags) | 18.29 | 18.29 | 18.29 | 18.29 |
| merge(users[0], products[0]) | 17.75 | 17.915 | 14.165 | 16.61 |
| users[0].name | 15.75 | 15.75 | 15.75 | 15.75 |
| [users[0], products[0]] | 15.4 | 15.33 | 15.4 | 15.38 |
| users.\*.address | 19.5 | 13.33 | 12.83 | 15.22 |
| users[0].address.city | 14.4 | 14.2 | 15.78 | 14.79 |
| products[\*].tags[] | 14.68 | 14.15 | 14.8 | 14.54 |
| sum(products[\*].price) | 11.3 | 11.35 | 10.83 | 11.16 |
| sort(products[\*].tags[]) | 12.69 | 7.17 | 12.72 | 10.86 |
| min(products[\*].price) | 9.93 | 9.64 | 9.57 | 9.71 |
| max(products[\*].price) | 9.54 | 9.79 | 9.64 | 9.66 |
| {"names": users[\*].name, "count": length(users)} | 10.33 | 9.02 | 8.16 | 9.17 |
| reverse(products[\*].price) | 9.08 | 9.08 | 8.69 | 8.95 |
| users[\*].name | 9.2 | 8.64 | 7.7 | 8.51 |
| min_by(users, &age) | 8.59 | 8.41 | 7.55 | 8.18 |
| avg(products[\*].price) | 5.92 | 5.5 | 10.08 | 7.17 |
| sort_by(users, &age)[\*].name | 4.22 | 6.72 | 6.38 | 5.77 |
| max_by(products, &price) | 4.09 | 4.16 | 6.07 | 4.77 |
| to_string(users[0]) | 1.92 | 1.87 | 1.94 | 1.91 |

<!-- END_BENCHMARK_RESULTS -->