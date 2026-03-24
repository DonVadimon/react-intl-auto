module.exports = {
    testEnvironment: "node",
    modulePathIgnorePatterns: ["<rootDir>/lib"],
    testMatch: ["**/tests/**/*.test.ts"],
    testTimeout: 10000,
    maxWorkers: process.env.CI ? 4 : '50%',
    transform: {
        "^.+\\.[tj]sx?$": "ts-jest",
    },
};
