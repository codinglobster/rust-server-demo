// Activity API Client 测试
import { describe, it, expect, beforeEach, vi } from 'vitest';
import { activityApi } from '$lib/services/api/activity.api';
import type { ActivityLogDto, ActivityLogsResponse } from '$lib/types/activity';

// Mock apiClient
vi.mock('$lib/services/api/client', () => ({
	apiClient: {
		request: vi.fn()
	}
}));

import { apiClient } from '$lib/services/api/client';

describe('Activity API', () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	describe('getRecentActivities', () => {
		it('should fetch recent activities with default limit', async () => {
			const mockActivities: ActivityLogDto[] = [
				{
					id: '123e4567-e89b-12d3-a456-426614174000',
					user_id: '123e4567-e89b-12d3-a456-426614174001',
					username: 'test_user',
					event_type: 'user_logged_in',
					event_type_category: 'user',
					description: 'User logged in successfully',
					metadata: null,
					ip_address: '192.168.1.1',
					user_agent: 'Mozilla/5.0',
					created_at: '2024-01-15T10:30:00Z'
				}
			];

			vi.mocked(apiClient.request).mockResolvedValue({
				status: 200,
				statusText: 'OK',
				data: mockActivities,
				headers: new Headers()
			} as never);

			const result = await activityApi.getRecentActivities(50);

			expect(apiClient.request).toHaveBeenCalledWith('/api/activities/recent?limit=50');
			expect(result.data).toEqual(mockActivities);
		});

		it('should fetch recent activities with custom limit', async () => {
			vi.mocked(apiClient.request).mockResolvedValue({
				status: 200,
				statusText: 'OK',
				data: [],
				headers: new Headers()
			} as never);

			await activityApi.getRecentActivities(100);

			expect(apiClient.request).toHaveBeenCalledWith('/api/activities/recent?limit=100');
		});
	});

	describe('listActivities', () => {
		it('should fetch activities with default pagination', async () => {
			const mockResponse: ActivityLogsResponse = {
				activities: [],
				total: 100,
				page: 1,
				per_page: 20
			};

			vi.mocked(apiClient.request).mockResolvedValue({
				status: 200,
				statusText: 'OK',
				data: mockResponse,
				headers: new Headers()
			} as never);

			const result = await activityApi.listActivities();

			expect(apiClient.request).toHaveBeenCalledWith('/api/activities?page=1&per_page=20');
			expect(result.data).toEqual(mockResponse);
		});

		it('should fetch activities with custom pagination', async () => {
			const mockResponse: ActivityLogsResponse = {
				activities: [],
				total: 100,
				page: 2,
				per_page: 50
			};

			vi.mocked(apiClient.request).mockResolvedValue({
				status: 200,
				statusText: 'OK',
				data: mockResponse,
				headers: new Headers()
			} as never);

			const result = await activityApi.listActivities(2, 50);

			expect(apiClient.request).toHaveBeenCalledWith('/api/activities?page=2&per_page=50');
			expect(result.data).toEqual(mockResponse);
		});
	});

	describe('getUserActivities', () => {
		it('should fetch user activities with default pagination', async () => {
			const userId = '123e4567-e89b-12d3-a456-426614174000';
			const mockResponse: ActivityLogsResponse = {
				activities: [],
				total: 25,
				page: 1,
				per_page: 20
			};

			vi.mocked(apiClient.request).mockResolvedValue({
				status: 200,
				statusText: 'OK',
				data: mockResponse,
				headers: new Headers()
			} as never);

			const result = await activityApi.getUserActivities(userId);

			expect(apiClient.request).toHaveBeenCalledWith(
				`/api/activities/user/${userId}?page=1&per_page=20`
			);
			expect(result.data).toEqual(mockResponse);
		});

		it('should fetch user activities with custom pagination', async () => {
			const userId = '123e4567-e89b-12d3-a456-426614174000';
			const mockResponse: ActivityLogsResponse = {
				activities: [],
				total: 25,
				page: 2,
				per_page: 10
			};

			vi.mocked(apiClient.request).mockResolvedValue({
				status: 200,
				statusText: 'OK',
				data: mockResponse,
				headers: new Headers()
			} as never);

			const result = await activityApi.getUserActivities(userId, 2, 10);

			expect(apiClient.request).toHaveBeenCalledWith(
				`/api/activities/user/${userId}?page=2&per_page=10`
			);
			expect(result.data).toEqual(mockResponse);
		});
	});

	describe('createActivity', () => {
		it('should create a new activity log', async () => {
			const newActivity = {
				user_id: '123e4567-e89b-12d3-a456-426614174000',
				event_type: 'test_event',
				description: 'Test activity description',
				metadata: { key: 'value' },
				ip_address: '192.168.1.1',
				user_agent: 'TestAgent/1.0'
			};

			const mockCreatedActivity: ActivityLogDto = {
				id: '123e4567-e89b-12d3-a456-426614174002',
				...newActivity,
				username: 'test_user',
				event_type_category: 'other',
				metadata: { key: 'value' },
				created_at: '2024-01-15T10:30:00Z'
			};

			vi.mocked(apiClient.request).mockResolvedValue({
				status: 201,
				statusText: 'Created',
				data: mockCreatedActivity,
				headers: new Headers()
			} as never);

			const result = await activityApi.createActivity(newActivity);

			expect(apiClient.request).toHaveBeenCalledWith('/api/activities', {
				method: 'POST',
				body: JSON.stringify(newActivity)
			});
			expect(result.data).toEqual(mockCreatedActivity);
		});

		it('should create activity without optional fields', async () => {
			const minimalActivity = {
				event_type: 'test_event',
				description: 'Minimal test activity'
			};

			vi.mocked(apiClient.request).mockResolvedValue({
				status: 201,
				statusText: 'Created',
				data: {
					id: '123e4567-e89b-12d3-a456-426614174002',
					user_id: null,
					username: null,
					event_type: 'test_event',
					event_type_category: 'other',
					description: 'Minimal test activity',
					metadata: null,
					ip_address: null,
					user_agent: null,
					created_at: '2024-01-15T10:30:00Z'
				},
				headers: new Headers()
			} as never);

			const result = await activityApi.createActivity(minimalActivity);

			expect(apiClient.request).toHaveBeenCalledWith('/api/activities', {
				method: 'POST',
				body: JSON.stringify(minimalActivity)
			});
			expect(result.data.user_id).toBeNull();
		});
	});

	describe('error handling', () => {
		it('should handle API errors gracefully', async () => {
			vi.mocked(apiClient.request).mockRejectedValue(
				new Error('Network error')
			);

			await expect(activityApi.getRecentActivities()).rejects.toThrow('Network error');
		});

		it('should handle empty responses', async () => {
			vi.mocked(apiClient.request).mockResolvedValue({
				status: 200,
				statusText: 'OK',
				data: [],
				headers: new Headers()
			} as never);

			const result = await activityApi.getRecentActivities();
			expect(result.data).toEqual([]);
		});
	});
});
