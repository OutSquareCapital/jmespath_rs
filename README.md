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
| map(&length(@), users[\*].name) | 40.5 | 38.9 | 39.1 | 39.5 |
| users[\*].ends_with(name, `"s"`) | 37.0 | 38.2 | 36.6 | 37.3 |
| users[\*].keys(@) | 23.8 | 42.3 | 35.2 | 33.8 |
| users[\*].starts_with(name, `"A"`) | 25.3 | 38.1 | 36.8 | 33.4 |
| map(&ceil(@), users[\*].age) | 37.3 | 29.4 | 27.5 | 31.4 |
| users[\*].values(@) | 33.6 | 29.9 | 26.2 | 29.9 |
| users[\*].nested_scores[][] | 28.5 | 28.8 | 30.6 | 29.3 |
| users[\*].contains(category, `"VIP"`) | 29.1 | 30.2 | 28.6 | 29.3 |
| map(&floor(@), users[\*].age) | 28.7 | 29.1 | 27.4 | 28.4 |
| users[\*].merge(@, `{"extra_field":1}`) | 30.3 | 29.5 | 24.5 | 28.1 |
| map(&abs(@), users[\*].age) | 29.0 | 28.9 | 24.8 | 27.6 |
| users \| length(@) | 27.2 | 25.0 | 29.3 | 27.2 |
| sort(users[?((age > `40` && active == `true`) && contains(category, `"VIP"`))].name) | 17.1 | 27.1 | 32.9 | 25.7 |
| length(users) | 21.3 | 21.0 | 21.7 | 21.3 |
| users[?(age >= `30` && active == `true`)].name | 12.8 | 23.0 | 22.5 | 19.4 |
| users[\*].not_null(MISSING, name) | 20.1 | 15.6 | 20.0 | 18.6 |
| users[\*].category[] | 18.3 | 16.6 | 15.7 | 16.9 |
| users[\*].nested_scores[] | 17.1 | 16.8 | 16.1 | 16.7 |
| users[\*].join(`", "`, category) | 7.7 | 13.6 | 13.0 | 11.4 |
| users[\*].address.city | 11.5 | 6.5 | 11.1 | 9.7 |
| max(users[\*].age) | 9.3 | 9.6 | 8.5 | 9.1 |
| sum(users[\*].age) | 9.4 | 8.6 | 8.2 | 8.7 |
| length(users[\*].name) | 9.4 | 8.5 | 8.0 | 8.6 |
| reverse(users[\*].age) | 9.1 | 8.2 | 7.9 | 8.4 |
| abs(sum(users[\*].age)) | 9.9 | 5.2 | 8.2 | 7.8 |
| min_by(users, &age) | 8.7 | 7.3 | 7.1 | 7.7 |
| min(users[\*].age) | 9.2 | 4.8 | 8.4 | 7.5 |
| ((users[\*].age > `1` && !(users[\*].age == `5`)) \|\| `0`) | 5.6 | 8.7 | 7.8 | 7.4 |
| users[\*].name | 9.3 | 8.1 | 4.2 | 7.2 |
| sort(users[\*].category[]) | 5.3 | 8.4 | 7.9 | 7.2 |
| avg(users[\*].age) | 5.1 | 8.6 | 6.9 | 6.9 |
| max_by(users, &age) | 7.5 | 5.0 | 7.5 | 6.7 |
| users[\*].address | 4.4 | 8.1 | 7.4 | 6.6 |
| sort_by(users, &age)[\*].name | 7.2 | 6.4 | 5.7 | 6.4 |
| users[\*].age == `30` | 4.0 | 6.9 | 2.8 | 4.6 |

<!-- END_BENCHMARK_RESULTS -->