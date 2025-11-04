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
| users[\*].keys(@) | 37.9 | 42.3 | 37.7 | 34.5 | 38.1 |
| users[\*].ends_with(name, `"s"`) | 22.0 | 45.6 | 37.2 | 35.8 | 35.2 |
| users[\*].starts_with(name, `"A"`) | 21.6 | 42.7 | 38.3 | 33.8 | 34.1 |
| users[\*].nested_scores[][] | 33.3 | 30.5 | 31.3 | 33.6 | 32.2 |
| map(&length(@), users[\*].name) | 34.8 | 21.0 | 44.4 | 25.3 | 31.4 |
| map(&abs(@), users[\*].age) | 29.5 | 29.9 | 34.8 | 26.9 | 30.3 |
| map(&floor(@), users[\*].age) | 37.6 | 29.2 | 21.6 | 32.7 | 30.3 |
| users \| length(@) | 27.0 | 25.0 | 24.5 | 43.0 | 29.9 |
| users[\*].values(@) | 31.4 | 35.6 | 15.4 | 25.8 | 27.0 |
| users[\*].contains(nested_scores[], `50`) | 27.1 | 25.9 | 28.2 | 24.6 | 26.4 |
| sort(users[?((age > `40` && active == `true`) && contains(category, `"VIP"`))].name) | 15.9 | 25.9 | 34.6 | 26.1 | 25.6 |
| users[\*].merge(@, `{"extra_field":1}`) | 25.8 | 28.0 | 24.2 | 23.5 | 25.4 |
| map(&ceil(@), users[\*].age) | 28.7 | 16.0 | 28.2 | 20.1 | 23.2 |
| length(users) | 22.3 | 21.3 | 21.3 | 21.3 | 21.6 |
| users[?(age >= `30` && active == `true`)].name | 20.8 | 11.6 | 22.1 | 21.2 | 18.9 |
| users[\*].category[] | 18.1 | 18.9 | 17.9 | 16.6 | 17.9 |
| sort(users[\*].nested_scores[][]) | 22.8 | 17.2 | 12.3 | 12.0 | 16.1 |
| users[\*].not_null(MISSING, name) | 17.9 | 10.0 | 18.8 | 17.7 | 16.1 |
| sales[][] \| map(&join(`, `, keys(@)), @) | 15.3 | 15.2 | 14.3 | 12.6 | 14.4 |
| users[\*].nested_scores[] | 11.8 | 9.9 | 17.4 | 16.5 | 13.9 |
| min(users[\*].age) | 11.7 | 9.1 | 8.5 | 8.1 | 9.4 |
| users[\*].address.city | 12.3 | 7.1 | 7.2 | 10.9 | 9.4 |
| max(users[\*].age) | 11.2 | 8.9 | 9.1 | 8.0 | 9.3 |
| length(users[\*].name) | 11.9 | 9.2 | 8.3 | 7.9 | 9.3 |
| avg(users[\*].age) | 10.8 | 8.9 | 8.4 | 7.9 | 9.0 |
| sum(users[\*].age) | 10.7 | 8.9 | 8.1 | 7.8 | 8.9 |
| reverse(users[\*].age) | 10.1 | 8.6 | 7.8 | 7.6 | 8.5 |
| min_by(users, &age) | 10.0 | 7.5 | 7.2 | 7.0 | 7.9 |
| max_by(users, &age) | 5.9 | 7.6 | 7.0 | 7.0 | 6.9 |
| ((users[\*].age > `1` && !(users[\*].age == `5`)) \|\| `0`) | 9.5 | 5.2 | 8.4 | 4.2 | 6.8 |
| users[\*].address | 9.8 | 4.5 | 4.3 | 7.4 | 6.5 |
| abs(sum(users[\*].age)) | 8.1 | 5.5 | 4.4 | 8.0 | 6.5 |
| sort_by(users, &age)[\*].name | 9.3 | 4.0 | 6.4 | 5.9 | 6.4 |
| users[\*].age == `30` | 7.9 | 5.8 | 5.4 | 2.8 | 5.5 |
| users[\*].name | 7.9 | 4.4 | 4.5 | 3.9 | 5.2 |

<!-- END_BENCHMARK_RESULTS -->