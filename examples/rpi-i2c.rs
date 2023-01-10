use linux_embedded_hal::I2cdev;
use std::{thread, time};

use jrk_g2::{I2c as Jrk, JrkG2};

fn main() -> Result<(), linux_embedded_hal::i2cdev::linux::LinuxI2CError> {
    let i2c = I2cdev::new("/dev/i2c-1")?;
    let mut jrk = Jrk::new(i2c);
    println!("jrk initialized on rpi by i2c");
    let mut ret: String = String::new();

    loop {
        if let Err(e) = jrk.stop_motor() {
            println!("I2cError: {e:?}");
        }
        thread::sleep(time::Duration::from_secs(2));
        jrk.show_vars(&mut ret).ok();
        println!("{ret}");
        ret = String::new();

        if let Err(e) = jrk.set_target(1500) {
            println!("I2cError: {e:?}");
        }
        thread::sleep(time::Duration::from_secs(2));
        jrk.show_vars(&mut ret).ok();
        println!("{ret}");
        ret = String::new();
    }
}
