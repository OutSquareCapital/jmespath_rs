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
| users[\*].keys(@) | 43.4 | 38.9 | 37.2 | 39.8 |
| users[\*].starts_with(name, `"A"`) | 46.1 | 36.2 | 34.7 | 39.0 |
| users[\*].values(@) | 36.4 | 33.0 | 23.6 | 31.0 |
| sort(users[?((age > `40` && active == `true`) && contains(category, `"VIP"`))].name) | 32.0 | 34.2 | 26.5 | 30.9 |
| [users[0:10], users[-10:]] | 22.5 | 31.3 | 35.1 | 29.6 |
| users[\*].ends_with(name, `"s"`) | 18.3 | 35.1 | 33.3 | 28.9 |
| map(&ceil(@), users[\*].age) | 29.1 | 26.8 | 25.6 | 27.2 |
| users \| length(@) | 29.0 | 25.0 | 25.0 | 26.3 |
| map(&length(@), users[\*].name) | 20.3 | 19.6 | 36.6 | 25.5 |
| users[\*].merge(@, `{"extra_field":1}`) | 29.2 | 26.7 | 20.5 | 25.5 |
| users[\*].contains(category, `"VIP"`) | 19.6 | 27.5 | 27.8 | 25.0 |
| map(&floor(@), users[\*].age) | 28.6 | 18.9 | 26.0 | 24.5 |
| map(&abs(@), users[\*].age) | 20.4 | 26.8 | 25.6 | 24.3 |
| length(users) | 21.0 | 21.0 | 21.3 | 21.1 |
| users[\*].not_null(MISSING, name) | 19.9 | 20.0 | 19.0 | 19.6 |
| users[?(age >= `30` && active == `true`)].name | 12.5 | 18.0 | 21.8 | 17.4 |
| users[\*].category[] | 17.3 | 16.7 | 16.2 | 16.7 |
| users[\*].join(`", "`, category) | 14.7 | 8.9 | 11.3 | 11.6 |
| users[\*].address.city | 7.3 | 11.9 | 11.2 | 10.1 |
| {"names": users[\*].name, "count": length(users)} | 10.3 | 8.2 | 7.9 | 8.8 |
| sort(users[\*].category[]) | 9.9 | 8.5 | 8.0 | 8.8 |
| max(users[\*].age) | 9.0 | 8.2 | 7.8 | 8.3 |
| min(users[\*].age) | 8.8 | 8.1 | 7.8 | 8.2 |
| users[\*].name | 8.5 | 7.9 | 7.6 | 8.0 |
| min_by(users, &age) | 7.7 | 8.7 | 7.4 | 7.9 |
| max_by(users, &age) | 7.7 | 7.3 | 7.0 | 7.3 |
| sum(users[\*].age) | 4.6 | 8.8 | 8.0 | 7.1 |
| avg(users[\*].age) | 5.0 | 8.1 | 7.8 | 7.0 |
| reverse(users[\*].age) | 8.8 | 4.5 | 7.4 | 6.9 |
| users[\*].address | 8.1 | 4.1 | 7.3 | 6.5 |
| sort_by(users, &age)[\*].name | 7.2 | 6.6 | 5.8 | 6.5 |
| ((users[\*].age > `1` && !(users[\*].age == `5`)) \|\| `0`) | 5.8 | 8.4 | 4.9 | 6.4 |
| length(users[\*].name) | 4.7 | 4.8 | 7.7 | 5.7 |
| users[\*].age == `30` | 3.2 | 5.8 | 5.1 | 4.7 |

<!-- END_BENCHMARK_RESULTS -->