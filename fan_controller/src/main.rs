#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp as hal;
use embassy_rp::block::ImageDef;
use embassy_rp::gpio;
use embassy_time::Timer;
use gpio::{Level, Output};

//Panic Handler
use panic_probe as _;
// Defmt Logging
use defmt_rtt as _;

/// Tell the Boot ROM about our application
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut leds: [Output; 8] = [
        Output::new(p.PIN_6, Level::Low),
        Output::new(p.PIN_7, Level::Low),
        Output::new(p.PIN_9, Level::Low),
        Output::new(p.PIN_8, Level::Low),
        Output::new(p.PIN_10, Level::Low),
        Output::new(p.PIN_11, Level::Low),
        Output::new(p.PIN_12, Level::Low),
        Output::new(p.PIN_13, Level::Low),
    ];
    let base_time = 1;
    let mut c = 0;
    let mut time_on = base_time;
    loop {
        for led in &mut leds {
            led.set_high();
            Timer::after_micros(time_on).await;
        }
        for led in &mut leds {
            led.set_low();
            Timer::after_micros(1000).await;
            c += 1;
            if c > 10 {
                time_on += 1;
                c = 0;
            }
            if time_on > 1000 {
                time_on = base_time;
            }
        }
    }
}

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"fan_controller"),
    embassy_rp::binary_info::rp_program_description!(c"your program description"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// End of file
