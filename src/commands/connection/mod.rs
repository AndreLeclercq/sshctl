pub mod add;
pub mod remove;
pub mod edit;
pub mod show;
pub mod list;

pub use add::add;
pub use remove::remove;
pub use edit::edit;
pub use show::show;
pub use list::list;

use std::{fs, path::Path};
use dialoguer::Completion;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("Connection name '{0}', is already used. Use 'edit' to update the connection.")]
    NameAlreadyExist(String),

    #[error("Connection name '{0}', not found.")]
    NameNotFound(String),
}

pub fn expand_tilde(path: &str) -> String {
    if path.starts_with("~/") {
        if let Some(home) = std::env::var_os("HOME")
            .or_else(|| std::env::var_os("USERPROFILE"))
        {
            return path.replacen("~", &home.to_string_lossy(), 1);
        }
    }
    path.to_string()
}

fn find_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    if strings.len() == 1 {
        return strings[0].clone();
    }
    
    let mut prefix = String::new();
    let first = strings[0].chars().collect::<Vec<_>>();
    
    for (i, ch) in first.iter().enumerate() {
        if strings.iter().all(|s| {
            s.chars().nth(i).map(|c| c == *ch).unwrap_or(false)
        }) {
            prefix.push(*ch);
        } else {
            break;
        }
    }
    prefix
}

struct PathCompletion;

impl Completion for PathCompletion {
    fn get(&self, input: &str) -> Option<String> {
        if input.is_empty() { return None; }
        
        let expanded = expand_tilde(input);
        let path = Path::new(&expanded);
        
        let (dir, prefix) = if expanded.ends_with('/') || expanded.ends_with('\\') {
            (path, "")
        } else {
            (path.parent()?, path.file_name()?.to_str()?)
        };
        
        let matches: Vec<String> = fs::read_dir(dir).ok()?
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                if name.starts_with(prefix) {
                    Some(name)
                } else {
                    None
                }
            })
            .collect();
        
        if matches.is_empty() {
            return None;
        }

        let common = find_common_prefix(&matches); 
        let completed_path = dir.join(&common);
        let full_path = completed_path.to_str()?.to_string();
        
        if input.starts_with("~/") {
            if let Some(home) = std::env::var_os("HOME")
                .or_else(|| std::env::var_os("USERPROFILE"))
            {
                let home_str = home.to_string_lossy();
                if full_path.starts_with(home_str.as_ref()) {
                    return Some(full_path.replacen(&home_str.to_string(), "~", 1));
                }
            }
        }
        
        Some(full_path)
    }
}
