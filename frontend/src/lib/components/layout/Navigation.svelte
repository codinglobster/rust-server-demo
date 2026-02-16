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
		{ label: '用户管理', href: '/users', icon: '👥' },
		{ label: '角色权限', href: '/roles', icon: '⚙️' },
		{ label: '系统设置', href: '/settings', icon: '⚙️' },
		{ label: '帮助中心', href: '/help', icon: '❓' },
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
