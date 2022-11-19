# TDA4VM R5F embedded Rust

Work-in-progress infrastructure for Rust firmware targeting the Cortex-R5F cores of TI's TDA4VM,
J721E, and other similar SoCs.

## Capabilities

The following is currently implemented:
- Booting from TCM, loaded via remoteproc in Linux

## Status

This repo is currently **only a proof-of-concept**, and is not useful in practice. I hope to expand
it into a family of crates to support functions such as:

- remoteproc resource table generation and firmware-facing abstractions
- GPIO and peripheral access for important features of the chip
- Support for one or more of the common async-based task scheduling frameworks, likely Embassy

For the foreseeable future, I will be testing this only on the BeagleBone AI-64. It is probably
applicable to other boards and SoCs but I don't yet know.

I can't make any promises on future expansion; if nothing else, I hope it's a helpful reference for
others' work.

## Known limitations

- The DDR memory range allocated to the core is currently hard-coded in the linkerscript

## Why would you want bare metal Rust on coprocessor cores?

The TDA4VM chips target embedded deep learning applications. This makes them very attractive for
mobile robotics applications. However, embedded robotics platforms are minimally useful without
control of actuators and sensors. Linux on Cortex-A cores is great for ease of use with high-level
libraries and tools, but introduces unpredictable scheduling latency, high syscall overhead, and
challenges in software reliability.

For these reasons, I much prefer running control loops, simple sensor fusion and safety-critical
logic in a bare-metal environment. The R5F cores provide an excellent opportunity for this
application, since they have immediate shared memory access and synchronization primitives to
interact with the host CPU.

I'm interested in embedded Rust and its robotics applications, so I figured this would be a good
avenue to explore.

## Setup

```
rustup target add armv7r-none-eabihf
cargo install cargo-make
```

## Usage

Build the demo app:

```
cargo make build
```

Run the demo app (on-device):

```
cp demo-app /lib/firmware/demo-app.elf

echo stop | sudo tee /dev/remoteproc/j7-main-r5f1_0/state
# Ignore "Invalid argument" errors here; this likely means the core is already stopped
echo demo-app.elf | sudo tee /dev/remoteproc/j7-main-r5f1_0/firmware
echo start | sudo tee /dev/remoteproc/j7-main-r5f1_0/state

sudo ls /dev/remoteproc/j7-main-r5f1_0/device/remoteproc/
# Note which remoteproc node name is printed, and substitute it below
sudo cat /sys/kernel/debug/remoteproc/remoteproc18/trace0
```

## Prior art, references and thanks

I'm reimplementing needed logic from scratch, since there is no canonical Cortex-R runtime,
startup code or other infrastructure for Rust. However, I've drawn on the following as inspiration
and prior art:

- https://github.com/kaofishy/bbai64_cortex-r5_example
- https://github.com/paoloteti/cortex-r-rt
- https://github.com/japaric/ultrascale-plus/tree/master/firmware
- https://github.com/japaric/rm42/tree/master/rt

Additional thanks to Nishanth Menon for help with the family of chips.
