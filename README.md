# `Six-step sensorless control of a BLDC motor. RUST implementation`
This project was recycled as a semestral project for the AVS course at FEE CTU.

## Building & programming
Rust's package manager, [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html), is required to build this project. For Linux-based operating systems, or Mac-os, you can install cargo with `curl https://sh.rustup.rs -sSf | sh`.
After cloning this repository to your machine, run `cargo build --release` to build the binary, which can be the found in `Steval-spin3202/target/thumbv6m-none-eabi/release`. For programming the board, you can use [STM32CubeProgrammer](https://www.st.com/en/development-tools/stm32cubeprog.html). In order to load it to the device memory, rename this file to `whatever.elf`, since without the `.elf.` file extension, STM32CubeProgrammer will fail to upload it.

## TODO
Speed control does not work, increasing the PWM duty-cycle only increases current drawn by the motor, and also increases the torque. I suspect this has to do with improper implementation of the commutation delay, which is very rigid, and maybe struggles to change appropriately.
Alternative way to implement this would by to configure a timer in one-pulse mode, and set its' delay to the commutation delay measured. Second improvement would be to improve the time measurement, like measuring multiple electronic revolutions.


## `Clock`
  - The Phase-locked loop (PLL) is used as the clock source, with the resulting SYSCLK running at 16MHz.
  - In a perfect world, the PLL would be skipped, and only the 8MHz HSI oscillator would be used in order to lower the power consumption, but since the board is network powered, power consumption is not a factor, so we can go ham on the watts.
  - The higher the clock frequency, the higher the PWM resolution we can use, but I will stick only to the 16MHz frequency in this project, since it is likely to never encounter crazy high frequencies.
    
