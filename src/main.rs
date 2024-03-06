#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embassy_sync::{
    mutex::Mutex,
    blocking_mutex::raw::ThreadModeRawMutex,
};
use microbit_bsp::{
    display::{Brightness, Frame},
    embassy_nrf::{
        bind_interrupts,
        gpio::{AnyPin, Level, Output, OutputDrive},
        saadc,
    },
    LedMatrix, Microbit,
};
use panic_probe as _;

static KNOB_VALUE: Mutex<ThreadModeRawMutex, i16> = Mutex::new(0);

struct Mb2Blinker {
    display: LedMatrix,
    state: bool,
}

impl Mb2Blinker {
    fn new(mut display: LedMatrix) {
        display.clear();
        display.set_brightness(Brightness::MAX);
        Self{ display, state: false }
    }

    #[embassy_executor::task]
    async fn step(&mut self) {
        if self.state {
            self.display.frame.set(0, 0);
        } else {
            self.display.frame.unset(0, 0);
        }
    }
}

struct RgbBlinker {
    rgb: [Output<'static, AnyPin>; 3],
    cur: u8,
}

impl RgbBlinker {
    fn new(rgb: [Output<'static, AnyPin>; 3]) {
        Self { rgb, cur: 0 }
    }

    #[embassy_executor::task]
    async fn step(&mut self) {
        let prev = (self.cur + 2) % 3;
        self.rgb[prev].set_low();
        self.rgb[self.cur].set_high();
        Timer::after(interval).await;
        self.cur = (self.cur + 1) % 3;
    }
}

#[embassy_executor::task]
async fn knob(mut saadc: saadc::Saadc<'static, 1>, interval: Duration) -> ! {
    loop {
        let mut buf = [0];
        saadc.sample(&mut buf).await;

        let mut knob_value = KNOB_VALUE.lock().await;
        *knob_value = buf[0];
        drop(knob_value);

        Timer::after(interval).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let board = Microbit::default();
    let display = board.display;

    bind_interrupts!(struct Irqs {
        SAADC => saadc::InterruptHandler;
    });

    let led_pin = |p| Output::new(p, Level::Low, OutputDrive::Standard);
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

    let mb2_blinker = Mb2Blinker::new(display);
    let rgb_blinker = RgbBlinker::new([red, green, blue]);
    println!("spawning knob");
    unwrap!(spawner.spawn(knob(saadc, Duration::from_millis(100))));
    println!("stepping");
    loop {
        println!("step");
        unwrap!(spawner.spawn(mb2_blinker.step()));
        unwrap!(spawner.spawn(rgb_blinker.step()));
    }
}
