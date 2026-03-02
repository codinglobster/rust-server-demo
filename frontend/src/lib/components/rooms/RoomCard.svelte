<script lang="ts">
	import type { Room } from '$lib/types/models';
	import Badge from '$lib/components/ui/badge/badge.svelte';

	interface Props {
		room: Room;
		onclick?: () => void;
	}

	let { room, onclick }: Props = $props();

	function formatDate(dateString: string): string {
		const date = new Date(dateString);
		return date.toLocaleDateString('zh-CN', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
		});
	}
</script>

<button
	type="button"
	{onclick}
	class="w-full text-left bg-white rounded-lg border border-gray-200 p-4 hover:shadow-md hover:border-blue-300 transition-all cursor-pointer"
>
	<div class="flex items-start justify-between gap-4">
		<div class="flex-1 min-w-0">
			<div class="flex items-center gap-2 mb-2">
				<h3 class="text-lg font-semibold text-gray-900 truncate">{room.name}</h3>
				{#if room.is_private}
					<Badge variant="secondary">私密</Badge>
				{/if}
			</div>

			{#if room.description}
				<p class="text-sm text-gray-600 line-clamp-2 mb-2">{room.description}</p>
			{/if}

			<div class="flex items-center gap-4 text-xs text-gray-500">
				<span>👥 {room.member_count || 0} / {room.max_members} 人</span>
				<span>📅 {formatDate(room.created_at)}</span>
			</div>
		</div>

		<div class="flex-shrink-0 text-blue-600">
			<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M9 5l7 7-7 7"
				/>
			</svg>
		</div>
	</div>
</button>
