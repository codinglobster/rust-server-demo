const { test, expect } = require('@playwright/test');
const { ApiHelper } = require('../helpers/api-helper');

test.describe('Health Check APIs', () => {
  let api;

  test.beforeEach(() => {
    api = new ApiHelper();
  });

  test('GET /health - should return health status', async () => {
    const response = await api.getHealth();

    expect(response.status).toBe(200);
    expect(response.data).toHaveProperty('status', 'ok');
  });

  test('GET /api/health/health - should return detailed health status', async () => {
    const response = await api.getApiHealth();

    expect(response.status).toBe(200);
    expect(response.data).toHaveProperty('status');
    expect(response.data).toHaveProperty('database');
    expect(response.data).toHaveProperty('redis');
  });

  test('GET /api/health/liveness - should return liveness status', async () => {
    const response = await api.getLiveness();

    expect(response.status).toBe(200);
    expect(response.data).toHaveProperty('status', 'alive');
  });

  test('GET /api/health/readiness - should return readiness status', async () => {
    const response = await api.getReadiness();

    expect(response.status).toBe(200);
    expect(response.data).toHaveProperty('status', 'ready');
  });

  test('GET /api/health/version - should return version info', async () => {
    const response = await api.getVersion();

    expect(response.status).toBe(200);
    expect(response.data).toHaveProperty('version');
    expect(response.data).toHaveProperty('name');
  });
});
