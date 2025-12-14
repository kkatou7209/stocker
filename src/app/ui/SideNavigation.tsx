import { FilePenLineIcon, LibraryBigIcon, NotebookTextIcon, PackageIcon, PackageOpenIcon, StoreIcon, WheatIcon } from 'lucide-solid';
import { type Accessor, children, type ParentComponent } from 'solid-js';
import { useApp } from '@/app/contexts/AppContext';

const SideNavigation: ParentComponent<{ open?: Accessor<boolean> }> = (
	props,
) => {
	const app = useApp();

	const resolved = children(() => props.children);

	return (
		<aside class="drawer drawer-open h-full">
			<input
				class="drawer-toggle"
				checked={app.isDrawerOpen()}
				type="checkbox"
				id='navigation'
			></input>
			<section class="drawer-content">{resolved()}</section>
			<section class="drawer-side">
				<nav class="pt-5 flex flex-col items-center gap-5 min-h-full is-drawer-close:w-0 is-drawer-open:w-54 bg-base-200 transition-all duration-200 ease-in-out">
					<h1 class="text-lg w-full h-10 text-center">
                        {app.isDrawerOpen()
                            ? import.meta.env.VITE_APP_TITLE
                            : ''}
					</h1>
					<details class='collapse collapse-arrow is-drawer-close:hidden' name='supply' open>
						<summary class='collapse-title text-start text-nowrap'>
							<span>仕入情報</span>
						</summary>
						<div class='collapse-content is-drawer-open:pl-3'>
							<ul class="menu w-full grow gap-4 border-l-2 border-gray-500">
								<li>
									<a
										href="/supplier"
										class="text-nowrap flex items-center gap-3 h-9"
									>
										<StoreIcon class="size-4" />
										<span class="is-drawer-close:hidden">
											仕入先
										</span>
									</a>
								</li>
								<li>
									<a
										href="/supply"
										class="text-nowrap flex items-center gap-3 h-9"
									>
										<WheatIcon class="size-4" />
										<span class="is-drawer-close:hidden">
											仕入品
										</span>
									</a>
								</li>
							</ul>
						</div>
					</details>
					<details class='collapse collapse-arrow is-drawer-close:hidden' name='stock' open>
						<summary class='collapse-title text-start text-nowrap'>
							<span>資産管理</span>
						</summary>
						<div class='collapse-content is-drawer-open:pl-3'>
							<ul class="menu w-full grow gap-4 border-l-2 border-gray-500">
								<li>
									<a
										href="/journal"
										class="text-nowrap flex items-center gap-3 h-9"
									>
										<FilePenLineIcon class="size-4" />
										<span class="is-drawer-close:hidden">
											記帳
										</span>
									</a>
								</li>
								<li>
									<a
										href="/journal/list"
										class="text-nowrap flex items-center gap-3 h-9"
									>
										<LibraryBigIcon class="size-4" />
										<span class="is-drawer-close:hidden">
											記帳履歴
										</span>
									</a>
								</li>
								<li>
									<a
										href="/stocktaking"
										class="text-nowrap flex items-center gap-3 h-9"
									>
										<PackageOpenIcon class="size-4" />
										<span class="is-drawer-close:hidden">
											棚卸
										</span>
									</a>
								</li>
								<li>
									<a
										href="/stocktaking/list"
										class="text-nowrap flex items-center gap-3 h-9"
									>
										<NotebookTextIcon class="size-4" />
										<span class="is-drawer-close:hidden">
											棚卸履歴
										</span>
									</a>
								</li>
							</ul>
						</div>
					</details>
				</nav>
			</section>
		</aside>
	);
};

export default SideNavigation;
