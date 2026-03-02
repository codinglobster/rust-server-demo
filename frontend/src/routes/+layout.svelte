<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import Toast from '$lib/components/ui/Toast.svelte';
	import NetworkStatus from '$lib/components/ui/NetworkStatus.svelte';
	import { onMount } from 'svelte';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<title>Rust Server Demo</title>
</svelte:head>

<NetworkStatus />
<Toast />
{@render children()}
