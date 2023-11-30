# `Six-step sensorless control of a BLDC motor. RUST implementation`

## `Clock`
  - The Phase-locked loop (PLL) is used as the clock source, with the resulting SYSCLK running at 16MHz.
  - In a perfect world, the PLL would be skipped, and only the 8MHz HSI oscillator would be used in order to lower the power consumption, but since the board is network powered, power consumption is not a factor, so we can go ham on the watts.
  - The higher the clock frequency, the higher the PWM resolution we can use, but I will stick only to the 16MHz frequency in this project, since it is likely to never encounter crazy high frequencies.

## `PWM`
  - PWM runs in center-aligned mode, apparantely, this reduces harmonic noise in the machine. This mode however halves the frequency of the PWM, so if the desired frequency of the driving PWM is 20KHz, the timer pwm output has to be configured for 40KHz.
  - In order to produce 40KHz at the output, we need to consider the input frequency. The PSC register for TIM1 is left at reset value, which means, that the TIM1 input clock frequency is not divided, and is left at 16MHz.
  - The leaves us with the calculations of the ARR: $\frac{f_{clk}}{f_{pwm}} = 400$
  - This means, that the PWM can resolve 400 individual duty-cycle values, giving us PWM resolution of 9 bits.
    
