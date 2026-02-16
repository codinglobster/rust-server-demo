// 认证 API 方法

import { apiClient } from './client';
import type { RegisterFormData, LoginFormData } from '$lib/types/forms';
import type { LoginResponse, TokenResponse } from '$lib/types/models';
import type { ApiResponse } from '$lib/types/api';

export const authApi = {
	async register(data: RegisterFormData): Promise<ApiResponse<LoginResponse>> {
		console.log('authApi.register called with:', data);
		const response = await apiClient.request<LoginResponse>('/api/auth/register', {
			method: 'POST',
			body: JSON.stringify({
				username: data.username,
				email: data.email,
				password: data.password,
				full_name: data.full_name,
			}),
			skipAuth: true,
		});

		console.log('Registration response status:', response.status);
		console.log('Registration response data:', response.data);

		if (response.status === 201) {
			console.log('Setting tokens...');
			apiClient.setTokens(response.data.access_token, response.data.refresh_token);
			console.log('Tokens set successfully');
		} else {
			console.log('Registration failed with status:', response.status);
		}

		return response;
	},

	async login(data: LoginFormData): Promise<ApiResponse<LoginResponse>> {
		const response = await apiClient.request<LoginResponse>('/api/auth/login', {
			method: 'POST',
			body: JSON.stringify({
				username: data.username,
				password: data.password,
			}),
			skipAuth: true,
		});

		if (response.status === 200) {
			apiClient.setTokens(response.data.access_token, response.data.refresh_token);
		}

		return response;
	},

	async logout(): Promise<ApiResponse<void>> {
		const response = await apiClient.request<void>('/api/auth/logout', {
			method: 'POST',
		});

		if (response.status === 204) {
			apiClient.clearTokens();
		}

		return response;
	},

	async refreshToken(): Promise<ApiResponse<TokenResponse>> {
		if (!apiClient.refreshToken) {
			throw new Error('No refresh token available');
		}

		const response = await apiClient.request<TokenResponse>('/api/auth/refresh', {
			method: 'POST',
			body: JSON.stringify({
				refresh_token: apiClient.refreshToken,
			}),
			skipAuth: true,
		});

		if (response.status === 200) {
			apiClient.setTokens(response.data.access_token, response.data.refresh_token);
		}

		return response;
	},
};
