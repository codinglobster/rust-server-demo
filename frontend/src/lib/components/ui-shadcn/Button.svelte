<script lang="ts">
	import { cn } from 'sveltekit-shadcn';
	import type { cva } from 'component-types';
	import { buttonVariants, buttonSizes } from 'sveltekit-shadcn';

	export let variant: cva<buttonVariants> = 'primary';
	export let size: cva<buttonSizes> = 'md';
	export let disabled = false;
	export let href: string | undefined = undefined;
</script>

<sveltekit:component
	<cn.Button
		{variant}
		{size}
		{disabled}
		{href}
	>
		<slot />
	</cn.Button>
</sveltekit:component>
