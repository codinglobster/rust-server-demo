<script lang="ts">
	export let id: string;
	export let type: 'text' | 'email' | 'password' = 'text';
	export let value = '';
	export let label: string | undefined = undefined;
	export let error: string | undefined = undefined;
	export let placeholder = '';
	export let disabled = false;
	export let required = false;
</script>

<div class="mb-4">
	{#if label}
		<label for={id} class="block text-sm font-medium text-gray-700 mb-1">
			{label}
			{#if required}
				<span class="text-red-500 ml-1">*</span>
			{/if}
		</label>
	{/if}

	<input
		{id}
		{type}
		{value}
		{placeholder}
		{disabled}
		{required}
		class="w-full px-3 py-2 border rounded-md shadow-sm focus:outline-none focus:ring-2 {error ? 'border-red-500 focus:ring-red-500' : 'border-gray-300 focus:ring-blue-500'} transition-colors"
		oninput
	/>

	{#if error}
		<p class="mt-1 text-sm text-red-600">{error}</p>
	{/if}
</div>
