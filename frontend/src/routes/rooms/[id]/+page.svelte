<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { roomsApi } from '$lib/services/api/rooms.api';
	import { toastStore } from '$lib/services/stores/toast.store';
	import type { Room } from '$lib/types/models';
	import ChatRoom from '$lib/components/chat/ChatRoom.svelte';
	import Button from '$lib/components/ui/button/button.svelte';

	let roomId = $derived($page.params.id);
	let room = $state<Room | null>(null);
	let loading = $state(true);
	let error = $state<string | null>(null);

	async function loadRoom() {
		if (!roomId) {
			error = '无效的房间 ID';
			loading = false;
			return;
		}

		loading = true;
		error = null;

		try {
			const response = await roomsApi.getRoom(roomId);

			if (response.status === 200) {
				room = response.data;
			} else if (response.status === 404) {
				error = '房间不存在';
			} else {
				error = response.data.error || '加载房间失败';
			}
		} catch (err) {
			console.error('Failed to load room:', err);
			error = '加载失败';
		} finally {
			loading = false;
		}
	}

	function goBack() {
		goto('/rooms');
	}

	onMount(() => {
		loadRoom();
	});
</script>

<div class="h-screen flex flex-col bg-gray-50">
	{#if loading}
		<div class="flex-1 flex items-center justify-center">
			<div class="text-center">
				<div class="animate-spin rounded-full h-16 w-16 border-b-4 border-blue-600 mx-auto mb-4"></div>
				<p class="text-gray-600">加载中...</p>
			</div>
		</div>
	{:else if error || !room}
		<div class="flex-1 flex items-center justify-center">
			<div class="text-center max-w-md">
				<svg
					class="w-20 h-20 mx-auto mb-4 text-red-500"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
					/>
				</svg>
				<h2 class="text-2xl font-bold text-gray-900 mb-2">加载失败</h2>
				<p class="text-gray-600 mb-6">{error || '未知错误'}</p>
				<Button onclick={goBack}>返回房间列表</Button>
			</div>
		</div>
	{:else}
		<ChatRoom {room} />
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
