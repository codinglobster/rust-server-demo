// Activity Log API
import { apiClient } from './client';
import type { ApiResponse } from './client';
import type { ActivityLogDto, ActivityLogsResponse } from '$lib/types/activity';

export interface CreateActivityLogRequest {
	user_id?: string;
	event_type: string;
	description: string;
	metadata?: Record<string, unknown>;
	ip_address?: string;
	user_agent?: string;
}

export const activityApi = {
	// Get recent activities (cached)
	async getRecentActivities(limit: number = 50): Promise<ApiResponse<ActivityLogDto[]>> {
		return apiClient.request<ActivityLogDto[]>(`/api/activities/recent?limit=${limit}`);
	},

	// List all activities with pagination
	async listActivities(page: number = 1, perPage: number = 20): Promise<ApiResponse<ActivityLogsResponse>> {
		return apiClient.request<ActivityLogsResponse>(
			`/api/activities?page=${page}&per_page=${perPage}`
		);
	},

	// Get activities for a specific user
	async getUserActivities(
		userId: string,
		page: number = 1,
		perPage: number = 20
	): Promise<ApiResponse<ActivityLogsResponse>> {
		return apiClient.request<ActivityLogsResponse>(
			`/api/activities/user/${userId}?page=${page}&per_page=${perPage}`
		);
	},

	// Create an activity log
	async createActivity(data: CreateActivityLogRequest): Promise<ApiResponse<ActivityLogDto>> {
		return apiClient.request<ActivityLogDto>('/api/activities', {
			method: 'POST',
			body: JSON.stringify(data)
		});
	}
};
