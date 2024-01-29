use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    thread, time,
};
mod key_map;
mod pin_list_4_digit;

static KEYBOARD_DEVICE: &str = "/dev/hidg0";

fn send_scancode(file: &mut File, keycode: u8) -> Result<(), io::Error> {
    file.write(&[0x00, 0x00, 0x00, keycode, 0x00, 0x00, 0x00, 0x00])?;
    file.flush()?;
    thread::sleep(time::Duration::from_millis(50));
    Ok(())
}

fn write_to_device_file(device_file_path: &str, data: &str) -> io::Result<()> {
    println!("Sending pin: {}", data);

    let mut file = OpenOptions::new().write(true).open(device_file_path)?;

    for char in data.chars() {
        let keycode = key_map::char_to_scancode(char);
        match keycode {
            None => println!("Keycode not found for char: {}", char),
            Some(keycode) => {
                send_scancode(&mut file, keycode)?;
                send_scancode(&mut file, 0x00)?;
            }
        }
    }

    Ok(())
}

fn main() {
    println!("Starting app...");

    println!("Writing to device file...");
    let result = write_to_device_file(KEYBOARD_DEVICE, "Hello World");
    match result {
        Ok(_) => println!("Successfully wrote to device file"),
        Err(_) => println!("Failed to write to device file"),
    }

    println!("Finished app...");
}
