# nodejs-built-in-modules

Node.js built-in modules

## Benchmark

Comparison of `contains` vs `binary_search` for checking Node.js built-in modules:

```
is_builtin_contains     time:   [3.2416 µs 3.2444 µs 3.2476 µs]
                        change: [-1.3307% -1.0530% -0.7863%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe

is_builtin_binary_search
                        time:   [3.8228 µs 3.8340 µs 3.8559 µs]
                        change: [-3.3752% +0.1029% +3.3820%] (p = 0.96 > 0.05)
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe
```

**Result:** The `contains` method is ~18% faster than `binary_search` (3.24 µs vs 3.84 µs).

Run benchmarks with:
```bash
cargo bench
```

## License

MIT
