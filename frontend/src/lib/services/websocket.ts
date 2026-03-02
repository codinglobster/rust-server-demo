// WebSocket 客户端服务

import type { WebSocketMessage } from '$lib/types/models';

export type ConnectionStatus = 'connecting' | 'connected' | 'disconnecting' | 'disconnected' | 'error';

export interface WebSocketEventHandlers {
	onOpen?: () => void;
	onClose?: () => void;
	onError?: (error: Event) => void;
	onMessage?: (message: WebSocketMessage) => void;
	onConnectionStatusChange?: (status: ConnectionStatus) => void;
}

export class WebSocketClient {
	private ws: WebSocket | null = null;
	private url: string;
	private reconnectAttempts = 0;
	private maxReconnectAttempts = 5;
	private reconnectDelay = 1000;
	private heartbeatInterval: number | null = null;
	private heartbeatTimeout: number | null = null;
	private eventHandlers: WebSocketEventHandlers = {};
	private connectionStatus: ConnectionStatus = 'disconnected';
	private messageQueue: WebSocketMessage[] = [];
	private accessToken: string | null = null;

	constructor(baseUrl?: string) {
		const wsProtocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
		const host = baseUrl || window.location.host;
		this.url = `${wsProtocol}//${host}/ws`;
	}

	/**
	 * 设置访问令牌
	 */
	setAccessToken(token: string | null) {
		this.accessToken = token;
	}

	/**
	 * 注册事件处理器
	 */
	on(handlers: WebSocketEventHandlers) {
		this.eventHandlers = { ...this.eventHandlers, ...handlers };
	}

	/**
	 * 连接到 WebSocket 服务器
	 */
	connect(): Promise<void> {
		return new Promise((resolve, reject) => {
			if (this.ws?.readyState === WebSocket.OPEN) {
				resolve();
				return;
			}

			this.setConnectionStatus('connecting');

			try {
				// 如果有 token，添加到 URL
				const url = this.accessToken
					? `${this.url}?token=${this.accessToken}`
					: this.url;

				this.ws = new WebSocket(url);

				this.ws.onopen = () => {
					console.log('[WebSocket] Connected');
					this.setConnectionStatus('connected');
					this.reconnectAttempts = 0;
					this.startHeartbeat();
					this.flushMessageQueue();
					this.eventHandlers.onOpen?.();
					resolve();
				};

				this.ws.onclose = (event) => {
					console.log('[WebSocket] Disconnected', event.code, event.reason);
					this.setConnectionStatus('disconnected');
					this.stopHeartbeat();
					this.eventHandlers.onClose?.();

					// 自动重连（非正常关闭）
					if (!event.wasClean && this.reconnectAttempts < this.maxReconnectAttempts) {
						this.scheduleReconnect();
					}
				};

				this.ws.onerror = (error) => {
					console.error('[WebSocket] Error', error);
					this.setConnectionStatus('error');
					this.eventHandlers.onError?.(error);
					reject(error);
				};

				this.ws.onmessage = (event) => {
					try {
						const message: WebSocketMessage = JSON.parse(event.data);
						this.handleMessage(message);
					} catch (error) {
						console.error('[WebSocket] Failed to parse message', error);
					}
				};
			} catch (error) {
				console.error('[WebSocket] Connection failed', error);
				this.setConnectionStatus('error');
				reject(error);
			}
		});
	}

	/**
	 * 断开连接
	 */
	disconnect() {
		this.setConnectionStatus('disconnecting');
		this.stopHeartbeat();

		if (this.ws) {
			this.ws.close(1000, 'Client disconnecting');
			this.ws = null;
		}

		this.setConnectionStatus('disconnected');
	}

	/**
	 * 发送消息
	 */
	send(message: WebSocketMessage): boolean {
		if (this.ws?.readyState === WebSocket.OPEN) {
			try {
				this.ws.send(JSON.stringify(message));
				return true;
			} catch (error) {
				console.error('[WebSocket] Failed to send message', error);
				return false;
			}
		} else {
			console.warn('[WebSocket] Not connected, queueing message');
			this.messageQueue.push(message);
			return false;
		}
	}

