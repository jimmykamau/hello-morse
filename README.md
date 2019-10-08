# blue-pill-quickstart

Quickstart a Rust project for the [blue pill](https://wiki.stm32duino.com/index.php?title=Blue_Pill), or any similar STM32F103xx board.

## Quickstart a new project

This section assumes your computer is ready to hack on a blue pill.

```shell
cargo generate --git https://github.com/jimmykamau/blue-pill-quickstart
```

## Setting up your machine

Follow [these instructions](https://rust-embedded.github.io/book/intro/install.html) to set up your environment.

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

## Sources

This quickstart is a modification of [Guillaume's](https://github.com/TeXitoi/blue-pill-quickstart) guide.
