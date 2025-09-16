use std::{collections::HashMap, path::PathBuf, fs, net::Ipv4Addr};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub connection: Option<HashMap<String, Connection>>
}

#[derive(Serialize, Deserialize)]
pub struct Connection {
    pub host: Ipv4Addr,
    pub port: u16,
    pub user: String,
    pub description: Option<String>,
}

pub fn connection_exists(name: String) -> Result<bool> {
    let connections_path: PathBuf = config_connections_path().context("Error when get connections.toml path")?;
    let content = fs::read_to_string(connections_path).context("Error when read connections.toml file.")?;
    let config: Config = toml::from_str(&content)?;
    Ok(config.connection.map_or(false, |connection| {
        connection.keys().any(|key| key.to_lowercase() == name.to_lowercase())
    }))
}

pub fn add_connection(connection: HashMap<String, Connection>) -> Result<()> {
    let mut config_file: Config = get_connection_file_content().context("Error when getting connection file content")?;
    let connections = config_file.connection.get_or_insert_with(HashMap::new);
    connections.extend(connection);
    let toml = toml::to_string(&config_file)?;
    let connections_path: PathBuf = config_connections_path().context("Error when get config path")?;
    fs::write(connections_path, toml).context("Error when write into config file")?;
    Ok(())
}

pub fn remove_connection(name: String) -> Result<()> {
    let mut config_file: Config = get_connection_file_content().context("Error when getting connection file content")?;
    if let Some(ref mut connection) = config_file.connection {
        connection.remove(&name);
    }
    let toml = toml::to_string(&config_file)?;
    let connections_path: PathBuf = config_connections_path().context("Error when get config path")?;
    fs::write(connections_path, toml).context("Error when write into config file")?;
    Ok(())
}

fn get_connection_file_content() -> Result<Config> {
    let connections_path: PathBuf = config_connections_path().context("Error when get config path")?;
    let content = fs::read_to_string(&connections_path).context("Error when read connections.toml file.")?;

    let config_file: Config = if content.is_empty() {
        Config::default()
    } else {
        toml::from_str(&content).context("Error when parsing TOML")?
    };
    Ok(config_file)
}

fn config_directory_path() -> Result<PathBuf> {
    let mut config_path: PathBuf = dirs::config_dir().context("Error to get config directory path.")?;
    config_path.push("sshctl");
    
    if !fs::exists(&config_path)? {
        fs::create_dir_all(&config_path).context("Error to create sshctl directory.")?; 
    } 
    Ok(config_path)
}

fn config_connections_path() -> Result<PathBuf> {
    let mut connections_path: PathBuf = config_directory_path().context("Error when get config path")?;
    connections_path.push("connections.toml");
    
    if !fs::exists(&connections_path)? {
        fs::write(&connections_path, "").context("Error to create connections.toml file.")?;
    }
    Ok(connections_path)
}
