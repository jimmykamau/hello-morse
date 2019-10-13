# Blue-pill Hello Morse

Make the LED connected to PC13 on the generic STM32F103 board blink "hello morse" in morse code.

## Setting up your machine

Follow [these instructions](https://rust-embedded.github.io/book/intro/install.html) to set up your environment.
Kindly note that the [heapless](https://crates.io/crates/heapless) crate is currently only supported in the nightly rust builds.

## Compiling and running the default program

First connect your ST-Link to your blue pill, then connect the ST-Link to your computer.

Launch openocd:

```shell
openocd
```

You should see terminal output that's similar to this:

```
Open On-Chip Debugger 0.10.0
[...]
Info : stm32f3x.cpu: hardware has 6 breakpoints, 4 watchpoints
```
 
Open a new terminal, compile and flash

```shell
cargo run
```

Press the 'reset' button on your blue-pill. The on-board LED should now start blinking.
