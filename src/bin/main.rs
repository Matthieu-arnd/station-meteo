#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::clock::CpuClock;
use esp_hal::gpio::Pin;
use esp_hal::{
    main,
    gpio::{Level,Input,InputConfig, Output, OutputConfig},
};
use esp_hal::time::{Duration, Instant};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();


#[main]
fn main() -> ! {
    // generator version: 0.5.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);
    let mut led = Output::new(_peripherals.GPIO0, Level::Low, OutputConfig::default());
    let mut DHT11 = Output::new(_peripherals.GPIO4, Level::High, OutputConfig::default());
    led.set_high();

    loop {
        DHT11.set_low();
        let mut delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(18) {}
        DHT11.set_high();
        delay_start =Instant::now();
        while delay_start.elapsed() < Duration::from_micros(48) {}
        DHT11 = Input::new(_peripherals.GPIO, InputConfig::default());

    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}

fn wait_until_state<T: Pin>(pin:T) -> Result<(), P::Error> {
    loop {
                if pin.is_low()? {
                    break;
                }
                if pin.is_high()? {
                    break;
                }
        }
    
    Ok(())
}
