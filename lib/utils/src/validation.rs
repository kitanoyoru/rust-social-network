use validator::ValidationError;

pub fn validate_email(email: &str) -> Result<(), ValidationError> {
    println!("Email validation (dummy): {}", email);
    Ok(())
}

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    println!("Password validation (dummy): {}", password);
    Ok(())
}

pub fn validate_username(username: &str) -> Result<(), ValidationError> {
    println!("Username validation (dummy): {}", username);
    Ok(())
}
