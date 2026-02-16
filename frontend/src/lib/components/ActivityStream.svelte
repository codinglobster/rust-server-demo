<script lang="ts">
	import { onMount } from 'svelte';
	import { activityApi } from '$lib/services/api/activity.api';
	import type { ActivityLogDto } from '$lib/types/activity';
	import Card from '$lib/components/ui/card/card.svelte';

	let activities: ActivityLogDto[] = [];
	let loading = true;
	let error: string | null = null;
	let autoRefresh = true;
	let refreshInterval: number;

	onMount(async () => {
		await loadActivities();

		// Auto-refresh every 5 seconds
		refreshInterval = window.setInterval(async () => {
			if (autoRefresh) {
				await loadActivities();
			}
		}, 5000);

		return () => {
			if (refreshInterval) {
				clearInterval(refreshInterval);
			}
		};
	});

	async function loadActivities() {
		try {
			loading = true;
			error = null;
			const response = await activityApi.getRecentActivities(50);
			activities = response.data;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load activities';
		} finally {
			loading = false;
		}
	}

	function getEventIcon(eventType: string): string {
		if (eventType.startsWith('user_')) return '👤';
		if (eventType.startsWith('message_')) return '💬';
		if (eventType.startsWith('room_')) return '🏠';
		if (eventType.startsWith('system_')) return '⚙️';
		if (eventType.startsWith('error_')) return '❌';
		return '📝';
	}

	function getEventColor(eventType: string): string {
		if (eventType.includes('error')) return 'text-red-600';
		if (eventType.includes('success')) return 'text-green-600';
		if (eventType.startsWith('user_')) return 'text-blue-600';
		if (eventType.startsWith('message_')) return 'text-purple-600';
		if (eventType.startsWith('system_')) return 'text-orange-600';
		return 'text-gray-600';
	}

	function formatTime(timestamp: string): string {
		const date = new Date(timestamp);
		const now = new Date();
		const diffMs = now.getTime() - date.getTime();
		const diffSecs = Math.floor(diffMs / 1000);
		const diffMins = Math.floor(diffSecs / 60);
		const diffHours = Math.floor(diffMins / 60);
		const diffDays = Math.floor(diffHours / 24);

		if (diffSecs < 60) return `${diffSecs}s ago`;
		if (diffMins < 60) return `${diffMins}m ago`;
		if (diffHours < 24) return `${diffHours}h ago`;
		if (diffDays < 7) return `${diffDays}d ago`;
		return date.toLocaleDateString();
	}
</script>

<div class="max-w-4xl mx-auto p-6">
	<div class="flex items-center justify-between mb-6">
		<div>
			<h1 class="text-3xl font-bold text-gray-900">Activity Stream</h1>
			<p class="text-gray-600 mt-1">
				Real-time event tracking powered by Kafka
				<span class="ml-2 text-xs bg-orange-100 text-orange-800 px-2 py-1 rounded">
					{autoRefresh ? '● Live' : '○ Paused'}
				</span>
			</p>
		</div>
		<button
			onclick={() => {
				autoRefresh = !autoRefresh;
			}}
			class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
		>
			{autoRefresh ? '⏸ Pause' : '▶ Resume'}
		</button>
	</div>

	{#if error}
		<div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-md mb-4">
			{error}
		</div>
	{/if}

	{#if loading && activities.length === 0}
		<div class="text-center py-12">
			<div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
			<p class="text-gray-600 mt-4">Loading activities...</p>
		</div>
	{:else}
		<div class="space-y-3">
			{#each activities as activity (activity.id)}
				<Card class="p-4 hover:shadow-md transition-shadow">
					<div class="flex items-start gap-4">
						<div class="flex-shrink-0 text-2xl">{getEventIcon(activity.event_type)}</div>
						<div class="flex-grow min-w-0">
							<div class="flex items-center gap-2 mb-1">
								<span class="font-semibold text-gray-900">
									{activity.username || 'System'}
								</span>
								<span class="text-gray-400">•</span>
								<span class="{getEventColor(activity.event_type)} font-mono text-sm">
									{activity.event_type}
								</span>
								<span class="text-gray-400">•</span>
								<span class="text-gray-500 text-sm">{formatTime(activity.created_at)}</span>
							</div>
							<p class="text-gray-700">{activity.description}</p>
							{#if activity.metadata && Object.keys(activity.metadata).length > 0}
								<details class="mt-2">
									<summary class="text-sm text-gray-500 cursor-pointer hover:text-gray-700">
										View metadata
									</summary>
									<pre class="mt-2 text-xs bg-gray-50 p-2 rounded overflow-x-auto"
										>{JSON.stringify(activity.metadata, null, 2)}</pre
									>
								</details>
							{/if}
						</div>
					</div>
				</Card>
			{/each}
		</div>

		{#if activities.length === 0}
			<div class="text-center py-12 text-gray-500">
				<p class="text-lg">No activities yet</p>
				<p class="text-sm mt-2">Events will appear here as users interact with the system</p>
			</div>
		{/if}
	{/if}
</div>
