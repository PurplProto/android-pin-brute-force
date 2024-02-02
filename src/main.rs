use clap::{ArgAction, Args, Parser, Subcommand};
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::process::exit;

mod hid_ops;
mod key_map;
mod pin_list_4_digit;

static KEYBOARD_DEVICE: &str = "/dev/hidg0";

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Optional device file to use. Defaults to: /dev/hidg0
    #[arg(short, long)]
    device: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, action = ArgAction::Count)]
    verbose: u8,
}

#[derive(Subcommand)]
enum Commands {
    /// starts brute force attack
    Start,
    /// resumes brute force attack
    Resume(ResumeArgs),
}

#[derive(Args)]
struct ResumeArgs {
    /// pin to resume from
    pin: String,
}

fn main() {
    let cli = Cli::parse();
    let mut logger = SimpleLogger::new();

    logger = match cli.verbose {
        0 => logger.with_level(LevelFilter::Info),
        1 => logger.with_level(LevelFilter::Debug),
        _ => logger.with_level(LevelFilter::Trace),
    };

    logger.env().init().unwrap();

    log::debug!("Starting app...");

    match cli.command {
        Some(Commands::Start) => start(),
        Some(Commands::Resume(args)) => resume(args),
        None => {
            log::error!("No command provided. Use --help for usage information.");
            exit(1);
        }
    }

    log::debug!("Finished app...");
}

fn start() {
    log::info!("Starting brute force attack...");
    let result = hid_ops::write_to_device_file(KEYBOARD_DEVICE, "Hello World");
    match result {
        Ok(_) => log::info!("Brute force attack complete"),
        Err(e) => log::error!("Failed to start brute force attack: {}", e),
    }
}

fn resume(args: ResumeArgs) {
    log::info!("Resuming brute force attack from pin: {}", args.pin);
    let result = hid_ops::write_to_device_file(KEYBOARD_DEVICE, "Hello World");
    match result {
        Ok(_) => log::info!("Brute force attack complete"),
        Err(e) => log::error!("Failed to start brute force attack: {}", e),
    }
}
