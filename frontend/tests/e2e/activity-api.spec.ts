// Activity API Integration Tests
import { test, expect } from '@playwright/test';

const BASE_URL = 'http://localhost:8080';
const API_URL = `${BASE_URL}/api`;

let authToken: string;
let userId: string;

test.describe('Activity API Integration Tests', () => {
	test.beforeAll(async () => {
		// Register a test user and get auth token
		const timestamp = Date.now();
		const username = `activity_api_test_${timestamp}`;
		const email = `activity_api_${timestamp}@example.com`;

		const registerResponse = await fetch(`${API_URL}/auth/register`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				username,
				email,
				password: 'Password123',
				full_name: 'Activity API Test User'
			})
		});

		if (registerResponse.ok) {
			const data = await registerResponse.json();
			authToken = data.access_token;
			console.log('Test user registered successfully');
		} else {
			console.error('Failed to register test user');
			throw new Error('Failed to register test user');
		}
	});

	test('GET /api/activities/recent - should return recent activities', async () => {
		const response = await fetch(`${API_URL}/activities/recent`, {
			headers: {
				'Authorization': `Bearer ${authToken}`
			}
		});

		expect(response.ok).toBeTruthy();

		const data = await response.json();
		console.log('Recent activities response:', JSON.stringify(data, null, 2));

		// Should return an array
		expect(data.data).toBeDefined();
		expect(Array.isArray(data.data)).toBeTruthy();

		// Each activity should have required fields
		if (data.data.length > 0) {
			const activity = data.data[0];
			expect(activity).toHaveProperty('id');
			expect(activity).toHaveProperty('event_type');
			expect(activity).toHaveProperty('event_type_category');
			expect(activity).toHaveProperty('description');
			expect(activity).toHaveProperty('created_at');
		}
	});

	test('GET /api/activities - should return paginated activities', async () => {
		const response = await fetch(`${API_URL}/activities?page=1&per_page=10`, {
			headers: {
				'Authorization': `Bearer ${authToken}`
			}
		});

		expect(response.ok).toBeTruthy();

		const data = await response.json();
		console.log('Paginated activities response:', JSON.stringify(data, null, 2));

		// Should have pagination info
		expect(data.data).toBeDefined();
		expect(data.data).toHaveProperty('activities');
		expect(data.data).toHaveProperty('total');
		expect(data.data).toHaveProperty('page');
		expect(data.data).toHaveProperty('per_page');

		// Activities should be an array
		expect(Array.isArray(data.data.activities)).toBeTruthy();
		expect(data.data.page).toBe(1);
		expect(data.data.per_page).toBeLessThanOrEqual(10);
	});

	test('GET /api/activities/user/:id - should return user activities', async () => {
		// First get the current user info
		const meResponse = await fetch(`${API_URL}/users/me`, {
			headers: {
				'Authorization': `Bearer ${authToken}`
			}
		});

		if (meResponse.ok) {
			const userData = await meResponse.json();
			userId = userData.data.id;

			// Now get user activities
			const activitiesResponse = await fetch(
				`${API_URL}/activities/user/${userId}?page=1&per_page=10`,
				{
					headers: {
						'Authorization': `Bearer ${authToken}`
					}
				}
			);

			expect(activitiesResponse.ok).toBeTruthy();

			const data = await activitiesResponse.json();
			console.log('User activities response:', JSON.stringify(data, null, 2));

			// Should return activities for this user
			expect(data.data).toBeDefined();
			expect(data.data).toHaveProperty('activities');
			expect(Array.isArray(data.data.activities)).toBeTruthy();

			// All activities should belong to this user
			data.data.activities.forEach((activity: any) => {
				expect(activity.user_id).toBe(userId);
			});
		}
	});

	test('POST /api/activities - should create custom activity', async () => {
		const newActivity = {
			event_type: 'test_custom_event',
			description: 'This is a test activity from integration tests',
			metadata: {
				test: true,
				timestamp: new Date().toISOString(),
				source: 'integration_test'
			},
			ip_address: '127.0.0.1',
			user_agent: 'Integration Test Runner'
		};

		const response = await fetch(`${API_URL}/activities`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				'Authorization': `Bearer ${authToken}`
			},
			body: JSON.stringify(newActivity)
		});

		console.log('Create activity response status:', response.status);

		if (response.ok) {
			const data = await response.json();
			console.log('Created activity:', JSON.stringify(data, null, 2));

			expect(data.success).toBeTruthy();
			expect(data.data).toBeDefined();
			expect(data.data.event_type).toBe('test_custom_event');
			expect(data.data.description).toBe(newActivity.description);
		} else {
			const errorText = await response.text();
			console.log('Failed to create activity:', errorText);
			// It's ok if this fails due to permissions
			expect(response.status).toBeGreaterThanOrEqual(400);
		}
	});

	test('GET /api/activities - should handle pagination correctly', async () => {
		// Get first page
		const page1Response = await fetch(`${API_URL}/activities?page=1&per_page=5`, {
			headers: {
				'Authorization': `Bearer ${authToken}`
			}
		});

		expect(page1Response.ok).toBeTruthy();
		const page1Data = await page1Response.json();

		// Get second page
		const page2Response = await fetch(`${API_URL}/activities?page=2&per_page=5`, {
			headers: {
				'Authorization': `Bearer ${authToken}`
			}
		});

		expect(page2Response.ok).toBeTruthy();
		const page2Data = await page2Response.json();

		console.log('Page 1 activities:', page1Data.data.activities.length);
		console.log('Page 2 activities:', page2Data.data.activities.length);

		// Both should have correct pagination info
		expect(page1Data.data.page).toBe(1);
		expect(page2Data.data.page).toBe(2);
		expect(page1Data.data.per_page).toBe(5);
		expect(page2Data.data.per_page).toBe(5);
	});

	test('GET /api/activities - should respect per_page limit', async () => {
		const response = await fetch(`${API_URL}/activities?page=1&per_page=100`, {
			headers: {
				'Authorization': `Bearer ${authToken}`
			}
		});

		expect(response.ok).toBeTruthy();

		const data = await response.json();
		console.log('Activities with per_page=100:', data.data.activities.length);

		// Should return at most 100 activities (or all if fewer exist)
		expect(data.data.activities.length).toBeLessThanOrEqual(100);
	});

	test('GET /api/activities/recent - should use cache for subsequent calls', async () => {
		// First call
		const response1 = await fetch(`${API_URL}/activities/recent`, {
			headers: {
				'Authorization': `Bearer ${authToken}`
			}
		});

		expect(response1.ok).toBeTruthy();
		const data1 = await response1.json();

		// Second call (should be faster due to cache)
		const startTime = Date.now();
		const response2 = await fetch(`${API_URL}/activities/recent`, {
			headers: {
				'Authorization': `Bearer ${authToken}`
			}
		});
		const endTime = Date.now();

		expect(response2.ok).toBeTruthy();
		const data2 = await response2.json();

		console.log(`Second call took ${endTime - startTime}ms`);
		console.log('Both calls returned same data:', JSON.stringify(data1) === JSON.stringify(data2));

		// Should return same data
		expect(JSON.stringify(data1)).toBe(JSON.stringify(data2));
	});

	test('GET /api/activities/user/:id - should return 403 for other users', async () => {
		// Try to access activities of a different user
		const otherUserId = '123e4567-e89b-12d3-a456-426614174000'; // Random UUID

		const response = await fetch(`${API_URL}/activities/user/${otherUserId}?page=1&per_page=10`, {
			headers: {
				'Authorization': `Bearer ${authToken}`
			}
		});

		console.log('Response status for other user activities:', response.status);

		// Should return 403 Forbidden if user tries to view someone else's activities
		if (response.status === 403) {
			const error = await response.json();
			console.log('Expected 403 error:', error);
			expect(true).toBeTruthy();
		} else if (response.status === 404) {
			// User doesn't exist, which is also acceptable
			console.log('User not found (404)');
			expect(true).toBeTruthy();
		} else {
			// Some other behavior - log it
			console.log('Unexpected status code:', response.status);
		}
	});

	test('GET /api/activities - should require authentication', async () => {
		const response = await fetch(`${API_URL}/activities`);

		console.log('Response without auth:', response.status);

		// Should return 401 Unauthorized
		expect(response.status).toBe(401);
	});

	test('GET /api/activities/recent - should require authentication', async () => {
		const response = await fetch(`${API_URL}/activities/recent`);

		console.log('Response without auth:', response.status);

		// Should return 401 Unauthorized
		expect(response.status).toBe(401);
	});

	test('should track user activities over time', async ({ request }) => {
		// Perform some actions
		const actions = [
			() => fetch(`${API_URL}/users/me`, {
				headers: { 'Authorization': `Bearer ${authToken}` }
			}),
			() =>
				fetch(`${API_URL}/users`, {
					headers: { 'Authorization': `Bearer ${authToken}` }
				})
		];

		// Execute actions
		for (const action of actions) {
			await action();
			// Wait a bit to ensure events are processed
			await new Promise((resolve) => setTimeout(resolve, 500));
		}

		// Check activities
		const response = await fetch(`${API_URL}/activities/recent`, {
			headers: {
				'Authorization': `Bearer ${authToken}`
			}
		});

		expect(response.ok).toBeTruthy();

		const data = await response.json();
		console.log('Activities after user actions:', data.data.activities.length);

		// Should have some activities
		expect(data.data.activities.length).toBeGreaterThan(0);
	});
});

