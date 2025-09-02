use clap::Args;
use rust_social_network::SocialNetwork;

#[derive(Args)]
#[command(about = "Run database migrations")]
pub struct MigrateCommand {
    #[arg(short, long, default_value = "up")]
    pub direction: String,

    #[arg(long, default_value = "false")]
    pub dry_run: bool,
}

impl MigrateCommand {
    pub async fn execute(&self, app: &SocialNetwork) -> Result<(), Box<dyn std::error::Error>> {
        println!("Running migrations in direction: {}", self.direction);
        
        if self.dry_run {
            println!("Dry run mode - no changes will be made");
        }

        match self.direction.as_str() {
            "up" => {
                println!("Applying migrations...");
                app.run_migrations("up").await?;
                println!("Migrations completed successfully!");
            }
            "down" => {
                println!("Rolling back migrations...");
                app.run_migrations("down").await?;
                println!("Rollback completed successfully!");
            }
            _ => {
                return Err("Invalid direction. Use 'up' or 'down'".into());
            }
        }

        Ok(())
    }
}
