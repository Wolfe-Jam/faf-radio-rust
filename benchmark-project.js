#!/usr/bin/env node

const fs = require('fs');
const { performance } = require('perf_hooks');
const YAML = require('yaml');

console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
console.log('FAF Parse Benchmark - REAL PROJECT FILE');
console.log('File: project.faf (realistic size)');
console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
console.log('');

// File sizes
const fafStats = fs.statSync('project.faf');

console.log('📦 File Size:');
console.log(`   project.faf: ${fafStats.size} bytes (${(fafStats.size/1024).toFixed(2)} KB)`);
console.log('');

// TypeScript parse time
console.log('⏱️  Parse Times:');
console.log('');

const fafContent = fs.readFileSync('project.faf', 'utf8');

// Warm-up
for (let i = 0; i < 100; i++) {
  YAML.parse(fafContent);
}

// Actual benchmark
const iterations = 1000;
const tsStart = performance.now();
for (let i = 0; i < iterations; i++) {
  YAML.parse(fafContent);
}
const tsEnd = performance.now();
const tsTotal = (tsEnd - tsStart).toFixed(3);
const tsAvg = ((tsEnd - tsStart) / iterations).toFixed(3);
const tsAvgMicro = ((tsEnd - tsStart) / iterations * 1000).toFixed(1);

console.log('┌─────────────────────────────────────────┐');
console.log('│ TypeScript YAML.parse()                 │');
console.log('├─────────────────────────────────────────┤');
console.log(`│ Iterations: ${iterations.toLocaleString()}                         │`);
console.log(`│ Total:      ${tsTotal}ms                       │`);
console.log(`│ Average:    ${tsAvg}ms per parse              │`);
console.log(`│             ${tsAvgMicro}µs per parse              │`);
console.log('└─────────────────────────────────────────┘');
console.log('');

console.log('┌─────────────────────────────────────────┐');
console.log('│ Rust Binary (FAFb) - PROJECTED          │');
console.log('├─────────────────────────────────────────┤');
console.log('│ Estimated:  ~0.12ms (120µs)             │');
console.log('│ Method:     O(1) section lookup         │');
console.log('│ Speedup:    ~20x faster (from docs)     │');
console.log('└─────────────────────────────────────────┘');
console.log('');

console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
console.log('📊 ACCURATE CLAIM FOR EMAIL:');
console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
console.log('');
console.log(`✅ TypeScript (YAML): ${tsAvg}ms = ${tsAvgMicro}µs`);
console.log('✅ Rust (Binary):     ~0.12ms = ~120µs (from prior benchmarks)');
console.log('');
console.log('💡 EMAIL CLAIM:');
if (parseFloat(tsAvg) < 1.0) {
  console.log('   "Sub-millisecond parses" ✅ ACCURATE');
} else {
  console.log(`   "~${tsAvg}ms parses" (be specific)`);
}
console.log('');
