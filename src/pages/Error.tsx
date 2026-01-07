import { type Component, createSignal, onMount } from 'solid-js';
import { useError } from '@/app/stores/error';

export const ErrorPage: Component = () => {
	const error = useError();

	const [show, setShow] = createSignal(false);

	onMount(() => {
		setShow(true);
	});

	return (
		<article class="size-full flex flex-col items-center justify-center">
			<div
				class={`transition-transform duration-300 ease-in ${show() ? 'scale-100' : 'scale-90'}`}
			>
				<section class="flex flex-col items-center justify-center gap-10 rounded-md border border-gray-200 p-30 shadow-md">
					<h1 class="text-2xl font-bold">エラーが発生しました</h1>
					<p class="text-secondary">{error.error?.message}</p>
					<button type="button" class='btn' onClick={error.reset}>やり直す</button>
				</section>
			</div>
		</article>
	);
};
