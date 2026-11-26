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
| `rust-toolchain.toml` | The Rust version and the build target, installed by rustup |
| `.cargo/config.toml` | The linker used to build for Lambda |
| `Taskfile.yml` | The tasks listed below |
| `.github/workflows/` | The automatic checks and the release, run by GitHub Actions |

The Terraform that deploys the function lives in its own repository,
`infra-lambda-budget-ledger-add`.

## Requirements

- [Rust](https://rustup.rs), installed with rustup
- [Task](https://taskfile.dev)
- `zip`

Nothing else is needed to build for Lambda: no extra cargo plugin, no C compiler for
arm64 and no container. See [How the build works](#how-the-build-works).

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

`task build` writes `target/lambda/bootstrap.zip`. The zip holds one file, `bootstrap`,
which is the name Lambda expects for a program on a custom runtime.

The function is built for arm64. Create it in Lambda with:

| Setting | Value |
| ------- | ----- |
| Runtime | `provided.al2023` |
| Architecture | `arm64` |
| Handler | `bootstrap` |

## Releases

A release is the built function, ready to deploy. Each one holds two files:

| File | Contents |
| ---- | -------- |
| `bootstrap.zip` | The zip file Lambda runs |
| `bootstrap.zip.sha256` | Its SHA-256 checksum |

GitHub Actions builds and publishes a release when a version tag is pushed. To release:

1. Set the new version in `Cargo.toml`, run `cargo build` so `Cargo.lock` follows, and
   commit.
2. Tag that commit with the same version, prefixed with `v`, and push the tag:

   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

The release fails if the tag does not match the version in `Cargo.toml`.

`infra-lambda-budget-ledger-add` deploys a release by its version. It never builds the
function itself.

## How the build works

Lambda runs the function on Linux arm64, which is usually not the machine it is built on.
The build handles that with the standard Rust toolchain alone:

- **The target is `aarch64-unknown-linux-musl`.** It produces a static executable: the
  program carries everything it needs and uses no library from the system it runs on, so
  it does not matter which Linux version built it or which one Lambda runs.
- **rustup installs the target.** `rust-toolchain.toml` lists it, so the first `cargo`
  command in this folder downloads it.
- **The linker is `rust-lld`,** which ships with Rust. `.cargo/config.toml` selects it,
  so no arm64 C toolchain has to be installed.

This works because the function and its dependencies are written entirely in Rust. A
dependency that compiles C code would need a C compiler for arm64.
