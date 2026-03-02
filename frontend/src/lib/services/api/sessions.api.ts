// 会话管理 API

import { apiClient } from './client';
import type { ApiResponse, PaginatedResponse } from '$lib/types/api';
import type { Session } from '$lib/types/models';

export const sessionsApi = {
	/**
	 * 获取当前用户的所有活跃会话
	 */
	async listSessions(
		page: number = 1,
		perPage: number = 20
	): Promise<ApiResponse<PaginatedResponse<Session>>> {
		return apiClient.request<PaginatedResponse<Session>>(
			`/api/sessions?page=${page}&per_page=${perPage}`
		);
	},

	/**
	 * 获取指定会话详情
	 */
	async getSession(sessionId: string): Promise<ApiResponse<Session>> {
		return apiClient.request<Session>(`/api/sessions/${sessionId}`);
	},

	/**
	 * 销毁指定会话
	 */
	async deleteSession(sessionId: string): Promise<ApiResponse<void>> {
		return apiClient.request<void>(`/api/sessions/${sessionId}`, {
			method: 'DELETE',
		});
	},

	/**
	 * 销毁除当前会话外的所有会话
	 */
	async deleteOtherSessions(): Promise<ApiResponse<void>> {
		return apiClient.request<void>('/api/sessions/other', {
			method: 'DELETE',
		});
	},

	/**
	 * 获取活跃会话统计
	 */
	async getActiveSessionsCount(): Promise<ApiResponse<{ count: number }>> {
		return apiClient.request<{ count: number }>('/api/sessions/active');
	},
};
