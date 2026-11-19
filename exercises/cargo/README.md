# Cargo

## Dependencies

To create a new Rust project, use carge.

Default is a binary, or use the lib option.

```
cargo new foo
cargo new --lib bar
```

After the above commands, you should see a file hierarchy like this:

```
├── bar
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── foo
    ├── Cargo.toml
    └── src
```