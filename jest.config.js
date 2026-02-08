module.exports = {
    testEnvironment: "node",
    modulePathIgnorePatterns: ["<rootDir>/lib"],
    testMatch: ["**/__tests__/**/*.test.js", "**/__tests__/**/*.test.ts"],
    testTimeout: 10000,
    transform: {
        "^.+\\.[tj]sx?$": "ts-jest",
    },
};
