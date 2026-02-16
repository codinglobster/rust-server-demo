// 认证状态管理

import { writable, derived } from 'svelte/store';
import { authApi } from '../api/auth.api';
import type { RegisterFormData, LoginFormData } from '$lib/types/forms';
import type { User } from '$lib/types/models';

export interface AuthState {
	user: User | null;
	isAuthenticated: boolean;
	isLoading: boolean;
	error: string | null;
}

function createAuthStore() {
	const { subscribe, set, update } = writable<AuthState>({
		user: null,
		isAuthenticated: false,
		isLoading: true,
		error: null,
	});

	return {
		subscribe,

		async initialize() {
			// 检查 token 是否存在
			const token = typeof window !== 'undefined' ? localStorage.getItem('access_token') : null;
			if (!token) {
				update((state) => ({ ...state, isLoading: false }));
				return;
			}

			// TODO: 验证 token 有效性，获取用户信息
			update((state) => ({ ...state, isAuthenticated: true, isLoading: false }));
		},

		async register(data: RegisterFormData) {
			update((state) => ({ ...state, isLoading: true, error: null }));

			try {
				const response = await authApi.register(data);

				if (response.status === 201) {
					set({
						user: response.data.user,
						isAuthenticated: true,
						isLoading: false,
						error: null,
					});
				} else {
					update((state) => ({
						...state,
						isLoading: false,
						error: response.data.error || 'Registration failed',
					}));
				}
			} catch (error) {
				update((state) => ({
					...state,
					isLoading: false,
					error: error instanceof Error ? error.message : 'An error occurred',
				}));
			}
		},

		async login(data: LoginFormData) {
			update((state) => ({ ...state, isLoading: true, error: null }));

			try {
				const response = await authApi.login(data);

				if (response.status === 200) {
					set({
						user: response.data.user,
						isAuthenticated: true,
						isLoading: false,
						error: null,
					});
				} else {
					update((state) => ({
						...state,
						isLoading: false,
						error: response.data.error || 'Login failed',
					}));
				}
			} catch (error) {
				update((state) => ({
					...state,
					isLoading: false,
					error: error instanceof Error ? error.message : 'An error occurred',
				}));
			}
		},

		async logout() {
			await authApi.logout();
			set({
				user: null,
				isAuthenticated: false,
				isLoading: false,
				error: null,
			});
		},

		clearError() {
			update((state) => ({ ...state, error: null }));
		},
	};
}

export const authStore = createAuthStore();
export const currentUser = derived(authStore, ($auth) => $auth.user);
export const isAuthenticated = derived(authStore, ($auth) => $auth.isAuthenticated);
