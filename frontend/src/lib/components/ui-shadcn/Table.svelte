<script lang="ts">
	import { Table } from 'sveltekit-shadcn';
	import type { cva } from 'component-types';

	export let data = [];
	export let columns: cva<Table.Column>[] = [];
	export let page = 1;
	export let perPage = 10;
	export let total = 0;
	export let loading = false;
</script>

<sveltekit:component>
	<cn.Table
		{columns}
		{page}
		{perPage}
		{total}
		{loading}
		{data}
	/>
</sveltekit:component>
