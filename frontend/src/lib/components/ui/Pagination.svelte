<script lang="ts">
	export let currentPage = 1;
	export let totalPages = 1;
	export let onPageChange: (page: number) => void = () => {};

	const pages = [];

	for (let i = 1; i <= totalPages; i++) {
		pages.push(i);
	}
</script>

<nav class="flex items-center justify-center space-x-2">
	{#if totalPages > 0}
		<button
		class="px-3 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
		disabled={currentPage === 1}
		onclick={() => onPageChange(currentPage - 1)}
	>
		上一页
	</button>

		{#each pages as page}
		<button
			class="px-3 py-2 text-sm font-medium {page === currentPage ? 'bg-blue-600 text-white' : 'text-gray-700 bg-white border border-gray-300'} rounded-lg hover:bg-gray-50 transition-colors"
			onclick={() => onPageChange(page)}
		>
			{page}
		</button>
		{/each}

		<button
		class="px-3 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
		disabled={currentPage === totalPages}
			onclick={() => onPageChange(currentPage + 1)}
		>
		下一页
	</button>
	{/if}
</nav>
