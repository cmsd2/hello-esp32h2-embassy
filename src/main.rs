#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::clock::ClockControl;
use esp_hal::peripherals::Peripherals;
use esp_hal::prelude::*;
use esp_hal::system::SystemControl;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::timer::{ErasedTimer, OneShotTimer};
use esp_hal_procmacros::main;

// from https://github.com/esp-rs/esp-hal/blob/main/examples/src/bin/embassy_hello_world.rs
// When you are okay with using a nightly compiler it's better to use https://docs.rs/static_cell/2.1.0/static_cell/macro.make_static.html
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

#[embassy_executor::task]
async fn one_second_task() {
    let mut count = 0;
    loop {
        log::info!("Spawn Task Count: {}", count);
        count += 1;
        Timer::after(Duration::from_millis(1_000)).await;
    }
}

#[main]
async fn main(spawner: Spawner) {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let timg0 = TimerGroup::new(peripherals.TIMG0, &clocks, None);
    let timer0: ErasedTimer = timg0.timer0.into();
    let timers = [OneShotTimer::new(timer0)];
    let timers = mk_static!([OneShotTimer<ErasedTimer>; 1], timers);

    esp_println::logger::init_logger_from_env();

    log::info!("Hello World!");

    esp_hal_embassy::init(&clocks, timers);

    spawner.spawn(one_second_task()).unwrap();

    let mut count = 0;
    loop {
        log::info!("Main Task Count: {}", count);
        count += 1;
        Timer::after(Duration::from_millis(5_000)).await;
    }
}
