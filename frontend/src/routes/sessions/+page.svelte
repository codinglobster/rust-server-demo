<script lang="ts">
	import { onMount } from 'svelte';
	import { sessionsApi } from '$lib/services/api/sessions.api';
	import { toastStore } from '$lib/services/stores/toast.store';
	import type { Session } from '$lib/types/models';
	import SessionList from '$lib/components/sessions/SessionList.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import Alert from '$lib/components/ui/alert/alert.svelte';

	let sessions = $state<Session[]>([]);
	let loading = $state(false);
	let deletingSessionId = $state<string | null>(null);
	let showDeleteAllConfirm = $state(false);
	let deletingAll = $state(false);

	async function loadSessions() {
		loading = true;
		try {
			const response = await sessionsApi.listSessions(1, 50);

			if (response.status === 200) {
				sessions = response.data.data;
			} else {
				toastStore.error('加载会话列表失败');
			}
		} catch (error) {
			console.error('Failed to load sessions:', error);
			toastStore.error('加载失败');
		} finally {
			loading = false;
		}
	}

	async function handleDeleteSession(sessionId: string) {
		deletingSessionId = sessionId;
		try {
			const response = await sessionsApi.deleteSession(sessionId);

			if (response.status === 204 || response.status === 200) {
				toastStore.success('会话已删除');
				sessions = sessions.filter((s) => s.id !== sessionId);
			} else {
				toastStore.error('删除失败');
			}
		} catch (error) {
			console.error('Failed to delete session:', error);
			toastStore.error('删除失败');
		} finally {
			deletingSessionId = null;
		}
	}

	async function handleDeleteAllOther() {
		deletingAll = true;
		try {
			const response = await sessionsApi.deleteOtherSessions();

			if (response.status === 204 || response.status === 200) {
				toastStore.success('其他会话已全部删除');
				await loadSessions();
			} else {
				toastStore.error('删除失败');
			}
		} catch (error) {
			console.error('Failed to delete other sessions:', error);
			toastStore.error('删除失败');
		} finally {
			deletingAll = false;
			showDeleteAllConfirm = false;
		}
	}

	$: otherSessionsCount = sessions.filter((s) => !s.is_current).length;

	onMount(() => {
		loadSessions();
	});
</script>

<div class="min-h-screen bg-gray-50">
	<div class="max-w-4xl mx-auto px-4 py-8">
		<!-- 头部 -->
		<div class="mb-8">
			<h1 class="text-3xl font-bold text-gray-900 mb-2">会话管理</h1>
			<p class="text-gray-600">管理您的登录会话，保护账户安全</p>
		</div>

		<!-- 统计信息 -->
		<div class="bg-white rounded-lg shadow-sm p-6 border mb-6">
			<div class="flex items-center justify-between">
				<div>
					<h3 class="text-lg font-semibold text-gray-900 mb-2">活跃会话</h3>
					<p class="text-sm text-gray-600">
						当前共有 <span class="font-semibold text-blue-600">{sessions.length}</span> 个活跃会话
					</p>
				</div>
				{#if otherSessionsCount > 0}
					<Button
						variant="destructive"
						onclick={() => (showDeleteAllConfirm = true)}
						disabled={loading || deletingAll}
					>
						删除其他会话
					</Button>
				{/if}
			</div>
		</div>

		<!-- 删除确认 -->
		{#if showDeleteAllConfirm}
			<Alert variant="destructive" class="mb-6">
				<div class="flex items-start justify-between gap-4">
					<div>
						<h4 class="font-semibold mb-1">确认删除所有其他会话？</h4>
						<p class="text-sm">
							这将删除除当前会话外的所有 {otherSessionsCount} 个会话，其他设备将需要重新登录。
						</p>
					</div>
					<div class="flex gap-2 flex-shrink-0">
						<Button
							size="sm"
							variant="outline"
							onclick={() => (showDeleteAllConfirm = false)}
							disabled={deletingAll}
						>
							取消
						</Button>
						<Button
							size="sm"
							variant="destructive"
							onclick={handleDeleteAllOther}
							disabled={deletingAll}
						>
							{deletingAll ? '删除中...' : '确认删除'}
						</Button>
					</div>
				</div>
			</Alert>
		{/if}

		<!-- 安全提示 -->
		<div class="bg-blue-50 border border-blue-200 rounded-lg p-4 mb-6">
			<div class="flex items-start gap-3">
				<svg class="w-5 h-5 text-blue-600 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
					<path
						fill-rule="evenodd"
						d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z"
						clip-rule="evenodd"
					/>
				</svg>
				<div class="flex-1">
					<h4 class="text-sm font-semibold text-blue-900 mb-1">安全提示</h4>
					<p class="text-sm text-blue-800">
						如果您在列表中发现不认识的会话，请立即删除并修改密码。这可能意味着您的账户已被未经授权访问。
					</p>
				</div>
			</div>
		</div>

		<!-- 会话列表 -->
		<SessionList {sessions} {loading} onDelete={handleDeleteSession} {deletingSessionId} />
	</div>
</div>
