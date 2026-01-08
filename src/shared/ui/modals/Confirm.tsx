import { XIcon } from 'lucide-solid';
import { children, type ParentComponent } from 'solid-js';

export const Confirm: ParentComponent<{
	open?: boolean;
	onCancel?: () => unknown;
	onConfirm?: () => unknown;
}> = (props) => {
	const resolved = children(() => props.children);

	return (
		<dialog class="modal" open={props.open}>
			<div class="modal-box max-h-[70vh] flex flex-col gap-6 p-10 min-w-50">
                <button type='button' class='hover:cursor-pointer hover:opacity-50 absolute right-4 top-4' onclick={props.onCancel}>
                    <XIcon />
                </button>
				<section class="w-full">{resolved()}</section>
				<section class="modal-action flex justify-between">
					<button
						type="button"
						class="btn btn-ghost"
						onclick={props.onCancel}
					>
						キャンセル
					</button>
					<button 
						type="button"
						class="btn btn-secondary"
						onclick={props.onConfirm}
					>
						はい
					</button>
				</section>
			</div>
		</dialog>
	);
};
