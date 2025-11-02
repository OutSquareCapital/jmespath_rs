# dictexprs

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

Each value is the median Python time divided by the median Rust time (median across runs)

A value of 2 meaning that Rust is twice as fast, a value of 0.5 meaning that Rust is half as fast.

<!-- BENCHMARK_RESULTS -->
| query | 50 | 200 | 800 | average_speedup |
|---|---|---|---|---|
| users[\*].type(@) | 41.3 | 36.1 | 44.0 | 40.5 |
| map(&length(@), users[\*].name) | 43.2 | 36.6 | 40.7 | 40.2 |
| users[\*].to_array(@) | 37.0 | 32.2 | 30.3 | 33.2 |
| users[\*].keys(@) | 24.8 | 39.3 | 35.1 | 33.1 |
| users[\*].ends_with(name, `"s"`) | 30.8 | 31.3 | 29.1 | 30.4 |
| users[\*].values(@) | 34.1 | 29.4 | 26.7 | 30.1 |
| map(&floor(@), users[\*].age) | 28.3 | 30.3 | 29.1 | 29.2 |
| map(&abs(@), users[\*].age) | 31.6 | 27.8 | 27.9 | 29.1 |
| map(&ceil(@), users[\*].age) | 28.8 | 28.5 | 27.6 | 28.3 |
| users[\*].starts_with(name, `"A"`) | 21.4 | 31.6 | 30.9 | 28.0 |
| [users[0:10], users[-10:]] | 30.3 | 30.4 | 22.8 | 27.8 |
| users \| length(@) | 29.7 | 21.8 | 22.3 | 24.6 |
| sort(users[?((age > `40` && active == `true`) && contains(category, `"VIP"`))].name) | 22.7 | 24.0 | 22.9 | 23.2 |
| users[\*].merge(@, `{"extra_field":1}`) | 15.7 | 28.5 | 24.4 | 22.9 |
| users[?(age >= `30` && active == `true`)].name | 20.8 | 18.7 | 20.5 | 20.0 |
| length(users) | 17.5 | 21.3 | 12.0 | 16.9 |
| users[\*].not_null(MISSING, name) | 20.7 | 10.4 | 17.3 | 16.1 |
| users[\*].contains(category, `"VIP"`) | 8.6 | 19.6 | 16.8 | 15.0 |
| users[\*].category[] | 9.9 | 17.1 | 17.1 | 14.7 |
| users.\*.address | 15.0 | 13.3 | 14.0 | 14.1 |
| users[\*].join(`", "`, category) | 13.9 | 12.7 | 12.4 | 13.0 |
| sort(users[\*].category[]) | 13.7 | 11.7 | 11.0 | 12.1 |
| users[\*].address.city | 10.3 | 10.0 | 9.9 | 10.1 |
| ((users[\*].age > `1` && !(users[\*].age == `5`)) \|\| `0`) | 10.5 | 9.4 | 8.7 | 9.5 |
| length(users[\*].name) | 9.7 | 8.8 | 8.6 | 9.0 |
| users[\*].age == `30` | 9.0 | 8.9 | 8.5 | 8.8 |
| max(users[\*].age) | 9.0 | 8.4 | 8.3 | 8.6 |
| users[\*].name | 8.7 | 8.1 | 8.3 | 8.4 |
| min_by(users, &age) | 8.5 | 8.0 | 7.9 | 8.1 |
| sum(users[\*].age) | 8.5 | 8.0 | 7.8 | 8.1 |
| avg(users[\*].age) | 8.5 | 7.9 | 7.5 | 8.0 |
| reverse(users[\*].age) | 9.5 | 4.7 | 8.4 | 7.5 |
| users[\*].active == `true` | 9.3 | 4.5 | 8.1 | 7.3 |
| min(users[\*].age) | 9.2 | 4.5 | 8.0 | 7.2 |
| max_by(users, &age) | 4.8 | 8.0 | 7.9 | 6.9 |
| sort_by[users, &age](\*).name | 4.6 | 7.4 | 6.4 | 6.1 |
| {"names": users[\*].name, "count": length(users)} | 5.3 | 4.5 | 8.3 | 6.0 |
| users[\*].to_number(to_string(age)) | 2.9 | 2.0 | 2.0 | 2.3 |
| users[\*].to_string(@) | 1.4 | 1.3 | 1.3 | 1.3 |

<!-- END_BENCHMARK_RESULTS -->