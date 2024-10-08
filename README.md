# scamper

## Building

### Prerequisites

I am currently using fonts not licensed for distribution, so you will need to provide your own.
Create the expected directories and copy a desired ttf-font in.

```
mkdir -p resources/font
cp /my/desired/font.ttf resources/font/font.ttf
```

---

You will need the following installed in order to build and run this project:

* A recent version of `rustup`. See the [rust website](https://www.rust-lang.org/tools/install) for instructions for your operating system

You will also want to install an emulator. The best support in agb is with [mgba](https://mgba.io), with
`println!` support via `agb::println!` but any emulator should work. You'll get the best experience if
`mgba-qt` is in your `PATH`.

### Running in an emulator

Once you have the prerequisites installed, you should be able to build using

```sh
cargo build
```

or in release mode (recommended for the final version to ship to players)

```sh
cargo build --release
```

The resulting file will be in `target/thumbv4t-none-eabi/debug/scamper` or `target/thumbv4t-none-eabi/release/scamper` depending on
whether you did a release or debug build.

If you have `mgba-qt` in your path, you will be able to run your game with

```sh
cargo run
```

or in release mode

```sh
cargo run --release
```

## Starting development

You can find the documentation for agb [here](https://docs.rs/agb/latest/agb/).

## Shipping a .gba file for real hardware

If you want to run your game on real hardware, you will also need to install `agb-gbafix` which you can do after installing rust with the following: `cargo install agb-gbafix`.

To make a game run on real hardware, you will need to convert the built file into a file suitable for
running on the real thing.

First build the binary in release mode using the instructions above, then do the following:

```sh
agb-gbafix target/thumbv4t-none-eabi/release/scamper -o scamper.gba
```