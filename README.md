## To build the benchmark

`cargo build --bench=my_benchmark --release --target wasm32-wasi`

Copy the latest wasm file out of the target folder

```
cp `ls -t target/wasm32-wasi/release/deps/*.wasm | head -n 1` hex.wasm
```

## To run in a browser

Go to `https://webassembly.sh/`, drag the hex.wasm file into the browser, run the following, replacing 'chrome' with what environment it is

`hex --bench --save-baseline chrome`

The browser will lock up during this time.

`hex --export chrome | download` can be used to download the resulting benchmarks, rename them to chrome.json.

On your computer, use the following to compare different environments

```
cargo bench --bench=my_benchmark -- --compare --baselines=firefox.json,chrome.json --compare-list
```

To just output the numbers

`hex --bench --output-format bencher --confidence-level 0.99`

## NodeJS

Can't seem to get it to interact with the filesystem

`yarn wasmer-js run --dir=. hex.wasm -- --bench --output-format bencher`

```
test min 20
bench:       28174 ns/iter (+/- 280)

test fft 20
bench:       28364 ns/iter (+/- 329)
```

## chrome

```
test min 20 ... bench:       28584 ns/iter (+/- 1189)

test fft 20 ... bench:       28673 ns/iter (+/- 1293)
```

## firefox

```
test min 20 ... bench:      116402 ns/iter (+/- 5764)

test fft 20 ... bench:      116161 ns/iter (+/- 3507)
```
