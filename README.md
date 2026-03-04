[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=for-the-badge)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=for-the-badge)](CONTRIBUTING.md)

![LOGO_CRAKEN](https://github.com/user-attachments/assets/a123ab8d-f727-46fd-852e-825d5a277bd1)

## About Craken

Craken is an opinionated and batteries-included web framework for Rust. It is designed to provide a developer experience similar to [Laravel](https://laravel.com/) or [Spring Boot](https://spring.io/projects/spring-boot), but built idiomatically for the Rust ecosystem.

We welcome stars, PRs, and issues!

## Goals

- **Opinionated & Integrated**: A cohesive ecosystem that "just works" out of the box.
- **Clean Architecture**: Strong separation of concerns between core, HTTP, and infrastructure.
- **Async-First**: Built on top of `tokio` and `axum`.
- **Compile-Time Safe DI**: A robust dependency injection container without reflection.
- **Developer-Friendly CLI**: Powerful scaffolding and development tools.

## Workspace Structure

Craken is structured as a Cargo workspace to ensure modularity and clear boundaries:

- `craken-core`: The application kernel and extension points.
- `craken-http`: Routing, middleware, and HTTP server integration.
- `craken-container`: Compile-time safe dependency injection.
- `craken-database`: SQLx-powered ORM and migration system.
- `craken-macros`: Procedural macros for routing and models.
- `craken-config`: Hierarchical configuration management.
- `craken-logging`: Structured, asynchronous logging.
- `craken-cli`: CLI for project management and hot-reloading.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [PostgreSQL](https://www.postgresql.org/download/)

### Installing the CLI

To use Craken, first install the unified command-line tool:

```bash
git clone https://github.com/shayyz-code/craken.git
cd craken
cargo install --path crates/craken-cli
```

### Creating a New Project

1. **Scaffold Your App**:

   ```bash
   # For PostgreSQL
   craken new my-app

   # For SQLite
   craken new my-app --db sqlite
   ```

2. **Setup Your Environment**:
   Configure your `DATABASE_URL` in the generated `.env` file.

3. **Run Your App**:
   For production:
   ```bash
   craken serve
   ```
   For development (with hot-reload):
   ```bash
   craken dev
   ```

## Development Tools

The Craken CLI provides several commands to accelerate your development:

- `craken make controller <Name>`: Scaffold a new route controller.
- `craken make service <Name>`: Scaffold an application service.
- `craken make module <Name>`: Generate a self-contained module (controller + service).
- `craken make migration <name>`: Create a new database migration file.
- `craken migrate`: Run all pending database migrations.

## Documentation

Explore our comprehensive documentation to learn more about Craken:

- [**Introduction**](DOCS/INTRODUCTION.md) — What is Craken and why we built it.
- [**Getting Started**](DOCS/GETTING_STARTED.md) — Your first steps with the framework.
- [**Core Concepts**](DOCS/CORE_CONCEPTS.md) — Deep dive into DI, Routing, and Middleware.

> To optimize the documentation, please submit a PR to the documentation.

## Contributing

We love contributions! Whether you're fixing a bug, improving documentation, or proposing a new feature, your help is welcome.

- 🌟 **Star the project** to show your support.
- **Report bugs** by opening an issue.
- **Suggest features** to help us grow.
- **Submit PRs** to improve the codebase.

Check out our [**Contributing Guide**](CONTRIBUTING.md) to get started.

## Security & Conduct

- [**Security Policy**](SECURITY.md) — How to report vulnerabilities.
- [**Code of Conduct**](CODE_OF_CONDUCT.md) — Our commitment to a welcoming environment.

## For Contributors

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

## License

The Craken framework is open-sourced software licensed under the [Apache License, Version 2.0](LICENSE).

<p align="center">
  <em>Made with ❤️ but not for the Rust community.</em>
</p>
