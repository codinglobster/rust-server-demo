<script lang="ts">
	import type { Room } from '$lib/types/models';
	import RoomCard from './RoomCard.svelte';
	import { goto } from '$app/navigation';

	interface Props {
		rooms: Room[];
		loading?: boolean;
	}

	let { rooms = [], loading = false }: Props = $props();

	function goToRoom(roomId: string) {
		goto(`/rooms/${roomId}`);
	}
</script>

<div class="space-y-4">
	{#if loading}
		<div class="flex justify-center items-center py-12">
			<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
		</div>
	{:else if rooms.length === 0}
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
					d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
				/>
			</svg>
			<p class="text-lg font-medium">还没有房间</p>
			<p class="text-sm mt-2">创建第一个房间开始聊天</p>
		</div>
	{:else}
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
			{#each rooms as room (room.id)}
				<RoomCard {room} onclick={() => goToRoom(room.id)} />
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
