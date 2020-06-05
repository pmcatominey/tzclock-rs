#![deny(unsafe_code)]
#![no_std]
#![no_main]

extern crate panic_semihosting;


extern crate stm32f1xx_hal as hal;
use hal::delay::Delay;
use hal::gpio::gpioa::{PA8, PA9, PA10, PA11};
use hal::gpio::gpiob::{PB6, PB7, PB8, PB9, PB14, PB15};
use hal::gpio::{OpenDrain, Output};
use hal::stm32::Peripherals;
use hal::{adc, prelude::*};

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

extern crate tm1637;
use tm1637::TM1637;

struct Resources<'a> {
    delay: &'a mut Delay,
    display0: &'a mut TM1637<'a, PB9<Output<OpenDrain>>, PB8<Output<OpenDrain>>>,
    display1: &'a mut TM1637<'a, PB7<Output<OpenDrain>>, PB6<Output<OpenDrain>>>,
    display2: &'a mut TM1637<'a, PA11<Output<OpenDrain>>, PA10<Output<OpenDrain>>>,
    display3: &'a mut TM1637<'a, PA9<Output<OpenDrain>>, PA8<Output<OpenDrain>>>,
    display4: &'a mut TM1637<'a, PB15<Output<OpenDrain>>, PB14<Output<OpenDrain>>>,
}

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.adcclk(2.mhz()).freeze(&mut flash.acr);

    hprintln!("clocl {}", clocks.sysclk().0).unwrap();

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    // let gpiod = dp.GPIOD.split(&mut rcc.ahb);
    let mut pb15 =  gpiob.pb15.into_open_drain_output(&mut gpiob.crh);
    let mut pb14 =  gpiob.pb14.into_open_drain_output(&mut gpiob.crh);
    let mut pb9 =  gpiob.pb9.into_open_drain_output(&mut gpiob.crh);
    let mut pb8 =  gpiob.pb8.into_open_drain_output(&mut gpiob.crh);
    let mut pb7 =  gpiob.pb7.into_open_drain_output(&mut gpiob.crl);
    let mut pb6 =  gpiob.pb6.into_open_drain_output(&mut gpiob.crl);
    let mut pa11 =  gpioa.pa11.into_open_drain_output(&mut gpioa.crh);
    let mut pa10 =  gpioa.pa10.into_open_drain_output(&mut gpioa.crh);
    let mut pa9 =  gpioa.pa9.into_open_drain_output(&mut gpioa.crh);
    let mut pa8 =  gpioa.pa8.into_open_drain_output(&mut gpioa.crh);
    let mut dx = Delay::new(cp.SYST, clocks);


    let mut res = Resources {
        delay: & mut dx,
        display0: &mut TM1637::new(&mut pb9,&mut  pb8),
        display1: &mut TM1637::new(&mut pb7,&mut  pb6),
        display2: &mut TM1637::new(&mut pa11,&mut  pa10),
        display3: &mut TM1637::new(&mut pa9,&mut  pa8),
        display4: &mut TM1637::new(&mut pb15,&mut  pb14),
    };

    res.init();
    res.set_display_brightness(100);
    res.set_display_hex(&[1, 2, 3, 4]);

    let mut adc = adc::Adc::adc2(dp.ADC2, &mut rcc.apb2, clocks);
    let mut ch1 = gpiob.pb1.into_analog(&mut gpiob.crl);
    let adc_max = adc.max_sample() as u32;
    let mut b: u32 = 100;

    loop {
        let data1: u32 = adc.read(&mut ch1).unwrap();
        let x: u32 = 255 * data1 / adc_max;
        if x.max(b) - x.min(b) > 10 {
            b = x;
            res.set_display_brightness(b as u8);
            res.delay.delay_ms(5_u16);
        }
    }
}

impl<'a> Resources<'a> {
    fn init(&mut self) {
        self.display0.init(self.delay);
        self.display1.init(self.delay);
        self.display2.init(self.delay);
        self.display3.init(self.delay);
        self.display4.init(self.delay);

        self.display0.clear(self.delay);
        self.display1.clear(self.delay);
        self.display2.clear(self.delay);
        self.display3.clear(self.delay);
        self.display4.clear(self.delay);
    }

    fn set_display_brightness(&mut self, b: u8) {
        self.display0.set_brightness(b, self.delay);
        self.display1.set_brightness(b, self.delay);
        self.display2.set_brightness(b, self.delay);
        self.display3.set_brightness(b, self.delay);
        self.display4.set_brightness(b, self.delay);
    }

    fn set_display_hex(&mut self, d: &[u8]) {
        self.display0.print_hex(0, d, self.delay);
        self.display1.print_hex(0, d, self.delay);
        self.display2.print_hex(0, d, self.delay);
        self.display3.print_hex(0, d, self.delay);
        self.display4.print_hex(0, d, self.delay);
    }
}
