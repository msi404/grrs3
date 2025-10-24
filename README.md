# grrs3

A minimal Rust CLI tool for substring search in files, inspired by `grep`.

## Features
- Case-sensitive substring matching.
- Line-by-line file reading.
- Error handling with Arabic messages.

## Installation
1. Install Rust: [rustup.rs](https://rustup.rs/).
2. Create project: `cargo new grrs3 && cd grrs3`.
3. Replace `src/main.rs` with the code.
4. Add to `Cargo.toml`:
   ```toml
   [dependencies]
   anyhow = "1.0"
   clap = { version = "4.0", features = ["derive"] }
   ```
5. Build: `cargo build --release`.

Executable: `target/release/grrs3`.

## Usage
```
grrs3 <PATTERN> <PATH>
```

## Examples
```bash
# Search for "error" in log
./grrs3 error app.log

# Search for "Rust" in code
./grrs3 Rust main.rs
```

## Limitations
- Substring only (no regex).
- Single file support.

## License
Apache-2.0.