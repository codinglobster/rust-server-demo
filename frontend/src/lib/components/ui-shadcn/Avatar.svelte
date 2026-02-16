<script lang="ts">
	import { Avatar } from 'veltekit-shadcn';
	import type { cva } from 'component-types';

	export let name: string;
	export let src: string | undefined = undefined;
	export let alt: string | undefined = undefined;
	export let size: 'xs' | 'sm' | 'md' | 'lg' = 'md';
	export let fallback: boolean = false;
</script>

<sveltekit:component>
	<cn.Avatar
		{name}
		{src}
		{alt}
		{size}
		{fallback}
	/>
</sveltekit:component>
