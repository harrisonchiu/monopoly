#![allow(unused)]

pub struct Colour;

impl Colour {
    //! Make sure all colour strings are the same length
    //! That way, it can be used in board string without disrupting its length
    //! Many things depend on indices of certain chars and its length

    pub fn background<'a>(colour: &'a str) -> &'a str {
        match colour {
            "Red" => "\x1b[48;5;009m",
            "Orange" => "\x1b[48;5;202m",
            "Yellow" => "\x1b[48;5;011m",
            "Green" => "\x1b[48;5;010m",
            "Cyan" => "\x1b[48;5;014m",
            "Blue" => "\x1b[48;5;012m",
            "Magenta" => "\x1b[48;5;013m",
            "Brown" => "\x1b[48;5;094m",
            "Gray" | "Railroad" => "\x1b[48;5;008m",
            "White" | "Utility" => "\x1b[48;5;015m",
            "Black" | "Event" => "\x1b[48;5;000m",
            _ => "\x1b[00000049m", // Default background colour
        }
    }

    pub fn foreground<'a>(colour: &'a str) -> &'a str {
        match colour {
            "Bright Cyan" => "\x1b[96m",
            "Bright Green" => "\x1b[92m",
            "Bright Red" => "\x1b[91m",
            "Bright Magenta" => "\x1b[95m",
            _ => "\x1b[39m", // Default foreground colour
        }
    }
}
