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
| map(&length(@), users[\*].name) | 39.5 | 42.7 | 38.6 | 40.3 |
| users[\*].type(@) | 25.4 | 43.3 | 45.5 | 38.1 |
| users[\*].starts_with(name, `"A"`) | 32.1 | 42.6 | 31.7 | 35.5 |
| users[\*].to_array(@) | 38.8 | 31.1 | 31.4 | 33.8 |
| users[\*].keys(@) | 42.0 | 38.1 | 18.6 | 32.9 |
| users[\*].ends_with(name, `"s"`) | 32.4 | 32.6 | 31.1 | 32.0 |
| users[\*].values(@) | 36.1 | 32.6 | 27.1 | 31.9 |
| map(&abs(@), users[\*].age) | 29.3 | 28.7 | 27.7 | 28.6 |
| map(&floor(@), users[\*].age) | 29.6 | 28.5 | 27.3 | 28.5 |
| [users[0:10], users[-10:]] | 34.6 | 22.3 | 23.0 | 26.6 |
| users \| length(@) | 25.5 | 24.5 | 28.7 | 26.2 |
| sort(users[?((age > `40` && active == `true`) && contains(category, `"VIP"`))].name) | 29.5 | 23.1 | 23.4 | 25.3 |
| map(&ceil(@), users[\*].age) | 16.9 | 28.7 | 27.4 | 24.3 |
| users[\*].merge(@, `{"extra_field":1}`) | 28.9 | 14.2 | 23.8 | 22.3 |
| users[?(age >= `30` && active == `true`)].name | 20.3 | 19.8 | 20.5 | 20.2 |
| length(users) | 21.0 | 17.0 | 21.7 | 19.9 |
| users[\*].category[] | 18.9 | 18.2 | 17.0 | 18.0 |
| users[\*].contains(category, `"VIP"`) | 16.7 | 18.8 | 16.1 | 17.2 |
| users[\*].not_null(MISSING, name) | 10.7 | 17.2 | 17.5 | 15.1 |
| users.\*.address | 13.0 | 12.0 | 13.7 | 12.9 |
| users[\*].join(`", "`, category) | 12.7 | 12.9 | 12.8 | 12.8 |
| sort(users[\*].category[]) | 13.6 | 12.2 | 11.1 | 12.3 |
| {"names": users[\*].name, "count": length(users)} | 10.1 | 9.0 | 8.0 | 9.0 |
| users[\*].age == `30` | 9.3 | 9.1 | 8.7 | 9.0 |
| users[\*].address.city | 10.6 | 5.7 | 9.8 | 8.7 |
| max_by(users, &age) | 9.4 | 8.0 | 7.7 | 8.4 |
| min_by(users, &age) | 8.5 | 8.1 | 7.7 | 8.1 |
| avg(users[\*].age) | 8.7 | 7.8 | 7.4 | 8.0 |
| length(users[\*].name) | 10.0 | 4.6 | 8.4 | 7.7 |
| max(users[\*].age) | 9.6 | 4.8 | 8.3 | 7.6 |
| users[\*].active == `true` | 9.7 | 4.5 | 8.1 | 7.4 |
| ((users[\*].age > `1` && !(users[\*].age == `5`)) \|\| `0`) | 8.3 | 5.0 | 8.9 | 7.4 |
| sum(users[\*].age) | 9.0 | 4.4 | 7.9 | 7.1 |
| users[\*].name | 4.8 | 8.5 | 4.7 | 6.0 |
| sort_by(users, &age)[\*].name | 4.5 | 6.9 | 6.3 | 5.9 |
| reverse(users[\*].age) | 5.3 | 7.5 | 4.9 | 5.9 |
| min(users[\*].age) | 5.3 | 4.9 | 4.5 | 4.9 |
| users[\*].to_number(to_string(age)) | 2.0 | 2.8 | 2.0 | 2.3 |
| users[\*].to_string(@) | 1.3 | 1.3 | 1.3 | 1.3 |

<!-- END_BENCHMARK_RESULTS -->