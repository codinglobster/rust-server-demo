<script lang="ts">
	import type { Session } from '$lib/types/models';
	import Badge from '$lib/components/ui/badge/badge.svelte';
	import Button from '$lib/components/ui/button/button.svelte';

	interface Props {
		session: Session;
		onDelete: () => void;
		deleting?: boolean;
	}

	let { session, onDelete, deleting = false }: Props = $props();

	function formatDate(dateString: string): string {
		const date = new Date(dateString);
		return date.toLocaleString('zh-CN', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit',
		});
	}

	function getBrowserInfo(userAgent?: string): string {
		if (!userAgent) return '未知浏览器';

		if (userAgent.includes('Chrome')) return 'Chrome';
		if (userAgent.includes('Firefox')) return 'Firefox';
		if (userAgent.includes('Safari')) return 'Safari';
		if (userAgent.includes('Edge')) return 'Edge';
		return '其他浏览器';
	}

	function getDeviceInfo(userAgent?: string): string {
		if (!userAgent) return '未知设备';

		if (userAgent.includes('Mobile')) return '移动设备';
		if (userAgent.includes('Tablet')) return '平板设备';
		return '桌面设备';
	}
</script>

<div
	class="bg-white rounded-lg border p-4 {session.is_current
		? 'border-blue-500 ring-2 ring-blue-200'
		: 'border-gray-200'}"
>
	<div class="flex items-start justify-between gap-4">
		<div class="flex-1 min-w-0">
			<div class="flex items-center gap-2 mb-2">
				<h3 class="font-semibold text-gray-900">
					{getBrowserInfo(session.user_agent)}
				</h3>
				{#if session.is_current}
					<Badge variant="default">当前会话</Badge>
				{/if}
			</div>

			<div class="space-y-1 text-sm text-gray-600">
				<div class="flex items-center gap-2">
					<span>🖥️</span>
					<span>{getDeviceInfo(session.user_agent)}</span>
				</div>

				{#if session.ip_address}
					<div class="flex items-center gap-2">
						<span>🌐</span>
						<span>{session.ip_address}</span>
					</div>
				{/if}

				<div class="flex items-center gap-2">
					<span>🕐</span>
					<span>最后活跃：{formatDate(session.last_used_at)}</span>
				</div>

				<div class="flex items-center gap-2">
					<span>📅</span>
					<span>创建时间：{formatDate(session.created_at)}</span>
				</div>
			</div>
		</div>

		{#if !session.is_current}
			<Button variant="destructive" size="sm" onclick={onDelete} disabled={deleting}>
				{deleting ? '删除中...' : '删除'}
			</Button>
		{/if}
	</div>
</div>
