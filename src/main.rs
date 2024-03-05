#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use microbit_bsp::{
    Microbit,
    LedMatrix,
    display::Frame,
    embassy_nrf::gpio::{Level, Output, OutputDrive, AnyPin},
};
use embassy_time::{Duration, Timer};
use defmt_rtt as _;
use panic_probe as _;

#[embassy_executor::task(pool = 1)]
async fn mb2_blinker(
    mut display: LedMatrix,
    interval: Duration,
) -> ! {
    display.clear();
    let mut frame = Frame::default();
    loop {
        frame.set(0, 0);
        println!("high");
        display.display(frame, interval).await;

        frame.unset(0, 0);
        println!("low");
        display.display(frame, interval).await;
    }
}

#[embassy_executor::task(pool = 1)]
async fn rgb_blinker(
    mut rgb: [Output<'static, AnyPin>; 3],
    interval: Duration,
) -> ! {
    let names = ["red", "green", "blue"];
    let mut cur = 0;
    loop {
        let prev = (cur + 2) % 3;
        rgb[prev].set_low();
        rgb[cur].set_high();
        println!("{}", names[cur]);
        Timer::after(interval).await;
        cur = (cur + 1) % 3;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let board = Microbit::default();
    let display = board.display;
    
    let led_pin = |p| {
        Output::new(p, Level::Low, OutputDrive::Standard)
    };
    let red = led_pin(AnyPin::from(board.p9));
    let green = led_pin(AnyPin::from(board.p8));
    let blue = led_pin(AnyPin::from(board.p16));

    println!("spawning");
    unwrap!(spawner.spawn(mb2_blinker(display, Duration::from_millis(500))));
    unwrap!(rgb_blinker([red, green, blue], Duration::from_millis(500)));
}
