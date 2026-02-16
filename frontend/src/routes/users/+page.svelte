<script lang="ts">
	import { onMount } from 'svelte';
	import { usersApi } from '$lib/services/api/users.api';
	import { apiClient } from '$lib/services/api/client';
	import Input from '$lib/components/ui/input/input.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import Badge from '$lib/components/ui/badge/badge.svelte';
	import Card from '$lib/components/ui/card/card.svelte';
	import {
		Table,
		TableBody,
		TableCell,
		TableHead,
		TableHeader,
		TableRow
	} from '$lib/components/ui/table/index.ts';

	let users = [];
	let total = 0;
	let currentPage = 1;
	let perPage = 10;
	let totalPages = 1;
	let loading = false;
	let error: string | null = null;
	let searchQuery = '';
	let filterRole = 'all';
	let filterStatus = 'all';

	async function loadUsers() {
		loading = true;
		error = null;

		try {
			const response = await usersApi.listUsers(currentPage, perPage);

			if (response.status === 200) {
				let filteredUsers = response.data.users;

				// Apply filters
				if (filterRole !== 'all' || filterStatus !== 'all') {
					filteredUsers = filteredUsers.filter((user) => {
						const roleMatch = filterRole === 'all' || user.role === filterRole;
						const statusMatch =
							filterStatus === 'all' || user.is_active === (filterStatus === 'active');
						return roleMatch && statusMatch;
					});
				}

				// Apply search
				if (searchQuery) {
					const query = searchQuery.toLowerCase();
					filteredUsers = filteredUsers.filter(
						(user) =>
							user.username.toLowerCase().includes(query) ||
							user.email?.toLowerCase().includes(query)
					);
				}

				users = filteredUsers;
				total = response.data.total;
				totalPages = Math.ceil(total / perPage);
			} else {
				error = response.data.error || '加载用户列表失败';
			}
		} catch (err) {
			error = err instanceof Error ? err.message : '加载失败';
		} finally {
			loading = false;
		}
	}

	function handleSearch() {
		currentPage = 1;
		loadUsers();
	}

	function handleFilter() {
		currentPage = 1;
		loadUsers();
	}

	onMount(() => {
		if (!apiClient.isAuthenticated) {
			window.location.href = '/auth/login';
			return;
		}
		loadUsers();
	});
</script>

<div class="space-y-6 p-6">
	<!-- Header -->
	<Card class="p-6">
		<div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
			<div>
				<h1 class="text-2xl font-bold text-gray-900">用户管理</h1>
				<p class="mt-1 text-sm text-gray-600">
					共 <span class="font-semibold text-gray-900">{total}</span> 位用户
				</p>
			</div>

			<div class="flex items-center space-x-4">
				<Input
					id="user-search"
					type="text"
					placeholder="搜索用户名或邮箱..."
					bind:value={searchQuery}
					onkeydown={(e) => e.key === 'Enter' && handleSearch()}
					class="sm:w-64"
				/>
				<Button onclick={handleSearch} variant="default">
					搜索
				</Button>
			</div>
		</div>

		<!-- Filters -->
		<div class="mt-4 flex flex-wrap gap-4">
			<div class="flex items-center space-x-2">
				<label for="filter-role" class="text-sm font-medium text-gray-700">角色:</label>
				<select
					bind:value={filterRole}
					onchange={handleFilter}
					id="filter-role"
					class="px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
				>
					<option value="all">全部</option>
					<option value="admin">管理员</option>
					<option value="user">普通用户</option>
				</select>
			</div>

			<div class="flex items-center space-x-2">
				<label for="filter-status" class="text-sm font-medium text-gray-700">状态:</label>
				<select
					bind:value={filterStatus}
					onchange={handleFilter}
					id="filter-status"
					class="px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
				>
					<option value="all">全部</option>
					<option value="active">活跃</option>
					<option value="inactive">未激活</option>
				</select>
			</div>
		</div>
	</Card>

	<!-- Users Table -->
	<Card class="p-6">
		{#if loading}
			<div class="flex justify-center py-12">
				<div class="animate-spin rounded-full h-12 w-12 border-b-4 border-gray-900 border-t-transparent"></div>
			</div>
		{:else if error}
			<div class="bg-red-50 border border-red-200 rounded-lg p-12 text-center">
				<p class="text-red-600 font-medium text-lg">{error}</p>
			</div>
		{:else if users.length === 0}
			<div class="text-center py-12">
				<p class="text-gray-500">暂无用户数据</p>
			</div>
		{:else}
			<div class="overflow-x-auto">
				<Table>
					<TableHeader>
						<TableRow>
							<TableHead>用户名</TableHead>
							<TableHead>邮箱</TableHead>
							<TableHead>角色</TableHead>
							<TableHead>状态</TableHead>
							<TableHead>注册时间</TableHead>
							<TableHead>最后登录</TableHead>
							<TableHead class="text-right">操作</TableHead>
						</TableRow>
					</TableHeader>
					<TableBody>
						{#each users as user (user.id)}
							<TableRow>
								<TableCell class="font-medium">{user.username}</TableCell>
								<TableCell>{user.email || '-'}</TableCell>
								<TableCell>
									<Badge variant={user.role === 'admin' ? 'default' : 'secondary'} class="whitespace-nowrap">
										{user.role === 'admin' ? '管理员' : '普通用户'}
									</Badge>
								</TableCell>
								<TableCell>
									<Badge variant={user.is_active ? 'default' : 'destructive'} class="whitespace-nowrap">
										{user.is_active ? '活跃' : '未激活'}
									</Badge>
								</TableCell>
								<TableCell>{new Date(user.created_at).toLocaleDateString()}</TableCell>
								<TableCell>
									{user.last_login_at ? new Date(user.last_login_at).toLocaleDateString() : '从未登录'}
								</TableCell>
								<TableCell class="text-right">
									<div class="flex justify-end space-x-2">
										<Button href="/users/{user.id}" variant="ghost" size="sm" class="whitespace-nowrap">
											查看
										</Button>
										<Button
											href="/users/{user.id}/edit"
											variant="default"
											size="sm"
											class="whitespace-nowrap"
										>
											编辑
										</Button>
									</div>
								</TableCell>
							</TableRow>
						{/each}
					</TableBody>
				</Table>
			</div>

			<!-- Pagination -->
			{#if totalPages > 1}
				<div class="mt-6 flex items-center justify-between">
					<p class="text-sm text-gray-600">
						第 {currentPage} / {totalPages} 页，共 {total} 条记录
					</p>
					<div class="flex space-x-2">
						<Button
							disabled={currentPage === 1}
							onclick={() => {
								currentPage = currentPage - 1;
								loadUsers();
							}}
							variant="outline"
							size="sm"
						>
							上一页
						</Button>
						{#each Array.from({ length: Math.min(5, totalPages) }) as _, i (i)}
							{#if i + 1 === currentPage}
								<Button variant="default" size="sm">{i + 1}</Button>
							{:else}
								<Button
									variant="ghost"
									size="sm"
									onclick={() => {
										currentPage = i + 1;
										loadUsers();
									}}
								>
									{i + 1}
								</Button>
							{/if}
						{/each}
						<Button
							disabled={currentPage === totalPages}
							onclick={() => {
								currentPage = currentPage + 1;
								loadUsers();
							}}
							variant="outline"
							size="sm"
						>
							下一页
						</Button>
					</div>
				</div>
			{/if}
		{/if}
	</Card>
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
