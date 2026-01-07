import { Navigate, type RouteDefinition } from '@solidjs/router';
import { ErrorPage } from '@/pages/Error';
import EntryJournalPage from '@/pages/stock/ui/EntryJournalPage';
import JournalListPage from '@/pages/stock/ui/JournalListPage';
import StocktakingListPage from '@/pages/stock/ui/StocktakingListPage';
import StocktakingPage from '@/pages/stock/ui/StocktakingPage';
import SupplierListPage from '@/pages/stock/ui/SupplierListPage';
import SupplyListPage from '@/pages/stock/ui/SupplyListPage';

export const routes: RouteDefinition[] = [
	{
		path: '/error',
		component: ErrorPage,
	},
	{
		path: '/',
		component: () => <Navigate href="/journal" />,
	},
	{
		path: '/journal',
		component: EntryJournalPage,
	},
	{
		path: '/journal/list',
		component: JournalListPage,
	},
	{
		path: '/stocktaking',
		component: StocktakingPage,
	},
	{
		path: '/stocktaking/list',
		component: StocktakingListPage,
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
