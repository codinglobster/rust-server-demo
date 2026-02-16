<script lang="ts">
	import { Badge } from 'sveltekit-shadcn';
	import type { cva } from 'component-types';

	export let count: number;
	export let variant: 'primary' | 'secondary' | 'success' | 'danger' | 'warning' | 'info' = 'primary';
	export let size: 'xs' | 'sm' | 'md' | 'lg' = 'md';
</script>

<sveltekit:component>
	<cn.Badge {variant} {size}>
		{#if count > 99}
			99+
		{/if}
		{count}
	</cn.Badge>
</sveltekit:component>
