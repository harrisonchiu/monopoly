#![allow(dead_code)]
pub fn get_set_background_colour(colour: &str) -> String {
    //! Use
    match colour {
        "Red" => String::from("\x1b[41m"),
        "Orange" => String::from("\x1b[48;5;166m"),
        "Yellow" => String::from("\x1b[43m"),
        "Green" => String::from("\x1b[42m"),
        "Cyan" => String::from("\x1b[46m"),
        "Blue" => String::from("\x1b[44m"),
        "Magenta" => String::from("\x1b[45m"),
        "Brown" => String::from("\x1b[48;5;94m"),
        "Railroad" => String::from("\x1b[100m"), // Gray
        "Utility" => String::from("\x1b[47m"),   // White
        _ => panic!(
            "Unknown set name! We assume uses of this function \
            and its parameter are hardcoded and NOT dynamically inputted."
        ),
    }
}

pub fn get_foreground_colour(colour: &str) -> &str {
    match colour {
        "Blue" => "\x1b[96m",
        "Green" => "\x1b[92m",
        "Red" => "\x1b[91m",
        "Magenta" => "\x1b[95m",
        _ => panic!(
            "Unknown colour name! We assume uses of this function \
            and its parameter are hardcoded and NOT dynamically inputted."
        ),
    }
}
