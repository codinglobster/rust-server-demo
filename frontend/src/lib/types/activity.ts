// Activity Log Types

export interface ActivityLogDto {
	id: string;
	user_id: string | null;
	username: string | null;
	event_type: string;
	event_type_category: string;
	description: string;
	metadata: Record<string, unknown> | null;
	ip_address: string | null;
	user_agent: string | null;
	created_at: string;
}

export interface ActivityLogsResponse {
	activities: ActivityLogDto[];
	total: number;
	page: number;
	per_page: number;
}

export type ActivityEventType =
	| 'user_registered'
	| 'user_logged_in'
	| 'user_logged_out'
	| 'user_updated'
	| 'user_password_changed'
	| 'message_sent'
	| 'message_edited'
	| 'message_deleted'
	| 'room_joined'
	| 'room_left'
	| 'room_created'
	| 'system_alert'
	| 'system_notification'
	| 'error_occurred';

export interface ActivityEvent {
	type: ActivityEventType;
	description: string;
	icon?: string;
	color?: string;
}
