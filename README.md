# Metabodecon-Rust

## Description
*PLACEHOLDER*

## Running Tests

To run the tests for this project, follow these steps:

1. Ensure you have Rust and Cargo installed. You can install them from [rustup.rs](https://rustup.rs/).
2. Navigate to the project directory.
3. Run the following command to execute the tests:

```sh
cargo test -p metabodecon
```

This will compile the project and run all the tests defined in the `tests` directory and in the `#[cfg(test)]` modules.

## Running Benchmarks

To run the benchmarks for this project, follow these steps:

1. Ensure you have Rust and Cargo installed. You can install them from [rustup.rs](https://rustup.rs/).
2. Navigate to the project directory.
3. Run the following command to execute the benchmarks:

```sh
cargo bench -p metabodecon --profile release
```

This will compile the project in release mode and run all the benchmarks defined in the `benches` directory.
The results can be found in the `target/criterion` directory.

## Python Bindings

To build the Python bindings for this project locally, follow these steps:

1. Ensure you have Rust and Cargo installed. You can install them from [rustup.rs](https://rustup.rs/).
2. Ensure you have maturin installed. You can install it using, for example, pipx:

    ```sh
    pipx install maturin
    ```

3. Navigate to the `metabodecon-python` directory.
4. Activate the Python virtual environment where you want to install the bindings.
5. Run the following command:

   ```sh
   maturin develop --release
   ```
