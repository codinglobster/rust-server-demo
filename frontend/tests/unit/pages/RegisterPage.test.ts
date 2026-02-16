import { describe, it, expect, vi } from 'vitest';
import { render, screen, waitFor } from '@testing-library/svelte';
import '@testing-library/jest-dom';
import userEvent from '@testing-library/user-event';
import Register from '$routes/auth-shadcn/register/+page.svelte';

// Mock fetch
global.fetch = vi.fn(() =>
	Promise.resolve({
		ok: true,
		status: 201,
		json: async () => ({
			id: '123',
			username: 'testuser',
			email: 'test@example.com'
		})
	}) as Response
) as typeof globalThis & typeof globalThis.fetch;

describe('Register Page', () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	it('renders registration form', () => {
		render(Register);
		expect(screen.getByLabelText('Username')).toBeInTheDocument();
		expect(screen.getByLabelText('Email')).toBeInTheDocument();
		expect(screen.getByLabelText('Password')).toBeInTheDocument();
		expect(screen.getByLabelText('Full Name')).toBeInTheDocument();
	});

	it('shows validation errors for invalid data', async () => {
		render(Register);
		const submitButton = screen.getByRole('button', { name: /Register/i });
		const user = userEvent.setup();

		await user.click(submitButton);
		await waitFor(() => {
			expect(screen.getByText('Username is required')).toBeInTheDocument();
			expect(screen.getByText('Email is required')).toBeInTheDocument();
			expect(screen.getByText('Password is required')).toBeInTheDocument();
		});
	});

	it('validates username length', async () => {
		render(Register);
		const usernameInput = screen.getByLabelText('Username');
		const submitButton = screen.getByRole('button', { name: /Register/i });
		const user = userEvent.setup();

		await user.type(usernameInput, 'ab');
		await user.click(submitButton);

		await waitFor(() => {
			expect(screen.getByText('Username must be at least 3 characters')).toBeInTheDocument();
		});
	});

	it('validates email format', async () => {
		render(Register);
		const emailInput = screen.getByLabelText('Email');
		const submitButton = screen.getByRole('button', { name: /Register/i });
		const user = userEvent.setup();

		await user.type(emailInput, 'invalid-email');
		await user.click(submitButton);

		await waitFor(() => {
			expect(screen.getByText('Email is invalid')).toBeInTheDocument();
		});
	});

	it('validates password length', async () => {
		render(Register);
		const passwordInput = screen.getByLabelText('Password');
		const submitButton = screen.getByRole('button', { name: /Register/i });
		const user = userEvent.setup();

		await user.type(passwordInput, 'short');
		await user.click(submitButton);

		await waitFor(() => {
			expect(screen.getByText('Password must be at least 8 characters')).toBeInTheDocument();
		});
	});

	it('submits form with valid data', async () => {
		render(Register);
		const usernameInput = screen.getByLabelText('Username');
		const emailInput = screen.getByLabelText('Email');
		const passwordInput = screen.getByLabelText('Password');
		const submitButton = screen.getByRole('button', { name: /Register/i });

		const user = userEvent.setup();
		await user.type(usernameInput, 'testuser');
		await user.type(emailInput, 'test@example.com');
		await user.type(passwordInput, 'password123');
		await user.click(submitButton);

		await waitFor(() => {
			expect(global.fetch).toHaveBeenCalledWith(
				expect.objectContaining({
					method: 'POST',
					headers: expect.objectContaining({
						'Content-Type': 'application/json'
					})
				})
			);
		});
	});
});
