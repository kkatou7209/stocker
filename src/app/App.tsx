import { type Component, createSignal } from 'solid-js';
import '@/app/App.css';
import { Router } from '@solidjs/router';
import { routes } from '@/app/routes';
import SideNavigation from '@/app/ui/SideNavigation';
import { AppContext, type AppContextValue } from './contexts/AppContext';
import AppBar from './ui/AppBar';

const App: Component = () => {
	const [pageTitle, setPageTitle] = createSignal('');

	const [isDrawerOpen, setOpen] = createSignal(false);

	const contextValue: AppContextValue = {
		pageTitle,
		isDrawerOpen,
		drawer: {
			open: () => setOpen(true),
			close: () => setOpen(false),
			toggle: () => setOpen(!isDrawerOpen()),
		},
		setPageTitle,
	};

	return (
		<AppContext.Provider value={contextValue}>
			<SideNavigation open={isDrawerOpen}>
				<AppBar />
				<main class='w-full'>
					<Router>{routes}</Router>
				</main>
			</SideNavigation>
		</AppContext.Provider>
	);
};

export default App;
