use std::fs::read_to_string;

fn main() {
    const FILENAME: &str = "input";

    let reader = read_to_string(FILENAME).expect("file not found!");

    let position = reader.lines().fold((0, 0), |mut acc, command| {
        let digit_index = command.find(|c: char| c.is_digit(10)).unwrap();
        let position = usize::from_str_radix(command.get(digit_index..).unwrap(), 10).unwrap();
        let command_type = command.get(..digit_index - 1).unwrap();

        match command_type {
            "forward" => acc.0 += position,
            "down" => acc.1 += position,
            _ => acc.1 -= position,
        };

        acc
    });

    println!("{}", position.0 * position.1);
}
