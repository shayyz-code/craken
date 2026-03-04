use anyhow::Result;
use clap::Parser;
use craken_cli::{Cli, Commands, MakeTarget};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::New { name, db } => {
            craken_cli::generate::make_app(&name, &db)?;
        }
        Commands::Serve { addr } => {
            println!("🦀 Starting Craken server on http://{}", addr);
            let mut child = tokio::process::Command::new("cargo")
                .args(["run", "--", "serve", &addr])
                .spawn()?;
            child.wait().await?;
        }
        Commands::Dev { addr } => {
            craken_cli::hot_reload::run_dev(&addr).await?;
        }
        Commands::Make { target } => match target {
            MakeTarget::Controller { name } => {
                craken_cli::generate::make_controller(&name)?;
            }
            MakeTarget::Service { name } => {
                craken_cli::generate::make_service(&name)?;
            }
            MakeTarget::Module { name } => {
                craken_cli::generate::make_module(&name)?;
            }
            MakeTarget::Migration { name } => {
                craken_cli::generate::make_migration(&name)?;
            }
        },
        Commands::Migrate => {
            println!("Running migrations...");
            let mut child = tokio::process::Command::new("cargo")
                .arg("run")
                .arg("--")
                .arg("migrate")
                .spawn()?;
            child.wait().await?;
        }
        Commands::Rollback => {
            println!("Rolling back last migration...");
            let mut child = tokio::process::Command::new("cargo")
                .arg("run")
                .arg("--")
                .arg("migrate:rollback")
                .spawn()?;
            child.wait().await?;
        }
    }

    Ok(())
}
