<script lang="ts">
	import Input from '$lib/components/ui/input/input.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import Alert from '$lib/components/ui/alert/alert.svelte';

	let formData = {
		old_password: '',
		new_password: '',
		confirm_password: '',
	};

	let errors: Record<string, string> = {};
	let isSubmitting = false;
	let successMessage = '';

	async function handleSubmit() {
		errors = {};
		isSubmitting = true;

		if (!formData.old_password) {
			errors.old_password = '请输入当前密码';
		}
		if (!formData.new_password) {
			errors.new_password = '请输入新密码';
		} else if (formData.new_password.length < 8) {
			errors.new_password = '新密码至少需要8个字符';
		}
		if (formData.new_password !== formData.confirm_password) {
			errors.confirm_password = '两次输入的密码不一致';
			errors.new_password = '两次输入的密码不一致';
		}

		if (Object.keys(errors).length === 0) {
			try {
				const response = await fetch('/api/users/me/password', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json',
						'Authorization': `Bearer ${localStorage.getItem('access_token')}`,
					},
					body: JSON.stringify({
						old_password: formData.old_password,
						new_password: formData.new_password,
					}),
				});

				if (response.ok) {
					successMessage = '密码修改成功！';
					setTimeout(() => window.location.href = '/users/me', 2000);
				} else {
					const data = await response.json();
					errors.form = data.error || '修改失败';
				}
			} catch (err) {
				errors.form = err instanceof Error ? err.message : '修改失败';
			} finally {
				isSubmitting = false;
			}
		} else {
			isSubmitting = false;
		}
	}
</script>

<div class="max-w-md mx-auto">
	<h2 class="text-2xl font-bold text-gray-900 mb-6">修改密码</h2>

	{#if successMessage}
		<Alert type="success" message={successMessage} />
	{/if}

	<div class="bg-white rounded-lg shadow-md p-8">
		{#if errors.form}
			<Alert type="error" message={errors.form} />
		{/if}

		<form onsubmit={handleSubmit}>
			<Input
				id="change-old-password"
				label="当前密码"
				type="password"
				bind:value={formData.old_password}
				placeholder="请输入当前密码"
				error={errors.old_password}
				required
			/>

			<Input
				id="change-new-password"
				label="新密码"
				type="password"
				bind:value={formData.new_password}
				placeholder="至少8个字符"
				error={errors.new_password}
				required
			/>

			<Input
				id="change-confirm-password"
				label="确认新密码"
				type="password"
				bind:value={formData.confirm_password}
				placeholder="再次输入新密码"
				error={errors.confirm_password}
				required
			/>

			<div class="mt-6">
				<button type="submit" class="w-full bg-blue-600 text-white py-3 rounded-lg hover:bg-blue-700 font-medium disabled:opacity-50 disabled:cursor-not-allowed" disabled={isSubmitting}>
					{isSubmitting ? '修改中...' : '确认修改'}
				</button>
			</div>
		</form>
	</div>
</div>
