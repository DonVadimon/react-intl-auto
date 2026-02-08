module.exports = {
    testEnvironment: "node",
    modulePathIgnorePatterns: ["<rootDir>/lib"],
    testMatch: ["**/tests/**/*.test.ts"],
    testTimeout: 10000,
    transform: {
        "^.+\\.[tj]sx?$": "ts-jest",
    },
};
