pub mod domain;
pub mod provided_ports;
pub mod required_ports;
pub(crate) mod services;
pub mod stocker;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Domain error: {0}")]
    DomainError(&'static str),

    #[error("Infrastructure error: {0}")]
    InfrastructureError(&'static str),

    #[error("Configuration error: {0}")]
    ConfigurationError(&'static str),
}
