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
| query | 50 | 200 | 800 |
|---|---|---|---|
| ((users[0].age > `1` && !(users[0].age == `5`)) \|\| `0`) | 20.5 | 26.62 | 16.5 |
| [users[0], products[0]] | 16.0 | 16.2 | 17.0 |
| avg(products[\*].price) | 6.68 | 11.0 | 11.52 |
| contains(products[0].tags, `"electronics"`) | 23.9 | 17.75 | 17.5 |
| ends_with(`"hello"`, `"lo"`) | 36.5 | 36.0 | 20.0 |
| join(`", "`, products[0].tags) | 17.0 | 18.29 | 15.6 |
| keys(users[0]) | 16.0 | 27.33 | 16.67 |
| length(users) | 18.0 | 21.67 | 18.0 |
| map(&abs(@), products[\*].price) | 18.93 | 33.03 | 31.03 |
| map(&ceil(@), products[\*].price) | 18.63 | 32.22 | 31.06 |
| map(&floor(@), products[\*].price) | 32.78 | 32.36 | 30.15 |
| map(&length(@), users[\*].name) | 22.78 | 33.76 | 37.98 |
| max(products[\*].price) | 5.71 | 10.07 | 5.85 |
| max_by(products, &price) | 2.73 | 2.43 | 4.58 |
| merge(users[0], products[0]) | 22.1 | 15.605 | 12.9 |
| min(products[\*].price) | 10.48 | 5.94 | 5.74 |
| min_by(users, &age) | 8.52 | 7.83 | 7.93 |
| not_null(`null`, `"a"`, `"b"`) | 32.25 | 30.75 | 34.5 |
| products[\*].tags[] | 14.07 | 14.54 | 13.93 |
| reverse(products[\*].price) | 10.22 | 5.29 | 5.56 |
| sort(products[\*].tags[]) | 12.7 | 12.54 | 12.81 |
| sort(users[?((age > `40` && active == `true`) && contains(category, `"VIP"`))].name) | 25.77 | 26.21 | 24.44 |
| sort_by[users, &age](\*).name | 7.61 | 6.84 | 6.23 |
| starts_with(`"hello"`, `"he"`) | 37.75 | 36.5 | 20.5 |
| sum(products[\*].price) | 11.83 | 6.3 | 6.73 |
| to_array(users[0]) | 23.67 | 21.0 | 23.0 |
| to_number(`"42"`) | 28.0 | 29.0 | 25.5 |
| to_string(users[0]) | 1.34 | 2.0 | 2.02 |
| type(users[0]) | 19.0 | 21.5 | 22.0 |
| users \| length(@) | 29.33 | 29.0 | 17.0 |
| users.\*.address | 13.33 | 13.67 | 13.33 |
| users[\*].name | 8.69 | 8.48 | 7.71 |
| users[0].active == `true` | 18.4 | 23.5 | 18.8 |
| users[0].address.city | 14.2 | 14.8 | 14.6 |
| users[0].age == `30` | 22.44 | 20.4 | 20.6 |
| users[0].name | 13.0 | 13.0 | 16.0 |
| users[1:10:2] | 33.0 | 25.0 | 25.25 |
| users[?(age >= `30` && active == `true`)].name | 13.66 | 24.23 | 22.39 |
| values(users[0]) | 16.0 | 20.5 | 20.75 |
| {"names": users[\*].name, "count": length(users)} | 9.82 | 8.78 | 6.08 |

<!-- END_BENCHMARK_RESULTS -->