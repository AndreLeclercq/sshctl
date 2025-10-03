use anyhow::{Context, Result};
use super::ConnectionError;
use crate::config::{
    connection_exists, 
    get_connection, 
    upsert_connection, 
    Connection
};
use dialoguer::Input;
use std::{collections::HashMap, net::Ipv4Addr, str::FromStr};


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
