use anyhow::{Context, Result};
use crate::ssh::connections::{
    get_connection_file_content, 
    Config,
};

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
