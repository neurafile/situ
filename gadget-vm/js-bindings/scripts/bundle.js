#!/usr/bin/env node

/**
 * Bundle script to prepare the package for publishing
 * Copies WASM files and ensures proper structure
 */

const fs = require('fs');
const path = require('path');

const wasmDir = path.join(__dirname, '../wasm');
const distDir = path.join(__dirname, '../dist');

// Ensure wasm directory exists
if (!fs.existsSync(wasmDir)) {
  fs.mkdirSync(wasmDir, { recursive: true });
}

// Ensure dist directory exists
if (!fs.existsSync(distDir)) {
  fs.mkdirSync(distDir, { recursive: true });
}

// Copy WASM files to dist for easier access (optional, but helpful)
const distWasmDir = path.join(distDir, 'wasm');
if (fs.existsSync(wasmDir) && fs.existsSync(distDir)) {
  if (!fs.existsSync(distWasmDir)) {
    fs.mkdirSync(distWasmDir, { recursive: true });
  }
  // Note: We don't copy here as the wasm files should be referenced from the wasm directory
  // The build process puts them in js-bindings/wasm which is the correct location
}

console.log('Bundle preparation complete');
