#![deny(unsafe_code)]
#![no_std]
#![no_main]

extern crate panic_semihosting;


extern crate stm32f1xx_hal as hal;
use hal::rtc::Rtc;
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

    // Clock setup
    let clocks = rcc.cfgr.adcclk(2.mhz()).freeze(&mut flash.acr);
    let mut pwr = dp.PWR;
    let mut backup_domain = rcc.bkp.constrain(dp.BKP, &mut rcc.apb1, &mut pwr);
    let mut delay = Delay::new(cp.SYST, clocks);
    let mut rtc = Rtc::rtc(dp.RTC, &mut backup_domain);

    // GPIO setup
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
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

    let mut res = Resources {
        delay: & mut delay,
        display0: &mut TM1637::new(&mut pb9,&mut  pb8),
        display1: &mut TM1637::new(&mut pb7,&mut  pb6),
        display2: &mut TM1637::new(&mut pa11,&mut  pa10),
        display3: &mut TM1637::new(&mut pa9,&mut  pa8),
        display4: &mut TM1637::new(&mut pb15,&mut  pb14),
    };

    let mut clock_utc = Clock{hours: 16, minutes: 50};

    res.init();
    res.set_display_brightness(100);
    

    let mut adc = adc::Adc::adc2(dp.ADC2, &mut rcc.apb2, clocks);
    let mut ch1 = gpiob.pb1.into_analog(&mut gpiob.crl);
    let adc_max = adc.max_sample() as u32;
    let mut b: u32 = 100;
    let mut minutes_acc: u32 = clock_utc.minutes as u32;

    loop {
        let data1: u32 = adc.read(&mut ch1).unwrap();
        let x: u32 = 255 * data1 / adc_max;
        if x.max(b) - x.min(b) > 5 {
            b = x;
            res.set_display_brightness(b as u8 >> 5);
        }

        // crude placeholder clock
        let t = rtc.current_time();
        if t >= 60 {
            minutes_acc += 1;
            rtc.set_time(0);
            clock_utc.add_minutes(1);

            if minutes_acc >= 60 {
                minutes_acc = 0;
                clock_utc.add_hours(1);
            }

            res.set_display_time(&clock_utc);
        }

        res.delay.delay_ms(50_u16);
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

    fn set_display_time(&mut self, c: &Clock) {
        self.display0.print_hex(0, &c.to_hex(9), self.delay);
        self.display1.print_hex(0, &c.to_hex(1), self.delay);
        self.display2.print_hex(0, &c.to_hex(-4), self.delay);
        self.display3.print_hex(0, &c.to_hex(-5), self.delay);
        self.display4.print_hex(0, &c.to_hex(-7), self.delay);
    }
}

struct Clock {
    hours: u8,
    minutes: u8,
}

impl Clock {
    fn new() -> Self {
        Clock{hours: 0, minutes: 0}
    }

    fn add_hours(&mut self, h: u8) {
        self.hours = self.hours.wrapping_add(h) % 24;
    }

    fn sub_hours(&mut self, m: u8) {
        self.hours = self.hours.wrapping_sub(m) % 24;
    }

    fn add_minutes(&mut self, m: u8) {
        self.minutes = self.minutes.wrapping_add(m) % 60;
    }

    fn sub_minutes(&mut self, m: u8) {
        self.minutes = self.minutes.wrapping_sub(m) % 60;
    }

    fn to_hex(&self, offset: i8) -> [u8; 4] {
        let h = if offset > 0 {
            self.hours.wrapping_add(offset as u8) % 24
        } else {
            self.hours.wrapping_sub(-offset as u8) % 24
        };
        [
            h /10 % 10,
            h % 10,
            self.minutes /10 % 10,
            self.minutes % 10,
        ]
    }
}
