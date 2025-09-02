use clap::Args;
use rust_social_network::SocialNetwork;

#[derive(Args)]
#[command(about = "Seed the database with initial data")]
pub struct SeedCommand {
    #[arg(short, long, default_value = "all")]
    pub data: String,

    #[arg(long, default_value = "false")]
    pub force: bool,
}

impl SeedCommand {
    pub async fn execute(&self, app: &SocialNetwork) -> Result<(), Box<dyn std::error::Error>> {
        println!("Seeding database with: {}", self.data);
        
        if self.force {
            println!("Force mode enabled - existing data will be overwritten");
        }

        match self.data.as_str() {
            "all" => {
                println!("Seeding all data types...");
                app.seed_database("all").await?;
                println!("Users seeded successfully!");
                println!("Posts seeded successfully!");
                println!("Comments seeded successfully!");
            }
            "users" => {
                println!("Seeding users...");
                app.seed_database("users").await?;
                println!("Users seeded successfully!");
            }
            "posts" => {
                println!("Seeding posts...");
                app.seed_database("posts").await?;
                println!("Posts seeded successfully!");
            }
            "comments" => {
                println!("Seeding comments...");
                app.seed_database("comments").await?;
                println!("Comments seeded successfully!");
            }
            _ => {
                return Err("Invalid data type. Use 'all', 'users', 'posts', or 'comments'".into());
            }
        }

        println!("Database seeding completed!");
        Ok(())
    }
}
