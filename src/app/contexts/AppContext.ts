import { type Accessor, createContext, useContext } from 'solid-js';

export type AppContextValue = Readonly<{
	pageTitle: Accessor<string>;
	isDrawerOpen: Accessor<boolean>;
	drawer: Readonly<{
		open: () => void;
		close: () => void;
		toggle: () => void;
	}>;
	setPageTitle: (title: string) => void;
}>;

export const AppContext = createContext<AppContextValue>();

export const useApp = (): AppContextValue => {
	const context = useContext(AppContext);

	if (!context) throw new Error('missing context AppContext');

	return context;
};
