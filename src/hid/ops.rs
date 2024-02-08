use crate::hid::key_map;
use log::{error, trace};
use std::{
    fs::OpenOptions,
    io::{self, Write},
    thread,
    time::{self, Duration},
};

struct MouseReport {
    pub button_state: String,
    pub x_axis: i8,
    pub y_axis: i8,
    pub wheel: i8,
}

pub fn perform_swipe_up(device_file_path: &str) -> io::Result<()> {
    for _i in 0..20 {
        trace!("Moving mouse to bottom of screen...");
        let report = MouseReport {
            button_state: "reset".to_string(),
            x_axis: 0,
            y_axis: 127, // Move down
            wheel: 0,
        };
        send_mouse_report(device_file_path, report).unwrap();
    }

    trace!("Mouse should be at the bottom of the screen");
    thread::sleep(Duration::from_millis(250));

    trace!("Move mouse up a bit to avoid any bottom edge gestures");
    let report = MouseReport {
        button_state: "reset".to_string(),
        x_axis: 0,
        y_axis: -100, // Move up
        wheel: 0,
    };
    send_mouse_report(device_file_path, report).unwrap();

    for _i in 0..10 {
        trace!("Holding left click and moving up...");
        let report = MouseReport {
            button_state: "b1".to_string(),
            x_axis: 0,
            y_axis: -100, // Move up
            wheel: 0,
        };
        send_mouse_report(device_file_path, report).unwrap();
    }

    trace!("Release left click");
    let report = MouseReport {
        button_state: "reset".to_string(),
        x_axis: 0,
        y_axis: 0,
        wheel: 0,
    };
    send_mouse_report(device_file_path, report).unwrap();

    Ok(())
}

fn send_mouse_report(device_file_path: &str, report: MouseReport) -> io::Result<()> {
    match key_map::char_to_reportcode(&report.button_state) {
        Some(button) => {
            let keycode = &[
                button,              // Button state
                report.x_axis as u8, // X-axis movement
                report.y_axis as u8, // Y-axis movement
                report.wheel as u8,  // Wheel movement
            ];
            trace!("Sending mouse report: {:?}", keycode);
            write_to_device_file(device_file_path, keycode)
        }
        None => {
            error!("Button state not found: {}", report.button_state);
            Ok(())
        }
    }
}

pub fn send_string_as_keyboard_scan_codes(device_file_path: &str, data: &str) -> io::Result<()> {
    log::info!("Sending pin: {}", data);

    for char in data.chars() {
        let keycode = key_map::char_to_scancode(char);
        match keycode {
            None => error!("Keycode not found for char: {}", char),
            Some(keycode) => {
                write_to_device_file(
                    device_file_path,
                    &[
                        0x00,    // Modifier byte
                        0x00,    // Reserved
                        keycode, // Keycode
                        0x00, 0x00, 0x00, 0x00, 0x00,
                    ],
                )?;
                write_to_device_file(
                    device_file_path,
                    &[
                        0x00, // Modifier byte
                        0x00, // Reserved
                        0x00, // Keycode
                        0x00, 0x00, 0x00, 0x00, 0x00,
                    ],
                )?;
            }
        }
    }

    Ok(())
}

fn write_to_device_file(device_file_path: &str, keycode: &[u8]) -> Result<(), io::Error> {
    let mut file = OpenOptions::new().write(true).open(device_file_path)?;

    // Write the raw byes to the device file
    let result = file.write(keycode)?;

    // Ensure the data is written to the device file
    file.flush()?;

    if result > 0 {
        log::trace!("Successfully wrote {} bits", result);
    } else {
        log::trace!("Failed to write scancode: {:?}", keycode);
    }

    // Give time for the device to process the keypress
    thread::sleep(time::Duration::from_millis(50));
    Ok(())
}
