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
| map(&length(@), users[\*].name) | 45.1 | 37.9 | 37.3 | 40.1 |
| users[\*].keys(@) | 42.6 | 38.5 | 37.1 | 39.4 |
| users[\*].starts_with(name, `"A"`) | 35.9 | 36.5 | 34.8 | 35.7 |
| users \| length(@) | 25.5 | 44.0 | 28.3 | 32.6 |
| map(&ceil(@), users[\*].age) | 39.3 | 28.4 | 27.1 | 31.6 |
| users[\*].ends_with(name, `"s"`) | 24.3 | 36.0 | 34.3 | 31.5 |
| [users[0:10], users[-10:]] | 22.5 | 34.4 | 35.0 | 30.6 |
| map(&abs(@), users[\*].age) | 29.7 | 28.8 | 27.5 | 28.7 |
| users[\*].values(@) | 36.3 | 17.9 | 27.7 | 27.3 |
| sort(users[?((age > `40` && active == `true`) && contains(category, `"VIP"`))].name) | 14.7 | 35.1 | 26.1 | 25.3 |
| users[?(age >= `30` && active == `true`)].name | 28.8 | 22.7 | 19.3 | 23.6 |
| users[\*].contains(category, `"VIP"`) | 16.8 | 27.4 | 26.5 | 23.6 |
| map(&floor(@), users[\*].age) | 16.3 | 27.1 | 27.0 | 23.5 |
| users[\*].merge(@, `{"extra_field":1}`) | 18.3 | 24.7 | 25.3 | 22.8 |
| users[\*].not_null(MISSING, name) | 23.5 | 23.5 | 20.2 | 22.4 |
| length(users) | 17.5 | 21.3 | 21.3 | 20.0 |
| users[\*].category[] | 17.4 | 18.1 | 16.2 | 17.2 |
| users[\*].address.city | 13.3 | 12.0 | 11.3 | 12.2 |
| users[\*].join(`", "`, category) | 8.1 | 13.7 | 13.4 | 11.7 |
| sort(users[\*].category[]) | 10.0 | 8.5 | 7.9 | 8.8 |
| max(users[\*].age) | 9.2 | 8.5 | 8.0 | 8.6 |
| users[\*].name | 8.4 | 8.0 | 8.0 | 8.1 |
| min_by(users, &age) | 7.8 | 7.5 | 7.2 | 7.5 |
| max_by(users, &age) | 7.8 | 7.6 | 7.2 | 7.5 |
| ((users[\*].age > `1` && !(users[\*].age == `5`)) \|\| `0`) | 9.6 | 8.7 | 4.2 | 7.5 |
| min(users[\*].age) | 9.4 | 4.8 | 8.1 | 7.4 |
| length(users[\*].name) | 5.3 | 8.4 | 8.0 | 7.2 |
| avg(users[\*].age) | 4.9 | 8.4 | 8.1 | 7.1 |
| users[\*].address | 7.8 | 4.5 | 6.9 | 6.4 |
| sum(users[\*].age) | 5.3 | 8.5 | 4.3 | 6.0 |
| users[\*].age == `30` | 7.4 | 5.3 | 5.1 | 5.9 |
| reverse(users[\*].age) | 4.8 | 8.1 | 4.2 | 5.7 |
| {"names": users[\*].name, "count": length(users)} | 4.9 | 4.4 | 7.5 | 5.6 |
| sort_by(users, &age)[\*].name | 4.0 | 6.3 | 6.0 | 5.4 |

<!-- END_BENCHMARK_RESULTS -->