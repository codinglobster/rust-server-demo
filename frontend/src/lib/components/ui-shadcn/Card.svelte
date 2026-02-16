<script lang="ts">
	import { Card } from 'sveltekit-shadcn';
	import type { cva } from 'component-types';

	export let background: 'bg-white' | 'bg-gray-50' = 'bg-white';
	export let shadow: boolean = false;
	export let padding: string = 'p-6';
	export let hover: boolean = false;
</script>

<sveltekit:component>
	<div class={cn}>
		{#if shadow}
			<cn.Card background={background} shadow={hover ? 'md' : 'none'} {padding} {hover}>
		{/if}
		<slot />
	</div>
	</div>
</sveltekit:component>
