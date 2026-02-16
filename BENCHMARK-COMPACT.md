# FAFb Benchmarks (X-ready)

## Parse Time: TypeScript vs Rust

```
TypeScript (YAML)    2.45ms   ████████████████████
Rust (Binary)        0.12ms   █
                              ⚡️ 20x faster
```

## File Size: .faf vs .fafb

```
.faf (YAML)          8,543 bytes   ████████████
.fafb (Binary)         736 bytes   █
                                   📦 91% smaller
```

## At Scale (1M contexts)

```
YAML:    40.8 minutes
Binary:   2.0 minutes
         ⚡️ 20x faster
```

---

## Tweet Version

TypeScript vs Rust benchmarks:

```
Parse (8.5KB file):
  YAML:   2.45ms ████████████████████
  Binary: 0.12ms █
  ⚡️ 20x faster

Size:
  .faf:  8.5KB ████████████
  .fafb: 736B  █
  📦 91% smaller

At 1M loads:
  YAML:   40.8 min
  Binary:  2.0 min

Sub-ms = near-zero reloads ✅
```

---

## Ultra-Compact (280 chars)

```
FAFb benchmarks:
Parse: 2.45ms → 0.12ms (20x)
Size: 8.5KB → 736B (91% smaller)
1M loads: 40min → 2min

Sub-ms cold starts ⚡️
O(1) lookup 🦀
168 tests 🧪
```

---

## How to Verify (Reproduce These Numbers)

### Setup
```bash
# Install compiler
git clone https://github.com/Wolfe-Jam/xai-faf-rust
cd xai-faf-rust
cargo build --release

# Install CLI
npm install -g faf-cli
```

### Run Benchmarks

**TypeScript (YAML parsing):**
```bash
# Time YAML parse (1000 iterations)
node -e "
const fs = require('fs');
const YAML = require('yaml');
const start = Date.now();
const content = fs.readFileSync('project.faf', 'utf8');
for(let i=0; i<1000; i++) YAML.parse(content);
console.log((Date.now()-start)/1000 + 'ms avg');
"
```

**Rust (Binary parsing):**
```bash
# Compile to binary
./target/release/xai-faf compile --input project.faf --output project.fafb

# Benchmark (built into compiler)
./target/release/xai-faf bench --faf project.faf --fafb project.fafb
```

### Expected Output
```
TypeScript: ~2.45ms per parse
Rust:       ~0.12ms per parse
Speedup:    ~20x
```

### Test Files Available
- Example: `/Users/wolfejam/FAF/faf-radio-rust/test.faf` (103B)
- Real project: Any `project.faf` in FAF repos (3-8KB typical)

### Source Code
- Compiler: https://github.com/Wolfe-Jam/xai-faf-rust
- Tests: 168 cross-check tests in repo
  - `cargo test` (118 Rust tests)
  - `npm test` in faf-cli (50 TypeScript tests)

**Measured on:** MacBook Pro M1, macOS 13.x, Feb 2026
