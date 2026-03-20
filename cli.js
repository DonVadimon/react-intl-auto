#!/usr/bin/env node

// Run CLI with arguments from command line
const { runCli } = require('./extract.js');
// Add 'node' as fake program name since napi expects argv[0] to be program name
const args = ['node', ...process.argv.slice(2)];
const exitCode = runCli(args);
process.exit(exitCode);
