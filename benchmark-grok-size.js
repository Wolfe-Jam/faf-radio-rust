#!/usr/bin/env node

const fs = require('fs');
const { performance } = require('perf_hooks');
const YAML = require('yaml');

console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
console.log('FAF Parse Benchmark - GROK MINIMAL SIZE');
console.log('Comparing: Tiny vs Minimal vs Full Project');
console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
console.log('');

const files = [
  { name: 'test.faf', label: 'Tiny (bare minimum)' },
  { name: 'grok-minimal.faf', label: 'Grok Minimal (~5 slots)' },
  { name: 'project.faf', label: 'Full Project' }
];

const results = [];

for (const file of files) {
  const stats = fs.statSync(file.name);
  const content = fs.readFileSync(file.name, 'utf8');

  // Warm-up
  for (let i = 0; i < 100; i++) {
    YAML.parse(content);
  }

  // Benchmark
  const iterations = 10000;
  const start = performance.now();
  for (let i = 0; i < iterations; i++) {
    YAML.parse(content);
  }
  const end = performance.now();

  const totalMs = (end - start);
  const avgMs = totalMs / iterations;
  const avgUs = avgMs * 1000;

  results.push({
    name: file.name,
    label: file.label,
    size: stats.size,
    avgMs,
    avgUs
  });
}

console.log('📦 FILE SIZES:');
console.log('');
results.forEach(r => {
  console.log(`${r.label.padEnd(25)} ${r.size.toString().padStart(4)} bytes`);
});
console.log('');

console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
console.log('⏱️  PARSE TIMES (10,000 iterations each)');
console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
console.log('');

results.forEach(r => {
  console.log(`${r.label.padEnd(25)} ${r.avgMs.toFixed(4)}ms (${r.avgUs.toFixed(1)}µs)`);
});
console.log('');

console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
console.log('📊 GROK-SPECIFIC BENCHMARK');
console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
console.log('');

const grokResult = results.find(r => r.name === 'grok-minimal.faf');
const rustBinary = 0.120; // ms (from prior benchmarks)

console.log('┌─────────────────────────────────────────┐');
console.log('│ Grok Minimal .faf (~5 slots, <1KB)      │');
console.log('├─────────────────────────────────────────┤');
console.log(`│ File Size:    ${grokResult.size} bytes                   │`);
console.log(`│ TypeScript:   ${grokResult.avgMs.toFixed(4)}ms (${grokResult.avgUs.toFixed(1)}µs)       │`);
console.log(`│ Rust Binary:  ~${rustBinary}ms (~120µs)            │`);
console.log(`│ Speedup:      ~${(grokResult.avgMs / rustBinary).toFixed(1)}x faster              │`);
console.log('└─────────────────────────────────────────┘');
console.log('');

console.log('💡 EMAIL CLAIM (for Grok use case):');
console.log('');
if (grokResult.avgUs < 1000) {
  console.log('   ✅ TypeScript (YAML): sub-millisecond');
  console.log(`      (${grokResult.avgUs.toFixed(0)}µs for minimal files)`);
} else {
  console.log(`   ✅ TypeScript (YAML): ${grokResult.avgMs.toFixed(2)}ms average`);
}
console.log('   ✅ Rust (Binary): sub-millisecond (~120µs)');
console.log(`   ✅ Speedup: ~${(grokResult.avgMs / rustBinary).toFixed(0)}x faster for tiny files`);
console.log('');
console.log('⚠️  NOTE: For tiny files (<500 bytes), the speedup is less');
console.log('   dramatic because YAML parsing is already very fast.');
console.log('   Binary format shines at larger scales (3KB+).');
console.log('');
