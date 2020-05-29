#![deny(unsafe_code)]
#![no_std]
#![no_main]

extern crate panic_halt;

extern crate stm32f3xx_hal as hal;
use hal::delay::Delay;
use hal::prelude::*;
use hal::stm32::Peripherals;

extern crate embedded_hal;
use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::digital::v2::{InputPin, OutputPin};

use cortex_m_rt::entry;

extern crate tm1637;
use tm1637::TM1637;

// Color  - DIO - CLK
// White  - PB7 - PB6
// Blue   - PB5 - PB4
// Red    - PB3 - PD7
// Green  - PD6 - PD5
// Yellow - PD4 - PD3
#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();

    let gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let gpiod = dp.GPIOD.split(&mut rcc.ahb);

    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST, clocks);

    // White
    {
        let mut dio = gpiob.pb7.into_open_drain_output();
        let mut clk = gpiob.pb6.into_open_drain_output();
        let mut white = ClockDisplay::new(&mut clk, &mut dio, &mut delay);
        white.set_time(0, 0);
    }
    // Blue
    {
        let mut dio = gpiob.pb5.into_open_drain_output();
        let mut clk = gpiob.pb4.into_open_drain_output();
        let mut blue = ClockDisplay::new(&mut clk, &mut dio, &mut delay);
        blue.set_time(0, 0);
    }
    // Red
    {
        let mut dio = gpiob.pb3.into_open_drain_output();
        let mut clk = gpiod.pd7.into_open_drain_output();
        let mut red = ClockDisplay::new(&mut clk, &mut dio, &mut delay);
        red.set_time(0, 0);
    }
    // Green
    {
        let mut dio = gpiod.pd6.into_open_drain_output();
        let mut clk = gpiod.pd5.into_open_drain_output();
        let mut geen = ClockDisplay::new(&mut clk, &mut dio, &mut delay);
        geen.set_time(0, 0);
    }
    // Yellow
    {
        let mut dio = gpiod.pd4.into_open_drain_output();
        let mut clk = gpiod.pd3.into_open_drain_output();
        let mut yellow = ClockDisplay::new(&mut clk, &mut dio, &mut delay);
        yellow.set_time(0, 0);
    }

    loop {
    }
}

struct ClockDisplay<'a, CLK, DIO, D> {
    tm: TM1637<'a, CLK, DIO, D>,
}

impl<'a, CLK, DIO, D, E> ClockDisplay<'a, CLK, DIO, D>
    where
        CLK: OutputPin<Error = E>,
        DIO: InputPin<Error = E> + OutputPin<Error = E>,
        D: DelayUs<u16>,
{
    fn new(clk: &'a mut CLK, dio: &'a mut DIO, delay: &'a mut D) -> Self {
        let mut t = TM1637::new(clk, dio, delay);
        t.set_brightness(50);
        t.init();
        Self { tm: t }
    }

    fn set_time(&mut self, h: u8, m: u8) {
        self.tm.clear();
        self.tm.print_hex(0, &[0, 7, 3, 0]);
    }
}