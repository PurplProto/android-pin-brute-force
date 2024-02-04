use std::time::Duration;

#[derive(Debug)]
pub struct Settings<'a> {
    pub device: String,
    pub cool_down: Vec<CoolDown>,
    pub pin_list: &'a [&'static str],
}

#[derive(Debug)]
pub struct CoolDown {
    pub duration: Duration,
    pub count: i32,
}
