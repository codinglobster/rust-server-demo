<script lang="ts">
	export let name: string;
	export let size: 'xs' | 'sm' | 'md' | 'lg' | 'xl' = 'md';
	export let src: string | undefined = undefined;

	const sizes = {
		xs: 'w-6 h-6 text-xs',
		sm: 'w-8 h-8 text-sm',
		md: 'w-10 h-10 text-base',
		lg: 'w-12 h-12 text-lg',
		xl: 'w-16 h-16 text-xl',
	};
</script>

{#if src}
	<img
		{src}
		{sizes[size]}
		class="rounded-full object-cover border-2 border-white shadow-sm"
		alt={name}
	/>
{:else}
	<div
		class={sizes[size]}
		class="rounded-full bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center text-white font-semibold"
	>
		{name.charAt(0).toUpperCase()}
	</div>
{/if}
