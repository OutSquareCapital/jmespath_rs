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
| map(&length(@), users[\*].name) | 45.2 | 38.4 | 38.3 | 40.6 |
| users[\*].starts_with(name, `"A"`) | 36.7 | 37.2 | 36.0 | 36.6 |
| users[\*].ends_with(name, `"s"`) | 36.9 | 27.1 | 34.4 | 32.8 |
| map(&abs(@), users[\*].age) | 29.8 | 28.6 | 27.7 | 28.7 |
| users[\*].keys(@) | 26.6 | 22.1 | 37.4 | 28.7 |
| sort(users[?((age > `40` && active == `true`) && contains(category, `"VIP"`))].name) | 26.4 | 32.9 | 26.5 | 28.6 |
| users \| length(@) | 25.0 | 29.0 | 29.0 | 27.7 |
| users[\*].contains(category, `"VIP"`) | 19.7 | 31.1 | 28.2 | 26.3 |
| map(&ceil(@), users[\*].age) | 26.9 | 19.5 | 27.1 | 24.5 |
| users[\*].merge(@, `{"extra_field":1}`) | 18.8 | 27.1 | 25.3 | 23.7 |
| users[\*].values(@) | 21.0 | 18.4 | 28.8 | 22.7 |
| map(&floor(@), users[\*].age) | 16.1 | 19.7 | 27.5 | 21.1 |
| users[\*].not_null(MISSING, name) | 23.2 | 20.1 | 19.9 | 21.1 |
| length(users) | 18.0 | 21.7 | 22.0 | 20.6 |
| users[?(age >= `30` && active == `true`)].name | 16.3 | 17.3 | 20.6 | 18.1 |
| users[\*].category[] | 18.1 | 17.9 | 17.0 | 17.7 |
| users[\*].join(`", "`, category) | 8.0 | 14.2 | 13.7 | 12.0 |
| users[\*].address.city | 7.0 | 11.3 | 10.8 | 9.7 |
| sort(users[\*].category[]) | 10.4 | 8.8 | 8.3 | 9.2 |
| abs(sum(users[\*].age)) | 10.0 | 9.0 | 8.3 | 9.1 |
| avg(users[\*].age) | 9.1 | 8.2 | 7.9 | 8.4 |
| min_by(users, &age) | 8.8 | 7.4 | 7.0 | 7.7 |
| ((users[\*].age > `1` && !(users[\*].age == `5`)) \|\| `0`) | 5.9 | 8.9 | 8.4 | 7.7 |
| min(users[\*].age) | 5.4 | 8.4 | 8.3 | 7.4 |
| reverse(users[\*].age) | 5.2 | 8.7 | 7.8 | 7.2 |
| length(users[\*].name) | 5.1 | 8.3 | 7.8 | 7.1 |
| users[\*].name | 8.6 | 8.0 | 4.5 | 7.0 |
| sum(users[\*].age) | 4.8 | 8.3 | 7.9 | 7.0 |
| users[\*].address | 4.2 | 7.6 | 7.2 | 6.3 |
| max(users[\*].age) | 5.5 | 4.9 | 8.4 | 6.3 |
| sort_by(users, &age)[\*].name | 4.0 | 6.5 | 6.1 | 5.5 |
| max_by(users, &age) | 4.8 | 4.2 | 7.3 | 5.4 |
| users[\*].age == `30` | 4.1 | 5.6 | 5.3 | 5.0 |

<!-- END_BENCHMARK_RESULTS -->