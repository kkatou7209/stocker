import type { Component } from 'solid-js';
import { useApp } from '@/app/contexts/AppContext';

const StocktakingPage: Component = () => {
	const app = useApp();

	app.setPageTitle('棚卸');

	return <article></article>;
};

export default StocktakingPage;
