import { describe, it, expect, vi } from 'vitest';
import { render, screen, waitFor } from '@testing-library/svelte';
import '@testing-library/jest-dom';
import userEvent from '@testing-library/user-event';
import Login from '$routes/auth-shadcn/login/+page.svelte';

// Mock fetch
global.fetch = vi.fn(() =>
	Promise.resolve({
		ok: true,
		status: 200,
		json: async () => ({
			access_token: 'test_token',
			refresh_token: 'test_refresh_token'
		})
	}) as Response
) as typeof globalThis & typeof globalThis.fetch;

describe('Login Page', () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	it('renders login form', () => {
		render(Login);
		expect(screen.getByLabelText('Username')).toBeInTheDocument();
		expect(screen.getByLabelText('Password')).toBeInTheDocument();
	});

	it('shows validation errors for empty fields', async () => {
		render(Login);
		const submitButton = screen.getByRole('button', { name: /Sign In/i });
		const user = userEvent.setup();

		await user.click(submitButton);
		await waitFor(() => {
			expect(screen.getByText('Username is required')).toBeInTheDocument();
			expect(screen.getByText('Password is required')).toBeInTheDocument();
		});
	});

	it('submits form with valid data', async () => {
		render(Login);
		const usernameInput = screen.getByLabelText('Username');
		const passwordInput = screen.getByLabelText('Password');
		const submitButton = screen.getByRole('button', { name: /Sign In/i });

		const user = userEvent.setup();
		await user.type(usernameInput, 'testuser');
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

	it('stores tokens in localStorage on successful login', async () => {
		const setItemSpy = vi.spyOn(Storage.prototype, 'setItem');

		render(Login);
		const usernameInput = screen.getByLabelText('Username');
		const passwordInput = screen.getByLabelText('Password');
		const submitButton = screen.getByRole('button', { name: /Sign In/i });

		const user = userEvent.setup();
		await user.type(usernameInput, 'testuser');
		await user.type(passwordInput, 'password123');
		await user.click(submitButton);

		await waitFor(() => {
			expect(setItemSpy).toHaveBeenCalledWith('access_token', 'test_token');
			expect(setItemSpy).toHaveBeenCalledWith('refresh_token', 'test_refresh_token_token');
		});
	});
});
