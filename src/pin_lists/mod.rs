pub fn get_four_digit_pin_list() -> Vec<&'static str> {
    include_str!("../assets/4_digit_pins_optimised.txt")
        .split("\n")
        .filter(|s| !s.is_empty() && !s.starts_with("//"))
        .collect()
}
