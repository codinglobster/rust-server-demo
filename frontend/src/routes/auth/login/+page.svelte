<script lang="ts">
	import LoginForm from '$lib/components/auth/LoginForm.svelte';
</script>

<div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
	<div class="max-w-md w-full">
		<div class="text-center mb-8">
			<h1 class="text-3xl font-bold text-gray-900">Welcome Back</h1>
			<p class="mt-2 text-gray-600">Sign in to your account</p>
		</div>

		<div class="bg-white p-8 rounded-lg shadow">
			<LoginForm />
		</div>

		<p class="mt-4 text-center text-sm text-gray-600">
			Don't have an account?
			<a href="/auth/register" class="text-blue-600 hover:text-blue-500 font-medium">Register</a>
		</p>
	</div>
</div>
