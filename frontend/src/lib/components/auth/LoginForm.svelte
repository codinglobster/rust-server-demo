<script lang="ts">
	import { authStore } from '$lib/services/stores/auth.store';
	import { notificationStore } from '$lib/services/stores/notification.store';
	import Input from '$lib/components/ui/input/input.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import Alert from '$lib/components/ui/alert/alert.svelte';
	import type { LoginFormData } from '$lib/types/forms';

	let formData: LoginFormData = {
		username: '',
		password: '',
	};

	let errors: Record<string, string> = {};
	let isSubmitting = false;

	async function handleSubmit() {
		console.log('handleSubmit called from LoginForm');
		errors = {};
		isSubmitting = true;

		// 基础验证
		if (!formData.username) {
			errors.username = 'Username is required';
		}

		if (!formData.password) {
			errors.password = 'Password is required';
		}

		if (Object.keys(errors).length === 0) {
			try {
				await authStore.login(formData);
				notificationStore.success('Login successful! Redirecting...');
				setTimeout(() => {
					window.location.href = '/users/me';
				}, 1000);
			} catch (error) {
				errors.form = error instanceof Error ? error.message : 'Login failed';
				notificationStore.error('Login failed');
			} finally {
				isSubmitting = false;
			}
		} else {
			isSubmitting = false;
			errors.form = 'Please fix the errors below';
			notificationStore.error('Please fix the errors below');
		}
	}
</script>

<div class="max-w-md mx-auto">
	<h2 class="text-2xl font-bold mb-6 text-gray-900">Sign In</h2>

	{#if errors.form}
		<Alert variant="destructive">{errors.form}</Alert>
	{/if}

	<div class="space-y-4">
		<Input
			id="login-username"
			label="Username"
			type="text"
			bind:value={formData.username}
			placeholder="Enter username"
			error={errors.username}
			required
		/>

		<Input
			id="login-password"
			label="Password"
			type="password"
			bind:value={formData.password}
			placeholder="Enter password"
			error={errors.password}
			required
		/>

		<button
			onclick={handleSubmit}
			disabled={isSubmitting}
			class="w-full bg-primary text-primary-foreground hover:bg-primary/90 h-9 px-4 py-2 rounded-md text-sm font-medium inline-flex items-center justify-center gap-2 shadow-xs disabled:pointer-events-none disabled:opacity-50"
		>
			{isSubmitting ? 'Signing in...' : 'Sign In'}
		</button>
	</div>
</div>
