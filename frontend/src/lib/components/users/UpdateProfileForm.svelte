<script lang="ts">
	import Input from '$lib/components/ui/input/input.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import Alert from '$lib/components/ui/alert/alert.svelte';

	export let fullName = '';
	export let email = '';

	let formData = {
		full_name: '',
		email: '',
	};

	let errors: Record<string, string> = {};
	let isSubmitting = false;
	let successMessage = '';

	async function handleSubmit() {
		errors = {};
		isSubmitting = true;

		if (!formData.full_name.trim) {
			errors.full_name = '全名不能为空';
		}
		if (formData.email && !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(formData.email)) {
			errors.email = '邮箱格式不正确';
		}

		if (Object.keys(errors).length === 0) {
			try {
				const response = await fetch('/api/users/me', {
					method: 'PUT',
					headers: {
						'Content-Type': 'application/json',
						'Authorization': `Bearer ${localStorage.getItem('access_token')}`,
					},
					body: JSON.stringify(formData),
				});

				if (response.ok) {
					successMessage = '资料更新成功！';
					setTimeout(() => window.location.href = '/users/me', 1000);
				} else {
					const data = await response.json();
					errors.form = data.error || '更新失败';
				}
			} catch (err) {
				errors.form = err instanceof Error ? err.message : '更新失败';
			} finally {
				isSubmitting = false;
			}
		} else {
			isSubmitting = false;
		}
	}
</script>

<div class="max-w-2xl mx-auto">
	<h2 class="text-2xl font-bold text-gray-900 mb-6">编辑个人资料</h2>

	{#if successMessage}
		<Alert type="success" message={successMessage} />
	{/if}

	<div class="bg-white rounded-lg shadow-md p-8">
		{#if errors.form}
			<Alert type="error" message={errors.form} />
		{/if}

		<form onsubmit={handleSubmit}>
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<Input
					id="edit-fullname"
					label="全名"
					type="text"
					bind:value={formData.full_name}
					placeholder="请输入全名"
					error={errors.full_name}
				/>

				<Input
					id="edit-email"
					label="邮箱"
					type="email"
					bind:value={formData.email}
					placeholder="请输入邮箱"
					error={errors.email}
				/>
			</div>

			<div class="mt-6 flex justify-end space-x-4">
				<Button type="button" variant="secondary" onclick={() => window.location.href = '/users/me'}>
					取消
				</Button>
				<Button type="submit" variant="primary" disabled={isSubmitting}>
					{isSubmitting ? '保存中...' : '保存'}
				</Button>
			</div>
		</form>
	</div>
</div>
