# FAF vs FAFb: Side-by-Side Benchmark

## Performance Comparison: TypeScript (slowest) vs Rust (fastest)

### Test File
- **Size**: ~8.5KB project.faf (typical project)
- **Sections**: 11 (project, stack, slots, dependencies, etc.)
- **Compiler**: xai-faf-rust v1.0

---

## Results

```
┌──────────────────────────────────────────────────────────────┐
│                    PARSE TIME COMPARISON                      │
├──────────────────────────────────────────────────────────────┤
│                                                               │
│  TypeScript (faf-cli - YAML)              2.45ms             │
│  ────────────────────────────────────────────────────        │
│  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓          │
│                                                               │
│  Rust (xai-faf-rust - Binary)             0.12ms             │
│  ────────────────────────────────────────────────────        │
│  ▓▓                                                           │
│                                                               │
│  ⚡️ Speedup: 20.4x faster                                    │
│                                                               │
└──────────────────────────────────────────────────────────────┘
```

---

## File Size Comparison

```
┌──────────────────────────────────────────────────────────────┐
│                    FILE SIZE COMPARISON                       │
├──────────────────────────────────────────────────────────────┤
│                                                               │
│  .faf  (YAML)                             8,543 bytes        │
│  ────────────────────────────────────────────────────        │
│  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓          │
│                                                               │
│  .fafb (Binary)                           736 bytes          │
│  ────────────────────────────────────────────────────        │
│  ▓▓▓▓                                                         │
│                                                               │
│  📦 Compression: 91.4% smaller                               │
│                                                               │
└──────────────────────────────────────────────────────────────┘
```

---

## Why It Matters

### TypeScript (faf-cli) - SLOWEST
- **Method**: YAML.parse() - O(n) linear scan
- **Parse time**: 2.45ms
- **File size**: 8.5KB
- **Use case**: Development, initial setup

### Rust (xai-faf-rust) - FASTEST
- **Method**: Binary format - O(1) section lookup
- **Parse time**: 0.12ms
- **File size**: 736 bytes
- **Use case**: Production, scale, distribution

---

## At Grok Scale

When millions of contexts are loaded:

```
Format          Parse/Load    Total Time (1M loads)    Cost Impact
─────────────────────────────────────────────────────────────────────
.faf (YAML)     2.45ms        40.8 minutes             Baseline
.fafb (Binary)  0.12ms        2.0 minutes              20x faster
```

**Result**: Sub-millisecond cold starts enable near-zero reload latency.

---

## The Binary Advantage

1. **O(1) Section Lookup**: Jump directly to needed sections
2. **Priority Truncation**: Smart context window management
3. **Pre-parsed Structure**: No YAML overhead
4. **Compact Size**: 91% smaller = faster transmission

---

## Validation

- **168 Cross-Check Tests**:
  - 118 Rust tests (foundation layer)
  - 50 TypeScript tests (CLI integration)

The compiler that will create billions of binaries 🤯
Validated by the framework that tests the testing

---

*Benchmarked on xai-faf-rust v1.0 + faf-cli v4.3.2*
*Feb 13, 2026*
