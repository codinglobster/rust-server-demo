<script lang="ts">
	import type { Message } from '$lib/types/models';

	interface Props {
		message: Message;
		isOwnMessage: boolean;
	}

	let { message, isOwnMessage }: Props = $props();

	function formatTime(timestamp: string): string {
		const date = new Date(timestamp);
		return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
	}
</script>

<div class="flex {isOwnMessage ? 'justify-end' : 'justify-start'} mb-4">
	<div class="max-w-xs md:max-w-md lg:max-w-lg">
		{#if !isOwnMessage}
			<div class="text-xs text-gray-600 mb-1 px-2">
				{message.sender_username || 'Unknown'}
			</div>
		{/if}
		<div
			class="rounded-lg px-4 py-2 {isOwnMessage
				? 'bg-blue-600 text-white rounded-br-none'
				: 'bg-gray-200 text-gray-900 rounded-bl-none'}"
		>
			<p class="text-sm whitespace-pre-wrap break-words">{message.content}</p>
			<div class="text-xs mt-1 {isOwnMessage ? 'text-blue-200' : 'text-gray-500'}">
				{formatTime(message.created_at)}
			</div>
		</div>
	</div>
</div>
