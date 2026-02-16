// API 响应类型定义

export interface ApiResponse<T> {
	status: number;
	statusText: string;
	data: T;
	headers: Headers;
}

export interface ApiError {
	error: string;
	message?: string;
	details?: Record<string, string[]>;
}

export interface PaginatedResponse<T> {
	users: T[];
	total: number;
	page: number;
	per_page: number;
}
