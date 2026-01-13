import { platform } from '@tauri-apps/plugin-os';
import { relaunch } from '@tauri-apps/plugin-process';
import { check, type DownloadEvent } from '@tauri-apps/plugin-updater';
import { type Component, createSignal, onMount } from 'solid-js';

/**
 * Component that handles application updates
 */
export const Updater: Component = () => {
	// Whether the update modal is open
	const [open, setOpen] = createSignal(false);

	// Total content length of the update
	const [contentLength, setContentLength] = createSignal(1);

	// Length of data downloaded
	const [downloadedLength, setDownloadedLength] = createSignal(0);

    const [state, setState] = createSignal<'progress' | 'finished'>('progress');

	onMount(async () => {
		const os = platform();

        // Windows app installed quietly
		if (os === 'windows') return;

		const update = await check();

		if (!update) return;

		setOpen(true);

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
            }, 1000);

		})();
	});

	return (
		<dialog class="modal" open={open()}>
			<section class="modal-box w-50vw flex flex-col items-center gap-6 p-10">
				<h3 class="text-xl mb-4">アップデートがあります。</h3>
				<p>新バージョンをインストール中です。</p>
				{/** biome-ignore lint/a11y/useAriaPropsSupportedByRole: required for DaisyUI */}
				<div
					class="radial-progress text-primary"
					aria-valuenow={(downloadedLength() / contentLength()) * 100}
				>
					{Math.floor((downloadedLength() / contentLength()) * 100)}%
				</div>
			</section>
		</dialog>
	);
};
