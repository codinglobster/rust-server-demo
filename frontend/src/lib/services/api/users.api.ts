// 用户 API 方法

import { apiClient } from './client';
import type { User } from '$lib/types/models';
import type { UpdateUserFormData, ChangePasswordFormData } from '$lib/types/forms';
import type { ApiResponse, PaginatedResponse } from '$lib/types/api';

export const usersApi = {
	async getMe(): Promise<ApiResponse<User>> {
		return apiClient.request<User>('/api/users/me');
	},

	async getUser(id: string): Promise<ApiResponse<User>> {
		return apiClient.request<User>(`/api/users/${id}`);
	},

	async listUsers(
		page: number = 1,
		perPage: number = 20
	): Promise<ApiResponse<PaginatedResponse<User>>> {
		return apiClient.request<PaginatedResponse<User>>(
			`/api/users?page=${page}&per_page=${perPage}`
		);
	},

	async updateMe(data: UpdateUserFormData): Promise<ApiResponse<User>> {
		return apiClient.request<User>('/api/users/me', {
			method: 'PUT',
			body: JSON.stringify({
				full_name: data.full_name,
				email: data.email,
			}),
		});
	},

	async changePassword(data: ChangePasswordFormData): Promise<ApiResponse<void>> {
		return apiClient.request<void>('/api/users/me/password', {
			method: 'POST',
			body: JSON.stringify({
				old_password: data.old_password,
				new_password: data.new_password,
			}),
		});
	},
};
