# Introduction to Craken

Craken is a batteries-included, opinionated, and production-ready web framework for Rust. It is designed to provide a developer experience similar to Laravel or Spring Boot, but built idiomatically for the Rust ecosystem.

## Why Craken?

Rust has a powerful and growing ecosystem for web development, with high-performance libraries like `tokio` and `axum`. However, building a production-ready application often requires manual integration of multiple libraries for dependency injection, configuration, logging, database access, and more.

Craken aims to solve this by providing a cohesive framework that integrates these components out of the box, allowing you to focus on building your application's logic.

## Key Features

- **Opinionated & Integrated**: A single ecosystem where everything "just works".
- **Clean Architecture**: Clear boundaries between core application logic and external infrastructure.
- **Async-First**: Built from the ground up on top of the `tokio` runtime.
- **Type-Safe DI**: A robust dependency injection container that catches errors at compile-time.
- **Productivity CLI**: Generate code, manage migrations, and run your app with ease.

## Architecture Overview

Craken is built as a set of modular crates:

- `craken-core`: The core application kernel and extension points.
- `craken-http`: Routing, middleware, and HTTP server integration.
- `craken-container`: Our custom dependency injection system.
- `craken-database`: SQLx-powered ORM and migration management.
- `craken-macros`: Procedural macros to reduce boilerplate.
- `craken-config`: Flexible configuration management.
- `craken-logging`: High-performance structured logging.
- `craken-cli`: The unified command-line tool.

Ready to dive in? Head over to the [Getting Started](GETTING_STARTED.md) guide.
