// Toast 通知 Store

import { writable } from 'svelte/store';

export type ToastType = 'success' | 'error' | 'warning' | 'info';

export interface Toast {
	id: string;
	type: ToastType;
	message: string;
	duration?: number;
	dismissible?: boolean;
}

interface ToastStore {
	toasts: Toast[];
}

function createToastStore() {
	const { subscribe, update } = writable<ToastStore>({
		toasts: [],
	});

	let toastId = 0;

	return {
		subscribe,

		/**
		 * 显示通知
		 */
		show(
			type: ToastType,
			message: string,
			duration: number = 5000,
			dismissible: boolean = true
		) {
			const id = `toast-${++toastId}`;
			const toast: Toast = {
				id,
				type,
				message,
				duration,
				dismissible,
			};

			update((state) => ({
				toasts: [...state.toasts, toast],
			}));

			// 自动移除
			if (duration > 0) {
				setTimeout(() => {
					this.dismiss(id);
				}, duration);
			}

			return id;
		},

		/**
		 * 显示成功通知
		 */
		success(message: string, duration?: number) {
			return this.show('success', message, duration);
		},

		/**
		 * 显示错误通知
		 */
		error(message: string, duration?: number) {
			return this.show('error', message, duration);
		},

		/**
		 * 显示警告通知
		 */
		warning(message: string, duration?: number) {
			return this.show('warning', message, duration);
		},

		/**
		 * 显示信息通知
		 */
		info(message: string, duration?: number) {
			return this.show('info', message, duration);
		},

		/**
		 * 关闭通知
		 */
		dismiss(id: string) {
			update((state) => ({
				toasts: state.toasts.filter((t) => t.id !== id),
			}));
		},

		/**
		 * 清空所有通知
		 */
		clear() {
			update(() => ({ toasts: [] }));
		},
	};
}

export const toastStore = createToastStore();
