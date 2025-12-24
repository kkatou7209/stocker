use std::sync::Arc;

use crate::core::provided_ports::{
    JournalUsecase, StocktakingUsecase, SupplierUsecase, SupplyUsecase,
};
use crate::core::required_ports::*;
use crate::core::services::stock::{
    JournalService, StocktakingService, SupplierService, SupplyService,
};

pub struct Stocker {
    supply_respository: Arc<dyn ForSupplyPersistence>,
    supplier_repository: Arc<dyn ForSupplierPersistence>,
    jorunal_repository: Arc<dyn ForJournalPersistence>,
    stocktaking_repository: Arc<dyn ForStocktakingPersistence>,
}

pub struct Ports<SupplyRepository, SupplierRepository, JournalRepository, StocktakingRepository>
where
    SupplyRepository: ForSupplyPersistence,
    SupplierRepository: ForSupplierPersistence,
    JournalRepository: ForJournalPersistence,
    StocktakingRepository: ForStocktakingPersistence,
{
    pub for_supply_persistence: SupplyRepository,
    pub for_supplier_persistence: SupplierRepository,
    pub for_journal_persistence: JournalRepository,
    pub for_stocktaking_persistence: StocktakingRepository,
}

impl Stocker {
    pub fn plug<SupplyRepository, SupplierRepository, JournalRepository, StocktakingRepository>(
        ports: Ports<
            SupplyRepository,
            SupplierRepository,
            JournalRepository,
            StocktakingRepository,
        >,
    ) -> Self
    where
        SupplyRepository: ForSupplyPersistence + Send + Sync + 'static,
        SupplierRepository: ForSupplierPersistence + Send + Sync + 'static,
        JournalRepository: ForJournalPersistence + Send + Sync + 'static,
        StocktakingRepository: ForStocktakingPersistence + Send + Sync + 'static,
    {
        Self {
            supply_respository: Arc::new(ports.for_supply_persistence),
            supplier_repository: Arc::new(ports.for_supplier_persistence),
            jorunal_repository: Arc::new(ports.for_journal_persistence),
            stocktaking_repository: Arc::new(ports.for_stocktaking_persistence),
        }
    }

    pub fn supply_usecase(&self) -> impl SupplyUsecase {
        let supply_repository = Arc::clone(&self.supply_respository);
        let supplier_repository = Arc::clone(&self.supplier_repository);
        SupplyService::new(supply_repository, supplier_repository)
    }

    pub fn supplier_usecase(&self) -> impl SupplierUsecase {
        SupplierService::new(Arc::clone(&self.supplier_repository))
    }

    pub fn journal_usecase(&self) -> impl JournalUsecase {
        JournalService::new(
            Arc::clone(&self.supply_respository),
            Arc::clone(&self.supplier_repository),
            Arc::clone(&self.jorunal_repository),
        )
    }

    pub fn stocktaking_usecase(&self) -> impl StocktakingUsecase {
        StocktakingService::new(
            Arc::clone(&self.supply_respository),
            Arc::clone(&self.supplier_repository),
            Arc::clone(&self.stocktaking_repository),
        )
    }
}
