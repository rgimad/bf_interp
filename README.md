## Brainfuck interpreter written in Rust
### Usage:
```
bf <file>
```
### TODO:
- Add autotests, and test also with incorrect bf programs
- Maybe use more efficient parsing algorithm (precompile jump positions.)
- Maybe perform optimizations before running (pack + - > < series like +++++ change to += 5)
