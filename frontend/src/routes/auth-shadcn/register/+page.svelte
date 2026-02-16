<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import Alert from '$lib/components/ui/alert/alert.svelte';
	import { authStore } from '$lib/services/stores/auth.store';
	import type { RegisterFormData } from '$lib/types/forms';
	import { notificationStore } from '$lib/services/stores/notification.store';

	let formData: RegisterFormData = {
		username: '',
		email: '',
		password: '',
		full_name: '',
	};

	let errors: Record<string, string> = {};
	let isSubmitting = false;
	let successMessage = '';

	async function handleSubmit() {
		errors = {};
		isSubmitting = true;

		// 基础验证
		if (!formData.username.trim()) {
			errors.username = 'Username is required';
		} else if (formData.username.length < 3) {
			errors.username = 'Username must be at least 3 characters';
		} else if (formData.username.length > 50) {
			errors.username = 'Username must be at most 50 characters';
		}

		if (!formData.email.trim()) {
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
				const response = await fetch('/api/auth/register', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json',
					},
					body: JSON.stringify(formData),
				});

				if (response.ok) {
					const data = await response.json();

					if (response.status === 201 || response.status === 200) {
						successMessage = 'Registration successful! Redirecting...';
						setTimeout(() => (window.location.href = '/users/me'), 1000);
					} else {
						errors.form = data.error || 'Registration failed';
						notificationStore.error('Registration failed');
					}
				} else {
					const data = await response.json();
					errors.form = data.error || 'Registration failed';
					notificationStore.error('Registration failed');
				}
			} catch (err) {
				errors.form = err instanceof Error ? err.message : 'Registration failed';
				notificationStore.error('Registration failed');
			} finally {
				isSubmitting = false;
			}
		} else {
			isSubmitting = false;
			notificationStore.error('Please fix the errors below');
		}
	}
</script>

<div class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 flex items-center justify-center py-12 px-4 sm:px-6">
	<div class="max-w-md w-full">
		<div class="bg-white rounded-lg shadow-xl p-8">
			<h2 class="text-3xl font-bold text-gray-900 mb-6">Create Account</h2>
			<p class="mt-2 text-gray-600">Join Rust Server Demo today</p>

			{#if successMessage}
				<Alert variant="default" class="mb-4">{successMessage}</Alert>
			{/if}

			<form onsubmit={handleSubmit}>
				<div class="space-y-4">
					<div>
						<label for="register-username" class="block text-sm font-medium text-gray-700 mb-1">
							Username
						</label>
						<Input
							id="register-username"
							type="text"
							bind:value={formData.username}
							placeholder="Enter username (3-50 characters)"
							required
							class={errors.username ? 'border-red-500' : ''}
						/>
						{#if errors.username}
							<p class="mt-1 text-sm text-red-600">{errors.username}</p>
						{/if}
					</div>

					<div>
						<label for="register-email" class="block text-sm font-medium text-gray-700 mb-1">
							Email
						</label>
						<Input
							id="register-email"
							type="email"
							bind:value={formData.email}
							placeholder="Enter email"
							required
							class={errors.email ? 'border-red-500' : ''}
						/>
						{#if errors.email}
							<p class="mt-1 text-sm text-red-600">{errors.email}</p>
						{/if}
					</div>

					<div>
						<label for="register-password" class="block text-sm font-medium text-gray-700 mb-1">
							Password
						</label>
						<Input
							id="register-password"
							type="password"
							bind:value={formData.password}
							placeholder="Enter password (min 8 characters)"
							required
							class={errors.password ? 'border-red-500' : ''}
						/>
						{#if errors.password}
							<p class="mt-1 text-sm text-red-600">{errors.password}</p>
						{/if}
					</div>

					<div>
						<label for="register-fullname" class="block text-sm font-medium text-gray-700 mb-1">
							Full Name (Optional)
						</label>
						<Input
							id="register-fullname"
							type="text"
							bind:value={formData.full_name}
							placeholder="Enter full name"
						/>
					</div>

					{#if errors.form}
						<div class="p-4 bg-red-50 border border-red-200 rounded-lg">
							<p class="text-red-600 font-medium">{errors.form}</p>
						</div>
					{/if}
				</div>

				<div class="mt-6">
					<Button type="submit" variant="default" disabled={isSubmitting} class="w-full">
						{isSubmitting ? 'Creating account...' : 'Register'}
					</Button>
				</div>
			</form>

			<div class="mt-4 text-center">
				<p class="mt-2 text-sm text-gray-600">
					Already have an account?
				</p>
				<a href="/auth/login" class="text-blue-600 hover:text-blue-500 font-medium">
					Sign in
				</a>
			</div>
		</div>
	</div>
</div>
