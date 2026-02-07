module.exports = {
    testEnvironment: "node",
    modulePathIgnorePatterns: ["<rootDir>/lib"],
    testMatch: ["**/__tests__/**/*.test.js", "**/__tests__/**/*.test.ts"],
    setupFilesAfterEnv: ["<rootDir>/__tests__/setup.js"],
    testTimeout: 10000,
    transform: {
        "^.+\\.[tj]sx?$": "ts-jest",
    },
};
