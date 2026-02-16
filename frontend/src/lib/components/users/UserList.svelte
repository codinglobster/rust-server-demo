<script lang="ts">
	import UserCard from './UserCard.svelte';

	export interface User {
		id: string;
		username: string;
		email: string;
		full_name?: string;
		is_active: boolean;
		is_verified: boolean;
		role: string;
		created_at: string;
		updated_at: string;
		last_login_at?: string;
	}

	export let users: User[];
	export let loading = false;
	export let error: string | null = null;
</script>

<div class="space-y-4">
	{#if loading}
		<div class="flex justify-center py-12">
			<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-gray-900 border-t-transparent"></div>
			<p class="ml-4 text-gray-600">加载中...</p>
		</div>
	{:else if error}
		<div class="bg-red-50 border border-red-200 rounded-lg p-12 text-center">
			<p class="text-red-600 font-medium">{error}</p>
			<p class="mt-2 text-sm text-red-500">请稍后重试</p>
		</div>
	{:else if users.length === 0}
		<div class="bg-gray-50 border border-gray-200 rounded-lg p-12 text-center">
			<p class="text-gray-600">暂无用户数据</p>
		</div>
	{:else}
		<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
			{#each users as user (user.id)}
				<UserCard {user} />
			{/each}
		</div>
	{/if}
</div>

<style>
	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.animate-spin {
		animation: spin 1s linear infinite;
	}
</style>
