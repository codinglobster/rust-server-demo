/**
 * Authentication Token Helper
 *
 * Provides helper functions to manage authentication tokens and state across Playwright tests.
 * This ensures that authenticated endpoints (like user management) can be tested properly
 */

/**
 * Set authentication tokens
 * @param {string} accessToken - JWT access token
 * @param {string} refreshToken - JWT refresh token
 */
function setAuth(accessToken, refreshToken = null) {
  return {
    accessToken,
    refreshToken,
    isAuthenticated: !!accessToken,
  };
}

/**
 * Get authentication state
 */
function getAuthState() {
  return {
    accessToken: state.accessToken || null,
    refreshToken: state.refreshToken || null,
    isAuthenticated: !!(state.accessToken || null),
  };
}

module.exports = {
  setAuth,
  getAuthState,
};
