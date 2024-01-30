mod hid_ops;
mod key_map;
mod pin_list_4_digit;

static KEYBOARD_DEVICE: &str = "/dev/hidg0";

fn main() {
    println!("Starting app...");

    println!("Writing to device file...");
    let result = hid_ops::write_to_device_file(KEYBOARD_DEVICE, "Hello World");
    match result {
        Ok(_) => println!("Successfully wrote to device file"),
        Err(_) => println!("Failed to write to device file"),
    }

    println!("Finished app...");
}
