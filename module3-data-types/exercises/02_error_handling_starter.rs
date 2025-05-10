use std::num::ParseIntError;
use std::fmt;

// Part 1: Configuration Parser
// Create a configuration struct and parser that returns Results

// Config struct - already defined
#[derive(Debug)]
struct Config {
    username: String,
    timeout: u32,
    max_retries: u32,
}

// Custom error type for configuration parsing errors - already defined with variants
#[derive(Debug)]
enum ConfigError {
    MissingField(String),
    InvalidTimeout(String),
    InvalidRetryCount(String),
}

// Display implementation for ConfigError - just needs message content
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::MissingField(field) => write!(f, "Missing required field: {}", field),
            ConfigError::InvalidTimeout(val) => write!(f, "Invalid timeout value: {}", val),
            ConfigError::InvalidRetryCount(val) => write!(f, "Invalid retry count: {}", val),
        }
    }
}

// Parse configuration string function - implementation needed
fn parse_config(config_str: &str) -> Result<Config, ConfigError> {
    let mut username = None;
    let mut timeout = None;
    let mut max_retries = None;

    // Split the configuration string by commas and process each key-value pair
    for pair in config_str.split(',') {
        let parts: Vec<&str> = pair.split('=').collect();
        if parts.len() != 2 {
            continue; // Skip invalid pairs
        }

        let key = parts[0].trim();
        let value = parts[1].trim();

        match key {
            "username" => username = Some(value.to_string()),
            "timeout" => {
                match value.parse::<u32>() {
                    Ok(val) => timeout = Some(val),
                    Err(_) => return Err(ConfigError::InvalidTimeout(value.to_string())),
                }
            },
            "max_retries" => {
                match value.parse::<u32>() {
                    Ok(val) => max_retries = Some(val),
                    Err(_) => return Err(ConfigError::InvalidRetryCount(value.to_string())),
                }
            },
            _ => {} // Ignore unknown keys
        }
    }

    let username = username.ok_or(ConfigError::MissingField("username".to_string()))?;
    let timeout = timeout.ok_or(ConfigError::MissingField("timeout".to_string()))?;
    let max_retries = max_retries.ok_or(ConfigError::MissingField("max_retries".to_string()))?;

    Ok(Config {
        username,
        timeout,
        max_retries,
    })
}

// Part 2: Safe String to Integer Conversion
// Create a function that safely converts a string to an integer

fn parse_number(s: &str) -> Option<i32> {
    match s.parse::<i32>() {
        Ok(number) => Some(number),
        Err(_) => None,
    }
}

// Part 3: Data Validation with Custom Errors
// Implement a user data validator with multiple error types

// User struct - already defined
#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    age: u32,
}

// ValidationError enum - already defined with variants
#[derive(Debug)]
enum ValidationError {
    InvalidId,
    NameTooShort,
    InvalidAge,
}

// Display implementation for ValidationError
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::InvalidId => write!(f, /* TODO */),
            ValidationError::NameTooShort => write!(f, /* TODO */),
            ValidationError::InvalidAge => write!(f, /* TODO */),
        }
    }
}

// Validate user function - partially implemented
fn validate_user(user: &User) -> Result<(), ValidationError> {
    // Check ID validity
    if user.id == 0 {
        return /* TODO */;
    }
    
    if user.name.len() < 2 {
        return /* TODO */;
    }
    
    if user.age < 18 {
        return /* TODO */;
    }
    
    Ok(())
}

// Part 4: Error Propagation Chain
// Create a series of functions that use the ? operator to propagate errors

// ProcessError type
#[derive(Debug)]
enum ProcessError {
    ConfigError(ConfigError),
    ParseError(ParseIntError),
    ValidationError(ValidationError),
}

// From implementations for automatic conversions
impl From<ConfigError> for ProcessError {
    fn from(err: ConfigError) -> Self {
        ProcessError::ConfigError(err)
    }
}

impl From<ParseIntError> for ProcessError {
    fn from(err: ParseIntError) -> Self {
        ProcessError::ParseError(err)
    }
}

