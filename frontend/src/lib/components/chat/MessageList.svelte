<script lang="ts">
	import type { Message } from '$lib/types/models';
	import MessageItem from './MessageItem.svelte';
	import { onMount, tick } from 'svelte';

	interface Props {
		messages: Message[];
		currentUserId: string;
		loading?: boolean;
	}

	let { messages = [], currentUserId, loading = false }: Props = $props();

	let messagesContainer: HTMLDivElement;
	let shouldAutoScroll = $state(true);

	// 自动滚动到底部
	async function scrollToBottom() {
		if (shouldAutoScroll && messagesContainer) {
			await tick();
			messagesContainer.scrollTop = messagesContainer.scrollHeight;
		}
	}

	// 检测用户是否在底部
	function handleScroll() {
		if (!messagesContainer) return;

		const { scrollTop, scrollHeight, clientHeight } = messagesContainer;
		const isAtBottom = scrollHeight - scrollTop - clientHeight < 50;
		shouldAutoScroll = isAtBottom;
	}

	// 当消息更新时滚动
	$effect(() => {
		if (messages.length > 0) {
			scrollToBottom();
		}
	});

	onMount(() => {
		scrollToBottom();
	});
</script>

<div
	bind:this={messagesContainer}
	onscroll={handleScroll}
	class="flex-1 overflow-y-auto px-4 py-6 space-y-2 bg-gray-50"
>
	{#if loading}
		<div class="flex justify-center items-center h-full">
			<div class="animate-spin rounded-full h-10 w-10 border-b-2 border-blue-600"></div>
		</div>
	{:else if messages.length === 0}
		<div class="flex flex-col items-center justify-center h-full text-gray-500">
			<svg
				class="w-16 h-16 mb-4"
				fill="none"
				stroke="currentColor"
				viewBox="0 0 24 24"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"
				/>
			</svg>
			<p class="text-lg">还没有消息</p>
			<p class="text-sm mt-1">发送第一条消息开始聊天</p>
		</div>
	{:else}
		{#each messages as message (message.id)}
			<MessageItem {message} isOwnMessage={message.sender_id === currentUserId} />
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
