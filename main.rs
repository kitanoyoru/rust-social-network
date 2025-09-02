mod cli;

use clap::Parser;
use cli::Cli;
use rust_social_network::SocialNetwork;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let app = SocialNetwork::new();

    match cli.command {
        cli::Commands::Serve(cmd) => cmd.execute(&app).await?,
        cli::Commands::Migrate(cmd) => cmd.execute(&app).await?,
        cli::Commands::Seed(cmd) => cmd.execute(&app).await?,
    }

    Ok(())
}
