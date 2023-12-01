# exc

exc is a new programming language with experimental syntax and type system.

```
fn main() {
  printf("hello, world!");
}
```

## tests

To run tests:

```
cargo test --all
```

To run fuzz tests:

```
cargo +nightly fuzz run fuzz_target_parse -- -timeout=1 -only_ascii=1
```

> **NOTE**: This project uses `cargo-fuzz`, so make sure that you have it before running fuzz test.

```
cargo install cargo-fuzz
```
