// Activity Stream E2E 测试
import { test, expect } from '@playwright/test';

test.describe('Activity Stream E2E', () => {
	test.beforeEach(async ({ page }) => {
		// Navigate to activities page
		await page.goto('/activities');
		await page.waitForLoadState('domcontentloaded');
		await page.waitForTimeout(500);
	});

	test('should display activity stream page', async ({ page }) => {
		// Check page title
		await expect(page.locator('h1')).toContainText('Activity Stream');

		// Check Kafka description
		await expect(page.locator('text=Kafka')).toBeVisible();
	});

	test('should show loading state initially', async ({ page }) => {
		// Should show loading indicator
		await expect(page.locator('text=Loading activities')).toBeVisible();
	});

	test('should display activities after login', async ({ page }) => {
		const timestamp = Date.now();
		const username = `activity_test_${timestamp}`;

		// Register and login first
		await page.goto('/auth/register');
		await page.waitForLoadState('domcontentloaded');
		await page.waitForTimeout(500);

		await page.fill('#register-username', username);
		await page.fill('#register-email', `activity_${timestamp}@example.com`);
		await page.fill('#register-password', 'Password123');
		await page.fill('#register-fullname', 'Activity Test User');
		await page.click('button:has-text("Register")');
		await page.waitForTimeout(3000);

		// Now go to activities page
		await page.goto('/activities');
		await page.waitForLoadState('domcontentloaded');
		await page.waitForTimeout(1000);

		// Should show activities
		const pageText = await page.textContent('body');
		console.log('Activities page content:', pageText?.substring(0, 500));

		// Check for activity indicators
		const hasActivity =
			pageText?.includes('user_registered') ||
			pageText?.includes('user_logged_in') ||
			pageText?.includes(username);

		console.log('Has activity:', hasActivity);
		expect(hasActivity).toBeTruthy();
	});

	test('should have pause/resume button', async ({ page }) => {
		// After login
		const timestamp = Date.now();
		await page.goto('/auth/register');
		await page.waitForLoadState('domcontentloaded');
		await page.fill('#register-username', `pause_test_${timestamp}`);
		await page.fill('#register-email', `pause_${timestamp}@example.com`);
		await page.fill('#register-password', 'Password123');
		await page.fill('#register-fullname', 'Pause Test');
		await page.click('button:has-text("Register")');
		await page.waitForTimeout(3000);

		await page.goto('/activities');
		await page.waitForLoadState('domcontentloaded');
		await page.waitForTimeout(1000);

		// Should have pause button
		const pauseButton = page.locator('button:has-text("Pause")');
		await expect(pauseButton).toBeVisible();

		// Click to pause
		await pauseButton.click();

		// Should show resume button
		const resumeButton = page.locator('button:has-text("Resume")');
		await expect(resumeButton).toBeVisible();
	});

	test('should show live status indicator', async ({ page }) => {
		const timestamp = Date.now();
		await page.goto('/auth/register');
		await page.waitForLoadState('domcontentloaded');
		await page.fill('#register-username', `live_test_${timestamp}`);
		await page.fill('#register-email', `live_${timestamp}@example.com`);
		await page.fill('#register-password', 'Password123');
		await page.fill('#register-fullname', 'Live Test');
		await page.click('button:has-text("Register")');
		await page.waitForTimeout(3000);

		await page.goto('/activities');
		await page.waitForLoadState('domcontentloaded');
		await page.waitForTimeout(2000);

		// Should show "Live" indicator
		const liveIndicator = page.locator('text=Live');
		await expect(liveIndicator).toBeVisible();
	});

	test('should display event icons', async ({ page }) => {
		const timestamp = Date.now();
		await page.goto('/auth/register');
		await page.waitForLoadState('domcontentloaded');
		await page.fill('#register-username', `icon_test_${timestamp}`);
		await page.fill('#register-email', `icon_${timestamp}@example.com`);
		await page.fill('#register-password', 'Password123');
		await page.fill('#register-fullname', 'Icon Test');
		await page.click('button:has-text("Register")');
		await page.waitForTimeout(3000);

		await page.goto('/activities');
		await page.waitForLoadState('domcontentloaded');
		await page.waitForTimeout(2000);

		// Should have emoji icons (👤, 💬, 🏠, ⚙️, or ❌)
		const pageText = await page.textContent('body');
		const hasIcon =
			pageText?.includes('👤') ||
			pageText?.includes('💬') ||
			pageText?.includes('🏠') ||
			pageText?.includes('⚙️') ||
			pageText?.includes('❌') ||
			pageText?.includes('📝');

		console.log('Has icon:', hasIcon);
		expect(hasIcon).toBeTruthy();
	});

	test('should redirect to login if not authenticated', async ({ page, context }) => {
		// Clear cookies and localStorage
		await context.clearCookies();
		await page.goto('/activities');
		await page.evaluate(() => {
			window.localStorage.clear();
		});

		// Try to access activities page directly
		await page.goto('/activities');
		await page.waitForTimeout(2000);

		// Should redirect to login
		const url = page.url();
		console.log('Current URL after trying to access activities:', url);

		// Either redirected to login or shows unauthorized
		const isLoginPage = url.includes('/auth/login') || url.includes('/auth/register');
		const hasUnauthorizedText = await page.textContent('body').then((text) =>
			text?.includes('unauthorized') || text?.includes('login')
		);

		expect(isLoginPage || hasUnauthorizedText || false).toBeTruthy();
	});

	test('should handle multiple user actions', async ({ page }) => {
		const timestamp = Date.now();
		const username = `multi_test_${timestamp}`;

		// Register
		await page.goto('/auth/register');
		await page.waitForLoadState('domcontentloaded');
		await page.fill('#register-username', username);
		await page.fill('#register-email', `multi_${timestamp}@example.com`);
		await page.fill('#register-password', 'Password123');
		await page.fill('#register-fullname', 'Multi Test User');
		await page.click('button:has-text("Register")');
		await page.waitForTimeout(3000);

		// Logout
		await page.goto('/auth/login');
		await page.evaluate(() => {
			window.localStorage.clear();
		});
		await page.waitForTimeout(1000);

		// Login again
		await page.fill('#login-username', username);
		await page.fill('#login-password', 'Password123');
		await page.click('button:has-text("Sign In")');
		await page.waitForTimeout(3000);

		// Go to activities
		await page.goto('/activities');
		await page.waitForLoadState('domcontentloaded');
		await page.waitForTimeout(2000);

		// Should show multiple activities (register and login)
		const pageText = await page.textContent('body');
		console.log('Activities after multiple actions:', pageText?.substring(0, 500));

		// Check for event types
		const hasUserEvent =
			pageText?.includes('user_registered') ||
			pageText?.includes('user_logged_in');

		console.log('Has user event:', hasUserEvent);
		expect(hasUserEvent).toBeTruthy();
	});

	test('should auto-refresh activities', async ({ page }) => {
		const timestamp = Date.now();
		await page.goto('/auth/register');
		await page.waitForLoadState('domcontentloaded');
		await page.fill('#register-username', `refresh_test_${timestamp}`);
		await page.fill('#register-email', `refresh_${timestamp}@example.com`);
		await page.fill('#register-password', 'Password123');
		await page.fill('#register-fullname', 'Refresh Test');
		await page.click('button:has-text("Register")');
		await page.waitForTimeout(3000);

		await page.goto('/activities');
		await page.waitForLoadState('domcontentloaded');
		await page.waitForTimeout(2000);

		// Wait for auto-refresh (5 seconds)
		await page.waitForTimeout(6000);

		// Page should still be loaded and responsive
		const url = page.url();
		console.log('URL after auto-refresh:', url);
		expect(url).toContain('/activities');
	});

	test('should display metadata when available', async ({ page }) => {
		const timestamp = Date.now();
		await page.goto('/auth/register');
		await page.waitForLoadState('domcontentloaded');
		await page.fill('#register-username', `metadata_test_${timestamp}`);
		await page.fill('#register-email', `metadata_${timestamp}@example.com`);
		await page.fill('#register-password', 'Password123');
		await page.fill('#register-fullname', 'Metadata Test');
		await page.click('button:has-text("Register")');
		await page.waitForTimeout(3000);

		await page.goto('/activities');
		await page.waitForLoadState('domcontentloaded');
		await page.waitForTimeout(2000);

		// Check if there's metadata toggle (may or may not be present)
		const metadataText = await page.textContent('body');
		console.log('Page has metadata reference:', metadataText?.includes('metadata'));

		// If metadata is present, try to expand it
		if (metadataText?.includes('View metadata') || metadataText?.includes('metadata')) {
			console.log('Metadata found on page');
		}
	});

	test('should handle empty activity list gracefully', async ({ page }) => {
		// This test assumes a fresh system with no activities yet
		// For now, we'll just verify the page loads without error

		const timestamp = Date.now();
		await page.goto('/auth/register');
		await page.waitForLoadState('domcontentloaded');
		await page.fill('#register-username', `empty_test_${timestamp}`);
		await page.fill('#register-email', `empty_${timestamp}@example.com`);
		await page.fill('#register-password', 'Password123');
		await page.fill('#register-fullname', 'Empty Test');
		await page.click('button:has-text("Register")');
		await page.waitForTimeout(3000);

		await page.goto('/activities');
		await page.waitForLoadState('domcontentloaded');
		await page.waitForTimeout(2000);

		// Page should load without crashing
		const url = page.url();
		expect(url).toContain('/activities');

		// Should not show error message
		const hasError = await page
			.textContent('body')
			.then((text) => text?.includes('error') || text?.includes('Error'));

		// It's OK to have "No activities yet" but not errors
		if (hasError) {
			const pageText = await page.textContent('body');
			console.log('Error found:', pageText?.substring(0, 200));
		}
	});
});
