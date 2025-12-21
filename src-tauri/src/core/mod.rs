pub mod configurator;
pub mod domain;
pub mod provided_ports;
pub mod required_ports;
pub mod services;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Domain error: {0}")]
    DomainError(&'static str),

    #[error("Infrastructure error: {0}")]
    InfrastructureError(&'static str),
}
