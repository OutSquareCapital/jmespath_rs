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
| map(&length(@), users[\*].name) | 44.28 | 39.1 | 40.0 | 41.13 |
| ends_with(users[0].name, `"s"`) | 39.67 | 30.25 | 30.25 | 33.39 |
| starts_with(users[0].name, `"A"`) | 30.25 | 30.0 | 30.25 | 30.17 |
| map(&ceil(@), users[\*].age) | 32.25 | 28.39 | 27.77 | 29.47 |
| map(&floor(@), users[\*].age) | 29.41 | 28.28 | 29.11 | 28.93 |
| map(&abs(@), users[\*].age) | 29.21 | 28.46 | 27.92 | 28.53 |
| users \| length(@) | 25.0 | 24.5 | 29.33 | 26.28 |
| sort(users[?((age > `40` && active == `true`) && contains(category, `"VIP"`))].name) | 24.56 | 27.43 | 25.47 | 25.82 |
| users[?(age >= `30` && active == `true`)].name | 17.46 | 31.86 | 24.54 | 24.62 |
| ((users[0].age > `1` && !(users[0].age == `5`)) \|\| `0`) | 20.67 | 20.33 | 29.71 | 23.57 |
| users[0].age == `30` | 20.33 | 25.5 | 20.6 | 22.14 |
| not_null(users[0].MISSING, users[0].name) | 20.5 | 20.67 | 20.67 | 20.61 |
| contains(users[0].category, `"VIP"`) | 19.33 | 23.2 | 17.75 | 20.09 |
| to_array(users[0]) | 17.5 | 20.5 | 21.14 | 19.71 |
| values(users[0]) | 20.5 | 16.0 | 20.75 | 19.08 |
| length(users) | 18.0 | 17.5 | 21.33 | 18.94 |
| [users[0], users[1]] | 15.6 | 19.5 | 20.5 | 18.53 |
| keys(users[0]) | 16.0 | 16.0 | 23.43 | 18.48 |
| users[0].active == `true` | 13.75 | 18.4 | 23.0 | 18.38 |
| users[\*].category[] | 18.84 | 17.44 | 17.67 | 17.98 |
| join(`", "`, users[0].category) | 16.38 | 18.43 | 18.43 | 17.75 |
| users[1:10:2] | 8.79 | 19.33 | 24.75 | 17.62 |
| type(users[0]) | 18.5 | 14.0 | 18.25 | 16.92 |
| merge(users[0], users[-1]) | 17.67 | 17.83 | 13.2 | 16.23 |
| users[0].name | 18.33 | 12.33 | 16.0 | 15.55 |
| users[0].address.city | 14.62 | 13.67 | 18.0 | 15.43 |
| users.\*.address | 12.0 | 12.0 | 13.0 | 12.33 |
| sort(users[\*].category[]) | 7.63 | 12.03 | 11.3 | 10.32 |
| reverse(users[\*].age) | 10.18 | 8.85 | 8.62 | 9.22 |
| {"names": users[\*].name, "count": length(users)} | 10.21 | 8.92 | 8.51 | 9.21 |
| min(users[\*].age) | 9.68 | 8.64 | 8.25 | 8.86 |
| sum(users[\*].age) | 8.69 | 8.1 | 7.82 | 8.2 |
| max(users[\*].age) | 5.42 | 8.7 | 8.25 | 7.46 |
| users[\*].name | 4.79 | 8.38 | 8.37 | 7.18 |
| min_by(users, &age) | 4.92 | 8.23 | 8.06 | 7.07 |
| avg(users[\*].age) | 4.98 | 7.96 | 7.74 | 6.89 |
| sort_by(users, &age)[\*].name | 8.4 | 4.21 | 6.36 | 6.32 |
| max_by(users, &age) | 4.84 | 5.13 | 7.75 | 5.91 |
| to_number(to_string(users[-1].age)) | 2.72 | 3.13 | 2.81 | 2.89 |
| to_string(users[0]) | 1.91 | 1.63 | 1.89 | 1.81 |

<!-- END_BENCHMARK_RESULTS -->