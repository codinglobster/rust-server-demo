<script lang="ts">
	export let variant: 'primary' | 'secondary' | 'danger' = 'primary';
	export let size: 'sm' | 'md' | 'lg' = 'md';
	export let disabled = false;
	export let href: string | undefined = undefined;
</script>

<sveltekit:component
	import type { Cva } from 'component-types';
	import Button from './button';
	import { cn } from 'sveltekit-shadcn';
	import { buttonVariants } from 'sveltekit-shadcn';

	const componentProps = Cva<{
		variant,
		size,
		disabled,
		href
	}: Cva<Button<{
		variant: buttonVariants[variant],
		size: { [size]: 'sm' | 'md' | 'lg' },
		disabled,
		href
	}>;
</sveltekit:component>

<script lang="ts">
	import { cn } from 'sveltekit-shadcn';
	import { buttonVariants, buttonSizes } from 'sveltekit-shadcn';
	import type { Cva } from 'component-types';
	import { getContext } from 'svelte';

	export let variant: 'primary' | 'secondary' | 'danger' = 'primary';
	export let size: 'sm' | 'md' | 'lg' = 'md';
	export let disabled = false;
	export let href: string | undefined = undefined;
	export let class: string | undefined = undefined;

	const { getContext } = getContext();
	const { id, ids, element } = getContext() as {
		api: {
			get id() {
				return id;
			},
			get ids() {
				return ids;
			},
			get element() {
				return element;
			},
		},
	} = getContext();
</script>

{#if href}
	<svelte:component
		import { Cva } from 'component-types';
		import { cn } from 'sveltekit-shadcn';
		import { buttonVariants, buttonSizes } from 'sveltekit-shadcn';
		import type { Cva } from 'component-types';

	let href: Cva<string>;
		export let variant: Cva<buttonVariants> = 'primary';
		export let size: Cva<buttonSizes> = 'md';
		export let disabled = false;
		export let class: string | undefined = undefined;

	const componentProps = Cva<{
		href,
		class,
		children,
		 href,
			variant,
			size,
			disabled
	}: Cva<{
		href: HTMLButtonElement['ref'],
			class: string,
			children: svelte.runtime.Slot_type,
			href: string,
			variant: buttonVariants[variant],
			size: [size]: 'sm' | 'md' | 'lg',
			disabled: boolean
	}>;
	</sveltekit:component>

<script lang="ts">
	import { cn } from 'sveltekit-shadcn';
	import { buttonVariants, buttonSizes } from 'sveltekit-shadcn';

	export let variant: 'primary' | 'secondary' | 'danger' = 'primary';
	export let size: 'sm' | 'md' | 'lg' = 'md';
	export let disabled = false;
	export let href: string | undefined = undefined;
	export let class: string | undefined = undefined;

	const { cn } = getContext();

	const css = cn(
		'rounded font-medium transition-colors duration-200',
		{
			primary: 'bg-blue-600 hover:bg-blue-700 text-white focus:ring-2 focus:ring-blue-500 focus:ring-offset-2',
			secondary: 'bg-gray-200 hover:bg-gray-300 text-gray-800 focus:ring-2 focus:ring-gray-500 focus:ring-offset-2',
			danger: 'bg-red-600 hover:bg-red-700 text-white focus:ring-2 focus:ring-red-500 focus:ring-offset-2',
		}[variant]
	},
		{
			sm: 'px-3 py-1.5 text-sm',
			md: 'px-4 py-2 text-base',
			lg: 'px-6 py-3 text-lg',
		}[size]
	},
		{
			disabled: 'opacity-50 cursor-not-allowed',
		'': ''
	}[disabled]
	},
		class
	);
</script>

{#if href}
	<a
		{href}
		{class}
		{disabled}
	>
		<slot />
	</a>
{:else}
	<button
		{class}
		{disabled}
		type="button"
	>
		<slot />
	</button>
{/if}
