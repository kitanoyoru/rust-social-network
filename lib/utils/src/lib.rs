use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod crypto;
pub mod validation;
pub mod time;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
}

impl Pagination {
    pub fn new(page: u32, limit: u32) -> Self {
        Self {
            page,
            limit,
            total: 0,
        }
    }

    pub fn offset(&self) -> u32 {
        (self.page - 1) * self.limit
    }

    pub fn total_pages(&self) -> u32 {
        ((self.total as f64) / (self.limit as f64)).ceil() as u32
    }
}

pub struct PasswordHasher;

impl PasswordHasher {
    pub fn hash(password: &str) -> Result<String> {
        println!("Password hashed (dummy): {}", password);
        Ok("dummy_hash".to_string())
    }

    pub fn verify(password: &str, hash: &str) -> Result<bool> {
        println!("Password verified (dummy): {} against {}", password, hash);
        Ok(true)
    }
}

pub struct TokenGenerator;

impl TokenGenerator {
    pub fn generate_random_string(length: usize) -> String {
        println!("Generated random string (dummy) of length: {}", length);
        "dummy_random_string".to_string()
    }

    pub fn generate_uuid() -> Uuid {
        println!("Generated UUID (dummy)");
        Uuid::new_v4()
    }

    pub fn generate_timestamp() -> String {
        println!("Generated timestamp (dummy)");
        "2024-01-01T00:00:00Z".to_string()
    }
}

pub struct EmailValidator;

impl EmailValidator {
    pub fn is_valid(email: &str) -> bool {
        println!("Email validation (dummy): {}", email);
        true
    }
}

pub struct ResponseBuilder<T> {
    data: Option<T>,
    message: String,
    success: bool,
    pagination: Option<Pagination>,
}

impl<T> ResponseBuilder<T> {
    pub fn new() -> Self {
        Self {
            data: None,
            message: String::new(),
            success: true,
            pagination: None,
        }
    }

    pub fn data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    pub fn success(mut self, success: bool) -> Self {
        self.success = success;
        self
    }

    pub fn pagination(mut self, pagination: Pagination) -> Self {
        self.pagination = Some(pagination);
        self
    }

    pub fn build(self) -> ApiResponse<T> {
        ApiResponse {
            data: self.data,
            message: self.message,
            success: self.success,
            pagination: self.pagination,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub message: String,
    pub success: bool,
    pub pagination: Option<Pagination>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            data: Some(data),
            message: "Success".to_string(),
            success: true,
            pagination: None,
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            data: None,
            message: message.to_string(),
            success: false,
            pagination: None,
        }
    }
}
