# roaring-benchmarks

## Goal

Compare performance of the C-based [croaring-rs](https://github.com/RoaringBitmap/croaring-rs)
implementation with the performance of the
Rust-based [roaring-rs](https://github.com/RoaringBitmap/roaring-rs) implementation.
The performance comparison will be done via benchmarks using real-world data.

## Run benchmarks

### Local machine

```bash
git submodule update --init --recursive 

cargo bench
```

### AWS instance

```bash
sudo yum install -y git 
git clone https://github.com/christianschleifer/roaring-benchmarks.git
cd roaring-benchmarks
./aws/setup_aws_instance.sh

. "$HOME/.cargo/env"

cargo bench
```

### Results

### Architecture

Benchmarks were run on x86 and ARM-based processors.

### Hardware

Benchmarks were run on bare metal AWS EC2 instances to minimize the impact of virtualization on the
benchmarks.

x86: [m6a.metal](https://aws.amazon.com/ec2/instance-types/m6a/)
ARM: [m6g.metal](https://aws.amazon.com/ec2/instance-types/m6g/)

### Results

#### croaring with SIMD vs. roaring without SIMD

x86: https://christianschleifer.github.io/benchmarks/roaring-bitmaps/x86/m6a-metal/report/index.html

ARM: https://christianschleifer.github.io/benchmarks/roaring-bitmaps/arm/m6g-metal/report/index.html

####   

x86:

ARM:

####   

TODO: croaring with SIMD vs. roaring with SIMD
TODO: croaring without SIMD vs. roaring without SIMD
TODO: Use croaring RunContainers via `.run_optimize` and compare with croaring
without `.run_optimize`.
