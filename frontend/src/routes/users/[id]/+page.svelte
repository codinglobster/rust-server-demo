<script lang="ts">
	import { onMount } from 'svelte';
	import { usersApi } from '$lib/services/api/users.api';
	import UserList from '$lib/components/users/UserList.svelte';
	import Pagination from '$lib/components/ui/Pagination.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Avatar from '$lib/components/ui/Avatar.svelte';
	import type { User } from '$lib/types/models';

	let user: User | null = null;
	let loading = false;
	let error: string | null = null;
	let isEditing = false;

	const userId = $page.params.id;

	async function loadUser() {
		loading = true;
		error = null;

		try {
			const response = await usersApi.getUser(userId);

			if (response.status === 200) {
				user = response.data;
			} else if (response.status === 404) {
				error = '用户不存在';
			} else {
				error = response.data.error || '加载用户详情失败';
			}
		} catch (err) {
			error = err instanceof Error ? err.message : '加载失败';
		} finally {
			loading = false;
		}
	}

	onMount(() => {
		loadUser();
	});
</script>

<div class="space-y-6">
	{#if loading}
		<div class="flex justify-center py-12">
			<div class="animate-spin rounded-full h-16 w-16 border-b-4 border-gray-900"></div>
			<p class="mt-4 text-gray-600">加载中...</p>
		</div>
	{:else if error}
		<div class="bg-red-50 border border-red-200 rounded-lg p-12 text-center">
			<p class="text-red-600 font-medium text-lg">{error}</p>
		</div>
	{:else if user}
		<div class="bg-white rounded-lg shadow-md p-6">
			<!-- Header with Actions -->
			<div class="px-6 py-4 border-b border-gray-200 flex justify-between items-center">
				<div>
					<h1 class="text-2xl font-bold text-gray-900">用户详情</h1>
					<p class="mt-1 text-sm text-gray-600">ID: {user.id}</p>
				</div>
				<div class="flex items-center space-x-3">
					<Badge variant="primary">{user.role}</Badge>
					{#if !isEditing}
						<Button size="sm" onclick={() => isEditing = true}>编辑</Button>
					{/if}
				</div>
			</div>

			<!-- User Info Card -->
			<div class="p-6">
				<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
					<!-- Username -->
					<div>
						<label class="block text-sm font-medium text-gray-700">用户名</label>
						<p class="mt-1 text-sm text-gray-900">{user.username}</p>
					</div>

					<!-- Email -->
					<div>
						<label class="block text-sm font-medium text-gray-700">邮箱</label>
						<p class="mt-1 text-sm text-gray-900">{user.email || ''}</p>
					</div>

					<!-- Full Name -->
					<div>
						<label class="block text-sm font-medium text-gray-700">全名</label>
						<p class="mt-1 text-sm text-gray-900">{user.full_name || '-'}</p>
					</div>

					<!-- Role -->
					<div>
						<label class="block text-sm font-medium text-gray-700">角色</label>
						<div class="mt-1">
							<Badge variant="secondary">{user.role}</Badge>
						</div>
					</div>

					<!-- Status -->
					<div>
						<label class="block text-sm font-medium text-gray-700">状态</label>
						<div class="mt-1 flex items-center space-x-2">
							<span class={user.is_active ? 'text-green-600' : 'text-red-600'}>
								{user.is_active ? '✓ 活跃' : '✗ 未激活'}
							</span>
							<span class="text-sm text-gray-600">
								{user.is_verified ? '(已验证)' : '(未验证)'}
							</span>
						</div>
					</div>

					<!-- Created At -->
					<div>
						<label class="block text-sm font-medium text-gray-700">注册时间</label>
						<p class="mt-1 text-sm text-gray-900">{new Date(user.created_at).toLocaleString()}</p>
					</div>

					<!-- Last Login -->
					<div>
						<label class="block text-sm font-medium text-gray-700">最后登录</label>
						<p class="mt-1 text-sm text-gray-900">
							{user.last_login_at ? new Date(user.last_login_at).toLocaleString() : '从未登录'}
						</p>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
