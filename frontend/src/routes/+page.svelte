<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { roomsApi } from '$lib/services/api/rooms.api';
	import { usersApi } from '$lib/services/api/users.api';
	import { activityApi } from '$lib/services/api/activity.api';
	import { authStore } from '$lib/services/stores/auth.store';
	import type { Room, User, ActivityLogDto } from '$lib/types/models';
	import Button from '$lib/components/ui/button/button.svelte';
	import Badge from '$lib/components/ui/badge/badge.svelte';

	let loading = $state(true);
	let stats = $state({
		totalUsers: 0,
		totalRooms: 0,
		activeRooms: 0,
		recentActivities: 0,
	});
	let recentRooms = $state<Room[]>([]);
	let recentActivities = $state<ActivityLogDto[]>([]);
	let currentUser = $state<User | null>(null);

	async function loadDashboardData() {
		loading = true;
		try {
			const [roomsRes, usersRes, activitiesRes, userRes] = await Promise.all([
				roomsApi.listRooms(1, 6),
				usersApi.listUsers(1, 1),
				activityApi.getRecentActivities(5),
				usersApi.getMe(),
			]);

			if (roomsRes.status === 200) {
				recentRooms = roomsRes.data.data.slice(0, 6);
				stats.totalRooms = roomsRes.data.total;
				stats.activeRooms = roomsRes.data.total;
			}

			if (usersRes.status === 200) {
				stats.totalUsers = usersRes.data.total;
			}

			if (activitiesRes.status === 200) {
				recentActivities = activitiesRes.data.slice(0, 5);
				stats.recentActivities = activitiesRes.data.length;
			}

			if (userRes.status === 200) {
				currentUser = userRes.data;
			}
		} catch (error) {
			console.error('Failed to load dashboard data:', error);
		} finally {
			loading = false;
		}
	}

	function getGreeting() {
		const hour = new Date().getHours();
		if (hour < 12) return '早上好';
		if (hour < 18) return '下午好';
		return '晚上好';
	}

	function formatTime(dateString: string): string {
		const date = new Date(dateString);
		const now = new Date();
		const diff = now.getTime() - date.getTime();
		const minutes = Math.floor(diff / 60000);
		const hours = Math.floor(diff / 3600000);
		const days = Math.floor(diff / 86400000);

		if (minutes < 1) return '刚刚';
		if (minutes < 60) return `${minutes} 分钟前`;
		if (hours < 24) return `${hours} 小时前`;
		return `${days} 天前`;
	}

	function getActivityIcon(eventType: string): string {
		const icons: Record<string, string> = {
			user_login: '🔑',
			user_logout: '👋',
			user_registered: '✨',
			room_created: '🏠',
			room_joined: '👥',
			room_left: '🚪',
			message_sent: '💬',
			profile_updated: '✏️',
		};
		return icons[eventType] || '📝';
	}

	onMount(() => {
		loadDashboardData();
	});
</script>