	/**
	 * 加入房间
	 */
	joinRoom(roomId: string): boolean {
		return this.send({
			type: 'join_room',
			room_id: roomId,
		});
	}

	/**
	 * 离开房间
	 */
	leaveRoom(roomId: string): boolean {
		return this.send({
			type: 'leave_room',
			room_id: roomId,
		});
	}

	/**
	 * 发送聊天消息
	 */
	sendChatMessage(roomId: string, content: string): boolean {
		return this.send({
			type: 'chat',
			room_id: roomId,
			content,
		});
	}

	/**
	 * 发送正在输入状态
	 */
	sendTyping(roomId: string): boolean {
		return this.send({
			type: 'typing',
			room_id: roomId,
		});
	}

	/**
	 * 获取连接状态
	 */
	getConnectionStatus(): ConnectionStatus {
		return this.connectionStatus;
	}

	/**
	 * 是否已连接
	 */
	isConnected(): boolean {
		return this.ws?.readyState === WebSocket.OPEN;
	}

	// ==================== 私有方法 ====================

	private setConnectionStatus(status: ConnectionStatus) {
		if (this.connectionStatus !== status) {
			this.connectionStatus = status;
			this.eventHandlers.onConnectionStatusChange?.(status);
		}
	}

	private handleMessage(message: WebSocketMessage) {
		// 处理 pong 消息
		if (message.type === 'pong') {
			this.resetHeartbeatTimeout();
			return;
		}

		// 触发消息处理器
		this.eventHandlers.onMessage?.(message);
	}

	private startHeartbeat() {
		// 每 30 秒发送一次 ping
		this.heartbeatInterval = window.setInterval(() => {
			if (this.isConnected()) {
				this.send({ type: 'ping' });

				// 设置超时检测（10 秒内未收到 pong 则认为连接断开）
				this.heartbeatTimeout = window.setTimeout(() => {
					console.warn('[WebSocket] Heartbeat timeout, reconnecting...');
					this.disconnect();
					this.scheduleReconnect();
				}, 10000);
			}
		}, 30000);
	}

	private stopHeartbeat() {
		if (this.heartbeatInterval !== null) {
			clearInterval(this.heartbeatInterval);
			this.heartbeatInterval = null;
		}
		if (this.heartbeatTimeout !== null) {
			clearTimeout(this.heartbeatTimeout);
			this.heartbeatTimeout = null;
		}
	}

	private resetHeartbeatTimeout() {
		if (this.heartbeatTimeout !== null) {
			clearTimeout(this.heartbeatTimeout);
			this.heartbeatTimeout = null;
		}
	}

	private scheduleReconnect() {
		this.reconnectAttempts++;
		const delay = this.reconnectDelay * Math.pow(2, this.reconnectAttempts - 1);

		console.log(
			`[WebSocket] Scheduling reconnect attempt ${this.reconnectAttempts}/${this.maxReconnectAttempts} in ${delay}ms`
		);

		setTimeout(() => {
			if (this.reconnectAttempts <= this.maxReconnectAttempts) {
				this.connect().catch((error) => {
					console.error('[WebSocket] Reconnect failed', error);
				});
			}
		}, delay);
	}

	private flushMessageQueue() {
		if (this.messageQueue.length > 0) {
			console.log(`[WebSocket] Flushing ${this.messageQueue.length} queued messages`);
			const queue = [...this.messageQueue];
			this.messageQueue = [];

			queue.forEach((message) => {
				this.send(message);
			});
		}
	}
}

// 创建全局单例
let wsClient: WebSocketClient | null = null;

export function getWebSocketClient(): WebSocketClient {
	if (!wsClient) {
		const apiUrl = import.meta.env.VITE_API_URL;
		const baseUrl = apiUrl ? new URL(apiUrl).host : undefined;
		wsClient = new WebSocketClient(baseUrl);
	}
	return wsClient;
}

export function destroyWebSocketClient() {
	if (wsClient) {
		wsClient.disconnect();
		wsClient = null;
	}
}
