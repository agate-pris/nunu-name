# nunu-name

A tiny CLI to generate random, pronounceable names with optional numeric suffixes.

## Requirements

- Rust nightly (this project uses `#![feature(random)]`)

## Build

```bash
cargo build --release
```

## Usage

```bash
cargo run -- [COUNT|MIN-MAX] [LETTERS|MIN-MAX] [DIGITS|MIN-MAX]
```

Each positional argument is optional.

- `COUNT`: number of names to print (default: `1`)
- `LETTERS`: number of letters per name (default: `1`)
- `DIGITS`: number of digits appended to the end (default: `1`)

Each argument accepts either:

- A fixed integer (example: `4`)
- A range (example: `3-8`), randomly chosen each run

### Help

```bash
cargo run -- --help
# or
cargo run -- -h
```

### Examples

```bash
# 1 name, 1 letter, 1 digit (defaults)
cargo run --

# Generate 5 names, each with 6 letters and 2 digits
cargo run -- 5 6 2

# Randomize all three arguments using ranges
cargo run -- 3-7 4-8 0-3
```

## Notes

- Invalid numbers fall back to defaults.
- For ranges, the bounds are normalized, so `8-3` behaves like `3-8`.
