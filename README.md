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

100 runs are computed for each test.

Each value is the median Python time divided by the median Rust time (median across runs)

A value of 2 meaning that Rust is twice as fast, a value of 0.5 meaning that Rust is half as fast.

<!-- BENCHMARK_RESULTS -->
| query | 10 | 50 | 250 | 1250 | average_speedup |
|---|---|---|---|---|---|
| map(&length(@), users[\*].name) | 36.5 | 40.2 | 43.9 | 38.2 | 39.7 |
| users[\*].starts_with(name, `"A"`) | 22.9 | 46.4 | 37.1 | 43.0 | 37.3 |
| users \| length(@) | 22.7 | 26.0 | 26.5 | 58.7 | 33.5 |
| users[\*].keys(@) | 34.1 | 22.6 | 37.1 | 34.9 | 32.2 |
| users[\*].values(@) | 30.3 | 34.8 | 28.9 | 25.8 | 30.0 |
| users[\*].ends_with(name, `"s"`) | 23.0 | 22.9 | 37.0 | 35.3 | 29.6 |
| map(&ceil(@), users[\*].age) | 28.7 | 28.2 | 26.8 | 31.9 | 28.9 |
| map(&floor(@), users[\*].age) | 30.2 | 30.3 | 27.6 | 27.4 | 28.9 |
| users[\*].nested_scores[][] | 33.4 | 16.1 | 31.7 | 33.4 | 28.6 |
| users[\*].contains(nested_scores[], `50`) | 26.4 | 26.3 | 28.2 | 24.8 | 26.4 |
| sales[][] \| map(&join(`, `, keys(@)), @) | 28.9 | 24.4 | 27.5 | 24.4 | 26.3 |
| map(&abs(@), users[\*].age) | 27.9 | 29.5 | 15.6 | 30.4 | 25.8 |
| users[\*].merge(@, `{"extra_field":1}`) | 25.6 | 28.0 | 24.1 | 24.1 | 25.5 |
| length(users) | 22.3 | 22.0 | 21.3 | 21.3 | 21.7 |
| users[?(age >= `30` && active == `true`)].name | 21.6 | 17.8 | 22.8 | 23.9 | 21.5 |
| sort(users[?((age > `40` && active == `true`) && contains(category, `"VIP"`))].name) | 12.5 | 26.5 | 17.9 | 24.6 | 20.4 |
| users[\*].nested_scores[] | 19.6 | 18.5 | 17.1 | 17.0 | 18.0 |
| users[\*].not_null(MISSING, name) | 11.8 | 11.9 | 23.7 | 19.2 | 16.6 |
| users[\*].category[] | 11.2 | 18.1 | 18.3 | 17.1 | 16.2 |
| sort(users[\*].nested_scores[][]) | 22.5 | 16.6 | 12.3 | 12.0 | 15.8 |
| users[\*].address.city | 11.6 | 11.9 | 11.4 | 10.9 | 11.4 |
| min(users[\*].age) | 12.0 | 9.2 | 8.5 | 8.2 | 9.5 |
| avg(users[\*].age) | 11.5 | 9.3 | 8.4 | 8.0 | 9.3 |
| length(users[\*].name) | 11.6 | 9.2 | 8.4 | 7.6 | 9.2 |
| ((users[\*].age > `1` && !(users[\*].age == `5`)) \|\| `0`) | 14.4 | 9.7 | 8.4 | 4.2 | 9.2 |
| reverse(users[\*].age) | 9.5 | 9.0 | 8.0 | 7.8 | 8.6 |
| max(users[\*].age) | 12.1 | 9.2 | 8.5 | 4.4 | 8.5 |
| max_by(users, &age) | 9.9 | 7.6 | 8.6 | 7.2 | 8.3 |
| sum(users[\*].age) | 11.3 | 9.0 | 4.5 | 7.7 | 8.1 |
| abs(sum(users[\*].age)) | 7.3 | 10.7 | 4.7 | 8.0 | 7.7 |
| min_by(users, &age) | 7.4 | 7.6 | 5.7 | 7.4 | 7.0 |
| users[\*].address | 6.1 | 8.1 | 4.6 | 5.1 | 6.0 |
| users[\*].name | 6.3 | 5.5 | 4.9 | 5.9 | 5.6 |
| sort_by(users, &age)[\*].name | 5.5 | 4.4 | 6.5 | 6.0 | 5.6 |
| users[\*].age == `30` | 7.7 | 5.9 | 2.8 | 5.2 | 5.4 |

<!-- END_BENCHMARK_RESULTS -->