# Tiny Morse

`probe-run` is configured as the default runner, so you can start your program as easy as
```sh
cargo run --release
```

If you aren't using a debugger (or want to use cargo-embed/probe-rs-debugger), change the runner in .cargo/config.toml


<!-- Requirements -->
<details open="open">
  <summary><h2 style="display: inline-block" id="requirements">Requirements</h2></summary>

- The standard Rust tooling (cargo, rustup) which you can install from https://rustup.rs/

- Toolchain support for the cortex-m0+ processors in the rp2040 (thumbv6m-none-eabi)

- flip-link - this allows you to detect stack-overflows on the first core, which is the only supported target for now.

- probe-run. Upstream support for RP2040 was added with version 0.3.1.

- A CMSIS-DAP probe. (This can just be another pi pico)

</details>

<!-- Installation of development dependencies -->
<details open="open">
  <summary><h2 style="display: inline-block" id="installation-of-development-dependencies">Installation of development dependencies</h2></summary>

```sh
rustup target install thumbv6m-none-eabi
cargo install flip-link
# This is our suggested default 'runner'
# (Because of https://github.com/knurling-rs/probe-run/issues/391, use an older version for now)
cargo install probe-run --version=0.3.6 --locked
# If you want to use elf2uf2-rs instead of probe-run, instead do...
cargo install elf2uf2-rs --locked
```

</details>

<!-- Recommended Steps -->
<details open="open">
  <summary><h2 style="display: inline-block" id="recommended-steps">Recommended Steps</h2></summary>
You can use any library (called crates) you like as long as it is no-std compatible.

For this project I recommend using usb-device and usbd-serial (already imported).

You can use https://morsecode.world/international/timing.html to find the timings for the morse code.

It's probably best you use a pwm module for the buzzer to hopefully create a better sound.

If you ever get stuck you can look at the [main branch](https://github.com/miam-miam100/tiny-morse) for the solution.

</details>