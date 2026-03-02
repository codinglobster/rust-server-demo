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

// Room types
export type RoomRole = 'owner' | 'admin' | 'member';

export interface Room {
	id: string;
	name: string;
	description?: string;
	owner_id: string;
	is_private: boolean;
	max_members: number;
	member_count?: number;
	created_at: string;
	updated_at: string;
}

export interface RoomMember {
	room_id: string;
	user_id: string;
	username?: string;
	role: RoomRole;
	joined_at: string;
}

export interface CreateRoomRequest {
	name: string;
	description?: string;
	is_private?: boolean;
	max_members?: number;
}

export interface UpdateRoomRequest {
	name?: string;
	description?: string;
	is_private?: boolean;
	max_members?: number;
}

// Message types
export interface Message {
	id: string;
	room_id: string;
	sender_id: string;
	sender_username?: string;
	content: string;
	created_at: string;
	updated_at: string;
}

export interface CreateMessageRequest {
	room_id: string;
	content: string;
}

export interface UpdateMessageRequest {
	content: string;
}

// Session types
export interface Session {
	id: string;
	user_id: string;
	token_hash: string;
	ip_address?: string;
	user_agent?: string;
	expires_at: string;
	created_at: string;
	last_used_at: string;
	is_current?: boolean;
}

// WebSocket message types
export type WebSocketMessageType =
	| 'ping'
	| 'pong'
	| 'join_room'
	| 'leave_room'
	| 'chat'
	| 'user_joined'
	| 'user_left'
	| 'message'
	| 'typing'
	| 'error';

export interface WebSocketMessage {
	type: WebSocketMessageType;
	room_id?: string;
	content?: string;
	user_id?: string;
	username?: string;
	message_id?: string;
	timestamp?: string;
	error?: string;
}
