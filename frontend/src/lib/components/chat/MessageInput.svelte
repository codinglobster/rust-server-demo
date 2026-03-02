<script lang="ts">
	import { onMount, onDestroy } from 'svelte';

	interface Props {
		onSend: (content: string) => void;
		disabled?: boolean;
		placeholder?: string;
	}

	let { onSend, disabled = false, placeholder = '输入消息...' }: Props = $props();

	let message = $state('');
	let textarea: HTMLTextAreaElement;
	let typingTimeout: number | null = null;
	let isTyping = $state(false);

	function handleSubmit(e: Event) {
		e.preventDefault();

		const content = message.trim();
		if (!content || disabled) return;

		onSend(content);
		message = '';

		// 重置高度
		if (textarea) {
			textarea.style.height = 'auto';
		}
	}

	function handleKeyPress(e: KeyboardEvent) {
		// Ctrl/Cmd + Enter 发送
		if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
			e.preventDefault();
			handleSubmit(e);
		}
	}

	function handleInput() {
		// 自动调整高度
		if (textarea) {
			textarea.style.height = 'auto';
			textarea.style.height = textarea.scrollHeight + 'px';
		}

		// 触发 typing 事件
		if (!isTyping) {
			isTyping = true;
		}

		// 清除旧的超时
		if (typingTimeout) {
			clearTimeout(typingTimeout);
		}

		// 2秒后停止 typing
		typingTimeout = window.setTimeout(() => {
			isTyping = false;
		}, 2000);
	}

	onDestroy(() => {
		if (typingTimeout) {
			clearTimeout(typingTimeout);
		}
	});
</script>

<form onsubmit={handleSubmit} class="border-t bg-white p-4">
	<div class="flex items-end gap-2">
		<div class="flex-1 relative">
			<textarea
				bind:this={textarea}
				bind:value={message}
				oninput={handleInput}
				onkeydown={handleKeyPress}
				{disabled}
				{placeholder}
				rows="1"
				class="w-full resize-none rounded-lg border border-gray-300 px-4 py-2 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-200 disabled:bg-gray-100 disabled:cursor-not-allowed"
				style="max-height: 150px;"
			/>
			<div class="text-xs text-gray-500 mt-1">
				按 Ctrl+Enter 发送
			</div>
		</div>
		<button
			type="submit"
			disabled={disabled || !message.trim()}
			class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:bg-gray-300 disabled:cursor-not-allowed transition-colors font-medium"
		>
			发送
		</button>
	</div>
</form>
