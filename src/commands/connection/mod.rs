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

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("Connection name '{0}', is already used. Use 'edit' to update the connection.")]
    NameAlreadyExist(String),

    #[error("Connection name '{0}', not found.")]
    NameNotFound(String),
}
