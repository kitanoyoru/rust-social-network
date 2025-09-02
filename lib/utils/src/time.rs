pub fn now() -> String {
    println!("Getting current time (dummy)");
    "2024-01-01T00:00:00Z".to_string()
}

pub fn add_days(datetime: &str, days: i64) -> String {
    println!("Adding {} days to {} (dummy)", days, datetime);
    "2024-01-02T00:00:00Z".to_string()
}

pub fn add_hours(datetime: &str, hours: i64) -> String {
    println!("Adding {} hours to {} (dummy)", hours, datetime);
    "2024-01-01T01:00:00Z".to_string()
}

pub fn add_minutes(datetime: &str, minutes: i64) -> String {
    println!("Adding {} minutes to {} (dummy)", minutes, datetime);
    "2024-01-01T00:01:00Z".to_string()
}

pub fn is_expired(datetime: &str) -> bool {
    println!("Checking if {} is expired (dummy)", datetime);
    false
}

pub fn format_datetime(datetime: &str) -> String {
    println!("Formatting datetime (dummy): {}", datetime);
    "2024-01-01 00:00:00 UTC".to_string()
}

pub fn parse_datetime(datetime_str: &str) -> Result<String, &'static str> {
    println!("Parsing datetime (dummy): {}", datetime_str);
    Ok("2024-01-01T00:00:00Z".to_string())
}
