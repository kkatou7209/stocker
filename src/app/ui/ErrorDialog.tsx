import type { Component } from 'solid-js';

/**
 * Common error dialog component.
 */
export const ErrorDialog: Component<{
	message?: string | null;
	open?: boolean;
	reset?: () => unknown;
}> = (props) => {

	return (
		<dialog class='modal' open={props.open ?? false}>
			<article class="modal-box flex flex-col items-center justify-center p-10">
				<section class="flex flex-col items-center justify-center gap-10 rounded-md">
					<h1 class="text-2xl font-bold">エラーが発生しました</h1>
					<p class="text-secondary">{props.message}</p>
					<button type="button" class="btn" onClick={props.reset}>
						戻る
					</button>
				</section>
			</article>
		</dialog>
	);
};
