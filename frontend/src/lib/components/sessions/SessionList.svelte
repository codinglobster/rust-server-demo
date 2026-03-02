<script lang="ts">
	import type { Session } from '$lib/types/models';
	import SessionCard from './SessionCard.svelte';

	interface Props {
		sessions: Session[];
		loading?: boolean;
		onDelete: (sessionId: string) => void;
		deletingSessionId?: string | null;
	}

	let { sessions = [], loading = false, onDelete, deletingSessionId = null }: Props = $props();
</script>

<div class="space-y-4">
	{#if loading}
		<div class="flex justify-center items-center py-12">
			<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
		</div>
	{:else if sessions.length === 0}
		<div class="text-center py-12 text-gray-500">
			<svg
				class="w-16 h-16 mx-auto mb-4 text-gray-400"
				fill="none"
				stroke="currentColor"
				viewBox="0 0 24 24"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
				/>
			</svg>
			<p class="text-lg font-medium">没有活跃会话</p>
		</div>
	{:else}
		{#each sessions as session (session.id)}
			<SessionCard
				{session}
				onDelete={() => onDelete(session.id)}
				deleting={deletingSessionId === session.id}
			/>
		{/each}
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
