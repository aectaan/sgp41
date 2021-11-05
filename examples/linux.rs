use embedded_hal::blocking::delay::DelayMs;
use hal::{Delay, I2cdev};
use linux_embedded_hal as hal;

use sgp41::sgp41::Sgp41;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Sgp41::new(dev, Delay);

    sensor.soft_reset().unwrap();
    let sn = sensor.get_serial_number().unwrap();
    println!(" Serial number: {}", sn);
    sensor.execute_self_test().unwrap();

    sensor.execute_conditioning().unwrap();
    println!("Start measurement");

    loop {
        let data = sensor.measure_raw().unwrap();
        println!("VOC ticks: {}, NOx ticks: {}", data.voc_ticks, data.nox_ticks);
        hal::Delay.delay_ms(1000u16);
    }
}
