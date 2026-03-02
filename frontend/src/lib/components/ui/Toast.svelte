<script lang="ts">
	import { toastStore } from '$lib/services/stores/toast.store';
	import { fade, fly } from 'svelte/transition';

	$: toasts = $toastStore.toasts;

	function getToastIcon(type: string) {
		switch (type) {
			case 'success':
				return '✓';
			case 'error':
				return '✕';
			case 'warning':
				return '⚠';
			case 'info':
				return 'ℹ';
			default:
				return '';
		}
	}

	function getToastColors(type: string) {
		switch (type) {
			case 'success':
				return 'bg-green-50 border-green-500 text-green-900';
			case 'error':
				return 'bg-red-50 border-red-500 text-red-900';
			case 'warning':
				return 'bg-yellow-50 border-yellow-500 text-yellow-900';
			case 'info':
				return 'bg-blue-50 border-blue-500 text-blue-900';
			default:
				return 'bg-gray-50 border-gray-500 text-gray-900';
		}
	}

	function getIconColors(type: string) {
		switch (type) {
			case 'success':
				return 'text-green-600';
			case 'error':
				return 'text-red-600';
			case 'warning':
				return 'text-yellow-600';
			case 'info':
				return 'text-blue-600';
			default:
				return 'text-gray-600';
		}
	}
</script>

<div class="fixed top-4 right-4 z-50 flex flex-col gap-2 max-w-md w-full pointer-events-none">
	{#each toasts as toast (toast.id)}
		<div
			class="pointer-events-auto rounded-lg border-l-4 p-4 shadow-lg {getToastColors(toast.type)}"
			transition:fly={{ x: 300, duration: 300 }}
		>
			<div class="flex items-start gap-3">
				<div class="flex-shrink-0 text-xl {getIconColors(toast.type)}">
					{getToastIcon(toast.type)}
				</div>
				<div class="flex-1 min-w-0">
					<p class="text-sm font-medium">{toast.message}</p>
				</div>
				{#if toast.dismissible}
					<button
						type="button"
						class="flex-shrink-0 text-gray-400 hover:text-gray-600 transition-colors"
						onclick={() => toastStore.dismiss(toast.id)}
					>
						<span class="text-xl">×</span>
					</button>
				{/if}
			</div>
		</div>
	{/each}
</div>
