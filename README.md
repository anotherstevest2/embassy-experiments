(As is apparent from the commit history, my starting point was shamlessly stolen from: https://github.com/SillyFreak/embassy-experiments, Copyright (c) 2023 Clemens Koza under the terms of the MIT License.)

# Embassy Experiments

This repo contains example programs for the [STM32F3 Discovery board](https://www.st.com/en/evaluation-tools/stm32f3discovery.html) - which uses the [STM32F303VC MCU](https://www.st.com/en/microcontrollers-microprocessors/stm32f303vc.html) - utilizing the [Embassy](https://embassy.dev/) embedded Rust framework.

## Prerequisites

To run these programs, you'll need a nightly rust toolchain (this may no longer be true but I haven't checked) for the thumbv7em-none-eabihf target:

```sh
rustup install nightly
rustup target add thumbv7em-none-eabihf
```

In addition, `cargo run` will try to flash the discovery board using [probe-rs](https://probe.rs/docs/getting-started/installation/), so you'll need that as well. Alternatively, you can only `cargo build` the programs and flash them via other means, or adapt the `runner` option in `.cargo/config.toml`.

Also, you will naturally need an STM32F3 Discovery board.

## Usage

Several programs are contained in `src/bin/`. For example, to run the `hello.rs` program, execute this command:

```sh
cargo run --release --bin hello
```

The programs are based on and inspired by Embassy's [stm32f3](https://github.com/embassy-rs/embassy/tree/main/examples/stm32f3/src/bin) and [stm32f4](https://github.com/embassy-rs/embassy/tree/main/examples/stm32f4/src/bin) examples.

These programs work:

- **hello**: prints "Hello World!" to the debugging interface, that's it.
  [See this example](https://github.com/embassy-rs/embassy/blob/main/examples/stm32f3/src/bin/hello.rs)
- **blinky**: blinks the eight LEDs of the discovery board at different frequencies, by multiplexing eight independent tasks.
  [See this example](https://github.com/embassy-rs/embassy/blob/main/examples/stm32f3/src/bin/blinky.rs)
- **button**: waits for button presses and releases using interrupts and controls an LED accordingly.
  [See this example](https://github.com/embassy-rs/embassy/blob/main/examples/stm32f3/src/bin/button_exti.rs)
- **pwm**: controls an LED's brightness by varying its duty cycle.
  [See this example](https://github.com/embassy-rs/embassy/blob/main/examples/stm32f4/src/bin/pwm.rs)
- **channel**: uses a Channel to communicate between two async tasks. If the receiver interval is larger than the sender interval, this will demonstrate backpressure.
- **signal**: uses a Signal to communicate between two async tasks. If the receiver interval is larger than the sender interval, this will demonstrate overwriting without backpressure.
- **adc**: reads ref and temp voltage from the ADC capable pins periodically and utilizes the factory-saved adc and temp calibrations to print the die 
temperature in degrees C.
  [See this example](https://github.com/embassy-rs/embassy/blob/main/examples/stm32f4/src/bin/adc.rs)

This doesn't:

- **uart**: when TX and RX (PE0, PE1) are connected, should echo the UART output. However, no data is received and printed (I have not yet explicitly tested if the data is sent correctly).
  [See this example](https://github.com/embassy-rs/embassy/blob/main/examples/stm32f4/src/bin/usart_dma.rs)

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

