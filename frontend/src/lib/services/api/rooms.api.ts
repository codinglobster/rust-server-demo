// 房间管理 API

import { apiClient } from './client';
import type { ApiResponse, PaginatedResponse } from '$lib/types/api';
import type {
	Room,
	RoomMember,
	CreateRoomRequest,
	UpdateRoomRequest,
} from '$lib/types/models';

export const roomsApi = {
	/**
	 * 创建房间
	 */
	async createRoom(data: CreateRoomRequest): Promise<ApiResponse<Room>> {
		return apiClient.request<Room>('/api/rooms', {
			method: 'POST',
			body: JSON.stringify(data),
		});
	},

	/**
	 * 获取房间列表
	 */
	async listRooms(
		page: number = 1,
		perPage: number = 20
	): Promise<ApiResponse<PaginatedResponse<Room>>> {
		return apiClient.request<PaginatedResponse<Room>>(
			`/api/rooms?page=${page}&per_page=${perPage}`
		);
	},

	/**
	 * 获取房间详情
	 */
	async getRoom(roomId: string): Promise<ApiResponse<Room>> {
		return apiClient.request<Room>(`/api/rooms/${roomId}`);
	},

	/**
	 * 更新房间信息
	 */
	async updateRoom(roomId: string, data: UpdateRoomRequest): Promise<ApiResponse<Room>> {
		return apiClient.request<Room>(`/api/rooms/${roomId}`, {
			method: 'PUT',
			body: JSON.stringify(data),
		});
	},

	/**
	 * 删除房间
	 */
	async deleteRoom(roomId: string): Promise<ApiResponse<void>> {
		return apiClient.request<void>(`/api/rooms/${roomId}`, {
			method: 'DELETE',
		});
	},

	/**
	 * 加入房间
	 */
	async joinRoom(roomId: string): Promise<ApiResponse<void>> {
		return apiClient.request<void>(`/api/rooms/${roomId}/join`, {
			method: 'POST',
		});
	},

	/**
	 * 离开房间
	 */
	async leaveRoom(roomId: string): Promise<ApiResponse<void>> {
		return apiClient.request<void>(`/api/rooms/${roomId}/leave`, {
			method: 'POST',
		});
	},

	/**
	 * 获取房间成员列表
	 */
	async getRoomMembers(
		roomId: string,
		page: number = 1,
		perPage: number = 50
	): Promise<ApiResponse<PaginatedResponse<RoomMember>>> {
		return apiClient.request<PaginatedResponse<RoomMember>>(
			`/api/rooms/${roomId}/members?page=${page}&per_page=${perPage}`
		);
	},

	/**
	 * 更新房间成员角色
	 */
	async updateMemberRole(
		roomId: string,
		userId: string,
		role: 'admin' | 'member'
	): Promise<ApiResponse<void>> {
		return apiClient.request<void>(`/api/rooms/${roomId}/members/${userId}`, {
			method: 'PUT',
			body: JSON.stringify({ role }),
		});
	},

	/**
	 * 移除房间成员
	 */
	async removeMember(roomId: string, userId: string): Promise<ApiResponse<void>> {
		return apiClient.request<void>(`/api/rooms/${roomId}/members/${userId}`, {
			method: 'DELETE',
		});
	},
};
