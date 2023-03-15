extern crate int_enum;

use int_enum::IntEnum;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, IntEnum, Copy, Clone)]
pub enum Actions {
    Wink = 0b00001,
    DoubleBlink = 0b00010,
    CloseYourEyes = 0b00100,
    Jump = 0b01000,
    Reverse = 0b10000,
}

pub fn actions(n: u8) -> Vec<&'static str> {
    let mut v: Vec<&'static str> = Vec::new();
    if n & Actions::Wink.int_value() == Actions::Wink.int_value() {
        v.push("wink");
    }
    if n & Actions::DoubleBlink.int_value() == Actions::DoubleBlink.int_value() {
        v.push("double blink");
    }
    if n & Actions::CloseYourEyes.int_value() == Actions::CloseYourEyes.int_value() {
        v.push("close your eyes");
    }
    if n & Actions::Jump.int_value() == Actions::Jump.int_value() {
        v.push("jump");
    }
    if n & Actions::Reverse.int_value() == Actions::Reverse.int_value() {
        v.reverse();
    }

    v

}
