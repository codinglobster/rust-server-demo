<script lang="ts">
	interface Props {
		icon?: string;
		title?: string;
		description?: string;
		actionText?: string;
		onAction?: () => void;
	}

	let {
		icon = '📭',
		title = '暂无数据',
		description = '目前还没有任何内容',
		actionText,
		onAction,
	}: Props = $props();
</script>

<div class="flex flex-col items-center justify-center py-12 text-center">
	<div class="text-6xl mb-4">{icon}</div>
	<h3 class="text-xl font-semibold text-gray-900 mb-2">{title}</h3>
	<p class="text-gray-600 mb-6 max-w-md">{description}</p>
	{#if actionText && onAction}
		<button
			type="button"
			onclick={onAction}
			class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors font-medium"
		>
			{actionText}
		</button>
	{/if}
</div>
