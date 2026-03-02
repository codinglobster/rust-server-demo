// 消息管理 API

import { apiClient } from './client';
import type { ApiResponse, PaginatedResponse } from '$lib/types/api';
import type { Message, CreateMessageRequest, UpdateMessageRequest } from '$lib/types/models';

export const messagesApi = {
	/**
	 * 创建消息
	 */
	async createMessage(data: CreateMessageRequest): Promise<ApiResponse<Message>> {
		return apiClient.request<Message>('/api/messages', {
			method: 'POST',
			body: JSON.stringify(data),
		});
	},

	/**
	 * 获取消息列表（支持按房间筛选）
	 */
	async listMessages(
		page: number = 1,
		perPage: number = 50,
		roomId?: string
	): Promise<ApiResponse<PaginatedResponse<Message>>> {
		const params = new URLSearchParams({
			page: page.toString(),
			per_page: perPage.toString(),
		});

		if (roomId) {
			params.append('room_id', roomId);
		}

		return apiClient.request<PaginatedResponse<Message>>(
			`/api/messages?${params.toString()}`
		);
	},

	/**
	 * 获取单条消息
	 */
	async getMessage(messageId: string): Promise<ApiResponse<Message>> {
		return apiClient.request<Message>(`/api/messages/${messageId}`);
	},

	/**
	 * 编辑消息
	 */
	async updateMessage(
		messageId: string,
		data: UpdateMessageRequest
	): Promise<ApiResponse<Message>> {
		return apiClient.request<Message>(`/api/messages/${messageId}`, {
			method: 'PUT',
			body: JSON.stringify(data),
		});
	},

	/**
	 * 删除消息
	 */
	async deleteMessage(messageId: string): Promise<ApiResponse<void>> {
		return apiClient.request<void>(`/api/messages/${messageId}`, {
			method: 'DELETE',
		});
	},

	/**
	 * 获取房间消息历史
	 */
	async getRoomMessages(
		roomId: string,
		page: number = 1,
		perPage: number = 50
	): Promise<ApiResponse<PaginatedResponse<Message>>> {
		return apiClient.request<PaginatedResponse<Message>>(
			`/api/rooms/${roomId}/messages?page=${page}&per_page=${perPage}`
		);
	},
};
