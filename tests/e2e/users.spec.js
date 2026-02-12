const { test, expect } = require('@playwright/test');
const { ApiHelper } = require('../helpers/api-helper');

test.describe('User Management APIs', () => {
  let api;
  let testUser;

  test.beforeAll(async () => {
    api = new ApiHelper();

    // Create admin user for role update tests
    const timestamp = Date.now();
    const response = await api.register(
      `admin_${timestamp}`,
      `admin_${timestamp}@example.com`,
      'admin123',
      'Admin User'
    );

    testUser = {
      id: response.data.user.id,
      username: `admin_${timestamp}`,
      email: `admin_${timestamp}@example.com`,
      password: 'admin123',
    };
  });

  test.afterEach(async () => {
    // Reset to authenticated state
    if (!api.getAuthState().isAuthenticated && testUser) {
      await api.login(testUser.username, testUser.password);
    }
  });

  test.describe('GET /api/users/me', () => {
    test('should get current user profile', async () => {
      const response = await api.getMe();

      expect(response.status).toBe(200);
      expect(response.data).toHaveProperty('id');
      expect(response.data).toHaveProperty('username', testUser.username);
      expect(response.data).toHaveProperty('email', testUser.email);
      expect(response.data).toHaveProperty('role');
      expect(response.data).toHaveProperty('is_active');
      expect(response.data).toHaveProperty('created_at');
    });

    test('should fail without authentication', async () => {
      api.clearAuth();

      const response = await api.getMe();

      expect(response.status).toBe(401);
    });
  });

  test.describe('GET /api/users/:id', () => {
    test('should get user by ID', async () => {
      const response = await api.getUser(testUser.id);

      expect(response.status).toBe(200);
      expect(response.data).toHaveProperty('id', testUser.id);
      expect(response.data).toHaveProperty('username', testUser.username);
    });

    test('should return 404 for non-existent user', async () => {
      const fakeId = '00000000-0000-0000-0000-000000000000';
      const response = await api.getUser(fakeId);

      expect(response.status).toBe(404);
    });

    test('should fail without authentication', async () => {
      api.clearAuth();

      const response = await api.getUser(testUser.id);

      expect(response.status).toBe(401);
    });
  });

  test.describe('GET /api/users (list)', () => {
    test.beforeAll(async () => {
      // Create additional test users
      for (let i = 0; i < 5; i++) {
        const timestamp = Date.now() + i;
        const helper = new ApiHelper();
        await helper.register(
          `list_test_${timestamp}`,
          `list_test_${timestamp}@example.com`,
          'password123'
        );
      }
    });

    test('should list users with default pagination', async () => {
      const response = await api.listUsers();

      expect(response.status).toBe(200);
      expect(response.data).toHaveProperty('users');
      expect(Array.isArray(response.data.users)).toBe(true);
      expect(response.data).toHaveProperty('total');
      expect(response.data).toHaveProperty('page', 1);
      expect(response.data).toHaveProperty('per_page', 20);
    });

    test('should support custom pagination', async () => {
      const response = await api.listUsers(1, 5);

      expect(response.status).toBe(200);
      expect(response.data.users.length).toBeLessThanOrEqual(5);
      expect(response.data.page).toBe(1);
      expect(response.data.per_page).toBe(5);
    });

    test('should fail without authentication', async () => {
      api.clearAuth();

      const response = await api.listUsers();

      expect(response.status).toBe(401);
    });
  });

  test.describe('PUT /api/users/me', () => {
    test('should update user profile successfully', async () => {
      const updateData = {
        full_name: 'Updated Name',
        email: `updated_${Date.now()}@example.com`,
      };

      const response = await api.updateUser(updateData);

      expect(response.status).toBe(200);
      expect(response.data).toHaveProperty('full_name', updateData.full_name);
      expect(response.data).toHaveProperty('email', updateData.email);
    });

    test('should update only full_name', async () => {
      const updateData = {
        full_name: 'Only Name Update',
      };

      const response = await api.updateUser(updateData);

      expect(response.status).toBe(200);
      expect(response.data).toHaveProperty('full_name', updateData.full_name);
    });

    test('should fail with invalid email format', async () => {
      const updateData = {
        email: 'invalid-email-format',
      };

      const response = await api.updateUser(updateData);

      expect(response.status).toBe(400);
      expect(response.data).toHaveProperty('error');
    });

    test('should fail without authentication', async () => {
      api.clearAuth();

      const response = await api.updateUser({ full_name: 'Test' });

      expect(response.status).toBe(401);
    });
  });

  test.describe('POST /api/users/me/password', () => {
    let userWithPassword;

    test.beforeAll(async () => {
      // Create a user specifically for password tests
      const timestamp = Date.now();
      const helper = new ApiHelper();
      const response = await helper.register(
        `pwd_test_${timestamp}`,
        `pwd_test_${timestamp}@example.com`,
        'oldPassword123',
        'Password Test User'
      );

      userWithPassword = {
        id: response.data.user.id,
        username: `pwd_test_${timestamp}`,
        oldPassword: 'oldPassword123',
        newPassword: 'newPassword456',
      };

      // Switch to this user
      await api.login(userWithPassword.username, userWithPassword.oldPassword);
    });

    test('should change password successfully', async () => {
      const response = await api.changePassword(
        userWithPassword.oldPassword,
        userWithPassword.newPassword
      );

      expect(response.status).toBe(204);
    });

    test('should fail with wrong old password', async () => {
      const response = await api.changePassword(
        'wrongOldPassword',
        'newPassword456'
      );

      expect(response.status).toBe(401);
      expect(response.data).toHaveProperty('error');
    });

    test('should fail with short new password', async () => {
      const response = await api.changePassword(
        userWithPassword.oldPassword,
        'short'
      );

      expect(response.status).toBe(400);
      expect(response.data).toHaveProperty('error');
    });

    test('should be able to login with new password', async () => {
      // Change password
      await api.changePassword(
        userWithPassword.oldPassword,
        userWithPassword.newPassword
      );

      // Logout
      await api.logout();

      // Login with new password
      const response = await api.login(
        userWithPassword.username,
        userWithPassword.newPassword
      );

      expect(response.status).toBe(200);
      expect(api.getAuthState().isAuthenticated).toBe(true);
    });

    test('should fail without authentication', async () => {
      api.clearAuth();

      const response = await api.changePassword('oldPass', 'newPass');

      expect(response.status).toBe(401);
    });
  });

  test.describe('PUT /api/users/:id/role (Admin Only)', () => {
    let regularUser;
    let regularUserHelper;

    test.beforeAll(async () => {
      // Create a regular user
      const timestamp = Date.now();
      regularUserHelper = new ApiHelper();
      const response = await regularUserHelper.register(
        `regular_${timestamp}`,
        `regular_${timestamp}@example.com`,
        'password123',
        'Regular User'
      );

      regularUser = {
        id: response.data.user.id,
        username: `regular_${timestamp}`,
      };

      // Switch back to admin user
      await api.login(testUser.username, testUser.password);
    });

    test('should update user role to moderator (admin only)', async () => {
      const response = await api.updateUserRole(regularUser.id, 'moderator');

      expect(response.status).toBe(200);
      expect(response.data).toHaveProperty('role', 'moderator');
    });

    test('should update user role to admin (admin only)', async () => {
      const response = await api.updateUserRole(regularUser.id, 'admin');

      expect(response.status).toBe(200);
      expect(response.data).toHaveProperty('role', 'admin');
    });

    test('should fail with invalid role', async () => {
      const response = await api.updateUserRole(regularUser.id, 'invalid_role');

      expect(response.status).toBe(400);
      expect(response.data).toHaveProperty('error');
    });

    test('should fail for non-admin user', async () => {
      // Login as regular user
      await api.login(regularUser.username, 'password123');

      const response = await api.updateUserRole(testUser.id, 'user');

      expect(response.status).toBe(403);
    });

    test('should fail without authentication', async () => {
      api.clearAuth();

      const response = await api.updateUserRole(regularUser.id, 'moderator');

      expect(response.status).toBe(401);
    });

    test('should fail for non-existent user', async () => {
      await api.login(testUser.username, testUser.password);

      const fakeId = '00000000-0000-0000-0000-000000000000';
      const response = await api.updateUserRole(fakeId, 'admin');

      expect(response.status).toBe(404);
    });
  });
});
