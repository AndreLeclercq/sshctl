use anyhow::{Context, Result};
use super::ConnectionError;
use crate::config::{
    connection_exists, 
    remove_connection, 
};

pub fn remove(name: &str) -> Result<()> {
    let name_lower: String = name.to_lowercase();

    if !connection_exists(name_lower.to_string()).context("Error when check if connections name is already used")? {
        return Err(ConnectionError::NameNotFound(name_lower.to_string()).into());
    }

    remove_connection(name_lower.to_string())?;
    eprintln!("Connection '{}' remove with success !", name_lower);
    Ok(())
}
