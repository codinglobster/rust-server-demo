import { describe, it, expect, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import '@testing-library/jest-dom';
import * as Alert from '$lib/components/ui/alert/index.js';

describe('Alert Component', () => {
	beforeEach(() => {
		render(Alert.Root, {
			children: 'Test Alert Message'
		});
	});

	it('renders alert with title and description', () => {
		expect(screen.getByText('Test Alert Message')).toBeInTheDocument();
	});

	it('applies variant classes correctly', () => {
		const { container } = render(Alert.Root, {
			variant: 'destructive',
			children: 'Error Message'
		});
		expect(container?.querySelector('[data-slot="alert"]')).toHaveClass('variant-destructive');
	});

	it('renders title component', () => {
		render(Alert.Root, {
			children: [
				Alert.Title,
				'Alert Title'
			]
		});
		expect(screen.getByText('Alert Title')).toBeInTheDocument();
	});

	it('renders description component', () => {
		render(Alert.Root, {
			children: [
				Alert.Description,
				'Alert Description'
			]
		});
		expect(screen.getByText('Alert Description')).toBeInTheDocument();
	});

	it('hides when visible prop is false', () => {
		const { container } = render(Alert.Root, {
			visible: false,
			children: 'Hidden Alert'
		});
		expect(container?.querySelector('[data-slot="alert"]')).not.toBeInTheDocument();
	});
});
