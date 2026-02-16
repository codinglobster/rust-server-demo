<script lang="ts">
	import { page } from '$app/stores';
	import { authStore } from '$lib/services/stores/auth.store';
	import * as Badge from '$lib/components/ui/badge/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Avatar } from '$lib/components/ui/avatar/index.js';

	export let toggleSidebar: () => void;

	let showUserMenu = false;
</script>

<header class="bg-white shadow-sm border-b border-gray-200 h-16 flex items-center justify-between px-6">
	<div class="flex items-center space-x-4">
		<!-- Breadcrumb -->
		<div class="flex items-center text-sm">
			<a href="/" class="text-gray-600 hover:text-blue-600 font-medium">首页</a>
			<span class="text-gray-400">/</span>
			<span class="text-gray-800 font-semibold">{$page.url.pathname}</span>
		</div>
	</div>

	<div class="flex items-center space-x-4">
		<!-- Notifications -->
		<Button
			variant="ghost"
			size="sm"
			onclick={() => console.log('Notifications')}
			class="relative p-2 rounded-full"
		>
			<span class="text-xl">🔔</span>
			<Badge.Root variant="destructive" class="absolute -top-1 -right-1 h-5 w-5 rounded-full p-0 flex items-center justify-center">
				3
			</Badge.Root>
		</Button>

		<!-- User Menu -->
		<div class="relative">
			<Button
				variant="ghost"
				size="sm"
				onclick={() => showUserMenu = !showUserMenu}
				class="flex items-center space-x-2 p-2 rounded-full"
			>
				<Avatar name={$authStore.user?.username || 'User'} size="sm" fallback />
				<span class="text-sm font-medium text-gray-700">{$authStore.user?.username || 'User'}</span>
			</Button>

			{#if showUserMenu}
				<div
					class="absolute right-0 mt-2 w-64 bg-white rounded-lg shadow-xl border border-gray-200 py-2 z-50"
					in:click_outside={() => showUserMenu = false}
					transition:fade
				>
					<div class="px-4 py-3 border-b border-gray-100">
						<div class="flex items-center space-x-3">
							<Avatar name={$authStore.user?.username || 'User'} size="md" fallback />
							<div class="flex-1">
								<p class="text-sm font-semibold text-gray-900">{$authStore.user?.username || 'User'}</p>
								<p class="text-xs text-gray-500">{$authStore.user?.email || ''}</p>
							</div>
						</div>
					</div>

					<div class="py-2">
						<a href="/users/me" class="flex items-center px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 rounded-lg">
							<span class="mr-3">👤</span>
							我的资料
						</a>
					</div>

					<div class="py-2 border-t border-gray-100">
						<a href="/users/me/password" class="flex items-center px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 rounded-lg">
							<span class="mr-3">🔒</span>
							修改密码
						</a>
					</div>

					<div class="py-2 border-t border-gray-100">
						<Button
							variant="ghost"
							size="sm"
							onclick={() => authStore.logout()}
							class="flex items-center w-full px-4 py-2 text-sm text-red-600 hover:bg-red-50 rounded-lg justify-start"
						>
							<span class="mr-3">🚪</span>
							登出
						</Button>
					</div>
				</div>
			{/if}
		</div>
	</div>
</header>

<style>
	.fade\:in {
		animation: fade-in 0.2s ease-out;
	}

	@keyframes fade-in {
		from {
			opacity: 0;
			transform: translateY(-10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}
</style>
