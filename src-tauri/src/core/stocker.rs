use std::sync::Arc;

use crate::core::provided_ports::*;
use crate::core::required_ports::*;
use crate::core::services::stock::*;

pub struct Stocker {
    supply_respository: Arc<dyn ForSupplyPersistence + Send + Sync + 'static>,
    supplier_repository: Arc<dyn ForSupplierPersistence + Send + Sync + 'static>,
    jorunal_repository: Arc<dyn ForJournalPersistence + Send + Sync + 'static>,
    stocktaking_repository: Arc<dyn ForStocktakingPersistence + Send + Sync + 'static>,
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
        let supplier_repository = Arc::clone(&self.supplier_repository);

        SupplierService::new(supplier_repository)
    }

    pub fn journal_usecase(&self) -> impl JournalUsecase {
        let supply_repository = Arc::clone(&self.supply_respository);
        let supplier_repository = Arc::clone(&self.supplier_repository);
        let journal_repository = Arc::clone(&self.jorunal_repository);

        JournalService::new(supply_repository, supplier_repository, journal_repository)
    }

    pub fn stocktaking_usecase(&self) -> impl StocktakingUsecase {
        let supply_repository = Arc::clone(&self.supply_respository);
        let stocktaking_repository = Arc::clone(&self.stocktaking_repository);

        StocktakingService::new(supply_repository, stocktaking_repository)
    }
}
