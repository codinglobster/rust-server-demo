import { describe, it, expect, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import '@testing-library/jest-dom';
import { Input } from '$lib/components/ui/input/index.js';

describe('Input Component', () => {
	it('renders with default props', () => {
		render(Input);
		const input = screen.getByRole('textbox');
		expect(input).toBeInTheDocument();
	});

	it('applies placeholder correctly', () => {
		render(Input, { placeholder: 'Enter text' });
		expect(screen.getByPlaceholderText('Enter text')).toBeInTheDocument();
	});

	it('applies type correctly', () => {
		render(Input, { type: 'email' });
		const input = screen.getByRole('textbox');
		expect(input).toHaveAttribute('type', 'email');
	});

	it('binds value correctly', () => {
		let value = 'test value';
		const { container } = render(Input, { bind: value });
		const input = container?.querySelector('input');
		expect(input?.value).toBe('test value');
	});

	it('is disabled when disabled prop is true', () => {
		render(Input, { disabled: true });
		const input = screen.getByRole('textbox');
		expect(input).toBeDisabled();
	});

	it('applies custom classes', () => {
		const { container } = render(Input, { class: 'custom-class' });
		expect(container?.querySelector('input')).toHaveClass('custom-class');
	});
});
