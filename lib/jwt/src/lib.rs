use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub jti: String,
    pub user_id: Uuid,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

pub struct JwtManager {
    secret: String,
    refresh_secret: String,
}

impl JwtManager {
    pub fn new(secret: String, refresh_secret: String) -> Self {
        println!("JWT Manager initialized (dummy)");
        Self {
            secret,
            refresh_secret,
        }
    }

    pub fn generate_tokens(&self, user_id: Uuid, email: &str) -> Result<TokenPair> {
        println!("Generating tokens (dummy) for user: {}", user_id);
        Ok(TokenPair {
            access_token: "dummy_access_token".to_string(),
            refresh_token: "dummy_refresh_token".to_string(),
            expires_in: 3600,
        })
    }

    pub fn verify_token(&self, _token: &str) -> Result<Claims> {
        println!("Verifying token (dummy)");
        Ok(Claims {
            sub: "dummy".to_string(),
            exp: 0,
            iat: 0,
            jti: "dummy".to_string(),
            user_id: Uuid::new_v4(),
            email: "dummy@example.com".to_string(),
        })
    }

    pub fn refresh_access_token(&self, _refresh_token: &str) -> Result<String> {
        println!("Refreshing token (dummy)");
        Ok("dummy_new_access_token".to_string())
    }

    pub fn extract_user_id_from_token(&self, _token: &str) -> Result<Uuid> {
        println!("Extracting user ID (dummy)");
        Ok(Uuid::new_v4())
    }
}
