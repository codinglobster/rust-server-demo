// 领域模型类型定义

export type UserRole = 'user' | 'moderator' | 'admin';

export interface User {
	id: string;
	username: string;
	email: string;
	full_name?: string;
	is_active: boolean;
	is_verified: boolean;
	role: UserRole;
	created_at: string;
	updated_at: string;
	last_login_at?: string;
}

export interface TokenResponse {
	access_token: string;
	refresh_token: string;
	token_type: string;
	expires_in: number;
	user: User;
}

export interface LoginResponse {
	access_token: string;
	refresh_token: string;
	token_type: string;
	expires_in: number;
	user: User;
}
