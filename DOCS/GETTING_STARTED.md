# Getting Started with Craken

Ready to build your first Craken application? Follow these steps to get started.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [PostgreSQL](https://www.postgresql.org/download/) (for database access)

## Installation

Currently, Craken is in early development and can be used by cloning the repository and using the crates as path dependencies in your project.

### Installing the CLI

The `craken-cli` is your primary tool for managing projects:

```bash
# Clone the repository
git clone https://github.com/shayyz-code/craken.git
cd craken

# Install the CLI locally
cargo install --path crates/craken-cli
```

## Creating Your First Project

To create a new project using the CLI:

```bash
# For PostgreSQL
craken new my-app

# For SQLite
craken new my-app --db sqlite
```

### 1. Setup Your Environment

Craken generates a `.env` file for you. Update your `DATABASE_URL`:

```bash
# Example:
DATABASE_URL=postgres://postgres:password@localhost/my_app
```

### 2. Run the App

Start your application in production mode:

```bash
craken serve
```

Or for development (with hot-reload):

```bash
craken dev
```

### 3. Explore the API

Your app will start at `http://127.0.0.1:8080`. Try accessing any routes you've defined!

## Development Tools

Accelerate your workflow with code generation:

```bash
# Generate a new controller
craken make controller UserController

# Generate a database migration
craken make migration create_users_table
```

Learn more about Craken's [Core Concepts](CORE_CONCEPTS.md) next.
