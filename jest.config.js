module.exports = {
  testEnvironment: 'node',
  modulePathIgnorePatterns: ['<rootDir>/lib'],
  testMatch: ['**/__tests__/**/*.test.js'],
  setupFilesAfterEnv: ['<rootDir>/__tests__/setup.js'],
  testTimeout: 10000,
}
