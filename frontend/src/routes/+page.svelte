<script lang="ts">
	import { page } from '$app/stores';
	import Badge from '$lib/components/ui/Badge.svelte';

	let stats = [
		{ label: '总用户', value: '0', icon: '👥', color: 'blue' },
		{ label: '活跃用户', value: '0', icon: '✓', color: 'green' },
		{ label: '今日注册', value: '0', icon: '📅', color: 'purple' },
		{ label: '系统状态', value: '正常', icon: '⚡️', color: 'gray' },
	];
</script>

<div class="p-6 space-y-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold text-gray-900">欢迎回来!</h1>
		<p class="mt-2 text-lg text-gray-600">这是您的管理仪表盘</p>
	</div>

	<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
		{#each stats as stat}
			<div class="bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow">
				<div class="flex items-center justify-between">
					<div>
						<p class="text-sm text-gray-600 font-medium">{stat.label}</p>
						<p class="text-2xl font-bold text-gray-900 mt-2">{stat.value}</p>
					</div>
					<div class="text-3xl">{stat.icon}</div>
				</div>
			</div>
		{/each}
		</div>

	<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
		<div class="bg-gradient-to-br from-blue-600 to-blue-800 rounded-lg shadow-md p-6 text-white">
			<h3 class="text-xl font-bold mb-2">用户管理</h3>
			<p class="text-blue-100">查看和管理所有系统用户</p>
			<a href="/users" class="mt-4 inline-block bg-white text-blue-700 px-6 py-2 rounded-lg font-semibold hover:bg-blue-50 transition-colors">
				查看用户 →
			</a>
		</div>

		<div class="bg-gradient-to-br from-purple-600 to-purple-800 rounded-lg shadow-md p-6 text-white">
			<h3 class="text-xl font-bold mb-2">快速操作</h3>
			<p class="text-purple-100">常用功能和操作入口</p>
			<div class="mt-4 grid grid-cols-2 gap-3">
				<a href="/users/me/edit" class="bg-white text-purple-700 px-4 py-2 rounded-lg text-center font-semibold hover:bg-purple-50 transition-colors">
					编辑资料
				</a>
				<a href="/users/me/password" class="bg-white text-purple-700 px-4 py-2 rounded-lg text-center font-semibold hover:bg-purple-50 transition-colors">
					修改密码
				</a>
			</div>
		</div>
	</div>
</div>
