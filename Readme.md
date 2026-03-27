# MiniOS: Operating System Implementation In Rust
- A minimal OS implementation based on [os.phil-opp.com](https://os.phil-opp.com/minimal-rust-kernel/)
- Goal: Learn how OS-internals work by building one

## Output:
![Qemu screen with breakpoint interrupt printed on screen](notes/x86_print_interrupt.png)
### Keyboard Input
![Qemu screen with Hello World printed on screen using keyboard input](notes/WithKeyboard.png)

## Progress
- [x] Free Standing Binary
- [x] Custom Target Triple for compilation
- [x] Learn about Qemu or other emulators
- [x] Interrupt Handlers
- [x] GDT and TSS
- [x] Interrupt Stack Tables
- [x] Keyboard and peripheral support
- [ ] Memory Allocators
- [ ] Async Runtimes
