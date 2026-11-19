# lambda-budget-ledger-add

An AWS Lambda function, written in Rust, that adds two numbers with the
[budget-ledger](https://github.com/iaireal/rust-lib-budget-ledger) library.

It is the first function of the iaireal project and exists to prove the path from a
library to a deployed function. It does nothing useful yet.

## What it does

The function receives JSON with two numbers and answers with their sum:

| Request | Response |
| ------- | -------- |
| `{"left": 2, "right": 3}` | `{"sum": 5}` |

Both numbers are unsigned 64-bit integers.

## What is in this repository

This repository holds the source code of this one function and nothing else.

| Path | Contents |
| ---- | -------- |
| `src/main.rs` | The function and its tests |
| `Cargo.toml` | The Rust package and its dependencies |
| `Taskfile.yml` | The tasks listed below |

The Terraform that deploys the function lives in its own repository,
`infra-lambda-budget-ledger-add`.

## Requirements

- [Rust](https://rustup.rs) 1.85 or later (the package uses the 2024 edition)
- [cargo-lambda](https://www.cargo-lambda.info), to build for Lambda
- [Zig](https://ziglang.org/download/), which cargo-lambda uses to link the program for
  the system Lambda runs on
- [Task](https://taskfile.dev)

Only the build needs cargo-lambda and Zig. The tests run with Rust alone.

## Usage

```bash
task check    # formatting, linter and tests
task build    # build the zip file for Lambda
```

| Task | What it does |
| ---- | ------------ |
| `task fmt` | Formats the code |
| `task test` | Runs the tests |
| `task check` | Checks formatting, runs the linter and runs the tests |
| `task build` | Builds the function for Lambda |

### The build output

`task build` writes `target/lambda/lambda-budget-ledger-add/bootstrap.zip`. The zip holds
one file, `bootstrap`, which is the name Lambda expects for a program on a custom
runtime.

The function is built for arm64. Create it in Lambda with:

| Setting | Value |
| ------- | ----- |
| Runtime | `provided.al2023` |
| Architecture | `arm64` |
| Handler | `bootstrap` |
