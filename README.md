# TDA4VM R5F embedded Rust

Work-in-progress infrastructure for Rust firmware targeting the Cortex-R5F cores of TI's J721E family such as the TDA4VM.

## Capabilities

The following is currently implemented:
- Booting from TCM, loaded via remoteproc in Linux
  - Helper macros for generating remoteproc tables
- Logging/panicking to remoteproc trace buffers
- Standard exception handlers
- Configuring and enabling MPU

## Status

This repo is currently **only a proof-of-concept**, and is not useful in practice. I am writing code
in a "demo app" crate and then reformulating it into external driver crates when it makes sense. I
hope to expand it into a family of crates to support functions such as:

- Firmware-facing abstractions for host processor interop (rpmesg, VirtIO)
- GPIO and peripheral access for important features of the chip
- Support for one or more of the common async-based task scheduling frameworks, likely Embassy

For the foreseeable future, I will be testing this only on the BeagleBone AI-64. It is probably
applicable to other boards and SoCs but I don't yet know. Most of what I have written should work
for any Cortex-R5.

I can't make any promises on future expansion; if nothing else, I hope it's a helpful reference for
others' work.

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

You will also need the arm-none-eabi binutils from Arm available on your PATH (for arm-none-eabi-ld).

## Usage

Build the demo app:

```
cargo make app
```

Copy the demo app to the device:

```
scp ./target/armv7r-none-eabihf/debug/demo_app.elf debian@beaglebone.local:~/
```

Run the demo app (on-device):

```
cp demo_app.elf /lib/firmware/

echo stop | sudo tee /dev/remoteproc/j7-main-r5f1_0/state
# Ignore "Invalid argument" errors here; this likely means the core is already stopped
echo demo_app.elf | sudo tee /dev/remoteproc/j7-main-r5f1_0/firmware
echo start | sudo tee /dev/remoteproc/j7-main-r5f1_0/state

sudo ls /dev/remoteproc/j7-main-r5f1_0/device/remoteproc/
# Note which remoteproc node name is printed, and substitute it below
# trace0: main debug log
sudo cat /sys/kernel/debug/remoteproc/remoteproc18/trace0
# trace1: panic log
sudo cat /sys/kernel/debug/remoteproc/remoteproc18/trace1
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
