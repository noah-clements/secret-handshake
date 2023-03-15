
pub fn actions(n: u8) -> Vec<&'static str> {
    let wink: u8 = 0b00001;
    let double_blink: u8 = 0b00010;
    let close_your_eyes: u8 = 0b00100;
    let jump: u8 = 0b01000;
    let reverse: u8 = 0b10000;

    let mut v: Vec<&'static str> = Vec::new();
    if n & wink == wink {
        v.push("wink");
    }
    if n & double_blink == double_blink {
        v.push("double blink");
    }
    if n & close_your_eyes == close_your_eyes {
        v.push("close your eyes");
    }
    if n & jump == jump {
        v.push("jump");
    }
    if n & reverse == reverse {
        v.reverse();
    }

    v

}
