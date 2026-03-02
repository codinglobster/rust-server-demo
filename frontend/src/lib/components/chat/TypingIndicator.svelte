<script lang="ts">
	let show = $state(false);
</script>

{#if show}
	<div class="px-4 py-2 text-sm text-gray-500 italic">
		正在输入...
	</div>
{/if}