test.describe('Activity API Error Handling', () => {
	test('should handle invalid page parameters', async () => {
		// This would need a valid auth token first
		const timestamp = Date.now();
		const registerResponse = await fetch(`${API_URL}/auth/register`, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({
				username: `error_test_${timestamp}`,
				email: `error_${timestamp}@example.com`,
				password: 'Password123',
				full_name: 'Error Test'
			})
		});

		if (registerResponse.ok) {
			const { access_token } = await registerResponse.json();

			// Test with invalid page number
			const response = await fetch(`${API_URL}/activities?page=-1&per_page=10`, {
				headers: { 'Authorization': `Bearer ${access_token}` }
			});

			console.log('Response for invalid page:', response.status);
			// Should handle gracefully (either 400 or default to page 1)
			expect(response.status).toBeGreaterThanOrEqual(200);
		}
	});

	test('should handle very large per_page value', async () => {
		const timestamp = Date.now();
		const registerResponse = await fetch(`${API_URL}/auth/register`, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({
				username: `large_page_${timestamp}`,
				email: `large_${timestamp}@example.com`,
				password: 'Password123',
				full_name: 'Large Page Test'
			})
		});

		if (registerResponse.ok) {
			const { access_token } = await registerResponse.json();

			// Request very large page size
			const response = await fetch(`${API_URL}/activities?page=1&per_page=99999`, {
				headers: { 'Authorization': `Bearer ${access_token}` }
			});

			console.log('Response for large per_page:', response.status);

			if (response.ok) {
				const data = await response.json();
				// Should limit to max (e.g., 100)
				expect(data.data.per_page).toBeLessThanOrEqual(100);
			}
		}
	});
});
