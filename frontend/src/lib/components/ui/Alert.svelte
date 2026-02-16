<script lang="ts">
	export let type: 'success' | 'error' | 'warning' | 'info' = 'info';
	export let message = '';
	export let onDismiss: (() => void) | undefined = undefined;

	const classes = {
		success: 'bg-green-50 border-green-200 text-green-700',
		error: 'bg-red-50 border-red-200 text-red-700',
		warning: 'bg-yellow-50 border-yellow-200 text-yellow-700',
		info: 'bg-blue-50 border-blue-200 text-blue-700',
	};

	const icons = {
		success: '✓',
		error: '✕',
		warning: '⚠',
		info: 'ℹ',
	};
</script>

{#if message}
	<div class="p-4 mb-4 rounded-md border {classes[type]} flex items-center justify-between">
		<div class="flex items-center">
			<span class="mr-2 text-lg">{icons[type]}</span>
			<span>{message}</span>
		</div>
		{#if onDismiss}
			<button
				onclick={onDismiss}
				class="ml-4 text-lg hover:opacity-75 transition-opacity"
				aria-label="Dismiss"
			>
				✕
			</button>
		{/if}
	</div>
{/if}
