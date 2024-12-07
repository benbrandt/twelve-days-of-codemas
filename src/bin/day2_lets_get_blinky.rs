//! Light up three LEDs in sequence.

#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let red = Output::new(p.PIN_18, Level::Low);
    let amber = Output::new(p.PIN_19, Level::Low);
    let green = Output::new(p.PIN_20, Level::Low);
    let mut outputs = [("red", red), ("amber", amber), ("green", green)];

    let mut i = 0;

    loop {
        // Based on the current iteration, which output should be "on"
        let on = i % outputs.len();

        for (index, (color, output)) in outputs.iter_mut().enumerate() {
            if index == on {
                info!("Flash {}", color);
                output.set_high();
            } else {
                output.set_low();
            }
        }

        Timer::after_millis(500).await;

        i = on + 1;
    }
}
