// HTTP 客户端封装

export interface RequestOptions {
	method?: string;
	body?: string;
	headers?: Record<string, string>;
	skipAuth?: boolean;
}

export interface ApiResponse<T> {
	status: number;
	statusText: string;
	data: T;
	headers: Headers;
}

export class ApiClient {
	private baseURL: string;
	public accessToken: string | null = null;
	public refreshToken: string | null = null;

	constructor() {
		this.baseURL = import.meta.env.VITE_API_URL || 'http://localhost:8080';
		// 从 localStorage 加载 token
		if (typeof window !== 'undefined') {
			this.accessToken = localStorage.getItem('access_token');
			this.refreshToken = localStorage.getItem('refresh_token');
		}
	}

	async request<T>(endpoint: string, options: RequestOptions = {}): Promise<ApiResponse<T>> {
		const url = `${this.baseURL}${endpoint}`;
		console.log('[ApiClient] Request:', options.method || 'GET', url);
		console.log('[ApiClient] skipAuth:', options.skipAuth);

		const headers: Record<string, string> = {
			'Content-Type': 'application/json',
			...options.headers,
		};

		if (this.accessToken && !options.skipAuth) {
			headers['Authorization'] = `Bearer ${this.accessToken}`;
			console.log('[ApiClient] Adding Authorization header');
		}

		const fetchOptions: RequestInit = {
			method: options.method || 'GET',
			headers,
		};

		if (options.body) {
			fetchOptions.body = options.body;
		}

		console.log('[ApiClient] Fetch options:', fetchOptions);

		const response = await fetch(url, fetchOptions);
		console.log('[ApiClient] Response status:', response.status);

		const result = {
			status: response.status,
			statusText: response.statusText,
			data: response.status !== 204 ? await response.json() : (null as T),
			headers: response.headers,
		};

		console.log('[ApiClient] Response data:', result.data);

		return result;
	}

	setTokens(accessToken: string, refreshToken: string) {
		this.accessToken = accessToken;
		this.refreshToken = refreshToken;
		if (typeof window !== 'undefined') {
			localStorage.setItem('access_token', accessToken);
			localStorage.setItem('refresh_token', refreshToken);
		}
	}

	clearTokens() {
		this.accessToken = null;
		this.refreshToken = null;
		if (typeof window !== 'undefined') {
			localStorage.removeItem('access_token');
			localStorage.removeItem('refresh_token');
		}
	}

	get isAuthenticated(): boolean {
		return !!this.accessToken;
	}
}

export const apiClient = new ApiClient();
