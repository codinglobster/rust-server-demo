<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { goto } from '$app/navigation';
	import type { Room, Message, WebSocketMessage, RoomMember } from '$lib/types/models';
	import { getWebSocketClient } from '$lib/services/websocket';
	import { messagesApi } from '$lib/services/api/messages.api';
	import { roomsApi } from '$lib/services/api/rooms.api';
	import { authStore } from '$lib/services/stores/auth.store';
	import { toastStore } from '$lib/services/stores/toast.store';
	import MessageList from './MessageList.svelte';
	import MessageInput from './MessageInput.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import Badge from '$lib/components/ui/badge/badge.svelte';

	interface Props {
		room: Room;
	}

	let { room }: Props = $props();

	let messages = $state<Message[]>([]);
	let members = $state<RoomMember[]>([]);
	let loading = $state(false);
	let loadingMembers = $state(false);
	let wsConnected = $state(false);
	let currentUserId = $state('');
	let showMembersSidebar = $state(false);
	let onlineUsers = $state<Set<string>>(new Set());

	const wsClient = getWebSocketClient();

	// 加载历史消息
	async function loadMessages() {
		loading = true;
		try {
			const response = await messagesApi.getRoomMessages(room.id, 1, 50);
			if (response.status === 200) {
				messages = response.data.data.reverse();
			} else {
				toastStore.error('加载消息失败');
			}
		} catch (error) {
			console.error('Failed to load messages:', error);
			toastStore.error('加载消息失败');
		} finally {
			loading = false;
		}
	}

	// 加载房间成员
	async function loadMembers() {
		loadingMembers = true;
		try {
			const response = await roomsApi.getRoomMembers(room.id, 1, 100);
			if (response.status === 200) {
				members = response.data.data;
			}
		} catch (error) {
			console.error('Failed to load members:', error);
		} finally {
			loadingMembers = false;
		}
	}

	// 发送消息
	function handleSendMessage(content: string) {
		if (!wsConnected) {
			toastStore.warning('未连接到服务器');
			return;
		}

		const success = wsClient.sendChatMessage(room.id, content);
		if (!success) {
			toastStore.error('发送失败');
		}
	}

	// 处理 WebSocket 消息
	function handleWebSocketMessage(msg: WebSocketMessage) {
		switch (msg.type) {
			case 'message':
				if (msg.room_id === room.id) {
					const newMessage: Message = {
						id: msg.message_id || '',
						room_id: msg.room_id,
						sender_id: msg.user_id || '',
						sender_username: msg.username,
						content: msg.content || '',
						created_at: msg.timestamp || new Date().toISOString(),
						updated_at: msg.timestamp || new Date().toISOString(),
					};
					messages = [...messages, newMessage];
				}
				break;

			case 'user_joined':
				if (msg.room_id === room.id) {
					toastStore.info(`${msg.username} 加入了房间`);
					if (msg.user_id) {
						onlineUsers.add(msg.user_id);
						onlineUsers = new Set(onlineUsers);
					}
					loadMembers();
				}
				break;

			case 'user_left':
				if (msg.room_id === room.id) {
					toastStore.info(`${msg.username} 离开了房间`);
					if (msg.user_id) {
						onlineUsers.delete(msg.user_id);
						onlineUsers = new Set(onlineUsers);
					}
					loadMembers();
				}
				break;

			case 'error':
				toastStore.error(msg.error || '发生错误');
				break;
		}
	}

	// 离开房间
	async function handleLeaveRoom() {
		try {
			const response = await roomsApi.leaveRoom(room.id);
			if (response.status === 200 || response.status === 204) {
				toastStore.success('已离开房间');
				goto('/rooms');
			} else {
				toastStore.error('离开房间失败');
			}
		} catch (error) {
			console.error('Failed to leave room:', error);
			toastStore.error('离开房间失败');
		}
	}

	// 初始化
	onMount(async () => {
		const auth = $authStore;
		if (auth.user) {
			currentUserId = auth.user.id;
		}

		await Promise.all([loadMessages(), loadMembers()]);

		const token = typeof window !== 'undefined' ? localStorage.getItem('access_token') : null;
		if (token) {
			wsClient.setAccessToken(token);
		}

		wsClient.on({
			onOpen: () => {
				wsConnected = true;
				wsClient.joinRoom(room.id);
			},
			onClose: () => {
				wsConnected = false;
			},
			onError: (error) => {
				console.error('WebSocket error:', error);
			},
			onMessage: handleWebSocketMessage,
		});

		try {
			await wsClient.connect();
		} catch (error) {
			console.error('Failed to connect:', error);
			toastStore.error('无法连接到服务器');
		}
	});

	onDestroy(() => {
		if (wsClient.isConnected()) {
			wsClient.leaveRoom(room.id);
		}
	});
</script>

