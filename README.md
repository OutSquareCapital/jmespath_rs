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
| users[::2] | 24.8 | 64.0 | 182.9 | 90.6 |
| users[\*].type(@) | 42.4 | 42.4 | 45.0 | 43.3 |
| [users[0:10], users[-10:]] | 39.5 | 40.5 | 34.7 | 38.2 |
| users[\*].ends_with(name, `"s"`) | 42.8 | 32.5 | 31.5 | 35.6 |
| users[\*].starts_with(name, `"A"`) | 32.4 | 41.8 | 31.2 | 35.1 |
| users[\*].to_array(@) | 37.7 | 34.0 | 33.0 | 34.9 |
| map(&length(@), users[\*].name) | 24.7 | 38.7 | 39.3 | 34.2 |
| users[\*].values(@) | 35.9 | 31.3 | 27.5 | 31.6 |
| map(&floor(@), users[\*].age) | 32.1 | 28.5 | 28.5 | 29.7 |
| map(&abs(@), users[\*].age) | 29.7 | 29.2 | 27.6 | 28.8 |
| map(&ceil(@), users[\*].age) | 29.8 | 27.7 | 26.7 | 28.1 |
| users \| length(@) | 29.7 | 25.5 | 25.5 | 26.9 |
| users[\*].keys(@) | 22.7 | 19.9 | 35.1 | 25.9 |
| users[\*].merge(@, `{"extra_field":1}`) | 28.1 | 25.2 | 24.0 | 25.8 |
| sort(users[?((age > `40` && active == `true`) && contains(category, `"VIP"`))].name) | 28.5 | 23.5 | 23.3 | 25.1 |
| users[?(age >= `30` && active == `true`)].name | 11.5 | 29.2 | 27.7 | 22.8 |
| length(users) | 21.0 | 21.3 | 21.3 | 21.2 |
| users[\*].category[] | 17.8 | 16.9 | 17.6 | 17.4 |
| users[\*].contains(category, `"VIP"`) | 18.1 | 17.6 | 15.8 | 17.2 |
| users[\*].not_null(MISSING, name) | 16.8 | 16.9 | 17.1 | 16.9 |
| users.\*.address | 13.7 | 13.3 | 13.0 | 13.3 |
| users[\*].join(`", "`, category) | 13.6 | 13.3 | 12.9 | 13.3 |
| sort(users[\*].category[]) | 13.1 | 6.9 | 11.1 | 10.4 |
| ((users[\*].age > `1` && !(users[\*].age == `5`)) \|\| `0`) | 11.2 | 9.7 | 9.1 | 10.0 |
| users[\*].address.city | 10.8 | 8.3 | 10.6 | 9.9 |
| {"names": users[\*].name, "count": length(users)} | 10.6 | 9.5 | 8.8 | 9.6 |
| reverse(users[\*].age) | 10.3 | 9.4 | 8.7 | 9.5 |
| users[\*].age == `30` | 9.4 | 9.1 | 8.8 | 9.1 |
| sum(users[\*].age) | 9.2 | 8.3 | 7.9 | 8.5 |
| max_by(users, &age) | 8.8 | 8.2 | 7.9 | 8.3 |
| users[\*].name | 9.4 | 8.7 | 5.6 | 7.9 |
| users[\*].active == `true` | 10.0 | 4.8 | 8.4 | 7.7 |
| length(users[\*].name) | 5.6 | 8.8 | 8.4 | 7.6 |
| min(users[\*].age) | 9.9 | 4.8 | 4.5 | 6.4 |
| max(users[\*].age) | 5.4 | 8.9 | 4.6 | 6.3 |
| min_by(users, &age) | 4.9 | 5.0 | 8.2 | 6.0 |
| sort_by(users, &age)[\*].name | 4.7 | 4.1 | 6.5 | 5.1 |
| avg(users[\*].age) | 4.8 | 4.5 | 4.2 | 4.5 |
| users[\*].to_number(to_string(age)) | 2.0 | 2.0 | 2.0 | 2.0 |
| users[\*].to_string(@) | 1.3 | 1.3 | 1.3 | 1.3 |

<!-- END_BENCHMARK_RESULTS -->