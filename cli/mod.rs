pub mod serve;
pub mod migrate;
pub mod seed;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rust-social-network")]
#[command(about = "A social network built with Rust")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Serve(serve::ServeCommand),
    Migrate(migrate::MigrateCommand),
    Seed(seed::SeedCommand),
}
