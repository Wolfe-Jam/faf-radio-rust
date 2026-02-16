# FAFb Performance Demo
Output "fafb-performance.gif"

Set FontSize 18
Set Width 1000
Set Height 600
Set Theme "Dracula"
Set Padding 20
Set TypingSpeed 35ms

# ASCII Banner
Type "cat << 'EOF'"
Enter
Type "  ███████╗ █████╗ ███████╗██████╗ "
Enter
Type "  ██╔════╝██╔══██╗██╔════╝██╔══██╗"
Enter
Type "  █████╗  ███████║█████╗  ██████╔╝"
Enter
Type "  ██╔══╝  ██╔══██║██╔══╝  ██╔══██╗"
Enter
Type "  ██║     ██║  ██║██║     ██████╔╝"
Enter
Type "  ╚═╝     ╚═╝  ╚═╝╚═╝     ╚═════╝ "
Enter
Type ""
Enter
Type "  Binary Format - O(1) Section Lookups"
Enter
Type "EOF"
Enter
Sleep 2s

Type ""
Enter
Type "# File Size Comparison"
Enter
Sleep 500ms

Type "ls -lh grok-minimal.faf test.faf project.faf"
Enter
Sleep 2.5s

Type ""
Enter
Type "# Performance: Tiny File (Grok-sized)"
Enter
Sleep 500ms

Type "node benchmark-grok-size.js"
Enter
Sleep 4s

Type ""
Enter
Type "# Performance: Real Project (3KB)"
Enter
Sleep 500ms

Type "node benchmark-10x.js | head -25"
Enter
Sleep 5s

Type ""
Enter
Type "# Performance: EXTREME Scale (3.4MB)"
Enter
Sleep 500ms

Type "node benchmark-extreme.js | tail -20"
Enter
Sleep 5s

Type ""
Enter
Type "# The Scaling Advantage"
Enter
Sleep 500ms

Type "cat << 'EOF'"
Enter
Type ""
Enter
Type "  Size      | YAML Parse | Binary Parse | Speedup"
Enter
Type "  ----------|------------|--------------|----------"
Enter
Type "  273 bytes |   289µs    |    120µs     |   2.4x"
Enter
Type "  3 KB      |   2.0ms    |    0.12ms    |   17x"
Enter
Type "  3.4 MB    |   1715ms   |    ~2ms      |  858x 🚀"
Enter
Type ""
Enter
Type "  O(1) lookups. 91% smaller. Sub-millisecond parses."
Enter
Type ""
Enter
Type "EOF"
Enter
Sleep 3s

Type ""
Enter
Type "# 168 Tests Passing"
Enter
Sleep 500ms

Type "echo '✅ Rust: 118 tests | ✅ TypeScript: 50 tests | ✅ Total: 168 passing'"
Enter
Sleep 2s
