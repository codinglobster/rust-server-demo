/**
 * Test Server Startup Helper
 *
 * This module helps start/stop the Rust server for testing
 */

const { spawn } = require('child_process');
const fs = require('fs');
const path = require('path');

class TestServer {
  constructor() {
    this.process = null;
    this.stdout = [];
    this.stderr = [];
  }

  /**
   * Start the server
   */
  async start() {
    if (this.process) {
      console.log('Server is already running');
      return;
    }

    console.log('Starting test server...');

    return new Promise((resolve, reject) => {
      this.process = spawn('cargo', ['run'], {
        env: {
          ...process.env,
          DATABASE_URL: process.env.DATABASE_URL || 'postgresql://postgres:postgres@localhost:5432/rust_server',
          REDIS_URL: process.env.REDIS_URL || 'redis://localhost:6379',
          JWT_SECRET: process.env.JWT_SECRET || 'test-secret-key-at-least-32-characters-long',
          RUST_LOG: 'info',
        },
      });

      this.process.stdout.on('data', (data) => {
        const output = data.toString();
        this.stdout.push(output);
        if (output.includes('Server listening on')) {
          console.log('✅ Server started successfully');
          resolve();
        }
      });

      this.process.stderr.on('data', (data) => {
        const output = data.toString();
        this.stderr.push(output);
        console.error('Server stderr:', output);
      });

      this.process.on('error', (error) => {
        console.error('Failed to start server:', error);
        reject(error);
      });

      // Timeout after 30 seconds
      setTimeout(() => {
        if (this.stdout.length === 0 || !this.stdout.some(s => s.includes('Server listening on'))) {
          reject(new Error('Server startup timeout'));
        }
      }, 30000);
    });
  }

  /**
   * Stop the server
   */
  async stop() {
    if (!this.process) {
      console.log('Server is not running');
      return;
    }

    console.log('Stopping test server...');

    return new Promise((resolve) => {
      this.process.on('exit', () => {
        console.log('✅ Server stopped');
        this.process = null;
        resolve();
      });

      this.process.kill('SIGTERM');

      // Force kill after 5 seconds
      setTimeout(() => {
        if (this.process) {
          this.process.kill('SIGKILL');
        }
        resolve();
      }, 5000);
    });
  }

  /**
   * Get server logs
   */
  getLogs() {
    return {
      stdout: this.stdout.join(''),
      stderr: this.stderr.join(''),
    };
  }
}

module.exports = { TestServer };
