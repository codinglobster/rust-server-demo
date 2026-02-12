const { test, expect } = require('@playwright/test');
const { ApiHelper } = require('../helpers/api-helper');
const { setAuth } = require('../helpers/auth-token-helper');

test.describe('Authentication APIs', () => {
  let api;

  test.beforeEach(() => {
    api = new ApiHelper();
  });

  test.afterEach(async () => {
    // Cleanup: try to logout if authenticated
    if (api.getAuthState().isAuthenticated) {
      try {
        await api.logout();
      } catch (e) {
        // Ignore cleanup errors
        test.info().catch(e => e.message);
      }
    }
  });

  test.describe('POST /api/auth/register', () => {
    test('should register a new user successfully', async () => {
      // Generate unique test data
      const timestamp = Date.now();
      test.info(`Timestamp: ${timestamp}`);

      const response = await api.register(
        `testuser_${timestamp}`,
        `test_${timestamp}@example.com`,
        'password123',
        'Test User'
      );

      expect(response.status).toBe(201);
      expect(response.data).toHaveProperty('access_token');
      expect(response.data).toHaveProperty('refresh_token');
      expect(response.data).toHaveProperty('token_type', 'Bearer');
      expect(response.data.user).toHaveProperty('username');
      expect(response.data.user).toHaveProperty('email');
      expect(response.data.user).toHaveProperty('id');
      expect(response.data.user).toHaveProperty('created_at');
      expect(response.data.user).toHaveProperty('is_active', true);
      expect(response.data.user).toHaveProperty('is_verified', false);
      expect(response.data.user).toHaveProperty('role', 'user');
    });
  });
