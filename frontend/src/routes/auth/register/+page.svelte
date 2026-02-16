<script lang="ts">
	import RegisterForm from '$lib/components/auth/RegisterForm.svelte';
</script>

<div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
	<div class="max-w-md w-full">
		<div class="text-center mb-8">
			<h1 class="text-3xl font-bold text-gray-900">Create Account</h1>
			<p class="mt-2 text-gray-600">Join Rust Server Demo today</p>
		</div>

		<div class="bg-white p-8 rounded-lg shadow">
			<RegisterForm />
		</div>

		<p class="mt-4 text-center text-sm text-gray-600">
			Already have an account?
			<a href="/auth/login" class="text-blue-600 hover:text-blue-500 font-medium">Sign in</a>
		</p>
	</div>
</div>
