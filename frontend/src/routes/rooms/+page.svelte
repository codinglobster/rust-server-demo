<script lang="ts">
	import { onMount } from 'svelte';
	import { roomsApi } from '$lib/services/api/rooms.api';
	import { toastStore } from '$lib/services/stores/toast.store';
	import type { Room } from '$lib/types/models';
	import RoomList from '$lib/components/rooms/RoomList.svelte';
	import CreateRoomModal from '$lib/components/rooms/CreateRoomModal.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import Input from '$lib/components/ui/input/input.svelte';

	let rooms = $state<Room[]>([]);
	let filteredRooms = $state<Room[]>([]);
	let loading = $state(false);
	let showCreateModal = $state(false);
	let currentPage = $state(1);
	let totalPages = $state(1);
	let total = $state(0);
	let searchQuery = $state('');
	let filterType = $state<'all' | 'public' | 'private'>('all');

	async function loadRooms(page: number = 1) {
		loading = true;
		try {
			const response = await roomsApi.listRooms(page, 20);

			if (response.status === 200) {
				rooms = response.data.data;
				currentPage = response.data.page;
				totalPages = response.data.total_pages;
				total = response.data.total;
				applyFilters();
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

	function applyFilters() {
		let result = [...rooms];

		// 搜索过滤
		if (searchQuery.trim()) {
			const query = searchQuery.toLowerCase();
			result = result.filter(
				(room) =>
					room.name.toLowerCase().includes(query) ||
					room.description?.toLowerCase().includes(query)
			);
		}

		// 类型过滤
		if (filterType === 'public') {
			result = result.filter((room) => !room.is_private);
		} else if (filterType === 'private') {
			result = result.filter((room) => room.is_private);
		}

		filteredRooms = result;
	}

	function handleSearch(e: Event) {
		const target = e.target as HTMLInputElement;
		searchQuery = target.value;
		applyFilters();
	}

	function handleFilterChange(type: typeof filterType) {
		filterType = type;
		applyFilters();
	}

	function handleCreateSuccess() {
		loadRooms(1);
	}

	onMount(() => {
		loadRooms();
	});
</script>

<div class="min-h-screen bg-gradient-to-br from-gray-50 via-blue-50 to-purple-50">
	<div class="max-w-7xl mx-auto px-4 py-8">
		<!-- Header -->
		<div class="mb-8 animate-fadeIn">
			<div class="flex items-center justify-between mb-6">
				<div>
					<h1 class="text-4xl font-bold text-gray-900 mb-2">💬 聊天室</h1>
					<p class="text-gray-600 text-lg">加入或创建聊天室与其他人交流</p>
				</div>
				<Button
					onclick={() => (showCreateModal = true)}
					class="btn-shimmer gradient-primary text-white shadow-lg"
				>
					<span class="text-xl mr-2">+</span>
					创建房间
				</Button>
			</div>

			<!-- Search and Filter -->
			<div class="glass rounded-xl p-6 shadow-lg">
				<div class="flex flex-col md:flex-row gap-4">
					<!-- Search -->
					<div class="flex-1">
						<div class="relative">
							<div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
								<svg class="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
									/>
								</svg>
							</div>
							<input
								type="text"
								placeholder="搜索房间名称或描述..."
								value={searchQuery}
								oninput={handleSearch}
								class="w-full pl-10 pr-4 py-3 border border-gray-200 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
							/>
						</div>
					</div>

					<!-- Filter Buttons -->
					<div class="flex gap-2">
						<button
							type="button"
							onclick={() => handleFilterChange('all')}
							class="px-6 py-3 rounded-lg font-medium transition-all {filterType === 'all'
								? 'bg-blue-600 text-white shadow-md'
								: 'bg-white text-gray-700 hover:bg-gray-50'}"
						>
							全部
						</button>
						<button
							type="button"
							onclick={() => handleFilterChange('public')}
							class="px-6 py-3 rounded-lg font-medium transition-all {filterType === 'public'
								? 'bg-blue-600 text-white shadow-md'
								: 'bg-white text-gray-700 hover:bg-gray-50'}"
						>
							公开
						</button>
						<button
							type="button"
							onclick={() => handleFilterChange('private')}
							class="px-6 py-3 rounded-lg font-medium transition-all {filterType === 'private'
								? 'bg-blue-600 text-white shadow-md'
								: 'bg-white text-gray-700 hover:bg-gray-50'}"
						>
							私密
						</button>
					</div>
				</div>

				<!-- Stats -->
				<div class="mt-4 flex items-center gap-6 text-sm text-gray-600">
					<div class="flex items-center gap-2">
						<span class="font-medium">总房间数：</span>
						<span class="font-bold text-gray-900">{total}</span>
					</div>
					<div class="flex items-center gap-2">
						<span class="font-medium">显示：</span>
						<span class="font-bold text-gray-900">{filteredRooms.length}</span>
					</div>
					{#if searchQuery}
						<div class="flex items-center gap-2 text-blue-600">
							<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z"
								/>
							</svg>
							<span>搜索结果</span>
						</div>
					{/if}
				</div>
			</div>
		</div>

		<!-- Rooms Grid -->
		{#if loading}
			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
				{#each Array(6) as _, i}
					<div class="skeleton h-48 rounded-xl animate-pulse" style="animation-delay: {i * 100}ms"></div>
				{/each}
			</div>
		{:else}
			<RoomList rooms={filteredRooms} {loading} />
		{/if}

		<!-- Pagination -->
		{#if !loading && totalPages > 1}
			<div class="mt-8 flex items-center justify-center gap-2 animate-fadeIn">
				<Button
					variant="outline"
					onclick={() => loadRooms(currentPage - 1)}
					disabled={currentPage <= 1}
					class="shadow-md"
				>
					<svg class="w-5 h-5 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
					</svg>
					上一页
				</Button>

				<div class="flex items-center gap-2">
					{#each Array(Math.min(totalPages, 5)) as _, i}
						{@const page = Math.max(1, currentPage - 2) + i}
						{#if page <= totalPages}
							<button
								type="button"
								onclick={() => loadRooms(page)}
								class="w-10 h-10 rounded-lg font-medium transition-all {page === currentPage
									? 'bg-blue-600 text-white shadow-md'
									: 'bg-white text-gray-700 hover:bg-gray-100'}"
							>
								{page}
							</button>
						{/if}
					{/each}
				</div>

				<Button
					variant="outline"
					onclick={() => loadRooms(currentPage + 1)}
					disabled={currentPage >= totalPages}
					class="shadow-md"
				>
					下一页
					<svg class="w-5 h-5 ml-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
					</svg>
				</Button>
			</div>
		{/if}

		<!-- Empty State for Search -->
		{#if !loading && filteredRooms.length === 0 && searchQuery}
			<div class="text-center py-16 animate-fadeIn">
				<div class="text-6xl mb-4">🔍</div>
				<h3 class="text-2xl font-bold text-gray-900 mb-2">没有找到匹配的房间</h3>
				<p class="text-gray-600 mb-6">试试其他关键词或创建一个新房间</p>
				<Button onclick={() => (showCreateModal = true)}>创建新房间</Button>
			</div>
		{/if}
	</div>
</div>

<!-- Create Room Modal -->
<CreateRoomModal
	isOpen={showCreateModal}
	onClose={() => (showCreateModal = false)}
	onSuccess={handleCreateSuccess}
/>
