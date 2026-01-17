import { platform } from '@tauri-apps/plugin-os';
import { relaunch } from '@tauri-apps/plugin-process';
import {
	check,
	type DownloadEvent,
	type Update,
} from '@tauri-apps/plugin-updater';
import { type Component, createSignal, onMount, Show } from 'solid-js';
import { useApp } from '../contexts/AppContext';

/**
 * Component that handles application updates
 */
export const Updater: Component = () => {
	const app = useApp();

	// Whether the update modal is open
	const [open, setOpen] = createSignal(false);

	// Total content length of the update
	const [contentLength, setContentLength] = createSignal(1);

	// Length of data downloaded
	const [downloadedLength, setDownloadedLength] = createSignal(0);

	// Current percentage of download
	const [percentage, setPercentage] = createSignal(0);

	const [state, setState] = createSignal<
		'confirming' | 'progress' | 'finished'
	>('confirming');

	// Updater instance
	const [updater, setUpdater] = createSignal<Update | null>(null);

	const update = async () => {
		const update = updater();

		if (!update) {
			app.handleError(
				'システムエラーが発生しました。',
				new Error('Updater is not set'),
			);
			return;
		}

		setState('progress');

		// Start download and installation
		(async () => {
			await update.downloadAndInstall((event: DownloadEvent) => {
				switch (event.event) {
					case 'Started':
						if (event.data.contentLength) {
							setContentLength(event.data.contentLength);
						}
						break;
					case 'Progress':
						setDownloadedLength(
							(prev) => prev + event.data.chunkLength,
						);
						setPercentage(
							(downloadedLength() / contentLength()) * 100,
						);
						break;
					case 'Finished':
						setOpen(false);
						break;
				}
			});

			setState('finished');

			const id = setTimeout(() => {
				clearTimeout(id);
				relaunch();
			}, 3000);
		})();
	};

	onMount(async () => {
		const os = platform();

		// Windows app installed quietly
		if (os === 'windows') return;

		const update = await check();

		if (!update) return;

		setUpdater(update);

		setOpen(true);
	});

	return (
		<dialog class="modal" open={open()}>

			{/* Update confirmation */}
			<Show when={state() === 'confirming'}>
				<section class="modal-box w-50vw flex flex-col items-center gap-6 p-10">
					<h3 class="text-xl mb-4">新しいアップデートがあります。</h3>
					<p>アップデートをダウンロードしますか？</p>
					<section class="w-full flex justify-between mt-5">
						<button
							type="button"
							class="btn btn-ghost"
							onclick={() => {
								setOpen(false);
							}}
						>
							キャンセル
						</button>
						<button
							type="button"
							class="btn btn-primary"
							onclick={update}
						>
							ダウンロード
						</button>
					</section>
				</section>
			</Show>

			{/* Download progress */}
			<Show when={state() === 'progress'}>
				<section class="modal-box w-50vw flex flex-col items-center gap-6 p-10">
					<h3 class="text-xl mb-4">
						新バージョンをインストール中です。
					</h3>
					<p>しばらくお待ちください。</p>
					{/** biome-ignore lint/a11y/useAriaPropsSupportedByRole: required for DaisyUI */}
					<div
						class="radial-progress text-primary"
						aria-valuenow={percentage()}
						style={{
							"--value": percentage()
						}}
					>
						{Math.floor(percentage())}%
					</div>
				</section>
			</Show>

			{/* Finished message */}
			<Show when={state() === 'finished'}>
				<section class="modal-box w-50vw flex flex-col justify-center items-center gap-6 p-10">
					<h3 class="text-xl">インストールが完了しました。</h3>
					<p>アプリケーションを再起動します。</p>
				</section>
			</Show>
		</dialog>
	);
};
