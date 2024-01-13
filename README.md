# Chip8 VM in Rust

This is a port of [my chip8 VM in OCaml](https://github.com/antoniskalou/chip8-ocaml).  

Unlike the OCaml version there is no debugger or disassembler, but the audio seems to work
significantly better.  

It also has less stuttering than the OCaml version.

## Building

```bash
# build & run the tests
$ cargo build && cargo test
# make a release build
$ cargo build --release
# run the release build
$ ./target/release/chip8 roms/SUPERFUNGAME.ch8
```

You can change the window scale along with the foreground and background colours using
the command line, see `chip8 --help`.

## Tested Platforms

- Windows 10
- Ubuntu 20.04

If you test on any other platforms, please open an issue and let me know if its
working or not.

## License

[GPLv3](LICENSE)
