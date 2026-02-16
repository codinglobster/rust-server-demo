<script lang="ts">
	import { onMount } from 'svelte';
	import { usersApi } from '$lib/services/api/users.api';
	import UpdateProfileForm from '$lib/components/users/UpdateProfileForm.svelte';
	import ChangePasswordForm from '$lib/components/users/ChangePasswordForm.svelte';
	import Card from '$lib/components/ui/card/card.svelte';
	import Badge from '$lib/components/ui/badge/badge.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import Avatar from '$lib/components/ui/avatar/avatar.svelte';

	let user = null;
	let loading = false;
	let error: string | null = null;
	let successMessage = '';

	async function loadUser() {
		loading = true;
		error = null;

		try {
			const response = await usersApi.getMe();

			if (response.status === 200) {
				user = response.data;
			} else {
				error = response.data.error || '加载用户信息失败';
			}
		} catch (err) {
			error = err instanceof Error ? err.message : '加载失败';
		} finally {
			loading = false;
		}
	}

	async function handleUpdate(data: any) {
		try {
			const response = await usersApi.updateMe(data);

			if (response.status === 200) {
				user = response.data;
				successMessage = '资料更新成功！';
				setTimeout(() => successMessage = '', 3000);
			} else {
				error = response.data.error || '更新失败';
			}
		} catch (err) {
			error = err instanceof Error ? err.message : '更新失败';
		}
	}

	onMount(() => {
		loadUser();
	});
</script>

<div class="space-y-6">
	{#if loading}
		<div class="flex justify-center py-12">
			<div class="animate-spin rounded-full h-16 w-16 border-b-4 border-gray-900 border-t-transparent"></div>
		</div>
	{:else if error}
		<div class="bg-red-50 border border-red-200 rounded-lg p-12 text-center">
			<p class="text-red-600 font-medium text-lg">{error}</p>
		</div>
	{:else if user}
	<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
		<!-- Profile Card -->
		<div class="lg:col-span-2 space-y-6">
			<!-- User Info -->
			<Card class="bg-white rounded-lg shadow-sm p-6 border">
				<h2 class="text-xl font-bold text-gray-900 mb-4">个人信息</h2>

				<div class="space-y-4">
					<div>
						<label class="block text-sm font-medium text-gray-700">用户名</label>
						<p class="mt-1 text-sm text-gray-900">{user.username}</p>
					</div>
					<div>
						<label class="block text-sm font-medium text-gray-700">邮箱</label>
						<p class="mt-1 text-sm text-gray-900">{user.email || ''}</p>
					</div>
					<div>
						<label class="block text-sm font-medium text-gray-700">角色</label>
						<div class="mt-1">
							<Badge variant="default">{user.role}</Badge>
						</div>
					</div>
				</div>
			</Card>

			<!-- Actions -->
			<div class="space-y-4">
				<a href="/users/me/edit" class="block">
					<div class="bg-white rounded-lg shadow-sm p-6 border hover:shadow-lg transition-shadow cursor-pointer">
						<div class="flex items-center">
							<div class="flex-1">
								<h3 class="text-lg font-semibold text-gray-900">编辑资料</h3>
								<p class="mt-1 text-sm text-gray-600">修改个人信息</p>
							</div>
							<div class="text-blue-600">
								→
							</div>
						</div>
					</div>
				</a>

				<a href="/users/me/password" class="block">
					<div class="bg-white rounded-lg shadow-sm p-6 border hover:shadow-lg transition-shadow cursor-pointer">
						<div class="flex items-center">
							<div class="flex-1">
								<h3 class="text-lg font-semibold text-gray-900">修改密码</h3>
								<p class="mt-1 text-sm text-gray-600">更新登录密码</p>
							</div>
							<div class="text-blue-600">
								→
							</div>
						</div>
					</div>
				</a>
			</div>
		</div>

		<!-- Right Column: Stats -->
		<div class="space-y-6">
			<div class="bg-white rounded-lg shadow-sm p-6 border">
				<h3 class="text-lg font-semibold text-gray-900 mb-4">账户状态</h3>

				<div class="space-y-3">
					<div class="flex justify-between">
						<span class="text-sm text-gray-600">账户状态</span>
						<span class="text-sm font-semibold {user.is_active ? 'text-green-600' : 'text-red-600'}">
							{user.is_active ? '✓ 活跃' : '✗ 未激活'}
						</span>
					</div>
					<div class="flex justify-between">
						<span class="text-sm text-gray-600">邮箱验证</span>
						<span class="text-sm font-semibold {user.is_verified ? 'text-green-600' : 'text-gray-600'}">
							{user.is_verified ? '✓ 已验证' : '✗ 未验证'}
						</span>
					</div>
					<div class="flex justify-between">
						<span class="text-sm text-gray-600">注册时间</span>
						<span class="text-sm text-gray-900">{new Date(user.created_at).toLocaleDateString()}</span>
					</div>
					<div class="flex justify-between">
						<span class="text-sm text-gray-600">最后登录</span>
						<span class="text-sm text-gray-900">
							{user.last_login_at ? new Date(user.last_login_at).toLocaleString() : '从未登录'}
						</span>
					</div>
				</div>
			</div>
		</div>
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
