# Getting Started with Craken

Ready to build your first Craken application? Follow these steps to get started.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [PostgreSQL](https://www.postgresql.org/download/) (for database access)

## Installation

Currently, Craken is in early development and can be used by cloning the repository and using the crates as path dependencies in your project.

### Using the CLI

The `craken-cli` is your primary tool for managing projects:

```bash
# Clone the repository
git clone https://github.com/yahs/craken.git
cd craken

# Install the CLI locally
cargo install --path crates/craken-cli
```

## Running the Example Application

Craken comes with an example application in `examples/craken-app` that demonstrates the framework's core features.

### 1. Setup Your Database

Ensure you have a PostgreSQL instance running. You can set the `DATABASE_URL` environment variable:

```bash
export DATABASE_URL=postgres://postgres:password@localhost/craken_app
```

### 2. Run the App

Run the example using Cargo:

```bash
cargo run -p craken-app -- serve
```

### 3. Explore the API

The example app provides several endpoints:

- `GET /users`: List all users.
- `GET /users/:id`: Fetch a single user.
- `GET /health`: Check the application health.

Try accessing them at `http://127.0.0.1:8080/users`.

## Creating Your First Project

To create a new project using the CLI:

```bash
craken new my-app
cd my-app
```

Learn more about Craken's [Core Concepts](CORE_CONCEPTS.md) next.
