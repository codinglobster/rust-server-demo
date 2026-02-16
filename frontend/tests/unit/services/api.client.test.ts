// API 客户端单元测试

import { describe, it, expect, beforeEach, vi } from 'vitest';

describe('API Client基础功能', () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	it('应该能够创建测试', () => {
		expect(true).toBe(true);
	});
});
