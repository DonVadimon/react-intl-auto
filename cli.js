#!/usr/bin/env node

// Run CLI with arguments from command line
process.exit(require('./extract.js').runCli(process.argv.slice(2)) || 0);
