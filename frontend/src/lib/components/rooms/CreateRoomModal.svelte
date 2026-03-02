<script lang="ts">
	import { roomsApi } from '$lib/services/api/rooms.api';
	import { toastStore } from '$lib/services/stores/toast.store';
	import type { CreateRoomRequest } from '$lib/types/models';
	import Button from '$lib/components/ui/button/button.svelte';
	import Input from '$lib/components/ui/input/input.svelte';

	interface Props {
		isOpen: boolean;
		onClose: () => void;
		onSuccess: () => void;
	}

	let { isOpen, onClose, onSuccess }: Props = $props();

	let formData = $state<CreateRoomRequest>({
		name: '',
		description: '',
		is_private: false,
		max_members: 100,
	});
	let loading = $state(false);
	let errors = $state<Record<string, string>>({});

	function validate(): boolean {
		errors = {};

		if (!formData.name.trim()) {
			errors.name = '房间名称不能为空';
		} else if (formData.name.length < 2) {
			errors.name = '房间名称至少2个字符';
		} else if (formData.name.length > 100) {
			errors.name = '房间名称最多100个字符';
		}

		if (formData.max_members && (formData.max_members < 2 || formData.max_members > 1000)) {
			errors.max_members = '成员数量应在 2-1000 之间';
		}

		return Object.keys(errors).length === 0;
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();

		if (!validate()) return;

		loading = true;
		try {
			const response = await roomsApi.createRoom(formData);

			if (response.status === 201 || response.status === 200) {
				toastStore.success('房间创建成功');
				resetForm();
				onClose();
				onSuccess();
			} else {
				toastStore.error(response.data.error || '创建失败');
			}
		} catch (error) {
			console.error('Failed to create room:', error);
			toastStore.error('创建失败');
		} finally {
			loading = false;
		}
	}

	function resetForm() {
		formData = {
			name: '',
			description: '',
			is_private: false,
			max_members: 100,
		};
		errors = {};
	}

	function handleClose() {
		resetForm();
		onClose();
	}
</script>

{#if isOpen}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50 p-4">
		<div class="bg-white rounded-lg shadow-xl max-w-md w-full max-h-[90vh] overflow-y-auto">
			<!-- 头部 -->
			<div class="flex items-center justify-between p-6 border-b">
				<h2 class="text-xl font-bold text-gray-900">创建新房间</h2>
				<button
					type="button"
					onclick={handleClose}
					class="text-gray-400 hover:text-gray-600 transition-colors"
					disabled={loading}
				>
					<span class="text-2xl">×</span>
				</button>
			</div>

			<!-- 表单 -->
			<form onsubmit={handleSubmit} class="p-6 space-y-4">
				<!-- 房间名称 -->
				<div>
					<label for="name" class="block text-sm font-medium text-gray-700 mb-1">
						房间名称 <span class="text-red-500">*</span>
					</label>
					<Input
						id="name"
						type="text"
						bind:value={formData.name}
						placeholder="例如：技术交流群"
						disabled={loading}
						class={errors.name ? 'border-red-500' : ''}
					/>
					{#if errors.name}
						<p class="text-sm text-red-600 mt-1">{errors.name}</p>
					{/if}
				</div>

				<!-- 房间描述 -->
				<div>
					<label for="description" class="block text-sm font-medium text-gray-700 mb-1">
						房间描述
					</label>
					<textarea
						id="description"
						bind:value={formData.description}
						placeholder="简要描述房间的用途..."
						rows="3"
						disabled={loading}
						class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:bg-gray-100"
					></textarea>
				</div>

				<!-- 最大成员数 -->
				<div>
					<label for="max_members" class="block text-sm font-medium text-gray-700 mb-1">
						最大成员数
					</label>
					<Input
						id="max_members"
						type="number"
						bind:value={formData.max_members}
						min="2"
						max="1000"
						disabled={loading}
						class={errors.max_members ? 'border-red-500' : ''}
					/>
					{#if errors.max_members}
						<p class="text-sm text-red-600 mt-1">{errors.max_members}</p>
					{/if}
				</div>

				<!-- 私密房间 -->
				<div class="flex items-center gap-2">
					<input
						id="is_private"
						type="checkbox"
						bind:checked={formData.is_private}
						disabled={loading}
						class="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
					/>
					<label for="is_private" class="text-sm text-gray-700">
						私密房间（需要邀请才能加入）
					</label>
				</div>

				<!-- 按钮 -->
				<div class="flex gap-3 pt-4">
					<Button
						type="button"
						variant="outline"
						onclick={handleClose}
						disabled={loading}
						class="flex-1"
					>
						取消
					</Button>
					<Button type="submit" disabled={loading} class="flex-1">
						{loading ? '创建中...' : '创建房间'}
					</Button>
				</div>
			</form>
		</div>
	</div>
{/if}
