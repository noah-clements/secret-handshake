
const SIGNS: [&str; 4] = ["wink", "double blink", "close your eyes", "jump"];
const REVERSE_SIGNS: u8 = 0b10000;

pub fn actions(n: u8) -> Vec<&'static str> {
    let (mut action, action_incr, end) = match n & REVERSE_SIGNS {
        0 => (0, 1, 4),
        _ => (3, -1, -1),
    };
    let mut output: Vec<&'static str> = Vec::new();
    
    while action != end {
        if (n & (1 << action)) != 0 {
            output.push(SIGNS[action as usize])
        }
        action += action_incr
    }
    output
}