<div class="min-h-screen bg-gradient-to-br from-gray-50 via-blue-50 to-purple-50">
	<div class="max-w-7xl mx-auto px-4 py-8 space-y-8">
		<!-- Welcome Section -->
		<div class="animate-fadeIn">
			<div class="gradient-primary rounded-2xl p-8 text-white shadow-xl">
				<div class="flex items-center justify-between flex-wrap gap-4">
					<div class="space-y-2">
						<h1 class="text-3xl md:text-4xl font-bold">
							{getGreeting()}，{currentUser?.username || '用户'}！
						</h1>
						<p class="text-blue-100 text-lg">欢迎回到你的工作台</p>
					</div>
					<div class="flex gap-3">
						<Button
							onclick={() => goto('/rooms')}
							class="bg-white text-blue-600 hover:bg-blue-50 shadow-lg"
						>
							<span class="text-xl mr-2">💬</span>
							进入聊天室
						</Button>
					</div>
				</div>
			</div>
		</div>

		<!-- Stats Cards -->
		<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 animate-slideInFromBottom">
			<div class="card-hover bg-white rounded-xl p-6 shadow-md border border-gray-100">
				<div class="flex items-center justify-between mb-4">
					<div class="p-3 bg-blue-50 rounded-lg">
						<span class="text-3xl">👥</span>
					</div>
					{#if loading}
						<div class="skeleton h-8 w-16"></div>
					{:else}
						<div class="text-right">
							<div class="text-3xl font-bold text-gray-900">{stats.totalUsers}</div>
						</div>
					{/if}
				</div>
				<h3 class="text-sm font-medium text-gray-600">总用户数</h3>
			</div>

			<div class="card-hover bg-white rounded-xl p-6 shadow-md border border-gray-100">
				<div class="flex items-center justify-between mb-4">
					<div class="p-3 bg-green-50 rounded-lg">
						<span class="text-3xl">🏠</span>
					</div>
					{#if loading}
						<div class="skeleton h-8 w-16"></div>
					{:else}
						<div class="text-right">
							<div class="text-3xl font-bold text-gray-900">{stats.totalRooms}</div>
						</div>
					{/if}
				</div>
				<h3 class="text-sm font-medium text-gray-600">聊天室数量</h3>
			</div>

			<div class="card-hover bg-white rounded-xl p-6 shadow-md border border-gray-100">
				<div class="flex items-center justify-between mb-4">
					<div class="p-3 bg-purple-50 rounded-lg">
						<span class="text-3xl">⚡</span>
					</div>
					{#if loading}
						<div class="skeleton h-8 w-16"></div>
					{:else}
						<div class="text-right">
							<div class="text-3xl font-bold text-gray-900">{stats.activeRooms}</div>
						</div>
					{/if}
				</div>
				<h3 class="text-sm font-medium text-gray-600">活跃房间</h3>
			</div>

			<div class="card-hover bg-white rounded-xl p-6 shadow-md border border-gray-100">
				<div class="flex items-center justify-between mb-4">
					<div class="p-3 bg-orange-50 rounded-lg">
						<span class="text-3xl">📊</span>
					</div>
					{#if loading}
						<div class="skeleton h-8 w-16"></div>
					{:else}
						<div class="text-right">
							<div class="text-3xl font-bold text-gray-900">{stats.recentActivities}</div>
						</div>
					{/if}
				</div>
				<h3 class="text-sm font-medium text-gray-600">最近活动</h3>
			</div>
		</div>

		<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
			<!-- Recent Rooms -->
			<div class="lg:col-span-2 space-y-4">
				<div class="flex items-center justify-between">
					<h2 class="text-2xl font-bold text-gray-900">🏠 热门聊天室</h2>
					<Button variant="ghost" onclick={() => goto('/rooms')}>
						查看全部 →
					</Button>
				</div>

				<div class="bg-white rounded-xl shadow-md border border-gray-100 overflow-hidden">
					{#if loading}
						<div class="p-6 space-y-4">
							{#each Array(3) as _}
								<div class="skeleton h-20"></div>
							{/each}
						</div>
					{:else if recentRooms.length === 0}
						<div class="p-12 text-center text-gray-500">
							<div class="text-6xl mb-4">🏠</div>
							<p class="text-lg font-medium mb-2">还没有聊天室</p>
							<p class="text-sm mb-4">创建第一个聊天室开始交流</p>
							<Button onclick={() => goto('/rooms')}>创建聊天室</Button>
						</div>
					{:else}
						<div class="divide-y divide-gray-100">
							{#each recentRooms as room, i}
								<button
									type="button"
									onclick={() => goto(`/rooms/${room.id}`)}
									class="w-full p-4 hover:bg-gray-50 transition-colors text-left flex items-center gap-4 animate-slideInFromLeft"
									style="animation-delay: {i * 100}ms"
								>
									<div class="flex-shrink-0 w-12 h-12 rounded-lg bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center text-white text-xl font-bold">
										{room.name.charAt(0).toUpperCase()}
									</div>
									<div class="flex-1 min-w-0">
										<div class="flex items-center gap-2 mb-1">
											<h3 class="font-semibold text-gray-900 truncate">{room.name}</h3>
											{#if room.is_private}
												<Badge variant="secondary" class="text-xs">私密</Badge>
											{/if}
										</div>
										{#if room.description}
											<p class="text-sm text-gray-600 truncate">{room.description}</p>
										{/if}
									</div>
									<div class="flex-shrink-0 text-sm text-gray-500">
										<div class="flex items-center gap-1">
											<span>👥</span>
											<span>{room.member_count || 0}</span>
										</div>
									</div>
								</button>
							{/each}
						</div>
					{/if}
				</div>
			</div>

			<!-- Recent Activities -->
			<div class="space-y-4">
				<div class="flex items-center justify-between">
					<h2 class="text-2xl font-bold text-gray-900">📋 最近活动</h2>
					<Button variant="ghost" onclick={() => goto('/activities')}>
						查看更多 →
					</Button>
				</div>

				<div class="bg-white rounded-xl shadow-md border border-gray-100 overflow-hidden">
					{#if loading}
						<div class="p-6 space-y-4">
							{#each Array(5) as _}
								<div class="skeleton h-16"></div>
							{/each}
						</div>
					{:else if recentActivities.length === 0}
						<div class="p-8 text-center text-gray-500">
							<div class="text-4xl mb-2">📝</div>
							<p class="text-sm">暂无活动记录</p>
						</div>
					{:else}
						<div class="divide-y divide-gray-100">
							{#each recentActivities as activity, i}
								<div
									class="p-4 hover:bg-gray-50 transition-colors animate-slideInFromRight"
									style="animation-delay: {i * 100}ms"
								>
									<div class="flex items-start gap-3">
										<div class="flex-shrink-0 text-2xl">
											{getActivityIcon(activity.event_type)}
										</div>
										<div class="flex-1 min-w-0">
											<p class="text-sm font-medium text-gray-900 mb-1">
												{activity.description}
											</p>
											<p class="text-xs text-gray-500">
												{formatTime(activity.created_at)}
											</p>
										</div>
									</div>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			</div>
		</div>

		<!-- Quick Actions -->
		<div class="bg-white rounded-xl shadow-md border border-gray-100 p-6 animate-scaleIn">
			<h2 class="text-xl font-bold text-gray-900 mb-4">⚡ 快捷操作</h2>
			<div class="grid grid-cols-2 md:grid-cols-4 gap-4">
				<button
					type="button"
					onclick={() => goto('/rooms')}
					class="card-hover p-6 border-2 border-gray-200 rounded-xl text-center hover:border-blue-500 transition-all"
				>
					<div class="text-4xl mb-2">💬</div>
					<div class="font-medium text-gray-900">聊天室</div>
				</button>
				<button
					type="button"
					onclick={() => goto('/users')}
					class="card-hover p-6 border-2 border-gray-200 rounded-xl text-center hover:border-blue-500 transition-all"
				>
					<div class="text-4xl mb-2">👥</div>
					<div class="font-medium text-gray-900">用户管理</div>
				</button>
				<button
					type="button"
					onclick={() => goto('/sessions')}
					class="card-hover p-6 border-2 border-gray-200 rounded-xl text-center hover:border-blue-500 transition-all"
				>
					<div class="text-4xl mb-2">🔐</div>
					<div class="font-medium text-gray-900">会话管理</div>
				</button>
				<button
					type="button"
					onclick={() => goto('/users/me')}
					class="card-hover p-6 border-2 border-gray-200 rounded-xl text-center hover:border-blue-500 transition-all"
				>
					<div class="text-4xl mb-2">⚙️</div>
					<div class="font-medium text-gray-900">个人设置</div>
				</button>
			</div>
		</div>
	</div>
</div>
