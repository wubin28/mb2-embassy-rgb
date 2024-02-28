#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_nrf::{
    gpio::{Level, Output, OutputDrive},
    peripherals::{P0_21, P0_28},
};
use embassy_time::{Duration, Timer};
use defmt_rtt as _;
use panic_probe as _;

#[embassy_executor::task]
async fn blinker(
    _col1: Output<'static, P0_28>,
    mut row1: Output<'static, P0_21>,
    interval: Duration,
) -> ! {
    loop {
        row1.set_high();
        println!("high");
        Timer::after(interval).await;
        row1.set_low();
        println!("low");
        Timer::after(interval).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let col1 = Output::new(p.P0_28, Level::Low, OutputDrive::Standard);
    let row1 = Output::new(p.P0_21, Level::Low, OutputDrive::Standard);

    println!("spawning");
    unwrap!(spawner.spawn(blinker(col1, row1, Duration::from_millis(500))));
}
