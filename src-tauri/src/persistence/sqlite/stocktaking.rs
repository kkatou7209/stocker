use std::path::Path;

use crate::core::domain::values::stock::*;
use crate::core::required_ports::*;
use crate::core::Result;

pub struct SqliteStocktakingRepository {
    db_path: String,
}

impl SqliteStocktakingRepository {
    pub fn new(db_path: impl AsRef<str>) -> Self {
        Self {
            db_path: Path::new(db_path.as_ref()).to_string_lossy().to_string(),
        }
    }
}

impl ForStocktakingPersistence for SqliteStocktakingRepository {
    fn next_id(&self) -> Result<StocktakingId> {
        todo!()
    }

    fn list(&self) -> Result<Vec<crate::core::domain::entities::stock::Stocktaking>> {
        todo!()
    }

    fn find(
        &self,
        query: FindStocktakingsQuery,
    ) -> Result<Vec<crate::core::domain::entities::stock::Stocktaking>> {
        todo!()
    }

    fn get(
        &self,
        query: GetStocktakingQuery,
    ) -> Result<Option<crate::core::domain::entities::stock::Stocktaking>> {
        todo!()
    }

    fn add(&self, stockatking: crate::core::domain::entities::stock::Stocktaking) -> Result<()> {
        todo!()
    }

    fn save(&self, stocktaking: crate::core::domain::entities::stock::Stocktaking) -> Result<()> {
        todo!()
    }
}
