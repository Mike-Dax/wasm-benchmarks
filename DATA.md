All done on a 2018 Mac Mini.

Maybe these should be re-done literally exported to WASM, benchmarked on the JS side, to take into consideration the FFI call cost.

# Firefox 98.0 (64-bit)

```
WASM - min - 4096 ... bench:        2446 ns/iter (+/- 296)
JS   - min - 4096 ... bench:        4946 ns/iter (+/- 152)

WASM - fft - 4096 ... bench:      150739 ns/iter (+/- 4384)
JS   - fft - 4096 ... bench:      252370 ns/iter (+/- 1363)
```

# Chrome 99.0.4844.51

```
WASM - min - 4096 ... bench:        1309 ns/iter (+/- 13)
JS   - min - 4096 ... bench:        3043 ns/iter (+/- 22)

WASM - fft - 4096 ... bench:      168126 ns/iter (+/- 2042)
JS   - fft - 4096 ... bench:      224086 ns/iter (+/- 695)
```

# NodeJS 16.13.1

```
WASM - min - 4096 ... bench:        1302 ns/iter (+/- 14)
WASM - fft - 4096 ... bench:      162672 ns/iter (+/- 2712)
```

# Rust Native

```
RUST - min - 4096 ... bench:        3791 ns/iter (+/- 4)
RUST - fft - 4096 ... bench:       97706 ns/iter (+/- 1655)
```

# fft

```
RUST         - fft - 4096 ... bench:       97706 ns/iter (+/- 1655)
WASM Firefox - fft - 4096 ... bench:      150739 ns/iter (+/- 4384)
WASM NodeJS  - fft - 4096 ... bench:      162672 ns/iter (+/- 2712)
WASM Chrome  - fft - 4096 ... bench:      168126 ns/iter (+/- 2042)
JS   Chrome  - fft - 4096 ... bench:      224086 ns/iter (+/- 695)
JS   Firefox - fft - 4096 ... bench:      252370 ns/iter (+/- 1363)
```
