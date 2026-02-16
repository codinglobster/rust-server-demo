<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import Navigation from './Navigation.svelte';
	import Header from './Header.svelte';
	import { Avatar } from '$lib/components/ui/avatar/index.js';
	import * as Card from '$lib/components/ui/card/index.js';

	export let hideSidebar = false;

	let sidebarOpen = $page.url.pathname !== '/';

	onMount(() => {
		if (typeof window !== 'undefined') {
			const saved = localStorage.getItem('sidebar_open');
			sidebarOpen = saved === 'true';
		}
	});

	function toggleSidebar() {
		sidebarOpen = !sidebarOpen;
		if (typeof window !== 'undefined') {
			localStorage.setItem('sidebar_open', String(sidebarOpen));
		}
	}

	$: isCollapsed = hideSidebar || !sidebarOpen;
</script>

<div class="flex h-screen bg-gray-50 overflow-hidden">
	<!-- Sidebar -->
	<aside
		class="fixed inset-y-0 left-0 z-20 transition-all duration-300 {isCollapsed ? '-translate-x-full' : 'translate-x-0'} w-64 bg-gradient-to-b from-slate-900 to-slate-800 shadow-2xl"
	>
		<div class="flex flex-col h-full">
			<!-- Logo -->
			<Card.Root class="border-0 border-b border-slate-700 rounded-none shadow-none">
				<Card.Content class="flex items-center justify-center h-16 px-4">
					<div class="flex items-center space-x-3">
						<div class="w-8 h-8 rounded-lg bg-gradient-to-br from-blue-500 to-blue-600 flex items-center justify-center text-white font-bold text-lg">
							R
						</div>
						<span class="text-xl font-bold text-white">Rust Admin</span>
					</div>
				</Card.Content>
			</Card.Root>

			<!-- Navigation -->
			<div class="flex-1 overflow-y-auto py-4">
				<Navigation />
			</div>

			<!-- User Info -->
			<div class="border-t border-slate-700 p-4">
				<div class="flex items-center space-x-3">
					<Avatar name={$page.data?.user?.username || 'User'} size="md" fallback />
					<div class="flex-1 min-w-0">
						<p class="text-sm font-medium text-white truncate">{$page.data?.user?.username || 'User'}</p>
						<p class="text-xs text-slate-400 truncate">{$page.data?.user?.email || ''}</p>
					</div>
				</div>
			</div>
		</div>
	</aside>

	<!-- Main Content -->
	<div class="flex-1 flex flex-col h-full overflow-hidden">
		<!-- Header -->
		<Header {toggleSidebar} />

		<!-- Page Content -->
		<main class="flex-1 overflow-y-auto bg-gray-50 p-6">
			<slot />
		</main>
	</div>
</div>

<style>
	:global(.bg-gradient-to-b) {
		background: linear-gradient(to bottom, rgb(15 23 42), rgb(30 41 59));
	}
</style>
