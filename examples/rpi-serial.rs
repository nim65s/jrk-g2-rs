use linux_embedded_hal::Serial;
use std::{path, thread, time};

use jrk_g2_rs::{JrkG2, Serial as Jrk};

fn main() -> Result<(), linux_embedded_hal::serial_core::Error> {
    let serial = Serial::open(path::Path::new("/dev/ttyS0"))?;
    let mut jrk = Jrk::new(serial);
    println!("jrk initialized on rpi by serial");
    let mut ret: String = String::new();

    loop {
        if let Err(e) = jrk.stop_motor() {
            println!("SerialError: {e:?}");
        }
        thread::sleep(time::Duration::from_secs(2));
        jrk.show_vars(&mut ret).ok();
        println!("{ret}");
        ret = String::new();

        if let Err(e) = jrk.set_target(1450) {
            println!("SerialError: {e:?}");
        }
        thread::sleep(time::Duration::from_secs(2));
        jrk.show_vars(&mut ret).ok();
        println!("{ret}");
        ret = String::new();
    }
}
