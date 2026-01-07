import { useNavigate } from '@solidjs/router';
import { error } from '@tauri-apps/plugin-log';
import { createStore } from 'solid-js/store';

const [state, setState] = createStore<{
	error: Error | null;
	reset: () => void;
}>({
	error: null,
	reset: () => setState({ error: null }),
});

const handle = async (err: Error, reset?: () => unknown) => {
	console.error(err);
	const navigation = useNavigate();

	setState({
		error: err,
		reset: () => {
			setState({ error: null });
			return reset?.();
		},
	});

	await error(`${err.message}\n${err.stack}`);

	navigation('/error');
};

export const useError = () => {
	return {
		error: state.error,
		reset: state.reset,
		handle,
	};
};
