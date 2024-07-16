# Risc0 Data Marketplace

This example was mainly taken from [composition example of Risc0's repo](https://github.com/risc0/risc0/tree/v0.21.0/examples/composition)

## Composition Example

This example demonstrates a basic example of a compute-2-data market place using risc0's zkVM.
This repository leverages recursion to first proof certain qualities of a dataset. 
And second run a zkML program on top of the latter.

The inner proof reads and processes [Heart Disease dataset](https://archive.ics.uci.edu/dataset/45/heart+disease) from UC Irvine Machine Learning Repository, and verifies that the original data has been correctly signed by a trusted "sensor". 
See [`input-data/methods/gest/src/main.rs`] for the proofs implementation.
For the outter proof, See the [`src/main.rs`] file for the [host] side implementation of composition, and [`methods/guest/src/main.rs`] for the guest side.


## Quick Start


For M1,M2.. run the proof using your machines GPUs with:

```bash
RUST_LOG="[executor]=info" RUST_BACKTRACE=1 cargo run --release -F metal
```

If your machine does not have GPUs, run:

```bash
RUST_LOG="[executor]=info" RUST_BACKTRACE=1 cargo run --release
```