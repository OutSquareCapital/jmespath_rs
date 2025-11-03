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
| users[\*].starts_with(name, `"A"`) | 37.2 | 37.0 | 36.2 | 36.8 |
| users[\*].ends_with(name, `"s"`) | 35.8 | 35.9 | 34.6 | 35.4 |
| users[\*].values(@) | 42.5 | 33.6 | 29.2 | 35.1 |
| map(&abs(@), users[\*].age) | 36.5 | 34.2 | 32.1 | 34.3 |
| map(&length(@), users[\*].name) | 23.1 | 37.3 | 35.5 | 32.0 |
| map(&ceil(@), users[\*].age) | 36.8 | 26.9 | 26.2 | 30.0 |
| users[\*].keys(@) | 24.3 | 21.7 | 38.7 | 28.2 |
| users \| length(@) | 25.0 | 25.5 | 29.7 | 26.7 |
| users[\*].contains(category, `"VIP"`) | 15.6 | 30.4 | 28.0 | 24.7 |
| users[\*].merge(@, `{"extra_field":1}`) | 19.3 | 27.6 | 25.2 | 24.0 |
| map(&floor(@), users[\*].age) | 16.1 | 26.0 | 26.1 | 22.7 |
| sort(users[?((age > `40` && active == `true`) && contains(category, `"VIP"`))].name) | 13.5 | 25.5 | 24.0 | 21.0 |
| length(users) | 18.0 | 21.3 | 22.3 | 20.5 |
| users[?(age >= `30` && active == `true`)].name | 15.0 | 21.3 | 21.1 | 19.1 |
| users[\*].not_null(MISSING, name) | 20.4 | 12.2 | 21.9 | 18.2 |
| users[\*].category[] | 10.8 | 18.0 | 17.5 | 15.4 |
| users[\*].address.city | 12.0 | 11.8 | 11.2 | 11.7 |
| users[\*].join(`", "`, category) | 8.0 | 12.3 | 13.6 | 11.3 |
| sort(users[\*].category[]) | 10.2 | 8.8 | 8.1 | 9.0 |
| length(users[\*].name) | 9.2 | 8.3 | 7.7 | 8.4 |
| reverse(users[\*].age) | 8.6 | 8.7 | 7.7 | 8.3 |
| abs(sum(users[\*].age)) | 9.1 | 8.1 | 7.8 | 8.3 |
| sum(users[\*].age) | 8.6 | 8.2 | 7.9 | 8.2 |
| max_by(users, &age) | 8.7 | 7.4 | 8.0 | 8.0 |
| ((users[\*].age > `1` && !(users[\*].age == `5`)) \|\| `0`) | 10.4 | 5.3 | 8.0 | 7.9 |
| min(users[\*].age) | 8.8 | 4.8 | 7.7 | 7.1 |
| users[\*].name | 8.9 | 7.9 | 4.0 | 6.9 |
| users[\*].address | 4.7 | 7.6 | 7.2 | 6.5 |
| sort_by(users, &age)[\*].name | 7.1 | 6.3 | 6.1 | 6.5 |
| min_by(users, &age) | 4.8 | 7.3 | 7.1 | 6.4 |
| max(users[\*].age) | 2.5 | 8.2 | 7.7 | 6.1 |
| avg(users[\*].age) | 4.6 | 4.5 | 7.5 | 5.5 |
| users[\*].age == `30` | 5.7 | 5.2 | 5.1 | 5.3 |

<!-- END_BENCHMARK_RESULTS -->