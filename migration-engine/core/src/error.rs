#![deny(missing_docs)]

//! Errors for the migration core.

use crate::commands::CommandError;
use datamodel::messages::MessageCollection;
use migration_connector::ConnectorError;
use thiserror::Error;

/// Top-level result type for the migration core.
pub type CoreResult<T> = Result<T, Error>;

/// Top-level migration core error.
#[derive(Debug, Error)]
pub enum Error {
    /// Error from a connector.
    #[error("Error in connector: {0}")]
    ConnectorError(
        #[source]
        #[from]
        ConnectorError,
    ),

    /// Error from a migration command.
    #[error("Failure during a migration command: {0}")]
    CommandError(
        #[source]
        #[from]
        CommandError,
    ),

    /// Error from the datamodel parser.
    #[error("Error in datamodel: {}", .0)]
    DatamodelError(MessageCollection),

    /// IO error.
    #[error("Error performing IO: {:?}", .0)]
    IOError(#[source] anyhow::Error),
}

impl From<MessageCollection> for Error {
    fn from(v: MessageCollection) -> Self {
        Error::DatamodelError(v)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e.into())
    }
}
