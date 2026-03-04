use clap::{Parser, Subcommand};

pub mod generate;
pub mod hot_reload;

/// The Craken command-line interface.
#[derive(Parser)]
#[command(
    name = "craken",
    version = "0.1.0",
    about = "Craken — batteries-included Rust web framework"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Scaffold a new Craken project with the default directory layout.
    ///
    ///   craken new my-app
    New {
        /// Project name (used as the crate name and root directory).
        name: String,
        /// The database to use.
        #[arg(long, default_value = "postgres")]
        db: String,
    },

    /// Start the production HTTP server.
    ///
    ///   craken serve --addr 0.0.0.0:8080
    Serve {
        /// Address to bind (default: 127.0.0.1:8080).
        #[arg(short, long, default_value = "127.0.0.1:8080")]
        addr: String,
    },

    /// Start the development server with hot-reload on source file changes.
    ///
    ///   craken dev --addr 127.0.0.1:3000
    Dev {
        /// Address to bind (default: 127.0.0.1:8080).
        #[arg(short, long, default_value = "127.0.0.1:8080")]
        addr: String,
    },

    /// Code generators — scaffold controllers, services, and modules.
    Make {
        #[command(subcommand)]
        target: MakeTarget,
    },

    /// Run database migrations.
    Migrate,

    /// Rollback the last database migration.
    #[command(name = "migrate:rollback")]
    Rollback,
}

/// Targets for `craken make <target> <Name>`.
#[derive(Subcommand)]
pub enum MakeTarget {
    /// Generate a controller.
    ///
    ///   craken make controller UserController
    Controller {
        /// PascalCase name, e.g. `UserController`.
        name: String,
    },

    /// Generate a service.
    ///
    ///   craken make service UserService
    Service {
        /// PascalCase name, e.g. `UserService`.
        name: String,
    },

    /// Generate a self-contained module (controller + service + mod.rs).
    ///
    ///   craken make module Blog
    Module {
        /// PascalCase name, e.g. `Blog`.
        name: String,
    },

    /// Generate a new migration file.
    ///
    ///   craken make migration create_users_table
    #[command(name = "migration")]
    Migration {
        /// Snake_case name, e.g. `create_users_table`.
        name: String,
    },
}
