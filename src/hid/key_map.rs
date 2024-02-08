// These mappings are for the US keyboard layout.
pub fn char_to_scancode(c: char) -> Option<u8> {
    match c {
        'a' | 'A' => Some(0x04),
        'b' | 'B' => Some(0x05),
        'c' | 'C' => Some(0x06),
        'd' | 'D' => Some(0x07),
        'e' | 'E' => Some(0x08),
        'f' | 'F' => Some(0x09),
        'g' | 'G' => Some(0x0a),
        'h' | 'H' => Some(0x0b),
        'i' | 'I' => Some(0x0c),
        'j' | 'J' => Some(0x0d),
        'k' | 'K' => Some(0x0e),
        'l' | 'L' => Some(0x0f),
        'm' | 'M' => Some(0x10),
        'n' | 'N' => Some(0x11),
        'o' | 'O' => Some(0x12),
        'p' | 'P' => Some(0x13),
        'q' | 'Q' => Some(0x14),
        'r' | 'R' => Some(0x15),
        's' | 'S' => Some(0x16),
        't' | 'T' => Some(0x17),
        'u' | 'U' => Some(0x18),
        'v' | 'V' => Some(0x19),
        'w' | 'W' => Some(0x1a),
        'x' | 'X' => Some(0x1b),
        'y' | 'Y' => Some(0x1c),
        'z' | 'Z' => Some(0x1d),
        '1' => Some(0x1e),
        '2' => Some(0x1f),
        '3' => Some(0x20),
        '4' => Some(0x21),
        '5' => Some(0x22),
        '6' => Some(0x23),
        '7' => Some(0x24),
        '8' => Some(0x25),
        '9' => Some(0x26),
        '0' => Some(0x27),
        ' ' => Some(0x2c),
        _ => None,
    }
}

pub fn char_to_reportcode(button: &str) -> Option<u8> {
    match button {
        "b1" => Some(0x01),
        "b2" => Some(0x02),
        "b3" => Some(0x03),
        _ => Some(0x00),
    }
}
