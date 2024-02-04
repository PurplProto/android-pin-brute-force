use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use regex::Regex;
use std::{
    io::{stdout, Write},
    thread,
    time::{self, Duration, Instant},
};

pub fn set_time_out(duration: Duration, message: &str) {
    let start = Instant::now();
    let mut stdout = stdout();

    stdout.execute(cursor::Hide).unwrap();

    loop {
        let elapsed = start.elapsed();
        if duration < elapsed {
            break;
        }
        let remaining_time = duration - elapsed;

        stdout.queue(cursor::SavePosition).unwrap();
        stdout
            .write_all(format!("{}: {}", message, format_duration(remaining_time)).as_bytes())
            .unwrap();
        stdout.queue(cursor::RestorePosition).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(100));

        stdout.queue(cursor::RestorePosition).unwrap();
        stdout
            .queue(terminal::Clear(terminal::ClearType::FromCursorDown))
            .unwrap();
    }

    stdout.execute(cursor::Show).unwrap();
}

pub fn parse_duration(duration: &str) -> Duration {
    let re = Regex::new(r"((?P<hour>\d+)h)?((?P<minute>\d+)m)?((?P<second>\d+)s)?").unwrap();
    let caps = re.captures(duration).unwrap();
    let h: u64 = caps.name("hour").map_or(0, |m| m.as_str().parse().unwrap());
    let m: u64 = caps
        .name("minute")
        .map_or(0, |m| m.as_str().parse().unwrap());
    let s: u64 = caps
        .name("second")
        .map_or(0, |m| m.as_str().parse().unwrap());
    time::Duration::new(3600 * h + 60 * m + s, 0)
}

fn format_duration(d: Duration) -> String {
    let total_seconds = d.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
