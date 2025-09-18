use anyhow::{Context, Result};
use thiserror::Error;
use crate::config::{
    connection_exists, get_connection, get_connection_file_content, remove_connection, upsert_connection, Config, Connection
};
use dialoguer::Input;
use std::{collections::HashMap, net::Ipv4Addr, str::FromStr};

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("Connection name '{0}', is already used. Use 'edit' to update the connection.")]
    NameAlreadyExist(String),

    #[error("Connection name '{0}', not found.")]
    NameNotFound(String),
}

pub fn add(name: &str) -> Result<()> { 
    let name_lower: String = name.to_lowercase();

    if connection_exists(name_lower.to_string()).context("Error when check if connections name is already used")? {
        return Err(ConnectionError::NameAlreadyExist(name_lower.to_string()).into());
    }
    eprintln!("Create new connection :");

    let host: Ipv4Addr = loop {
        let host_input: String = Input::new()
            .with_prompt("Host IP (required)")
            .interact_text()
            .unwrap();

        match Ipv4Addr::from_str(&host_input) {
            Ok(host) => break host,
            Err(_) => eprintln!("Invalid IP, Try again."),
        }
    };

    let port: u16 = loop { 
        let port_input: String = Input::new()
            .allow_empty(true)
            .with_prompt("Port (default: 22)")
            .interact_text()
            .unwrap();

        if port_input.trim().is_empty() {
            break 22;
        }

        match port_input.trim().parse::<u16>() {
            Ok(port) if port != 0 => break port,
            Ok(_) => break 22,
            Err(_) => eprintln!("Invalid Port, Try again."),
        }
    };

    let user: String = Input::new()
        .with_prompt("User (required)")
        .interact_text()
        .unwrap();

    let description: String = Input::new()
        .allow_empty(true)
        .with_prompt("Description (optionnal)")
        .interact_text()
        .unwrap();

    let mut connection = HashMap::new();
    connection.insert(
        name_lower.to_string(),
        Connection {
            host: host,
            port: port,
            user: user,
            description: Some(description),
        }
    );

    upsert_connection(connection).context("Error when add connection")?;
    eprintln!("New connection '{}' add with success !", name_lower);
    Ok(())
}

pub fn remove(name: &str) -> Result<()> {
    let name_lower: String = name.to_lowercase();

    if !connection_exists(name_lower.to_string()).context("Error when check if connections name is already used")? {
        return Err(ConnectionError::NameNotFound(name_lower.to_string()).into());
    }

    remove_connection(name_lower.to_string())?;
    eprintln!("Connection '{}' remove with success !", name_lower);
    Ok(())
}

pub fn edit(name: &str) -> Result<()> {
    let name_lower: String = name.to_lowercase();

    if !connection_exists(name_lower.to_string()).context("Error when check if connections name is already used")? {
        return Err(ConnectionError::NameNotFound(name_lower.to_string()).into());
    }

    let existing_connection = get_connection(name_lower.to_string())?;
    if let Some(connection) = existing_connection.get(&name_lower) {    

        let host: Ipv4Addr = loop {
            let host_input: String = Input::new()
                .with_prompt("Host IP (required)")
                .with_initial_text(connection.host.to_string())
                .interact_text()
                .unwrap();

            match Ipv4Addr::from_str(&host_input) {
                Ok(host) => break host,
                Err(_) => eprintln!("Invalid IP, Try again."),
            }
        };

        let port: u16 = loop { 
            let port_input: String = Input::new()
                .allow_empty(true)
                .with_prompt("Port (default: 22)")
                .with_initial_text(connection.port.to_string())
                .interact_text()
                .unwrap();

            if port_input.trim().is_empty() {
                break 22;
            }

            match port_input.trim().parse::<u16>() {
                Ok(port) if port != 0 => break port,
                Ok(_) => break 22,
                Err(_) => eprintln!("Invalid Port, Try again."),
            }
        };

        let user: String = Input::new()
            .with_prompt("User (required)")
            .with_initial_text(connection.user.to_string())
            .interact_text()
            .unwrap();

        let description: String = Input::new()
            .allow_empty(true)
            .with_prompt("Description (optionnal)")
            .with_initial_text(connection.description.as_deref().unwrap_or(""))
            .interact_text()
            .unwrap();

        let mut connection = HashMap::new();
        connection.insert(
            name_lower.to_string(),
            Connection {
                host: host,
                port: port,
                user: user,
                description: Some(description),
            }
        );        
        
        upsert_connection(connection)?;
    } 

    eprintln!("Connection '{}' was edited with success !", name_lower.to_string());
    Ok(())
}

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

pub fn list() -> Result<()> {
    let config_file: Config = get_connection_file_content().context("Error when getting connection file content.")?;
    if let Some(connections) = config_file.connection {
        eprintln!("List of connections:");
        for name in connections.keys() {
            eprintln!("{}", name);
        }
    }
    Ok(())
}
