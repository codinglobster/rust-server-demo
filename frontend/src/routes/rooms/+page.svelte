<script lang="ts">
	import { onMount } from 'svelte';
	import { roomsApi } from '$lib/services/api/rooms.api';
	import { toastStore } from '$lib/services/stores/toast.store';
	import type { Room } from '$lib/types/models';
	import RoomList from '$lib/components/rooms/RoomList.svelte';
	import CreateRoomModal from '$lib/components/rooms/CreateRoomModal.svelte';
	import Button from '$lib/components/ui/button/button.svelte';

	let rooms = $state<Room[]>([]);
	let loading = $state(false);
	let showCreateModal = $state(false);
	let currentPage = $state(1);
	let totalPages = $state(1);

	async function loadRooms(page: number = 1) {
		loading = true;
		try {
			const response = await roomsApi.listRooms(page, 20);

			if (response.status === 200) {
				rooms = response.data.data;
				currentPage = response.data.page;
				totalPages = response.data.total_pages;
			} else {
				toastStore.error('加载房间列表失败');
			}
		} catch (error) {
			console.error('Failed to load rooms:', error);
			toastStore.error('加载失败');
		} finally {
			loading = false;
		}
	}

	function handleCreateSuccess() {
		loadRooms(1);
	}

	onMount(() => {
		loadRooms();
	});
</script>

<div class="min-h-screen bg-gray-50">
	<div class="max-w-7xl mx-auto px-4 py-8">
		<!-- 头部 -->
		<div class="mb-8">
			<div class="flex items-center justify-between mb-4">
				<div>
					<h1 class="text-3xl font-bold text-gray-900">聊天室</h1>
					<p class="text-gray-600 mt-2">加入或创建聊天室与其他人交流</p>
				</div>
				<Button onclick={() => (showCreateModal = true)}>
					<span class="text-lg mr-1">+</span>
					创建房间
				</Button>
			</div>

			<!-- 统计 -->
			<div class="bg-white rounded-lg shadow-sm p-4 border">
				<div class="flex items-center gap-6 text-sm">
					<div class="flex items-center gap-2">
						<span class="text-gray-600">总房间数：</span>
						<span class="font-semibold text-gray-900">{rooms.length}</span>
					</div>
					<div class="flex items-center gap-2">
						<span class="text-gray-600">当前页：</span>
						<span class="font-semibold text-gray-900">{currentPage} / {totalPages}</span>
					</div>
				</div>
			</div>
		</div>

		<!-- 房间列表 -->
		<RoomList {rooms} {loading} />

		<!-- 分页 -->
		{#if totalPages > 1}
			<div class="mt-8 flex justify-center gap-2">
				<Button
					variant="outline"
					onclick={() => loadRooms(currentPage - 1)}
					disabled={currentPage <= 1 || loading}
				>
					上一页
				</Button>
				<div class="flex items-center px-4 text-sm text-gray-600">
					{currentPage} / {totalPages}
				</div>
				<Button
					variant="outline"
					onclick={() => loadRooms(currentPage + 1)}
					disabled={currentPage >= totalPages || loading}
				>
					下一页
				</Button>
			</div>
		{/if}
	</div>
</div>

<!-- 创建房间弹窗 -->
<CreateRoomModal
	isOpen={showCreateModal}
	onClose={() => (showCreateModal = false)}
	onSuccess={handleCreateSuccess}
/>
