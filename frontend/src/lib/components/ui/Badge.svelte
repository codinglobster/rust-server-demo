<script lang="ts">
	export let count: number;
	export let variant: 'primary' | 'secondary' | 'success' | 'danger' | 'warning' = 'primary';

	const variants = {
		primary: 'bg-blue-600',
		secondary: 'bg-gray-600',
		success: 'bg-green-600',
		danger: 'bg-red-600',
		warning: 'bg-yellow-600',
	};
</script>

{#if count > 0}
	<span
		class="absolute -top-1 -right-1 inline-flex items-center justify-center px-2 py-1 text-xs font-bold leading-none text-white rounded-full {variants[variant]}"
	>
		{count > 99 ? '99+' : count}
	</span>
{/if}
