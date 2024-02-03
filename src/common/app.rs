pub struct Settings {
    pub device: String,
    pub cool_down: Vec<CoolDown>,
}

pub struct CoolDown {
    pub duration: String,
    pub count: u8,
}
