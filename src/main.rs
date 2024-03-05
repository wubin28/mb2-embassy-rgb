#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use microbit_bsp::{
    Microbit,
    LedMatrix,
    display::{Frame, Brightness},
    embassy_nrf::{
        gpio::{Level, Output, OutputDrive, AnyPin},
        saadc,
        bind_interrupts,
    },
};
use embassy_time::{Duration, Timer};
use defmt_rtt as _;
use panic_probe as _;

#[embassy_executor::task]
async fn mb2_blinker(
    mut display: LedMatrix,
    interval: Duration,
) -> ! {
    display.clear();
    display.set_brightness(Brightness::MAX);
    let mut frame = Frame::default();
    loop {
        frame.set(0, 0);
        display.display(frame, interval).await;

        frame.unset(0, 0);
        display.display(frame, interval).await;
    }
}

#[embassy_executor::task]
async fn rgb_blinker(
    mut rgb: [Output<'static, AnyPin>; 3],
    interval: Duration,
) -> ! {
    let mut cur = 0;
    loop {
        let prev = (cur + 2) % 3;
        rgb[prev].set_low();
        rgb[cur].set_high();
        Timer::after(interval).await;
        cur = (cur + 1) % 3;
    }
}

#[embassy_executor::task]
async fn knob(mut saadc: saadc::Saadc<'static, 1>) -> ! {
    loop {
        let mut buf = [0];
        saadc.sample(&mut buf).await;
        println!("knob {}", buf[0]);
        Timer::after_millis(500).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let board = Microbit::default();
    let display = board.display;
    
    bind_interrupts!(struct Irqs {
        SAADC => saadc::InterruptHandler;
    });

    let led_pin = |p| {
        Output::new(p, Level::Low, OutputDrive::Standard)
    };
    let red = led_pin(AnyPin::from(board.p9));
    let green = led_pin(AnyPin::from(board.p8));
    let blue = led_pin(AnyPin::from(board.p16));

    let mut saadc_config = saadc::Config::default();
    saadc_config.resolution = saadc::Resolution::_14BIT;
    let saadc = saadc::Saadc::new(
        board.saadc,
        Irqs,
        saadc_config,
        [saadc::ChannelConfig::single_ended(board.p2)],
    );

    println!("spawning");
    unwrap!(spawner.spawn(mb2_blinker(display, Duration::from_millis(500))));
    unwrap!(spawner.spawn(rgb_blinker([red, green, blue], Duration::from_millis(500))));
    unwrap!(spawner.spawn(knob(saadc)));
}
