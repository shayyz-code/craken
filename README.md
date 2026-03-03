[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=for-the-badge)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=for-the-badge)](CONTRIBUTING.md)

## About Craken

Craken is an opinionated, batteries-included, and production-ready web framework for Rust. It is designed to provide a developer experience similar to [Laravel](https://laravel.com/) or [Spring Boot](https://spring.io/projects/spring-boot), but built idiomatically for the Rust ecosystem.

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

### Running the Example

1. **Setup Database**: Ensure you have a PostgreSQL instance running. You can set the `DATABASE_URL` environment variable:

   ```bash
   export DATABASE_URL=postgres://postgres:password@localhost/craken_app
   ```

2. **Run the App**:
   ```bash
   cargo run -p craken-app -- serve
   ```

Access the example API at `http://127.0.0.1:8080/users`.

## Documentation

Explore our comprehensive documentation to learn more about Craken:

- [**Introduction**](DOCS/INTRODUCTION.md) — What is Craken and why we built it.
- [**Getting Started**](DOCS/GETTING_STARTED.md) — Your first steps with the framework.
- [**Core Concepts**](DOCS/CORE_CONCEPTS.md) — Deep dive into DI, Routing, and Middleware.

> To optimize the documentation, please submit a PR to the documentation.

## 🤝 Contributing

We love contributions! Whether you're fixing a bug, improving documentation, or proposing a new feature, your help is welcome.

- 🌟 **Star the project** to show your support.
- **Report bugs** by opening an issue.
- **Suggest features** to help us grow.
- **Submit PRs** to improve the codebase.

Check out our [**Contributing Guide**](CONTRIBUTING.md) to get started.

## Security & Conduct

- [**Security Policy**](SECURITY.md) — How to report vulnerabilities.
- [**Code of Conduct**](CODE_OF_CONDUCT.md) — Our commitment to a welcoming environment.

## License

The Craken framework is open-sourced software licensed under the [Apache License, Version 2.0](LICENSE).

<p align="center">
  <em>Made with ❤️ for the Rust community.</em>
</p>
