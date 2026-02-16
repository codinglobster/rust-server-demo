// 通知系统

import { writable, derived } from 'svelte/store';

export type NotificationType = 'success' | 'error' | 'warning' | 'info';

export interface Notification {
	id: string;
	type: NotificationType;
	message: string;
	duration?: number;
}

function createNotificationStore() {
	const { subscribe, update } = writable<Notification[]>([]);

	return {
		subscribe,

		show(type: NotificationType, message: string, duration = 5000) {
			const id = crypto.randomUUID();
			const notification: Notification = { id, type, message, duration };

			update((notifications) => [...notifications, notification]);

			if (duration > 0) {
				setTimeout(() => {
					update((notifications) => notifications.filter((n) => n.id !== id));
				}, duration);
			}
		},

		success(message: string) {
			this.show('success', message);
		},

		error(message: string) {
			this.show('error', message);
		},

		info(message: string) {
			this.show('info', message);
		},

		warning(message: string) {
			this.show('warning', message);
		},

		dismiss(id: string) {
			update((notifications) => notifications.filter((n) => n.id !== id));
		},
	};
}

export const notificationStore = createNotificationStore();
