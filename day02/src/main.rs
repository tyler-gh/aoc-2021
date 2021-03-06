use regex::{Captures, Regex};

fn main() {
    let input_re = Regex::new(r#"(forward) (\d+)|(down) (\d+)|(up) (\d+)"#).unwrap();

    let instructions = INSTRUCTION_STRINGS
        .iter()
        .map(|instruction| input_re.captures(instruction).map(regex_capture_to_vec))
        .collect::<Vec<_>>();

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for instruction in instructions {
        match instruction.map(parse_instruction) {
            Some(("forward", val)) => {
                horizontal = horizontal + val;
                depth = depth + (val * aim)
            }
            Some(("down", val)) => aim = aim + val,
            Some(("up", val)) => aim = aim - val,
            _ => {}
        }
    }

    println!("{:?}", horizontal * depth);
}

fn parse_instruction(instruction: Vec<&str>) -> (&str, i32) {
    (instruction[0], instruction[1].parse::<i32>().unwrap())
}

fn regex_capture_to_vec(capture: Captures) -> Vec<&str> {
    capture
        .iter()
        .skip(1)
        .flat_map(|c| c)
        .map(|c| c.as_str())
        .collect::<Vec<_>>()
}

const INSTRUCTION_STRINGS: [&str; 1000] = [
    "forward 1",
    "forward 8",
    "down 9",
    "forward 3",
    "down 6",
    "down 1",
    "down 1",
    "forward 5",
    "forward 5",
    "forward 8",
    "down 6",
    "forward 7",
    "down 3",
    "down 4",
    "up 1",
    "forward 7",
    "forward 5",
    "forward 8",
    "down 1",
    "forward 9",
    "forward 9",
    "forward 7",
    "up 8",
    "forward 5",
    "forward 9",
    "down 3",
    "forward 2",
    "down 5",
    "down 4",
    "up 2",
    "up 5",
    "forward 3",
    "forward 8",
    "down 9",
    "down 1",
    "forward 2",
    "forward 6",
    "up 2",
    "up 9",
    "forward 8",
    "down 7",
    "forward 2",
    "up 5",
    "forward 7",
    "down 9",
    "forward 9",
    "forward 5",
    "down 8",
    "down 6",
    "forward 2",
    "up 4",
    "down 2",
    "down 9",
    "forward 4",
    "down 4",
    "forward 9",
    "down 8",
    "down 7",
    "down 4",
    "forward 6",
    "forward 9",
    "up 7",
    "down 2",
    "up 6",
    "down 8",
    "forward 3",
    "forward 9",
    "forward 7",
    "down 1",
    "forward 1",
    "forward 7",
    "forward 5",
    "down 4",
    "down 1",
    "forward 4",
    "forward 9",
    "forward 6",
    "down 5",
    "down 1",
    "forward 9",
    "down 6",
    "down 4",
    "forward 8",
    "up 8",
    "forward 9",
    "up 9",
    "up 9",
    "up 7",
    "forward 4",
    "down 4",
    "forward 4",
    "forward 2",
    "forward 2",
    "forward 9",
    "up 4",
    "forward 4",
    "forward 3",
    "forward 5",
    "down 3",
    "up 4",
    "forward 3",
    "forward 5",
    "forward 9",
    "forward 7",
    "down 1",
    "forward 4",
    "down 5",
    "up 6",
    "down 9",
    "forward 8",
    "down 1",
    "forward 3",
    "down 2",
    "up 8",
    "down 5",
    "down 8",
    "forward 5",
    "down 6",
    "forward 6",
    "down 7",
    "up 5",
    "forward 8",
    "forward 5",
    "forward 7",
    "up 3",
    "down 6",
    "up 9",
    "forward 1",
    "forward 7",
    "forward 3",
    "forward 8",
    "up 4",
    "up 9",
    "down 5",
    "forward 5",
    "forward 4",
    "forward 4",
    "down 9",
    "up 6",
    "forward 5",
    "forward 2",
    "down 6",
    "down 2",
    "forward 9",
    "down 7",
    "forward 8",
    "down 4",
    "forward 8",
    "forward 8",
    "up 7",
    "up 5",
    "forward 6",
    "forward 4",
    "up 4",
    "forward 2",
    "up 3",
    "down 8",
    "forward 1",
    "forward 8",
    "forward 8",
    "forward 4",
    "up 7",
    "forward 2",
    "down 8",
    "forward 1",
    "down 1",
    "down 4",
    "up 3",
    "forward 8",
    "forward 1",
    "down 9",
    "up 5",
    "down 5",
    "forward 9",
    "forward 4",
    "up 7",
    "down 2",
    "down 4",
    "forward 1",
    "forward 6",
    "up 8",
    "up 1",
    "forward 8",
    "down 5",
    "down 5",
    "forward 9",
    "down 8",
    "forward 9",
    "down 1",
    "up 7",
    "forward 8",
    "up 2",
    "down 2",
    "up 1",
    "up 9",
    "forward 6",
    "down 5",
    "forward 1",
    "down 2",
    "forward 5",
    "down 3",
    "up 8",
    "down 2",
    "down 8",
    "forward 7",
    "forward 2",
    "up 3",
    "forward 3",
    "down 1",
    "forward 4",
    "down 7",
    "up 5",
    "forward 8",
    "forward 8",
    "up 6",
    "up 4",
    "up 4",
    "down 7",
    "forward 2",
    "forward 1",
    "forward 6",
    "up 3",
    "forward 3",
    "forward 9",
    "down 4",
    "forward 2",
    "down 9",
    "down 8",
    "forward 5",
    "forward 2",
    "forward 4",
    "forward 5",
    "forward 7",
    "down 4",
    "up 5",
    "down 7",
    "forward 4",
    "up 7",
    "down 9",
    "down 4",
    "down 3",
    "forward 8",
    "down 5",
    "forward 8",
    "forward 8",
    "down 3",
    "down 8",
    "down 2",
    "forward 4",
    "forward 5",
    "down 7",
    "down 7",
    "down 6",
    "up 9",
    "down 9",
    "down 5",
    "forward 4",
    "down 1",
    "down 7",
    "up 4",
    "down 8",
    "up 5",
    "up 1",
    "down 9",
    "down 7",
    "forward 9",
    "forward 7",
    "down 6",
    "up 2",
    "forward 1",
    "down 8",
    "up 2",
    "forward 8",
    "down 2",
    "forward 2",
    "down 9",
    "forward 2",
    "forward 3",
    "forward 7",
    "up 3",
    "up 3",
    "up 3",
    "forward 5",
    "up 9",
    "up 1",
    "down 9",
    "down 4",
    "down 5",
    "up 9",
    "up 7",
    "down 9",
    "forward 3",
    "down 5",
    "down 6",
    "down 3",
    "up 6",
    "forward 8",
    "up 6",
    "up 8",
    "down 4",
    "forward 8",
    "down 8",
    "forward 7",
    "up 1",
    "forward 4",
    "down 2",
    "forward 7",
    "down 6",
    "up 8",
    "forward 1",
    "down 8",
    "down 4",
    "forward 7",
    "forward 2",
    "up 4",
    "forward 9",
    "forward 9",
    "down 8",
    "up 1",
    "down 3",
    "forward 7",
    "down 9",
    "forward 2",
    "forward 5",
    "down 4",
    "down 8",
    "down 3",
    "up 1",
    "down 2",
    "up 3",
    "forward 1",
    "forward 5",
    "forward 3",
    "down 2",
    "up 2",
    "forward 7",
    "down 6",
    "forward 3",
    "down 8",
    "forward 5",
    "forward 4",
    "up 2",
    "forward 4",
    "up 2",
    "down 4",
    "forward 8",
    "forward 5",
    "down 1",
    "forward 6",
    "down 2",
    "down 5",
    "up 8",
    "forward 5",
    "down 2",
    "up 5",
    "down 5",
    "forward 4",
    "forward 3",
    "forward 8",
    "down 9",
    "up 3",
    "forward 4",
    "forward 2",
    "forward 2",
    "forward 2",
    "down 4",
    "forward 8",
    "forward 5",
    "up 3",
    "down 1",
    "forward 3",
    "up 5",
    "forward 8",
    "down 6",
    "forward 6",
    "down 9",
    "forward 6",
    "up 5",
    "down 6",
    "up 9",
    "forward 8",
    "down 2",
    "forward 9",
    "down 7",
    "down 5",
    "down 4",
    "forward 3",
    "forward 2",
    "forward 1",
    "forward 7",
    "down 3",
    "forward 4",
    "up 6",
    "down 5",
    "down 4",
    "down 8",
    "down 4",
    "up 6",
    "forward 3",
    "down 4",
    "down 6",
    "forward 9",
    "forward 6",
    "up 4",
    "down 2",
    "down 7",
    "forward 2",
    "forward 9",
    "down 4",
    "down 8",
    "down 3",
    "down 4",
    "forward 9",
    "forward 4",
    "forward 1",
    "down 7",
    "forward 2",
    "up 1",
    "forward 7",
    "down 7",
    "forward 7",
    "forward 5",
    "up 8",
    "down 4",
    "up 7",
    "up 2",
    "up 7",
    "up 8",
    "down 9",
    "forward 8",
    "forward 8",
    "down 3",
    "forward 9",
    "down 3",
    "up 7",
    "down 1",
    "down 9",
    "forward 2",
    "up 4",
    "down 2",
    "forward 2",
    "up 5",
    "up 5",
    "up 1",
    "forward 7",
    "up 5",
    "down 3",
    "up 8",
    "down 9",
    "down 7",
    "up 4",
    "down 8",
    "down 4",
    "forward 8",
    "up 6",
    "down 2",
    "down 4",
    "forward 7",
    "forward 8",
    "forward 8",
    "forward 1",
    "down 4",
    "down 4",
    "forward 4",
    "down 7",
    "forward 3",
    "down 3",
    "down 6",
    "down 7",
    "down 6",
    "forward 8",
    "down 4",
    "down 7",
    "down 1",
    "down 7",
    "down 5",
    "down 2",
    "up 5",
    "forward 9",
    "down 3",
    "down 4",
    "down 4",
    "forward 5",
    "down 9",
    "forward 1",
    "up 1",
    "up 1",
    "down 3",
    "forward 8",
    "up 6",
    "up 6",
    "down 1",
    "up 2",
    "down 7",
    "down 9",
    "up 7",
    "forward 7",
    "down 6",
    "down 5",
    "down 5",
    "up 8",
    "forward 7",
    "down 1",
    "down 6",
    "forward 4",
    "forward 5",
    "forward 5",
    "forward 2",
    "down 5",
    "up 6",
    "down 5",
    "forward 1",
    "down 9",
    "up 4",
    "down 7",
    "down 8",
    "down 5",
    "down 8",
    "forward 7",
    "forward 2",
    "up 3",
    "down 7",
    "forward 1",
    "forward 4",
    "forward 5",
    "forward 1",
    "forward 7",
    "down 6",
    "forward 9",
    "forward 8",
    "down 5",
    "forward 5",
    "forward 8",
    "forward 3",
    "up 1",
    "forward 6",
    "forward 9",
    "forward 8",
    "down 3",
    "forward 8",
    "forward 7",
    "down 1",
    "forward 9",
    "down 9",
    "forward 6",
    "forward 4",
    "forward 2",
    "forward 9",
    "down 7",
    "down 6",
    "forward 4",
    "forward 1",
    "forward 3",
    "forward 9",
    "up 4",
    "down 5",
    "forward 6",
    "down 8",
    "up 5",
    "down 9",
    "down 6",
    "forward 7",
    "down 9",
    "forward 6",
    "forward 5",
    "down 1",
    "down 4",
    "up 6",
    "forward 2",
    "down 8",
    "down 5",
    "up 5",
    "forward 6",
    "forward 5",
    "down 8",
    "down 4",
    "down 3",
    "down 8",
    "forward 3",
    "forward 9",
    "up 6",
    "down 9",
    "down 1",
    "forward 7",
    "forward 5",
    "down 5",
    "down 1",
    "forward 7",
    "forward 3",
    "up 6",
    "forward 1",
    "up 9",
    "forward 1",
    "down 4",
    "down 3",
    "down 3",
    "up 6",
    "forward 9",
    "forward 5",
    "up 9",
    "up 5",
    "forward 5",
    "forward 7",
    "forward 9",
    "down 8",
    "forward 2",
    "down 7",
    "down 7",
    "forward 3",
    "down 2",
    "up 1",
    "down 4",
    "down 6",
    "down 8",
    "forward 8",
    "forward 6",
    "up 3",
    "forward 9",
    "down 8",
    "up 7",
    "forward 6",
    "forward 9",
    "up 8",
    "forward 3",
    "down 9",
    "forward 3",
    "forward 4",
    "down 3",
    "down 2",
    "forward 2",
    "down 5",
    "down 9",
    "down 2",
    "down 7",
    "down 4",
    "down 6",
    "forward 5",
    "up 1",
    "forward 9",
    "forward 2",
    "up 8",
    "forward 8",
    "down 3",
    "forward 7",
    "down 5",
    "forward 5",
    "down 5",
    "down 9",
    "down 9",
    "down 6",
    "down 5",
    "down 4",
    "up 6",
    "forward 9",
    "down 2",
    "down 5",
    "up 3",
    "forward 8",
    "forward 1",
    "down 4",
    "down 8",
    "forward 6",
    "forward 7",
    "up 2",
    "forward 1",
    "forward 2",
    "down 7",
    "down 6",
    "up 7",
    "forward 7",
    "down 6",
    "down 6",
    "down 8",
    "forward 8",
    "up 5",
    "up 9",
    "forward 8",
    "forward 1",
    "down 6",
    "down 6",
    "up 7",
    "forward 6",
    "up 5",
    "forward 7",
    "down 1",
    "forward 1",
    "forward 9",
    "up 7",
    "down 3",
    "forward 4",
    "down 6",
    "down 6",
    "up 5",
    "up 7",
    "down 1",
    "up 2",
    "down 8",
    "down 1",
    "forward 2",
    "down 3",
    "forward 8",
    "forward 2",
    "up 2",
    "down 7",
    "forward 5",
    "forward 7",
    "down 2",
    "up 1",
    "down 1",
    "down 6",
    "down 4",
    "up 1",
    "forward 2",
    "forward 7",
    "forward 8",
    "down 1",
    "forward 6",
    "down 5",
    "down 8",
    "up 6",
    "down 7",
    "forward 6",
    "down 8",
    "down 6",
    "down 9",
    "forward 8",
    "down 9",
    "down 6",
    "up 2",
    "down 2",
    "down 5",
    "down 5",
    "up 8",
    "forward 6",
    "forward 7",
    "up 4",
    "down 5",
    "up 5",
    "forward 6",
    "forward 8",
    "up 6",
    "up 7",
    "up 3",
    "up 9",
    "down 6",
    "forward 3",
    "forward 3",
    "down 6",
    "down 8",
    "down 2",
    "down 2",
    "up 7",
    "up 6",
    "forward 5",
    "forward 4",
    "down 2",
    "down 3",
    "forward 8",
    "down 9",
    "forward 3",
    "down 8",
    "down 8",
    "forward 9",
    "forward 7",
    "down 8",
    "down 7",
    "up 2",
    "down 8",
    "down 1",
    "down 7",
    "up 7",
    "forward 3",
    "forward 5",
    "up 1",
    "down 9",
    "forward 3",
    "down 4",
    "down 5",
    "down 3",
    "down 8",
    "up 7",
    "forward 4",
    "down 6",
    "forward 9",
    "forward 9",
    "forward 1",
    "up 6",
    "up 8",
    "forward 6",
    "down 6",
    "down 6",
    "forward 1",
    "up 4",
    "down 2",
    "forward 8",
    "forward 6",
    "down 2",
    "down 9",
    "down 6",
    "down 4",
    "forward 5",
    "down 5",
    "down 7",
    "down 6",
    "forward 8",
    "down 8",
    "down 2",
    "up 2",
    "up 2",
    "down 3",
    "forward 2",
    "down 2",
    "down 5",
    "down 3",
    "up 5",
    "down 8",
    "forward 8",
    "down 8",
    "down 4",
    "down 3",
    "forward 7",
    "forward 1",
    "forward 1",
    "down 7",
    "down 6",
    "down 2",
    "up 9",
    "up 7",
    "down 9",
    "forward 1",
    "down 3",
    "down 4",
    "down 7",
    "forward 6",
    "down 8",
    "forward 3",
    "down 6",
    "forward 4",
    "down 3",
    "down 5",
    "down 4",
    "forward 4",
    "up 4",
    "up 1",
    "up 2",
    "down 3",
    "forward 6",
    "up 6",
    "forward 8",
    "forward 9",
    "forward 3",
    "forward 4",
    "forward 2",
    "forward 8",
    "forward 7",
    "up 4",
    "down 5",
    "forward 8",
    "forward 6",
    "down 2",
    "forward 3",
    "down 5",
    "down 8",
    "forward 3",
    "forward 4",
    "down 3",
    "down 9",
    "down 6",
    "up 6",
    "down 1",
    "down 8",
    "forward 7",
    "down 3",
    "forward 8",
    "forward 4",
    "down 2",
    "up 7",
    "down 5",
    "forward 1",
    "forward 7",
    "forward 1",
    "forward 3",
    "down 8",
    "down 7",
    "forward 5",
    "forward 3",
    "down 1",
    "forward 7",
    "down 4",
    "down 3",
    "down 6",
    "down 9",
    "forward 6",
    "down 1",
    "forward 3",
    "forward 3",
    "forward 5",
    "forward 9",
    "up 3",
    "up 6",
    "forward 7",
    "up 5",
    "up 9",
    "down 2",
    "down 5",
    "up 5",
    "forward 4",
    "forward 4",
    "forward 6",
    "up 2",
    "down 9",
    "down 4",
    "down 2",
    "forward 7",
    "down 3",
    "up 4",
    "up 6",
    "forward 7",
    "forward 1",
    "forward 1",
    "down 7",
    "forward 5",
    "forward 4",
    "up 6",
    "forward 4",
    "forward 2",
    "forward 6",
    "up 1",
    "up 5",
    "forward 8",
    "up 2",
    "forward 3",
    "forward 5",
    "up 9",
    "down 4",
    "forward 1",
    "up 7",
    "down 5",
    "forward 7",
    "forward 2",
    "forward 8",
    "down 8",
    "down 2",
    "forward 2",
    "forward 4",
    "forward 8",
    "forward 1",
    "forward 8",
    "forward 2",
    "down 9",
    "forward 7",
    "down 7",
    "down 5",
    "up 9",
    "forward 5",
    "down 1",
    "down 9",
    "down 2",
    "forward 6",
    "down 8",
    "up 9",
    "forward 5",
    "down 2",
    "forward 1",
    "up 4",
    "forward 1",
    "down 9",
    "up 3",
    "down 3",
    "down 2",
    "forward 9",
    "down 5",
    "forward 4",
    "down 1",
    "forward 4",
    "down 8",
    "down 3",
    "forward 8",
    "forward 2",
    "forward 3",
    "down 1",
    "forward 1",
    "down 2",
    "forward 6",
    "up 3",
    "up 5",
    "up 9",
    "forward 9",
    "down 5",
    "down 5",
    "forward 4",
    "up 6",
    "down 7",
    "down 2",
    "forward 8",
    "forward 6",
    "forward 6",
    "up 6",
    "down 7",
    "forward 8",
];
