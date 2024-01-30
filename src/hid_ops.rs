use std::{
    fs::{File, OpenOptions},
    io::{self, Write}, thread, time,
};

use crate::key_map;

pub fn write_to_device_file(device_file_path: &str, data: &str) -> io::Result<()> {
    log::info!("Sending pin: {}", data);

    let mut file = OpenOptions::new().write(true).open(device_file_path)?;

    for char in data.chars() {
        let keycode = key_map::char_to_scancode(char);
        match keycode {
            None => log::error!("Keycode not found for char: {}", char),
            Some(keycode) => {
                send_scancode(&mut file, keycode)?;
                send_scancode(&mut file, 0x00)?;
            }
        }
    }

    Ok(())
}

fn send_scancode(file: &mut File, keycode: u8) -> Result<(), io::Error> {
    // Write the scancode to the device file
    let result = file.write(&[0x00, 0x00, keycode, 0x00, 0x00, 0x00, 0x00, 0x00])?;

    // Ensure the data is written to the device file
    file.flush()?;

    if result > 0 {
        log::debug!("Successfully wrote {} bits", result);
    } else {
        log::debug!("Failed to write scancode: {}", keycode);
    }

    // Give time for the device to process the keypress
    thread::sleep(time::Duration::from_millis(50));
    Ok(())
}
