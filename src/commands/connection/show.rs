use anyhow::{Context, Result};
use super::ConnectionError;
use crate::config::{
    connection_exists, 
    get_connection,
};

pub fn show(name: &str) -> Result<()> {
    let name_lower: String = name.to_lowercase();

    if !connection_exists(name_lower.to_string()).context("Error when check if connections name is already used")? {
        return Err(ConnectionError::NameNotFound(name_lower.to_string()).into());
    }

    let existing_connection = get_connection(name_lower.to_string())?;
    if let Some(connection) = existing_connection.get(&name_lower) { 
        eprintln!("Connection {}", name_lower.to_string());
        eprintln!("Host: {}", connection.host.to_string());
        eprintln!("Port: {}", connection.port.to_string());
        eprintln!("User: {}", connection.user.to_string());
        eprintln!("Description: {}", connection.description.as_deref().unwrap_or(""));
    }

    Ok(())
}
