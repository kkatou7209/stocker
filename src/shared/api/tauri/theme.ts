import { invoke } from '@tauri-apps/api/core';

/**
 * Application theme type.
 */
export type Theme = 'light' | 'dark';

/**
 * Get the current application theme from the config file.
 */
export const getCurrentTheme = async (): Promise<Theme> => {
	const theme = await invoke<Theme>('get_current_theme');

	return theme;
};

/**
 * Set the application theme in the config file.
 */
export const setTheme = async (theme: Theme): Promise<void> => {
	await invoke<void>('set_theme', { theme });
};
