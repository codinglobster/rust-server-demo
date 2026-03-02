<script lang="ts">
	interface Props {
		size?: 'sm' | 'md' | 'lg';
		text?: string;
	}

	let { size = 'md', text = '加载中...' }: Props = $props();

	const sizeClasses = {
		sm: 'h-8 w-8 border-2',
		md: 'h-12 w-12 border-2',
		lg: 'h-16 w-16 border-4',
	};
</script>

<div class="flex flex-col items-center justify-center gap-4">
	<div
		class="animate-spin rounded-full border-blue-600 border-t-transparent {sizeClasses[size]}"
	></div>
	{#if text}
		<p class="text-gray-600 {size === 'sm' ? 'text-sm' : 'text-base'}">{text}</p>
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
