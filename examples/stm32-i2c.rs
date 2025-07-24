//! Get state and start/stop motor on a Jrk with a `STM32f1xx`
//!
//! In this example, the Jrk is connected on I2C:
//! SCL is on PB8, SDA on PB9
//!
//! PA9 & PA10 are used as a serial monitor

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m_rt::entry;
use nb::block;
use panic_halt as _;
use stm32f1xx_hal::{i2c, pac, prelude::*, serial};

use jrk_g2::{I2c as Jrk, JrkG2};

#[entry]
fn main() -> ! {
    #[allow(clippy::unwrap_used)]
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let mut afio = dp.AFIO.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut timer = dp.TIM1.counter_ms(&clocks);
    timer.start(1.secs()).unwrap();

    let mut gpioa = dp.GPIOA.split();
    let mut gpiob = dp.GPIOB.split();

    // Initialize USART1 on PA9 & PA10 for monitoring
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;

    let serial = serial::Serial::new(
        dp.USART1,
        (tx, rx),
        &mut afio.mapr,
        serial::Config::default().baudrate(115_200.bps()),
        &clocks,
    );
    let (mut tx, _rx) = serial.split();
    writeln!(tx, "serial monitor initialized").unwrap();

    // Initialize connexion to the Jrk: I2C1 on PB8 & PB9 and USART3 on PB10 & PB11
    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = i2c::BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        i2c::Mode::Fast {
            frequency: 400.kHz(),
            duty_cycle: i2c::DutyCycle::Ratio2to1,
        },
        clocks,
        10000,
        100,
        10000,
        10000,
    );

    let mut jrk = Jrk::new(i2c);
    writeln!(tx, "Jrk initialized on stm32 by i2c").unwrap();

    loop {
        if let Err(e) = jrk.stop_motor() {
            write!(tx, "I2cError: {e:?}").ok();
        }
        block!(timer.wait()).unwrap();
        jrk.show_vars(&mut tx).ok();

        if let Err(e) = jrk.set_target(1400) {
            write!(tx, "I2cError: {e:?}").ok();
        }
        block!(timer.wait()).unwrap();
        jrk.show_vars(&mut tx).ok();
    }
}
