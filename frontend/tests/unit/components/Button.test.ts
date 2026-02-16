// Button 组件测试

import { render, screen } from '@testing-library/svelte';
import { describe, it, expect } from 'vitest';
import Button from '$lib/components/ui/Button.svelte';

describe('Button Component', () => {
	it('renders children', () => {
		render(Button, { props: { children: 'Click me' } });
		expect(screen.getByText('Click me')).toBeTruthy();
	});

	it('applies primary variant classes', () => {
		const { container } = render(Button, { props: { variant: 'primary', children: 'Test' } });
		expect(container.querySelector('button')).toHaveClass('bg-blue-600');
	});
});
