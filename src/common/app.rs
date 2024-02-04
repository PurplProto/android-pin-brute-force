use std::time::Duration;

#[derive(Debug)]
pub struct Settings {
    pub device: String,
    pub cool_down: Vec<CoolDown>,
}

#[derive(Debug)]
pub struct CoolDown {
    pub duration: Duration,
    pub count: i32,
}
