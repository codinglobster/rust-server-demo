<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import type { Room, Message, WebSocketMessage } from '$lib/types/models';
	import { getWebSocketClient } from '$lib/services/websocket';
	import { messagesApi } from '$lib/services/api/messages.api';
	import { authStore } from '$lib/services/stores/auth.store';
	import { toastStore } from '$lib/services/stores/toast.store';
	import MessageList from './MessageList.svelte';
	import MessageInput from './MessageInput.svelte';
	import TypingIndicator from './TypingIndicator.svelte';

	interface Props {
		room: Room;
	}

	let { room }: Props = $props();

	let messages = $state<Message[]>([]);
	let loading = $state(false);
	let wsConnected = $state(false);
	let currentUserId = $state('');

	const wsClient = getWebSocketClient();

	// 加载历史消息
	async function loadMessages() {
		loading = true;
		try {
			const response = await messagesApi.getRoomMessages(room.id, 1, 50);
			if (response.status === 200) {
				messages = response.data.data.reverse(); // 最新的在底部
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
				// 新消息到达
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
				}
				break;

			case 'user_left':
				if (msg.room_id === room.id) {
					toastStore.info(`${msg.username} 离开了房间`);
				}
				break;

			case 'error':
				toastStore.error(msg.error || '发生错误');
				break;
		}
	}

	// 初始化
	onMount(async () => {
		// 获取当前用户 ID
		const auth = $authStore;
		if (auth.user) {
			currentUserId = auth.user.id;
		}

		// 加载历史消息
		await loadMessages();

		// 设置 WebSocket token
		const token = typeof window !== 'undefined' ? localStorage.getItem('access_token') : null;
		if (token) {
			wsClient.setAccessToken(token);
		}

		// 连接 WebSocket
		wsClient.on({
			onOpen: () => {
				wsConnected = true;
				toastStore.success('已连接到服务器');

				// 加入房间
				wsClient.joinRoom(room.id);
			},
			onClose: () => {
				wsConnected = false;
				toastStore.warning('与服务器断开连接');
			},
			onError: (error) => {
				console.error('WebSocket error:', error);
				toastStore.error('连接错误');
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

	// 清理
	onDestroy(() => {
		if (wsClient.isConnected()) {
			wsClient.leaveRoom(room.id);
		}
	});
</script>

<div class="flex flex-col h-full bg-white">
	<!-- 房间头部 -->
	<div class="border-b bg-white px-6 py-4">
		<div class="flex items-center justify-between">
			<div>
				<h2 class="text-xl font-bold text-gray-900">{room.name}</h2>
				{#if room.description}
					<p class="text-sm text-gray-600 mt-1">{room.description}</p>
				{/if}
			</div>
			<div class="flex items-center gap-2">
				<div
					class="flex items-center gap-2 px-3 py-1 rounded-full text-sm {wsConnected
						? 'bg-green-100 text-green-700'
						: 'bg-gray-100 text-gray-700'}"
				>
					<div class="w-2 h-2 rounded-full {wsConnected ? 'bg-green-600' : 'bg-gray-400'}"></div>
					<span>{wsConnected ? '已连接' : '未连接'}</span>
				</div>
			</div>
		</div>
	</div>

	<!-- 消息列表 -->
	<MessageList {messages} {currentUserId} {loading} />

	<!-- 输入框 -->
	<MessageInput onSend={handleSendMessage} disabled={!wsConnected} />
</div>
