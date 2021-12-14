use std::fs::read_to_string;

fn main() {
    const FILENAME: &str = "input";

    let reader = read_to_string(FILENAME).expect("file not found!");

    let gamma_rate_b = reader
        .lines()
        .fold(vec![0; 12], |mut acc, diagnostic| {
            for (i, c) in diagnostic.chars().enumerate() {
                acc[i] += if c == '1' { 1 } else { -1 };
            }
            acc
        })
        .into_iter()
        .map(|gr| if gr > 0 { "1" } else { "0" })
        .collect::<Vec<&str>>()
        .join("");

    let epsilon_rate_b = gamma_rate_b
        .chars()
        .map(|er| if er == '1' { '0' } else { '1' })
        .collect::<String>();

    let gamma_rate = usize::from_str_radix(&gamma_rate_b, 2).unwrap();
    let epsilon_rate = usize::from_str_radix(&epsilon_rate_b, 2).unwrap();

    println!("g_r: {}", gamma_rate);
    println!("e_r: {}", epsilon_rate);
    println!("p_c: {}", gamma_rate * epsilon_rate);
}
