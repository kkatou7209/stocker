import { Navigate, type RouteDefinition } from '@solidjs/router';
import EntryJournalPage from '@/pages/stock/ui/EntryJournalPage';
import StocktakingPage from '@/pages/stock/ui/StocktakingPage';
import SupplierListPage from '@/pages/stock/ui/SupplierListPage';
import SupplyListPage from '@/pages/stock/ui/SupplyListPage';

export const routes: RouteDefinition[] = [
	{
		path: '/',
		component: () => <Navigate href="/journal" />,
	},
	{
		path: '/journal',
		component: EntryJournalPage,
	},
	{
		path: '/stocktaking',
		component: StocktakingPage,
	},
	{
		path: '/supply',
		component: SupplyListPage,
	},
	{
		path: '/supplier',
		component: SupplierListPage,
	},
] as const;
