# dictexprs

A simple package to test Rust interaction with dictionaries.

Meant to be a new version of querydict which hopefully can be more performant

## Developpement

```bash
uv run maturin build --release
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
| users[\*].ends_with(name, `"s"`) | 37.3 | 38.0 | 41.7 | 39.0 |
| users[\*].starts_with(name, `"A"`) | 47.2 | 30.5 | 32.1 | 36.6 |
| map(&ceil(@), users[\*].age) | 39.8 | 37.3 | 28.7 | 35.3 |
| users[\*].keys(@) | 25.4 | 38.6 | 35.7 | 33.2 |
| map(&length(@), users[\*].name) | 21.7 | 37.9 | 39.1 | 32.9 |
| users[\*].nested_scores[][] | 30.8 | 30.5 | 32.0 | 31.1 |
| users \| length(@) | 29.7 | 29.3 | 29.7 | 29.6 |
| users[\*].contains(category, `"VIP"`) | 19.1 | 36.4 | 29.1 | 28.2 |
| map(&floor(@), users[\*].age) | 17.0 | 37.8 | 28.5 | 27.8 |
| users[\*].values(@) | 38.2 | 17.2 | 26.9 | 27.4 |
| sort(users[?((age > `40` && active == `true`) && contains(category, `"VIP"`))].name) | 26.0 | 26.8 | 26.4 | 26.4 |
| map(&abs(@), users[\*].age) | 20.3 | 29.9 | 29.0 | 26.4 |
| users[?(age >= `30` && active == `true`)].name | 23.1 | 28.9 | 23.3 | 25.1 |
| length(users) | 20.3 | 18.0 | 33.0 | 23.8 |
| users[\*].merge(@, `{"extra_field":1}`) | 29.2 | 14.5 | 24.0 | 22.6 |
| users[\*].not_null(MISSING, name) | 12.3 | 21.0 | 20.7 | 18.0 |
| users[\*].category[] | 19.7 | 9.4 | 16.8 | 15.3 |
| users[\*].nested_scores[] | 11.5 | 17.3 | 16.7 | 15.2 |
| users[\*].join(`", "`, category) | 13.8 | 14.2 | 13.6 | 13.9 |
| users[\*].address.city | 12.6 | 12.1 | 11.1 | 11.9 |
| sort(users[\*].category[]) | 11.1 | 8.7 | 8.1 | 9.3 |
| max(users[\*].age) | 9.8 | 8.9 | 8.5 | 9.1 |
| min(users[\*].age) | 9.9 | 9.0 | 8.5 | 9.1 |
| abs(sum(users[\*].age)) | 9.6 | 9.1 | 7.9 | 8.9 |
| sum(users[\*].age) | 9.5 | 8.7 | 7.8 | 8.7 |
| min_by(users, &age) | 9.1 | 8.8 | 7.5 | 8.5 |
| ((users[\*].age > `1` && !(users[\*].age == `5`)) \|\| `0`) | 6.3 | 8.8 | 7.5 | 7.5 |
| avg(users[\*].age) | 5.1 | 8.7 | 8.3 | 7.4 |
| reverse(users[\*].age) | 9.5 | 4.4 | 8.0 | 7.3 |
| users[\*].name | 5.3 | 8.2 | 8.1 | 7.2 |
| users[\*].address | 6.2 | 7.8 | 7.6 | 7.2 |
| max_by(users, &age) | 4.5 | 8.9 | 7.5 | 7.0 |
| sort_by(users, &age)[\*].name | 7.5 | 6.7 | 6.1 | 6.8 |
| length(users[\*].name) | 4.5 | 8.6 | 4.0 | 5.7 |
| users[\*].age == `30` | 3.3 | 5.6 | 6.2 | 5.0 |

<!-- END_BENCHMARK_RESULTS -->