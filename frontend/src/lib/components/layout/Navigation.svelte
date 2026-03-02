<script lang="ts">
	import { page } from '$app/stores';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Badge from '$lib/components/ui/badge/index.js';

	export interface NavLink {
		label: string;
		href: string;
		icon: string;
		badge?: number;
	}

	const links: NavLink[] = [
		{ label: '仪表盘', href: '/', icon: '📊' },
		{ label: '聊天室', href: '/rooms', icon: '💬' },
		{ label: '用户管理', href: '/users', icon: '👥' },
		{ label: '活动日志', href: '/activities', icon: '📋' },
		{ label: '会话管理', href: '/sessions', icon: '🔐' },
	];
</script>

<nav class="space-y-1 px-3">
	{#each links as link}
		<a href={link.href}>
			<Button
				variant="{$page.url.pathname === link.href ? 'default' : 'ghost'}
				size="lg"
				class="w-full justify-start !px-4"
			>
				<span class="text-xl mr-3">{link.icon}</span>
				<span class="flex-1">{link.label}</span>
				{#if link.badge}
					<Badge.Root variant="default" class="ml-auto">
						{link.badge}
					</Badge.Root>
				{/if}
			</Button>
		</a>
	{/each}
</nav>
