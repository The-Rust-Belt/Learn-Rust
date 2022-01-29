# Learn Rust

## Some Rust Notes

### Cargo

Rust's built-in package manager and build system (similar to Maven)

* `cargo new` - Create a new project.
* `cargo init` - Create a new project in an existing directory.
* `cargo build` - Build the project.
* `cargo run` - Run the project.
* `cargo update` - Update project dependencies.
* `cargo test` - Run tests.
* `cargo bench` - Run benchmarks.
* `cargo doc` - Generate the project documentation via `rustdoc`.
* `cargo check` - Analyze the project to see if it has any errors (without building).

In addition, there are cargo commands to publish the project as a crate/ package to Rust’s official crate registry, `crates.io`.

### Recommended Project Layout

```text
.
├── Cargo.toml
├── Cargo.lock
├── src
│   ├── main.rs
│   ├── lib.rs
│   └── bin
│       └── another_executable.rs
├── tests
│   └── some_integration_tests.rs
├── benches
│   └── simple_bench.rs
└── examples
    └── simple_example.rs
```

* The source code goes in the src directory.
  * The default executable file is src/main.rs.
  * The default library file is src/lib.rs.
  * Other executables can be placed in src/bin/*.rs.
* Integration tests go in the tests directory (unit tests go in each file they’re testing).
* Benchmarks go in the benches directory.
* Examples go in the examples directory.