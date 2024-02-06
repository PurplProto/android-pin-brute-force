use super::{CoolDown, Settings, KEYBOARD_DEVICE};
use crate::{pin_lists, timeout::parse_duration};
use clap::{command, ArgAction, Args, Parser, Subcommand};
use log::error;
use std::process::exit;

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// List of cool down periods between pin attempts.
    /// Go format and count seperated by a colon i.e. -c 15s:3 -c 10m:3 -c 30m:-1
    /// Omitting the the count or using -1 sets the cool down period until the end of the pin list.
    #[arg(short, long, action = ArgAction::Append)]
    pub cool_down: Vec<String>,

    /// <Optional> device file to use. Defaults to: /dev/hidg0
    #[arg(short, long)]
    pub device: Option<String>,

    /// <Optional> Size of the pin to brute force. Defaults to 4. Currently only supports 4.
    #[arg(short, long)]
    pub pin_size: Option<u8>,

    /// <Optional> Turn debugging information on. Can be passed up to 2 times for more verbosity.
    #[arg(short, long, action = ArgAction::Count)]
    pub verbose: Option<u8>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// starts brute force attack
    Start,
    /// resumes brute force attack
    Resume(ResumeArgs),
}

#[derive(Args)]
pub struct ResumeArgs {
    /// pin to resume from
    pub pin: String,
}

pub fn parse_cli_args(cli: &Cli) -> Settings {
    return Settings {
        device: match &cli.device {
            Some(s) => s.to_string(),
            None => KEYBOARD_DEVICE.to_string(),
        },
        cool_down: match cli.cool_down.len() {
            0 => {
                error!("No cool down periods provided. Use --help for usage information.");
                exit(1);
            }
            _ => {
                let mut cool_downs: Vec<CoolDown> = Vec::new();
                for cd in &cli.cool_down {
                    let parts: Vec<&str> = cd.split(':').collect();
                    match parts.len() {
                        1 => cool_downs.push(CoolDown {
                            duration: parse_duration(parts[0]),
                            count: 0,
                        }),
                        2 => cool_downs.push(CoolDown {
                            duration: parse_duration(parts[0]),
                            count: match parts[1].parse::<i32>() {
                                Ok(c) => c,
                                Err(e) => {
                                    error!("Invalid count: {}, with error: {}", parts[1], e);
                                    exit(1);
                                }
                            },
                        }),
                        _ => {
                            error!("Invalid cool down period: {}", cd);
                            exit(1);
                        }
                    }
                }
                cool_downs
            }
        },
        pin_list: match cli.pin_size {
            Some(4) => pin_lists::get_four_digit_pin_list(),
            Some(6) => pin_lists::get_six_digit_pin_list(),
            Some(s) => {
                error!("Invalid pin size: {}", s);
                exit(1);
            }
            None => pin_lists::get_four_digit_pin_list(),
        },
    };
}
