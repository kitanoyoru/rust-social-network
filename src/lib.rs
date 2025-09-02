pub mod api;
pub mod entity;
pub mod repository;
pub mod service;
pub mod usecase;

use cache::Cache;
use database::Database;
use jwt::JwtManager;
use utils::{ApiResponse, PasswordHasher, TokenGenerator};

pub struct SocialNetwork {
    cache: Option<Cache>,
    database: Option<Database>,
    jwt_manager: Option<JwtManager>,
}

impl SocialNetwork {
    pub fn new() -> Self {
        Self {
            cache: None,
            database: None,
            jwt_manager: None,
        }
    }

    pub async fn initialize(&mut self, redis_url: Option<&str>, database_url: Option<&str>, jwt_secret: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(redis_url) = redis_url {
            self.cache = Some(Cache::new(redis_url).await?);
        }

        if let Some(database_url) = database_url {
            self.database = Some(Database::new(database_url).await?);
        }

        if let Some(jwt_secret) = jwt_secret {
            self.jwt_manager = Some(JwtManager::new(jwt_secret.to_string(), format!("{}_refresh", jwt_secret)));
        }

        Ok(())
    }

    pub async fn start_server(&self, host: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting social network server on {}:{}", host, port);
        
        if let Some(ref cache) = self.cache {
            println!("Cache initialized");
        }
        
        if let Some(ref database) = self.database {
            println!("Database initialized");
        }
        
        if let Some(ref jwt_manager) = self.jwt_manager {
            println!("JWT manager initialized");
        }

        Ok(())
    }

    pub async fn run_migrations(&self, direction: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Running migrations: {}", direction);
        
        if let Some(ref database) = self.database {
            database.run_migrations().await?;
        }

        Ok(())
    }

    pub async fn seed_database(&self, data_type: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Seeding database with: {}", data_type);
        
        let user_id = TokenGenerator::generate_uuid();
        let hashed_password = PasswordHasher::hash("password123")?;
        
        println!("Generated user ID: {}", user_id);
        println!("Hashed password: {}", hashed_password);

        Ok(())
    }
}
