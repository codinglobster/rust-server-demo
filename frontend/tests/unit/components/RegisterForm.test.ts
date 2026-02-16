// 表单组件单元测试

import { render, screen } from '@testing-library/svelte';
import { describe, it } from 'vitest';
import RegisterForm from '$lib/components/auth/RegisterForm.svelte';

describe('Register Form Component', () => {
	it('should render form fields correctly', () => {
		render(RegisterForm, { });

		expect(screen.getByLabelText('Username')).toBeInTheDocument();
		expect(screen.getByLabelText('Email')).toBeInTheDocument();
		expect(screen.getByLabelText('Password')).toBeInTheDocument();
		expect(screen.getByLabelText('Full Name (Optional)')).toBeInTheDocument();
	});

	it('should show validation errors for invalid input', () => {
		render(RegisterForm, { });

		const usernameInput = screen.getByLabelText('Username');
		const submitButton = screen.getByRole('button', { name: 'submit' });

		// 提交空表单
		await fireEvent.click(submitButton);

		// 等待错误提示
		await waitFor(() => {
			const errorText = screen.querySelector('text-red-600');
			expect(errorText).toBeTruthy();
		});
	});
});
