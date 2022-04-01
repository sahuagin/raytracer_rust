# Notes for rt_rs and reminders for other projects

## Building

### Clean

```bash
clear && cargo clean
```

### Quick Build Check while editing

```bash
clear && cargo check --color=always -- 2>&1 | less
```

### Building Debug and Release

```bash
 # debug
clear && cargo build --color=always --  2>&1 | less
 # release
clear && cargo build --release --color=always --  2>&1 | less
```

## Running with cmdline Parameters

```bash
 # quick run to check image
/usr/bin/time -lph cargo run  --release -- --num_samples 500 --max_depth 5 --image_width 100 million_spheres  | ppmtobmp >! /tmp/image.bmp
 # another test run using the --fast option
/usr/bin/time -lph cargo run  --release -- --fast two_perlin_spheres  | ppmtobmp >! /tmp/image.bmp

 # render with defaults, full size image, with time changes
/usr/bin/time -lph cargo run  --release -- --start_time 0.0 --stop_time 1.0 random_scene --movingspheres | ppmtobmp >! /tmp/image.bmp
```

## Running Miri to check low level information and program correctness

```bash
MIRIFLAGS="-Zmiri-tag-raw-pointers -Zmiri-disable-isolation" cargo miri run --target x86_64-unknown-linux-gnu -- --max_depth 1 -s 1 -w 100 cornell_box
```

**NOTE**: There is an option you should add to your config file `~/.cargo/config` to 
have miri use the linux VM for this.
```file
[build]
target = "x86_64-unknown-freebsd"
```

## Updating rust toolchains (rustup)

### Adding tools

```bash
rustup component add llvm-tools-preview-x86_64-unknown-freebsd miri-x86_64-unknown-freebsd rls-x86_64-unknown-freebsd rust-analysis rust-analyzer rust-src
rustup component add llvm-tools-preview miri rls rust-analysis rust-analyzer rust-src
component add rust-analysis clippy
rustup toolchain add clippy nightly
 # list available components
rustup component list | less
```

### Set Default toolchain to nightly

```bash
rustup default nightly
```

### Updating your toolchains

```bash
rustup update
```

**NOTE**: This often does not update because not all of the tools are built.
`rls` and `miri` in particular.
Commands used to originally install them (as above).

```bash
rustup component add llvm-tools-preview-x86_64-unknown-freebsd miri-x86_64-unknown-freebsd rls-x86_64-unknown-freebsd rust-analysis rust-analyzer rust-src
rustup component add llvm-tools-preview miri rls rust-analysis rust-analyzer rust-src
```

**Trying to update by doing a minimal install, and then reinstalling components.**
**NOTE**: `rustup toolchain add nightly --profile [minimal, default, complete]`

```bash
 # uninstalling the extra tools(that I really want). Next command didn't work.
rustup toolchain add nightly --profile minimal

 # list the installed components
> rustup component list --installed

cargo-x86_64-unknown-freebsd
clippy-x86_64-unknown-freebsd
llvm-tools-preview-x86_64-unknown-freebsd
miri-x86_64-unknown-freebsd
rls-x86_64-unknown-freebsd
rust-analysis-x86_64-unknown-freebsd
rust-docs-x86_64-unknown-freebsd
rust-src
rust-std-x86_64-unknown-freebsd
rustc-x86_64-unknown-freebsd
rustfmt-x86_64-unknown-freebsd

rustup component remove --toolchain nightly rls
info: removing component 'rls'

rustup component remove --toolchain nightly miri
info: removing component 'miri'

rustup update
rustup component add --toolchain nightly rls
**FAILED**
```

## Install a new version of nightly that has miri and rls

Check [Rustup Package Status Nightly](https://rust-lang.github.io/rustup-components-history/x86_64-unknown-freebsd.html)

### Install from a particular date

```bash
rustup install nightly-2022-03-27
# set as default
rustup default nightly-2022-03-27-x86_64-unknown-freebsd
rustup component add llvm-tools-preview miri rls rust-analysis rust-analyzer rust-src
```

## Rustup config

```bash
~/.rustup/settings.toml
```

[rustup config](https://rust-lang.github.io/rustup/overrides.html)

## What was installed and working with rls

```bash
 #rustc 1.61.0-nightly (68369a041 2022-02-22)
rustup install nightly-2022-02-22
rustup toolchain add nightly-2022-02-22 --profile complete
rustup default nightly-2022-02-22
rustup component add rls miri clippy rust-src llvm-tools rust-analysis
```


