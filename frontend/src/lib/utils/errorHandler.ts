// 全局错误处理工具

import { toastStore } from '$lib/services/stores/toast.store';
import type { ApiResponse } from '$lib/types/api';

/**
 * 处理 API 错误响应
 */
export function handleApiError(response: ApiResponse<any>, defaultMessage: string = '操作失败') {
	if (response.status >= 400 && response.status < 500) {
		// 客户端错误
		const errorMessage = response.data?.error || response.data?.message || defaultMessage;
		toastStore.error(errorMessage);
	} else if (response.status >= 500) {
		// 服务器错误
		toastStore.error('服务器错误，请稍后重试');
	} else {
		toastStore.error(defaultMessage);
	}
}

/**
 * 处理异常错误
 */
export function handleException(error: unknown, defaultMessage: string = '发生错误') {
	console.error('Exception:', error);

	if (error instanceof Error) {
		toastStore.error(error.message || defaultMessage);
	} else {
		toastStore.error(defaultMessage);
	}
}

/**
 * 通用错误处理包装器
 */
export async function withErrorHandling<T>(
	fn: () => Promise<T>,
	errorMessage?: string
): Promise<T | null> {
	try {
		return await fn();
	} catch (error) {
		handleException(error, errorMessage);
		return null;
	}
}

/**
 * 验证表单字段
 */
export interface ValidationRule {
	required?: boolean;
	minLength?: number;
	maxLength?: number;
	pattern?: RegExp;
	custom?: (value: any) => boolean;
	message?: string;
}

export function validateField(
	value: any,
	rules: ValidationRule
): { valid: boolean; message?: string } {
	// Required check
	if (rules.required && (!value || (typeof value === 'string' && !value.trim()))) {
		return { valid: false, message: rules.message || '此字段为必填项' };
	}

	// Skip other checks if value is empty and not required
	if (!value) {
		return { valid: true };
	}

	// MinLength check
	if (rules.minLength && value.length < rules.minLength) {
		return {
			valid: false,
			message: rules.message || `最少需要 ${rules.minLength} 个字符`,
		};
	}

	// MaxLength check
	if (rules.maxLength && value.length > rules.maxLength) {
		return {
			valid: false,
			message: rules.message || `最多只能 ${rules.maxLength} 个字符`,
		};
	}

	// Pattern check
	if (rules.pattern && !rules.pattern.test(value)) {
		return { valid: false, message: rules.message || '格式不正确' };
	}

	// Custom validation
	if (rules.custom && !rules.custom(value)) {
		return { valid: false, message: rules.message || '验证失败' };
	}

	return { valid: true };
}

/**
 * 验证多个字段
 */
export function validateForm(
	data: Record<string, any>,
	rules: Record<string, ValidationRule>
): { valid: boolean; errors: Record<string, string> } {
	const errors: Record<string, string> = {};

	for (const [field, fieldRules] of Object.entries(rules)) {
		const result = validateField(data[field], fieldRules);
		if (!result.valid && result.message) {
			errors[field] = result.message;
		}
	}

	return {
		valid: Object.keys(errors).length === 0,
		errors,
	};
}

/**
 * 防抖函数
 */
export function debounce<T extends (...args: any[]) => any>(
	func: T,
	wait: number
): (...args: Parameters<T>) => void {
	let timeout: number | null = null;

	return function (...args: Parameters<T>) {
		if (timeout !== null) {
			clearTimeout(timeout);
		}

		timeout = window.setTimeout(() => {
			func(...args);
		}, wait);
	};
}

/**
 * 节流函数
 */
export function throttle<T extends (...args: any[]) => any>(
	func: T,
	limit: number
): (...args: Parameters<T>) => void {
	let inThrottle: boolean = false;

	return function (...args: Parameters<T>) {
		if (!inThrottle) {
			func(...args);
			inThrottle = true;
			setTimeout(() => {
				inThrottle = false;
			}, limit);
		}
	};
}
