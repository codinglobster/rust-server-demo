const { test, expect } = require('@playwright/test');
const { ApiHelper } = require('../helpers/api-helper');

test.describe('Complete User Workflow', () => {
  let api;
  let userData;

  test.describe('New User Registration and Onboarding', () => {
    test('complete user journey from registration to first login', async () => {
      api = new ApiHelper();
      const timestamp = Date.now();

      // Step 1: Register
      const registerResponse = await api.register(
        `journey_user_${timestamp}`,
        `journey_${timestamp}@example.com`,
        'SecurePass123!',
        'Journey User'
      );

      expect(registerResponse.status).toBe(201);
      expect(registerResponse.data.user).toHaveProperty('id');
      expect(registerResponse.data.user).toHaveProperty('username', `journey_user_${timestamp}`);
      expect(registerResponse.data.user.is_active).toBe(true);

      userData = {
        id: registerResponse.data.user.id,
        username: `journey_user_${timestamp}`,
        email: `journey_${timestamp}@example.com`,
        password: 'SecurePass123!',
      };

      // Step 2: Verify user is logged in after registration
      expect(api.getAuthState().isAuthenticated).toBe(true);
      expect(api.getAuthState().accessToken).not.toBeNull();

      // Step 3: Get user profile
      const profileResponse = await api.getMe();

      expect(profileResponse.status).toBe(200);
      expect(profileResponse.data).toHaveProperty('id', userData.id);
      expect(profileResponse.data).toHaveProperty('full_name', 'Journey User');

      // Step 4: Update profile
      const updateResponse = await api.updateUser({
        full_name: 'Updated Journey User',
        email: `updated_${timestamp}@example.com`,
      });

      expect(updateResponse.status).toBe(200);
      expect(updateResponse.data.full_name).toBe('Updated Journey User');

      // Step 5: Logout
      const logoutResponse = await api.logout();

      expect(logoutResponse.status).toBe(204);
      expect(api.getAuthState().isAuthenticated).toBe(false);

      // Step 6: Login again
      const loginResponse = await api.login(
        userData.username,
        userData.password
      );

      expect(loginResponse.status).toBe(200);
      expect(loginResponse.data.user).toHaveProperty('full_name', 'Updated Journey User');

      // Step 7: Change password
      const passwordChangeResponse = await api.changePassword(
        userData.password,
        'NewSecurePass456!'
      );

      expect(passwordChangeResponse.status).toBe(204);

      // Step 8: Login with new password
      await api.logout();
      const newLoginResponse = await api.login(
        userData.username,
        'NewSecurePass456!'
      );

      expect(newLoginResponse.status).toBe(200);
      expect(api.getAuthState().isAuthenticated).toBe(true);

      // Step 9: List other users
      const usersResponse = await api.listUsers(1, 10);

      expect(usersResponse.status).toBe(200);
      expect(Array.isArray(usersResponse.data.users)).toBe(true);
      expect(usersResponse.data.total).toBeGreaterThan(0);

      // Step 10: Final logout
      const finalLogoutResponse = await api.logout();

      expect(finalLogoutResponse.status).toBe(204);
    });
  });

  test.describe('Token Refresh Flow', () => {
    test('should handle access token expiration gracefully', async () => {
      api = new ApiHelper();
      const timestamp = Date.now();

      // Register user
      const registerResponse = await api.register(
        `token_test_${timestamp}`,
        `token_test_${timestamp}@example.com`,
        'password123'
      );

      const initialAccessToken = registerResponse.data.access_token;
      const refreshToken = registerResponse.data.refresh_token;

      // Simulate access token expiration by clearing it
      api.clearAuth();

      // Use refresh token to get new access token
      const refreshResponse = await api.refreshToken(refreshToken);

      expect(refreshResponse.status).toBe(200);
      expect(refreshResponse.data.access_token).not.toBe(initialAccessToken);
      expect(refreshResponse.data.refresh_token).not.toBeNull();
      expect(api.getAuthState().isAuthenticated).toBe(true);

      // Verify new token works
      const profileResponse = await api.getMe();

      expect(profileResponse.status).toBe(200);
    });
  });

  test.describe('Concurrent User Operations', () => {
    test('should handle multiple users simultaneously', async () => {
      const users = [];
      const userCount = 5;

      // Create multiple users
      for (let i = 0; i < userCount; i++) {
        const timestamp = Date.now() + i;
        const helper = new ApiHelper();

        const response = await helper.register(
          `concurrent_${timestamp}`,
          `concurrent_${timestamp}@example.com`,
          'password123',
          `Concurrent User ${i}`
        );

        users.push({
          id: response.data.user.id,
          username: `concurrent_${timestamp}`,
          password: 'password123',
          helper,
        });
      }

      // All users should be able to get their profiles
      for (const user of users) {
        const response = await user.helper.getMe();

        expect(response.status).toBe(200);
        expect(response.data).toHaveProperty('id', user.id);
      }

      // All users should be able to list users
      for (const user of users) {
        const response = await user.helper.listUsers();

        expect(response.status).toBe(200);
        expect(response.data.total).toBeGreaterThanOrEqual(userCount);
      }

      // Cleanup: logout all users
      for (const user of users) {
        await user.helper.logout();
      }
    });
  });

  test.describe('Error Recovery Scenarios', () => {
    test('should recover from network errors gracefully', async () => {
      api = new ApiHelper();
      const timestamp = Date.now();

      // Register user
      await api.register(
        `recovery_test_${timestamp}`,
        `recovery_test_${timestamp}@example.com`,
        'password123'
      );

      // Simulate token loss
      const originalToken = api.getAuthState().accessToken;
      api.clearAuth();

      // Attempt to access protected resource
      const response = await api.getMe();

      expect(response.status).toBe(401);

      // Login again to recover
      const loginResponse = await api.login(
        `recovery_test_${timestamp}`,
        'password123'
      );

      expect(loginResponse.status).toBe(200);
      expect(api.getAuthState().isAuthenticated).toBe(true);

      // Should now be able to access protected resources
      const profileResponse = await api.getMe();

      expect(profileResponse.status).toBe(200);
    });
  });

  test.describe('Data Consistency', () => {
    test('should maintain consistent user data across operations', async () => {
      api = new ApiHelper();
      const timestamp = Date.now();

      // Register user
      const registerResponse = await api.register(
        `consistency_${timestamp}`,
        `consistency_${timestamp}@example.com`,
        'password123',
        'Consistency Test'
      );

      const userId = registerResponse.data.user.id;

      // Get profile via /me
      const meResponse = await api.getMe();
      const meData = meResponse.data;

      // Get profile via /:id
      const userResponse = await api.getUser(userId);
      const userData = userResponse.data;

      // Data should be consistent
      expect(meData.id).toBe(userData.id);
      expect(meData.username).toBe(userData.username);
      expect(meData.email).toBe(userData.email);
      expect(meData.role).toBe(userData.role);
      expect(meData.created_at).toBe(userData.created_at);

      // Update profile
      await api.updateUser({
        full_name: 'Consistency Updated',
      });

      // Get profile again
      const updatedMeResponse = await api.getMe();
      const updatedUserData = await api.getUser(userId);

      expect(updatedMeResponse.data.full_name).toBe(updatedUserData.data.full_name);
      expect(updatedMeResponse.data.full_name).toBe('Consistency Updated');
    });
  });
});
