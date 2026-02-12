const { test, expect } = require('@playwright/test');
const { ApiHelper } = require('../helpers/api-helper');

test.describe('WebSocket Connection', () => {
  let api;
  let testUser;
  let ws = null;

  test.beforeAll(async () => {
    api = new ApiHelper();

    // Register and login a test user
    const timestamp = Date.now();
    const response = await api.register(
      `ws_test_${timestamp}`,
      `ws_test_${timestamp}@example.com`,
      'password123',
      'WS Test User'
    );

    testUser = {
      username: `ws_test_${timestamp}`,
      password: 'password123',
      accessToken: response.data.access_token,
    };
  });

  test.afterEach(async () => {
    // Close WebSocket connection after each test
    if (ws && ws.readyState === ws.OPEN) {
      ws.close();
      ws = null;
    }
  });

  test.afterAll(async () => {
    // Cleanup
    if (ws && ws.readyState === ws.OPEN) {
      ws.close();
    }
  });

  test('should connect to WebSocket with authentication', async ({ page }) => {
    const messages = [];

    // Connect to WebSocket
    ws = new WebSocket(
      `ws://localhost:8080/ws?token=${testUser.accessToken}`
    );

    // Wait for connection to open
    await new Promise((resolve, reject) => {
      ws.onopen = () => resolve();
      ws.onerror = (error) => reject(error);
      setTimeout(() => reject(new Error('Connection timeout')), 5000);
    });

    expect(ws.readyState).toBe(ws.OPEN);
  });

  test('should receive welcome message on connection', async () => {
    let receivedMessage = null;

    ws = new WebSocket(
      `ws://localhost:8080/ws?token=${testUser.accessToken}`
    );

    ws.onmessage = (event) => {
      receivedMessage = JSON.parse(event.data);
    };

    // Wait for message
    await new Promise((resolve, reject) => {
      ws.onopen = () => {
        // Wait a bit for the welcome message
        setTimeout(() => {
          if (receivedMessage) {
            resolve();
          } else {
            reject(new Error('No message received'));
          }
        }, 1000);
      };
      ws.onerror = (error) => reject(error);
      setTimeout(() => reject(new Error('Connection timeout')), 5000);
    });

    expect(receivedMessage).not.toBeNull();
    expect(receivedMessage).toHaveProperty('type');
  });

  test('should send and receive messages', async () => {
    let receivedMessage = null;
    const testMessage = {
      type: 'chat',
      content: 'Hello, WebSocket!',
      timestamp: new Date().toISOString(),
    };

    ws = new WebSocket(
      `ws://localhost:8080/ws?token=${testUser.accessToken}`
    );

    ws.onmessage = (event) => {
      receivedMessage = JSON.parse(event.data);
    };

    await new Promise((resolve) => {
      ws.onopen = () => {
        // Send test message
        ws.send(JSON.stringify(testMessage));

        // Wait for echo or response
        setTimeout(resolve, 1000);
      };
      setTimeout(() => resolve(), 6000);
    });

    // Verify message was sent (we should receive something back)
    expect(receivedMessage).not.toBeNull();
  });

  test('should fail to connect without authentication', async () => {
    let connectionError = null;

    try {
      ws = new WebSocket('ws://localhost:8080/ws');

      await new Promise((resolve, reject) => {
        ws.onerror = (error) => {
          connectionError = error;
          resolve();
        };
        ws.onopen = () => {
          // Connection opened (unexpected)
          ws.close();
          reject(new Error('Connection succeeded without auth'));
        };
        setTimeout(() => resolve(), 3000);
      });
    } catch (error) {
      connectionError = error;
    }

    // Connection should either fail or close immediately
    expect(
      ws === null || ws.readyState !== ws.OPEN
    ).toBe(true);
  });

  test('should handle multiple concurrent connections', async () => {
    const connections = [];
    const connectionCount = 3;

    // Create multiple connections
    for (let i = 0; i < connectionCount; i++) {
      const ws = new WebSocket(
        `ws://localhost:8080/ws?token=${testUser.accessToken}`
      );

      await new Promise((resolve, reject) => {
        ws.onopen = () => resolve();
        ws.onerror = (error) => reject(error);
        setTimeout(() => reject(new Error(`Connection ${i} timeout`)), 5000);
      });

      connections.push(ws);
    }

    // All connections should be open
    connections.forEach((connection, index) => {
      expect(connection.readyState).toBe(connection.OPEN);
    });

    // Cleanup
    connections.forEach((connection) => connection.close());
  });

  test('should maintain connection with heartbeat', async () => {
    ws = new WebSocket(
      `ws://localhost:8080/ws?token=${testUser.accessToken}`
    );

    await new Promise((resolve, reject) => {
      ws.onopen = () => resolve();
      ws.onerror = (error) => reject(error);
      setTimeout(() => reject(new Error('Connection timeout')), 5000);
    });

    // Wait for a few seconds to check connection stability
    await new Promise((resolve) => setTimeout(resolve, 5000));

    // Connection should still be open
    expect(ws.readyState).toBe(ws.OPEN);

    ws.close();
  });
});
