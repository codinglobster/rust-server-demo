<script lang="ts">
	import type { User } from '$lib/types/models';
	import * as Card from '$lib/components/ui/card/index.js';
	import * as Badge from '$lib/components/ui/badge/index.js';
	import { Avatar } from '$lib/components/ui/avatar/index.js';
	import { Button } from '$lib/components/ui/button/index.js';

	export let user: User;
	export let onView: () => void = () => {};
	export let onEdit: () => void = () => {};
	export let onDelete: () => void = () => {};

	const roleVariants: Record<string, "default" | "secondary" | "destructive" | "outline"> = {
		user: 'secondary',
		moderator: 'default',
		admin: 'destructive',
	};
</script>

<Card.Root class="hover:shadow-xl transition-shadow duration-300 overflow-hidden">
	<Card.Content class="p-6">
		<div class="flex items-start justify-between">
			<div class="flex items-center space-x-4">
				<Avatar name={user.username} fallback />
				<div class="flex-1">
					<h3 class="text-lg font-bold text-gray-900">{user.username}</h3>
					<p class="text-sm text-gray-600">{user.email || ''}</p>
					<div class="flex items-center space-x-2 mt-2">
						<Badge.Root variant={roleVariants[user.role] || 'secondary'}>
							{user.role}
						</Badge.Root>
						<span class="text-xs {user.is_active ? 'text-green-600' : 'text-red-600'}">
							{user.is_active ? '✓ 活跃' : '✗ 未激活'}
						</span>
					</div>
				</div>
			</div>
			<div class="flex items-center space-x-2">
				{#if user.is_verified}
					<span class="text-green-600" title="已验证">✓</span>
				{/if}
			</div>
		</div>

		<div class="mt-4 pt-4 border-t border-gray-200">
			<div class="grid grid-cols-2 gap-4 text-sm text-gray-600">
				<div>
					<p class="text-gray-500">全名</p>
					<p class="font-medium text-gray-900">{user.full_name || '-'}</p>
				</div>
				<div>
					<p class="text-gray-500">注册时间</p>
					<p class="font-medium text-gray-900">{new Date(user.created_at).toLocaleDateString()}</p>
				</div>
				<div>
					<p class="text-gray-500">最后登录</p>
					<p class="font-medium text-gray-900">
						{user.last_login_at ? new Date(user.last_login_at).toLocaleString() : '从未登录'}
					</p>
				</div>
			</div>
		</div>
	</Card.Content>

	<Card.Footer class="mt-4 flex justify-end space-x-2">
		<Button variant="outline" size="sm" onclick={onView}>查看</Button>
		<Button variant="default" size="sm" onclick={onEdit}>编辑</Button>
		<Button variant="destructive" size="sm" onclick={onDelete}>删除</Button>
	</Card.Footer>
</Card.Root>
