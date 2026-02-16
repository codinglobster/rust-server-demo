<script lang="ts">
	import { onMount } from 'svelte';
	import { authStore } from '$lib/services/stores/auth.store';

	onMount(async () => {
		await authStore.logout();
		window.location.href = '/auth/login';
	});
</script>

<div class="min-h-screen flex items-center justify-center bg-gray-50">
	<p class="text-gray-600">Logging out...</p>
</div>
