# craken-cli

Command-line interface for the Craken framework.

## Features

- **Project Scaffolding**: Create new Craken projects with `craken new <name>`.
- **Development Server**: Start the production HTTP server with `craken serve`.
- **Hot-Reload Support**: Automatically restart your application on changes with `craken dev`.
- **Code Generators**:
  - `craken make controller <Name>`: Scaffold a new controller.
  - `craken make service <Name>`: Scaffold a new service.
  - `craken make module <Name>`: Scaffold a new module (controller + service).
- **Migration Management**: Unified interface for database migrations.

## Installation

Install the CLI locally from the source:

```bash
cargo install --path crates/craken-cli
```

## Usage

### Create a New Project

```bash
craken new my-app
cd my-app
```

### Start Development Server

```bash
craken dev
```

### Scaffold a Controller

```bash
craken make controller UserController
```

## Part of Craken

This crate is a core component of the [Craken Framework](https://github.com/shayyz-code/craken).
