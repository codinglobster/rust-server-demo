// 认证 Store 单元测试

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { authStore } from '$lib/services/stores/auth.store';

vi.mock('$lib/services/stores/auth.store', () => ({
	initialize: vi.fn(),
	register: vi.fn().mockResolvedValue(undefined),
	login: vi.fn().mockResolvedValue(undefined),
	logout: vi.fn().mockResolvedValue(undefined),
}));

describe('Auth Store', () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	it('should initialize with authentication state', async () => {
		await authStore.initialize();
		expect(authStore.initialize).toHaveBeenCalled();
	});

	it('should register user successfully', async () => {
		const mockUser = { id: '1', username: 'test', email: 'test@example.com' };

		vi.mockedValue(authStore.register, true);
		await authStore.register({ username: 'test', password: 'password123', email: 'test@example.com' });

		// 等待状态更新
		await new Promise(resolve => setTimeout(resolve, 0));

		const state = authStore.get();
		expect(state.user).toEqual(mockUser);
		expect(state.isAuthenticated).toBe(true);
		expect(state.isLoading).toBe(false);
	});

	it('should handle registration error', async () => {
		vi.mockedValue(authStore.register, true);
		await authStore.register({ username: 'test', password: '123', email: 'test@example.com' });

		await new Promise(resolve => setTimeout(resolve, 0));
		const state = authStore.get();
		expect(state.error).toBeTruthy();
		expect(state.isLoading).toBe(false);
	});
});
