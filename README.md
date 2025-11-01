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
| map(&length(@), users[\*].name) | 41.57 | 43.38 | 39.09 | 41.35 |
| ends_with(users[0].name, `"s"`) | 30.25 | 30.5 | 30.5 | 30.42 |
| starts_with(users[0].name, `"A"`) | 30.5 | 30.25 | 30.5 | 30.42 |
| map(&ceil(@), users[\*].age) | 32.22 | 28.72 | 27.79 | 29.58 |
| ((users[0].age > `1` && !(users[0].age == `5`)) \|\| `0`) | 34.0 | 20.33 | 26.0 | 26.78 |
| users \| length(@) | 25.0 | 29.0 | 25.5 | 26.5 |
| users[?(age >= `30` && active == `true`)].name | 23.67 | 24.93 | 28.8 | 25.8 |
| map(&floor(@), users[\*].age) | 33.04 | 16.06 | 27.82 | 25.64 |
| map(&abs(@), users[\*].age) | 17.64 | 28.7 | 28.03 | 24.79 |
| sort(users[?((age > `40` && active == `true`) && contains(category, `"VIP"`))].name) | 14.94 | 24.88 | 30.08 | 23.3 |
| to_array(users[0]) | 21.0 | 23.67 | 24.0 | 22.89 |
| users[1:10:2] | 19.67 | 24.75 | 19.33 | 21.25 |
| not_null(users[0].MISSING, users[0].name) | 21.0 | 20.83 | 21.0 | 20.94 |
| contains(users[0].category, `"VIP"`) | 11.8 | 29.0 | 19.83 | 20.21 |
| length(users) | 21.0 | 18.0 | 21.33 | 20.11 |
| values(users[0]) | 21.0 | 20.25 | 16.0 | 19.08 |
| keys(users[0]) | 20.25 | 16.0 | 20.75 | 19.0 |
| users[0].age == `30` | 12.6 | 20.67 | 20.8 | 18.02 |
| join(`", "`, users[0].category) | 16.37 | 18.43 | 18.43 | 17.74 |
| [users[0], users[1]] | 15.8 | 15.67 | 20.25 | 17.24 |
| merge(users[0], users[-1]) | 18.0 | 15.43 | 18.0 | 17.14 |
| users[0].active == `true` | 13.75 | 18.67 | 18.9 | 17.11 |
| type(users[0]) | 7.93 | 20.57 | 21.5 | 16.67 |
| users[0].name | 12.67 | 16.0 | 12.67 | 13.78 |
| users[0].address.city | 10.75 | 14.6 | 14.33 | 13.23 |
| users.\*.address | 12.0 | 13.0 | 14.5 | 13.17 |
| sort(users[\*].category[]) | 12.91 | 12.04 | 10.93 | 11.96 |
| users[\*].category[] | 9.32 | 9.47 | 16.75 | 11.85 |
| min_by(users, &age) | 9.65 | 8.27 | 8.36 | 8.76 |
| max_by(users, &age) | 8.45 | 9.24 | 8.01 | 8.57 |
| {"names": users[\*].name, "count": length(users)} | 9.9 | 4.9 | 8.25 | 7.68 |
| min(users[\*].age) | 9.46 | 4.7 | 8.4 | 7.52 |
| avg(users[\*].age) | 9.22 | 4.49 | 7.94 | 7.22 |
| sort_by(users, &age)[\*].name | 7.77 | 7.04 | 6.33 | 7.05 |
| sum(users[\*].age) | 8.83 | 4.37 | 7.84 | 7.01 |
| users[\*].name | 4.45 | 8.97 | 7.57 | 7.0 |
| reverse(users[\*].age) | 8.9 | 3.97 | 6.7 | 6.52 |
| max(users[\*].age) | 5.23 | 4.9 | 8.36 | 6.16 |
| to_number(to_string(users[-1].age)) | 3.42 | 3.37 | 3.32 | 3.37 |
| to_string(users[0]) | 2.02 | 1.97 | 1.34 | 1.78 |

<!-- END_BENCHMARK_RESULTS -->