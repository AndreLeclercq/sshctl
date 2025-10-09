pub mod connection;

use anyhow::{Result, Error, Context};
use crate::ssh::connections::{get_connection, connection_exists};
use std::{os::unix::process::CommandExt, process::Command};

pub fn connect(name: &str) -> Result<()> {
    let name_lower: String = name.to_lowercase();

    if !connection_exists(name_lower.to_string()).context("Error when check if connections name is already used")? {
        return Err(connection::ConnectionError::NameNotFound(name_lower.to_string()).into());
    }

    let existing_connection = get_connection(name_lower.to_string())?;
    if let Some(connection) = existing_connection.get(&name_lower) {
        eprintln!("Connect ssh {}@{} -p {}", connection.user, connection.host, connection.port);
        let error = Command::new("ssh")
            .arg(format!("{}@{}", connection.user, connection.host))
            .arg("-p")
            .arg(connection.port.to_string())
            .exec();
        Err(Error::new(error)).context("Can't launch 'ssh'")?
    }
    Ok(())
}
