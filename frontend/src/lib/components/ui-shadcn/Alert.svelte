<script lang="ts">
	import { Alert } from 'veltekit-shadcn';
	import type { cva } from 'component-types';

	export let message: string;
	export let variant: 'primary' | 'secondary' | 'success' | 'danger' | 'warning' | 'info' = 'primary';
	export let visible = true;
</script>

<sveltekit:component>
	{#if visible}
		<cn.Alert {variant}>
			{message}
		</cn.Alert>
	{/if}
</sveltekit:component>
