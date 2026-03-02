// 网络状态检测

import { writable } from 'svelte/store';

interface NetworkStatus {
	online: boolean;
	effectiveType?: string;
	downlink?: number;
	rtt?: number;
}

function createNetworkStore() {
	const { subscribe, set } = writable<NetworkStatus>({
		online: typeof navigator !== 'undefined' ? navigator.onLine : true,
	});

	if (typeof window !== 'undefined') {
		// 监听在线/离线状态
		window.addEventListener('online', () => {
			set({
				online: true,
			});
		});

		window.addEventListener('offline', () => {
			set({
				online: false,
			});
		});

		// 监听网络质量变化（如果浏览器支持）
		if ('connection' in navigator) {
			const connection = (navigator as any).connection;

			const updateConnectionInfo = () => {
				set({
					online: navigator.onLine,
					effectiveType: connection.effectiveType,
					downlink: connection.downlink,
					rtt: connection.rtt,
				});
			};

			connection.addEventListener('change', updateConnectionInfo);
			updateConnectionInfo();
		}
	}

	return {
		subscribe,
	};
}

export const networkStatus = createNetworkStore();