<div class="flex h-full bg-white">
	<!-- Main Chat Area -->
	<div class="flex-1 flex flex-col min-w-0">
		<!-- Room Header -->
		<div class="border-b bg-white px-6 py-4 shadow-sm">
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-4 min-w-0 flex-1">
					<button
						type="button"
						onclick={() => goto('/rooms')}
						class="flex-shrink-0 p-2 hover:bg-gray-100 rounded-lg transition-colors"
						title="返回"
					>
						<svg class="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
						</svg>
					</button>

					<div
						class="flex-shrink-0 w-12 h-12 rounded-lg bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center text-white text-xl font-bold"
					>
						{room.name.charAt(0).toUpperCase()}
					</div>

					<div class="min-w-0 flex-1">
						<div class="flex items-center gap-2 mb-1">
							<h2 class="text-xl font-bold text-gray-900 truncate">{room.name}</h2>
							{#if room.is_private}
								<Badge variant="secondary" class="flex-shrink-0">私密</Badge>
							{/if}
						</div>
						{#if room.description}
							<p class="text-sm text-gray-600 truncate">{room.description}</p>
						{/if}
					</div>
				</div>

				<div class="flex items-center gap-3 flex-shrink-0">
					<!-- Connection Status -->
					<div
						class="flex items-center gap-2 px-3 py-1.5 rounded-full text-sm {wsConnected
							? 'bg-green-50 text-green-700'
							: 'bg-gray-100 text-gray-700'}"
					>
						<div class="w-2 h-2 rounded-full {wsConnected ? 'bg-green-600 animate-pulse' : 'bg-gray-400'}"></div>
						<span class="font-medium">{wsConnected ? '已连接' : '未连接'}</span>
					</div>

					<!-- Members Button -->
					<button
						type="button"
						onclick={() => (showMembersSidebar = !showMembersSidebar)}
						class="flex items-center gap-2 px-4 py-2 rounded-lg hover:bg-gray-100 transition-colors"
						title="成员列表"
					>
						<svg class="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"
							/>
						</svg>
						<span class="font-medium text-gray-700">{members.length}</span>
					</button>

					<!-- Room Menu -->
					<button
						type="button"
						onclick={handleLeaveRoom}
						class="p-2 hover:bg-red-50 rounded-lg transition-colors"
						title="离开房间"
					>
						<svg class="w-5 h-5 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"
							/>
						</svg>
					</button>
				</div>
			</div>
		</div>

		<!-- Messages -->
		<MessageList {messages} {currentUserId} {loading} />

		<!-- Input -->
		<MessageInput onSend={handleSendMessage} disabled={!wsConnected} />
	</div>

	<!-- Members Sidebar -->
	{#if showMembersSidebar}
		<div class="w-80 border-l bg-gray-50 flex flex-col animate-slideInFromRight">
			<div class="p-4 border-b bg-white">
				<div class="flex items-center justify-between mb-2">
					<h3 class="text-lg font-bold text-gray-900">成员列表</h3>
					<button
						type="button"
						onclick={() => (showMembersSidebar = false)}
						class="p-1 hover:bg-gray-100 rounded transition-colors"
					>
						<svg class="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
						</svg>
					</button>
				</div>
				<p class="text-sm text-gray-600">{members.length} 位成员</p>
			</div>

			<div class="flex-1 overflow-y-auto p-4 space-y-2">
				{#if loadingMembers}
					{#each Array(5) as _}
						<div class="skeleton h-16 rounded-lg"></div>
					{/each}
				{:else if members.length === 0}
					<div class="text-center text-gray-500 py-8">
						<div class="text-4xl mb-2">👥</div>
						<p class="text-sm">暂无成员</p>
					</div>
				{:else}
					{#each members as member, i}
						<div
							class="flex items-center gap-3 p-3 bg-white rounded-lg hover:shadow-md transition-all animate-slideInFromRight"
							style="animation-delay: {i * 50}ms"
						>
							<div class="relative flex-shrink-0">
								<div class="w-10 h-10 rounded-full bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center text-white font-bold">
									{member.username ? member.username.charAt(0).toUpperCase() : '?'}
								</div>
								{#if onlineUsers.has(member.user_id)}
									<div class="absolute bottom-0 right-0 w-3 h-3 bg-green-500 rounded-full border-2 border-white"></div>
								{/if}
							</div>
							<div class="flex-1 min-w-0">
								<div class="font-medium text-gray-900 truncate">{member.username || 'Unknown'}</div>
								<div class="text-xs text-gray-500 capitalize">{member.role}</div>
							</div>
							{#if member.role === 'owner'}
								<Badge variant="default" class="flex-shrink-0">拥有者</Badge>
							{:else if member.role === 'admin'}
								<Badge variant="secondary" class="flex-shrink-0">管理员</Badge>
							{/if}
						</div>
					{/each}
				{/if}
			</div>
		</div>
	{/if}
</div>
