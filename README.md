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
| users[\*].type(@) | 25.9 | 42.7 | 45.0 | 37.9 |
| users[\*].to_array(@) | 42.6 | 34.4 | 32.1 | 36.4 |
| users[\*].keys(@) | 24.6 | 37.5 | 34.2 | 32.1 |
| users[\*].ends_with(name, `"s"`) | 32.4 | 32.0 | 30.0 | 31.5 |
| [users[0:10], users[-10:]] | 22.8 | 35.4 | 35.1 | 31.1 |
| users[\*].starts_with(name, `"A"`) | 21.9 | 37.8 | 31.7 | 30.5 |
| map(&length(@), users[\*].name) | 22.5 | 23.0 | 38.9 | 28.1 |
| users \| length(@) | 29.0 | 29.0 | 25.5 | 27.8 |
| users[\*].values(@) | 24.6 | 30.9 | 26.8 | 27.4 |
| map(&floor(@), users[\*].age) | 17.1 | 29.1 | 26.6 | 24.3 |
| map(&abs(@), users[\*].age) | 17.0 | 27.9 | 27.8 | 24.2 |
| users[\*].merge(@, `{"extra_field":1}`) | 18.6 | 25.7 | 24.1 | 22.8 |
| length(users) | 17.5 | 21.0 | 21.3 | 19.9 |
| map(&ceil(@), users[\*].age) | 15.6 | 16.9 | 27.0 | 19.8 |
| users[?(age >= `30` && active == `true`)].name | 11.9 | 21.1 | 25.3 | 19.4 |
| users[\*].not_null(MISSING, name) | 17.1 | 20.4 | 16.6 | 18.0 |
| users[\*].category[] | 17.6 | 18.6 | 17.2 | 17.8 |
| sort(users[?((age > `40` && active == `true`) && contains(category, `"VIP"`))].name) | 16.4 | 18.1 | 15.6 | 16.7 |
| users[\*].contains(category, `"VIP"`) | 17.4 | 16.1 | 15.6 | 16.4 |
| users.\*.address | 13.7 | 14.0 | 13.5 | 13.7 |
| users[\*].join(`", "`, category) | 7.0 | 13.1 | 12.5 | 10.9 |
| sort(users[\*].category[]) | 7.3 | 11.9 | 11.1 | 10.1 |
| reverse(users[\*].age) | 10.1 | 9.0 | 8.7 | 9.3 |
| users[\*].address.city | 7.0 | 10.0 | 9.7 | 8.9 |
| min(users[\*].age) | 9.4 | 8.6 | 8.4 | 8.8 |
| ((users[\*].age > `1` && !(users[\*].age == `5`)) \|\| `0`) | 6.0 | 9.6 | 8.5 | 8.0 |
| {"names": users[\*].name, "count": length(users)} | 5.8 | 9.0 | 8.5 | 7.8 |
| max(users[\*].age) | 9.7 | 4.7 | 8.3 | 7.6 |
| users[\*].age == `30` | 5.2 | 9.0 | 8.7 | 7.6 |
| max_by(users, &age) | 5.2 | 8.2 | 7.8 | 7.1 |
| avg(users[\*].age) | 8.4 | 4.2 | 7.4 | 6.7 |
| users[\*].active == `true` | 5.3 | 5.3 | 8.5 | 6.4 |
| length(users[\*].name) | 5.6 | 4.7 | 8.5 | 6.3 |
| sum(users[\*].age) | 4.8 | 4.2 | 7.7 | 5.6 |
| min_by(users, &age) | 4.7 | 4.5 | 6.0 | 5.1 |
| sort_by(users, &age)[\*].name | 4.3 | 4.0 | 6.3 | 4.9 |
| users[\*].name | 4.8 | 4.5 | 5.1 | 4.8 |
| users[\*].to_number(to_string(age)) | 2.1 | 2.0 | 2.0 | 2.0 |
| users[\*].to_string(@) | 0.9 | 1.3 | 1.3 | 1.2 |

<!-- END_BENCHMARK_RESULTS -->