<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import Alert from '$lib/components/ui/alert/alert.svelte';
	import { authStore } from '$lib/services/stores/auth.store';
	import { notificationStore } from '$lib/services/stores/notification.store';
	import type { LoginFormData } from '$lib/types/forms';

	let formData: LoginFormData = {
		username: '',
		password: '',
	};

	let errors: Record<string, string> = {};
	let isSubmitting = false;
	let successMessage = '';

	async function handleSubmit() {
		errors = {};
		isSubmitting = true;

		if (!formData.username.trim()) {
			errors.username = 'Username is required';
		}

		if (!formData.password) {
			errors.password = 'Password is required';
		}

		if (Object.keys(errors).length === 0) {
			try {
				const response = await fetch('/api/auth/login', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json',
					},
					body: JSON.stringify(formData),
				});

				if (response.ok) {
					const data = await response.json();

					if (response.status === 200) {
						// Store token and redirect
						localStorage.setItem('access_token', data.access_token);
						localStorage.setItem('refresh_token', data.refresh_token);

						successMessage = 'Login successful! Redirecting...';
						setTimeout(() => (window.location.href = '/users/me'), 1000);
					} else {
						errors.form = data.error || 'Login failed';
					}
				} else {
					const data = await response.json();
					errors.form = data.error || 'Login failed';
				}
			} catch (err) {
				errors.form = err instanceof Error ? err.message : 'Login failed';
			} finally {
				isSubmitting = false;
			}
		} else {
			isSubmitting = false;
		}
	}
</script>

<div class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 flex items-center justify-center py-12 px-4 sm:px-6">
	<div class="max-w-md w-full">
		<div class="bg-white rounded-lg shadow-xl p-8">
			<h2 class="text-3xl font-bold text-gray-900 mb-6">Welcome Back</h2>
			<p class="mt-2 text-gray-600">Sign in to your account</p>

			{#if errors.form}
				<Alert variant="destructive" class="mb-4">{errors.form}</Alert>
			{/if}

			{#if successMessage}
				<Alert variant="default" class="mb-4">{successMessage}</Alert>
			{/if}

			<form onsubmit={handleSubmit}>
				<div class="space-y-4">
					<div>
						<label for="login-username" class="block text-sm font-medium text-gray-700 mb-1">
							Username
						</label>
						<Input
							id="login-username"
							type="text"
							bind:value={formData.username}
							placeholder="Enter username"
							required
							class={errors.username ? 'border-red-500' : ''}
						/>
						{#if errors.username}
							<p class="mt-1 text-sm text-red-600">{errors.username}</p>
						{/if}
					</div>

					<div>
						<label for="login-password" class="block text-sm font-medium text-gray-700 mb-1">
							Password
						</label>
						<Input
							id="login-password"
							type="password"
							bind:value={formData.password}
							placeholder="Enter password"
							required
							class={errors.password ? 'border-red-500' : ''}
						/>
						{#if errors.password}
							<p class="mt-1 text-sm text-red-600">{errors.password}</p>
						{/if}
					</div>
				</div>

				<div class="mt-6">
					<Button type="submit" variant="default" disabled={isSubmitting} class="w-full">
						{isSubmitting ? 'Signing in...' : 'Sign In'}
					</Button>
				</div>
			</form>

			<div class="mt-4 text-center">
				<p class="mt-2 text-sm text-gray-600">
					Don't have an account?
				</p>
				<a href="/auth-shadcn/register" class="text-blue-600 hover:text-blue-500 font-medium">
					Register
				</a>
			</div>
		</div>
	</div>
</div>
