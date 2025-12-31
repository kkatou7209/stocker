mod jorunal;
mod migration;
mod stocktaking;
mod supplier;
mod supply;

#[cfg(test)]
mod supplier_test;

#[cfg(test)]
mod supply_test;

#[cfg(test)]
mod journal_test;

#[cfg(test)]
mod stocktaking_test;

pub use jorunal::*;
pub use migration::*;
pub use stocktaking::*;
pub use supplier::*;
pub use supply::*;

impl From<crate::core::Error> for rusqlite::Error {
    fn from(value: crate::core::Error) -> Self {
        rusqlite::Error::ToSqlConversionFailure(Box::new(value))
    }
}