impl From<ValidationError> for ProcessError {
    fn from(err: ValidationError) -> Self {
        ProcessError::ValidationError(err)
    }
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProcessError::ConfigError(e) => write!(f, "Configuration error: {}", e),
            ProcessError::ParseError(e) => write!(f, "Parse error: {}", e),
            ProcessError::ValidationError(e) => write!(f, "Validation error: {}", e),
        }
    }
}

// Process data function - needs implementation with ? operator
fn process_data(config_str: &str, user_id: &str, user_name: &str, user_age: &str) -> Result<(), ProcessError> {
    let config = parse_config(config_str)?;
    
    let id: u32 = user_id.parse()?;
    let age: u32 = user_age.parse()?;
    
    let user = User {
        id,
        name: user_name.to_string(),
        age,
    };
    
    validate_user(&user)?;
    
    Ok(())
}

fn main() {
    // Part 1: Test the configuration parser
    println!("--- Part 1: Configuration Parser ---");
    let config_str = "username=alice,timeout=30,max_retries=5";
    match parse_config(config_str) {
        Ok(config) => println!("Config parsed successfully: {:?}", config),
        Err(e) => println!("Failed to parse config: {}", e),
    }
    
    let invalid_config = "username=bob,timeout=invalid,max_retries=5";
    match parse_config(invalid_config) {
        Ok(config) => println!("Config parsed successfully: {:?}", config),
        Err(e) => println!("Failed to parse config: {}", e),
    }
    
    // Part 2: Test the string to integer conversion
    println!("\n--- Part 2: String to Integer Conversion ---");
    let valid_number = "42";
    let invalid_number = "four";
    
    match parse_number(valid_number) {
        Some(num) => println!("Successfully parsed '{}' to {}", valid_number, num),
        None => println!("Failed to parse '{}'", valid_number),
    }
    
    match parse_number(invalid_number) {
        Some(num) => println!("Successfully parsed '{}' to {}", invalid_number, num),
        None => println!("Failed to parse '{}'", invalid_number),
    }
    
    // Part 3: Test the user validation
    println!("\n--- Part 3: User Validation ---");
    // Create and test valid and invalid users
    let valid_user = User {
        id: 1001,
        name: String::from("Charlie"),
        age: 25,
    };
    
    let invalid_user1 = User {
        id: 1002,
        name: String::from("D"), // Too short
        age: 30,
    };
    
    let invalid_user2 = User {
        id: 1003,
        name: String::from("Eve"),
        age: 17, // Too young
    };
    
    println!("Validating user: {:?}", valid_user);
    match validate_user(&valid_user) {
        Ok(()) => println!("User is valid"),
        Err(e) => println!("Validation error: {}", e),
    }
    
    println!("Validating user: {:?}", invalid_user1);
    match validate_user(&invalid_user1) {
        Ok(()) => println!("User is valid"),
        Err(e) => println!("Validation error: {}", e),
    }
    
    println!("Validating user: {:?}", invalid_user2);
    match validate_user(&invalid_user2) {
        Ok(()) => println!("User is valid"),
        Err(e) => println!("Validation error: {}", e),
    }
    
    // Part 4: Test the error propagation chain
    println!("\n--- Part 4: Error Propagation ---");
    let test_cases = [
        ("username=charlie,timeout=30,max_retries=5", "1001", "Charlie", "25"),
        ("username=diana,timeout=invalid,max_retries=5", "1002", "Diana", "30"),
        ("username=eve,timeout=30,max_retries=5", "invalid_id", "Eve", "22"),
        ("username=frank,timeout=30,max_retries=5", "1004", "F", "17"), // Invalid name (too short)
    ];
    
    for (config, id, name, age) in test_cases.iter() {
        println!("Processing: Config='{}', ID='{}', Name='{}', Age='{}'", config, id, name, age);
        match process_data(config, id, name, age) {
            Ok(()) => println!("  Success: Data processed successfully"),
            Err(e) => println!("  Error: {}", e),
        }
    }
}