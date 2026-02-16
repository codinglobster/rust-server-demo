// Svelte page store
import { writable, derived } from 'svelte/store';

export interface PageState {
	url: URL;
	data: any;
}

function createPageStore() {
	const { subscribe, set, update } = writable<PageState>({
		url: new URL(window.location.href),
		data: null,
	});

	return {
		subscribe,
		update,
	};
}

export const page = createPageStore();
export const url = derived(page, ($page) => $page.url);
export const data = derived(page, ($page) => $page.data);
