use clap::Args;
use rust_social_network::SocialNetwork;

#[derive(Args)]
#[command(about = "Start the web server")]
pub struct ServeCommand {
    #[arg(short, long, default_value = "3000")]
    pub port: u16,

    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,

    #[arg(long, default_value = "false")]
    pub debug: bool,
}

impl ServeCommand {
    pub async fn execute(&self, app: &SocialNetwork) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting server on {}:{}", self.host, self.port);
        
        if self.debug {
            println!("Debug mode enabled");
        }

        app.start_server(&self.host, self.port).await?;
        println!("Server started successfully!");
        Ok(())
    }
}
