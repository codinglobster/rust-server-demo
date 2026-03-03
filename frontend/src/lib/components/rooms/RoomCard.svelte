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

	function getMemberStatus() {
		const count = room.member_count || 0;
		const max = room.max_members;
		const percentage = (count / max) * 100;

		if (percentage >= 90) return 'full';
		if (percentage >= 70) return 'busy';
		return 'available';
	}

	const statusColors = {
		available: 'bg-green-100 text-green-700',
		busy: 'bg-yellow-100 text-yellow-700',
		full: 'bg-red-100 text-red-700',
	};

	const status = getMemberStatus();
</script>

<button
	type="button"
	{onclick}
	class="group w-full text-left bg-white rounded-xl border-2 border-gray-100 p-6 hover:border-blue-300 hover:shadow-xl transition-all duration-300 card-hover animate-scaleIn"
>
	<div class="flex flex-col gap-4">
		<!-- Header -->
		<div class="flex items-start gap-4">
			<div
				class="flex-shrink-0 w-14 h-14 rounded-xl bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center text-white text-2xl font-bold shadow-lg group-hover:scale-110 transition-transform"
			>
				{room.name.charAt(0).toUpperCase()}
			</div>

			<div class="flex-1 min-w-0">
				<div class="flex items-center gap-2 mb-2">
					<h3 class="text-lg font-bold text-gray-900 truncate group-hover:text-blue-600 transition-colors">
						{room.name}
					</h3>
					{#if room.is_private}
						<Badge variant="secondary" class="flex-shrink-0">
							<svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
								<path
									fill-rule="evenodd"
									d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z"
									clip-rule="evenodd"
								/>
							</svg>
							私密
						</Badge>
					{/if}
				</div>

				{#if room.description}
					<p class="text-sm text-gray-600 truncate-2 leading-relaxed">{room.description}</p>
				{/if}
			</div>
		</div>

		<!-- Stats -->
		<div class="flex items-center gap-4 text-sm">
			<div class="flex items-center gap-2 px-3 py-1.5 rounded-lg {statusColors[status]}">
				<svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
					<path
						d="M9 6a3 3 0 11-6 0 3 3 0 016 0zM17 6a3 3 0 11-6 0 3 3 0 016 0zM12.93 17c.046-.327.07-.66.07-1a6.97 6.97 0 00-1.5-4.33A5 5 0 0119 16v1h-6.07zM6 11a5 5 0 015 5v1H1v-1a5 5 0 015-5z"
					/>
				</svg>
				<span class="font-semibold">{room.member_count || 0} / {room.max_members}</span>
			</div>

			<div class="flex items-center gap-1.5 text-gray-500">
				<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
					/>
				</svg>
				<span>{formatDate(room.created_at)}</span>
			</div>
		</div>

		<!-- Enter Button -->
		<div class="flex items-center justify-between pt-2 border-t border-gray-100">
			<span class="text-xs text-gray-500 font-medium">点击进入房间</span>
			<div
				class="p-2 rounded-lg bg-blue-50 text-blue-600 group-hover:bg-blue-600 group-hover:text-white transition-colors"
			>
				<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
				</svg>
			</div>
		</div>
	</div>
</button>
