# Deku Benchmark
This currently tracks the benchmark of the deku branch 
[impl-writer-inline-read-bytes](https://github.com/sharksforarms/deku/tree/impl-writer-inline-read-bytes).

This only touches the surface of comparison of `deku`, `binrw`, and `custom` and only tests
basic struct byte only reading.

## `+stable --profile=perf`
- [Full Results](https://wcampbell0x2a.github.io/deku-bench/perf/report/index.html)
![Deserialize](https://wcampbell0x2a.github.io/deku-bench/perf/Deserialize/report/violin.svg)
![Serialize](https://wcampbell0x2a.github.io/deku-bench/perf/Serialize/report/violin.svg)

## `+stable --release`
- [Full Results](https://wcampbell0x2a.github.io/deku-bench/release/report/index.html)
![Deserialize](https://wcampbell0x2a.github.io/deku-bench/release/Deserialize/report/violin.svg)
![Serialize](https://wcampbell0x2a.github.io/deku-bench/release/Serialize/report/violin.svg)

### Nightly Compare
`./run.bench`
nightly rust version: See `rust-toolchain.toml`
![Critcmp](critcmp.png)
