<script lang="ts">
	import { TextField } from 'sveltekit-shadcn';
	import type { cva } from 'component-types';

	export let id: string;
	export let value = '';
	export let label: string | undefined = undefined;
	export let error: string | undefined = undefined;
	export let placeholder = '';
	export let disabled = false;
	export let required = false;
	export let type: 'text' | 'email' | 'password' = 'text' = 'text';
</script>

<sveltekit:component>
	<cn.TextField
		{id}
			{value}
		{label}
		{type}
		{placeholder}
		{disabled}
		{required}
		error={error}
	/>
</sveltekit:component>
