<script lang="ts">
	import { authStore } from '$lib/services/stores/auth.store';
	import { notificationStore } from '$lib/services/stores/notification.store';
	import Input from '$lib/components/ui/input/input.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import Alert from '$lib/components/ui/alert/alert.svelte';
	import type { RegisterFormData } from '$lib/types/forms';

	let formData: RegisterFormData = {
		username: '',
		email: '',
		password: '',
		full_name: '',
	};

	let errors: Record<string, string> = {};
	let isSubmitting = false;

	async function handleSubmit() {
		console.log('handleSubmit called from RegisterForm');
		errors = {};
		isSubmitting = true;

		// 基础验证
		if (!formData.username) {
			errors.username = 'Username is required';
		} else if (formData.username.length < 3) {
			errors.username = 'Username must be at least 3 characters';
		} else if (formData.username.length > 50) {
			errors.username = 'Username must be at most 50 characters';
		}

		if (!formData.email) {
			errors.email = 'Email is required';
		} else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(formData.email)) {
			errors.email = 'Email is invalid';
		}

		if (!formData.password) {
			errors.password = 'Password is required';
		} else if (formData.password.length < 8) {
			errors.password = 'Password must be at least 8 characters';
		}

		if (Object.keys(errors).length === 0) {
			try {
				await authStore.register(formData);
				notificationStore.success('Registration successful! Redirecting...');
				setTimeout(() => {
					window.location.href = '/users/me';
				}, 1000);
			} catch (error) {
				errors.form = error instanceof Error ? error.message : 'Registration failed';
				notificationStore.error('Registration failed');
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
	<h2 class="text-2xl font-bold mb-6 text-gray-900">Create Account</h2>

	{#if errors.form}
		<Alert variant="destructive">{errors.form}</Alert>
	{/if}

	<div class="space-y-4">
		<Input
			id="register-username"
			label="Username"
			type="text"
			bind:value={formData.username}
			placeholder="Enter username (3-50 characters)"
			error={errors.username}
			required
		/>

		<Input
			id="register-email"
			label="Email"
			type="email"
			bind:value={formData.email}
			placeholder="Enter email"
			error={errors.email}
			required
		/>

		<Input
			id="register-password"
			label="Password"
			type="password"
			bind:value={formData.password}
			placeholder="Enter password (min 8 characters)"
			error={errors.password}
			required
		/>

		<Input
			id="register-fullname"
			label="Full Name (Optional)"
			type="text"
			bind:value={formData.full_name}
			placeholder="Enter full name"
		/>

		<button
			onclick={handleSubmit}
			disabled={isSubmitting}
			class="w-full bg-primary text-primary-foreground hover:bg-primary/90 h-9 px-4 py-2 rounded-md text-sm font-medium inline-flex items-center justify-center gap-2 shadow-xs disabled:pointer-events-none disabled:opacity-50"
		>
			{isSubmitting ? 'Creating account...' : 'Register'}
		</button>
	</div>
</div>
