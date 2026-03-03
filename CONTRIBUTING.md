# Contributing to Craken

Thank you for your interest in contributing to Craken! We welcome all contributions, including bug reports, feature requests, documentation improvements, and pull requests.

## How to Contribute

1.  **Search for existing issues**: Before opening a new issue or pull request, please search the repository to see if someone else has already reported the same thing or started working on a similar feature.
2.  **Open an issue**: If you find a bug or have a feature request, please open an issue to discuss it.
3.  **Submit a pull request**: If you want to contribute code, please submit a pull request. Make sure to follow the existing coding style and include tests for your changes.

## Development Setup

1.  **Fork the repository**: Create a fork of the Craken repository.
2.  **Clone your fork**: `git clone https://github.com/your-username/craken.git`
3.  **Install dependencies**: `cargo build`
4.  **Run tests**: `cargo test`

## Coding Standards

- Follow idiomatic Rust patterns.
- Ensure all code is formatted with `rustfmt`.
- Document public APIs with doc comments.
- Include unit tests for new functionality.

## Publication

To publish all Craken crates to [crates.io](https://crates.io) in the correct order:

1.  **Dry Run**:
    ```bash
    make publish-dry
    ```
2.  **Publish**:
    ```bash
    make publish
    ```

Ensure you have run `cargo login` and that your working directory is clean.

## Code of Conduct

Please note that this project is released with a [Code of Conduct](CODE_OF_CONDUCT.md). By participating in this project, you agree to abide by its terms.
