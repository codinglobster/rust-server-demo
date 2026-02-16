import { describe, it, expect, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import '@testing-library/jest-dom';

// 注意：shadcn-svelte 组件从 index.js 导出，而不是 .svelte 文件
import { Button } from '$lib/components/ui/button/index.js';

describe('Button Component', () => {
	beforeEach(() => {
		render(Button, { children: 'Click Me' });
	});

	it('renders with default props', () => {
		const button = screen.getByRole('button');
		expect(button).toBeInTheDocument();
	});

	it('renders children content', () => {
		expect(screen.getByText('Click Me')).toBeInTheDocument();
	});
});
