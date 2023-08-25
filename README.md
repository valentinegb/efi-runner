# `efi-runner`

A runner for EFI executables using QEMU and OVMF.

## Prerequisites

You must have [QEMU](https://www.qemu.org/download) installed, as it is used for emulating a UEFI.

## How to Install

To install `efi-runner`, you must use `cargo install` on the nightly channel with the `bindeps` unstable feature enabled:

```shell
cargo +nightly install efi-runner -Z bindeps
```

## How to Use

Create a `.cargo/config.toml` file in your project if it doesn't already exist and add the following:

```toml
[target.x86_64-unknown-uefi]
runner = "efi-runner"
```

After that, you're good to go! `efi-runner` is configured as your UEFI target runner and whenever you run your project, for example, like so:

```shell
cargo run
```

a QEMU window will open with your program loaded in it.

![Screenshot 2023-08-24 at 10 50 14 PM](https://github.com/valentinegb/efi-runner/assets/35977727/59fb9cf1-b20c-40f5-8fab-25132381f716)
