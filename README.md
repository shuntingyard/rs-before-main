# rs-before-main
Control Rust's build process, learn about glibc/binutils and produce freestanding Rust code

## Reduce Size
### Basics
In `Cargo.toml` use
```toml
strip = true
opt-level = "z"
lto = true
panic = "abort"
```
Binary size will be about 260K.

### Advanced
Discard Rust's standard library, have a `extern "C" main` function and your own panic handler.

Binary size will be about 14K. 

As we didn't succeed with dynamic linkage to `__libc_start_main` and thus defined
```toml
[dependencies]
libc = { version = "0.2", default-features = false }
```
in a minimalistic way, we ended up with a rust binary containing only three sections:
 `.init`, `.text` and `.fini`.

To compensate for this (in regard to inspecting binaries) a minimalistic
C program was added.

### fn _start() in Rust
Build `main.rs` when it implements `fn _start()`:
```bash
cargo rustc --release -- -C link-arg=-nostartfiles
```

## Inspect Binary
Show type of file:
```bash
hexdump -C target/release/rs-before-main | head -n 1
```
Show assembly in sections:
```bash
objdump -d target/release/rs-before-main
```
Print all relocation information (including GLIBC_ versions):
```
ldd -v target/release/rs-before-main
```
Get the entry point address (among other things):
```bash
readelf -h target/release/rs-before-main
```
Show the dynamic linker used:
```bash 
readelf -x .interp target/release/rs-before-main
```

## Videos
https://youtu.be/q8irLfXwaFM

[Make your Rust Binaries TINY!](https://youtu.be/b2qe3L4BX-Y)

## More About Building
### Freestanding II
[A Freestanding Rust Binary](https://os.phil-opp.com/freestanding-rust-binary/) (from "Writing an OS in Rust")

### Support Older Libc Versions
[Build and bind against older libc version (stackoverflow)](https://stackoverflow.com/questions/63724484/build-and-bind-against-older-libc-version)

## Homework for Another Day
[Making our own executable packer](https://fasterthanli.me/series/making-our-own-executable-packer) by @fasterthanlime
