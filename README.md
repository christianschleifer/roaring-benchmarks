# roaring-benchmarks

## Goal

Compare performance of the C-based [croaring-rs](https://github.com/RoaringBitmap/croaring-rs)
implementation with the performance of the
Rust-based [roaring-rs](https://github.com/RoaringBitmap/roaring-rs ) implementation.
The performance comparison will be done via benchmarks using real-world data.

## Run benchmarks

```bash
git submodule update --init --recursive 

cargo bench
```