/**
 * API Helper for Playwright Tests
 */

const BASE_URL = process.env.BASE_URL || 'http://localhost:8080';

class ApiHelper {
  constructor() {
    this.baseURL = BASE_URL;
    this.accessToken = null;
    this.refreshToken = null;
  }

  /**
   * Make API request
   */
  async request(endpoint, options = {}) {
    const url = `${this.baseURL}${endpoint}`;
    const headers = {
      'Content-Type': 'application/json',
      ...options.headers,
    };

    if (this.accessToken && !options.skipAuth) {
      headers['Authorization'] = `Bearer ${this.accessToken}`;
    }

    const fetchOptions = {
      method: options.method || 'GET',
      headers: headers,
    };

    if (options.body) {
      fetchOptions.body = options.body;
    }

    const response = await fetch(url, fetchOptions);

    return {
      status: response.status,
      statusText: response.statusText,
      data: response.status !== 204 ? await response.json() : null,
      headers: response.headers,
    };
  }

  /**
   * Health Check APIs
   */
  async getHealth() {
    return this.request('/health');
  }

  async getApiHealth() {
    return this.request('/api/health/health');
  }

  async getLiveness() {
    return this.request('/api/health/liveness');
  }

  async getReadiness() {
    return this.request('/api/health/readiness');
  }

  async getVersion() {
    return this.request('/api/health/version');
  }

  /**
   * Authentication APIs
   */
  async register(username, email, password, fullName = null) {
    const response = await this.request('/api/auth/register', {
      method: 'POST',
      body: JSON.stringify({
        username,
        email,
        password,
        full_name: fullName,
      }),
      skipAuth: true,
    });

    if (response.status === 201) {
      this.accessToken = response.data.access_token;
      this.refreshToken = response.data.refresh_token;
    }

    return response;
  }

  async login(username, password) {
    const response = await this.request('/api/auth/login', {
      method: 'POST',
      body: JSON.stringify({ username, password }),
      skipAuth: true,
    });

    if (response.status === 200) {
      this.accessToken = response.data.access_token;
      this.refreshToken = response.data.refresh_token;
    }

    return response;
  }

  async refreshToken(refreshToken) {
    const response = await this.request('/api/auth/refresh', {
      method: 'POST',
      body: JSON.stringify({ refresh_token: refreshToken }),
      skipAuth: true,
    });

    if (response.status === 200) {
      this.accessToken = response.data.access_token;
      this.refreshToken = response.data.refresh_token;
    }

    return response;
  }

  async logout() {
    const response = await this.request('/api/auth/logout', {
      method: 'POST',
    });

    if (response.status === 204) {
      this.accessToken = null;
      this.refreshToken = null;
    }

    return response;
  }

  /**
   * User APIs
   */
  async getMe() {
    return this.request('/api/users/me');
  }

  async getUser(id) {
    return this.request(`/api/users/${id}`);
  }

  async listUsers(page = 1, perPage = 20) {
    return this.request(`/api/users/?page=${page}&per_page=${perPage}`);
  }

  async updateUser(data) {
    return this.request('/api/users/me', {
      method: 'PUT',
      body: JSON.stringify(data),
    });
  }

  async changePassword(oldPassword, newPassword) {
    return this.request('/api/users/me/password', {
      method: 'POST',
      body: JSON.stringify({
        old_password: oldPassword,
        new_password: newPassword,
      }),
    });
  }

  async updateUserRole(userId, role) {
    return this.request(`/api/users/${userId}/role`, {
      method: 'PUT',
      body: JSON.stringify({ role }),
    });
  }

  /**
   * Refresh access token
   */
  async refreshToken(refeshToken) {
    const response = await this.request('/api/auth/refresh', {
      method: 'POST',
      body: JSON.stringify({ refresh_token: refreshToken }),
      skipAuth: true,
    });

    if (response.status === 200) {
      this.accessToken = response.data.access_token;
      this.refreshToken = response.data.refresh_token;
    }

    return response;
  }

  /**
   * Get current auth state
   */
  getAuthState() {
    return {
      accessToken: this.accessToken,
      refreshToken: this.refreshToken,
      isAuthenticated: !!this.accessToken,
    };
  }

  /**
   * Get current auth state
   */
  getAuthState() {
    return {
      accessToken: this.accessToken,
      refreshToken: this.refreshToken,
      isAuthenticated: !!this.accessToken,
    };
  }
}

  /**
   * Reset auth state
   */
  clearAuth() {
    this.accessToken = null;
    this.refreshToken = null;
  }
}

module.exports = { ApiHelper };
