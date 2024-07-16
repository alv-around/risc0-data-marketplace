# Heart Disease Input validate

A risc0 zkVM read and process [Heart Disease dataset](https://archive.ics.uci.edu/dataset/45/heart+disease) from UC Irvine Machine Learning Repository. This example demonstrates how to deserialize CSV data into a specified struct, HeartDiseaseMeasure, and run the data within a zkVM.

## Quick Start

First, make sure [rustup] is installed. The
[`rust-toolchain.toml`][rust-toolchain] file will be used by `cargo` to
automatically install the correct version.

To create the dataset with each entry signed run:
```bash
cargo run --bin sign_data
```

To execute the method within the zkVM, run the following
command:

```bash
cargo run --bin input-data
```

This is an empty template, and so there is no expected output (until you modify
the code).

### Executing the project locally in development mode

During development, faster iteration upon code changes can be achieved by leveraging [dev-mode], we strongly suggest activating it during your early development phase. Furthermore, you might want to get insights into the execution statistics of your project, and this can be achieved by specifying the environment variable `RUST_LOG="executor=info"` before running your project.

Put together, the command to run your project in development mode while getting execution statistics is:

```bash
RUST_LOG="executor=info" RISC0_DEV_MODE=1 cargo run --bin input-data
```



## Directory Structure


```text
input-data
├── Cargo.toml
├── host
│   ├── Cargo.toml
│   └── src
│       └── main.rs                        <-- [Host code goes here]
└── methods
    ├── Cargo.toml
    ├── build.rs
    ├── guest
    │   ├── Cargo.toml
    │   └── src
    │       └── main.rs                   <-- [Guest code goes here]
    └── src
        └── lib.rs
└── utils
    ├── Cargo.toml
    └── src
        └── lib.rs                         <-- [data types]
```

