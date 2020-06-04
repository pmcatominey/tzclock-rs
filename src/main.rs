#![deny(unsafe_code)]
#![no_std]
#![no_main]

extern crate panic_semihosting;

extern crate stm32f1xx_hal as hal;
use hal::delay::Delay;
use hal::prelude::*;
use hal::stm32::Peripherals;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

extern crate tm1637;
use tm1637::TM1637;

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    // let gpiod = dp.GPIOD.split(&mut rcc.ahb);

    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST, clocks);

    // White
    let mut dio = gpiob.pb5.into_open_drain_output(&mut gpiob.crl);
    let mut clk = gpiob.pb6.into_open_drain_output(&mut gpiob.crl);
    let mut white = TM1637::new(&mut clk, &mut dio, &mut delay);
    white.init();
    white.set_brightness(200);
    white.print_hex(0, &[0, 1, 2, 3]);

    hprintln!("init").unwrap();    
    loop {}
}
