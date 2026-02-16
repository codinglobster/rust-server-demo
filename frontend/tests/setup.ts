// Vitest 测试设置文件

import { afterEach, vi } from 'vitest';
import { cleanup } from '@testing-library/svelte';

// 每个测试后清理
afterEach(() => {
	cleanup();
	vi.clearAllMocks();
});

// 全局 mocks
global.localStorage = {
	getItem: vi.fn(),
	setItem: vi.fn(),
	removeItem: vi.fn(),
	clear: vi.fn(),
} as unknown as Storage;
