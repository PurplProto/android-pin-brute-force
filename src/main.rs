use log::LevelFilter;
use simple_logger::SimpleLogger;

mod hid_ops;
mod key_map;
mod pin_list_4_digit;

static KEYBOARD_DEVICE: &str = "/dev/hidg0";

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .env()
        .init()
        .unwrap();
    log::info!("Starting app...");

    log::info!("Writing to device file...");
    let result = hid_ops::write_to_device_file(KEYBOARD_DEVICE, "Hello World");
    match result {
        Ok(_) => log::info!("Successfully wrote to device file"),
        Err(_) => log::error!("Failed to write to device file"),
    }

    log::info!("Finished app...");
}
