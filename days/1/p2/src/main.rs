use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    const FILENAME: &str = "input";

    let file = File::open(FILENAME)?;
    let reader = BufReader::new(file);
    let measurements = reader
        .lines()
        .flat_map(|depth| depth.unwrap().parse::<u32>())
        .collect::<Vec<u32>>();

    let windows = measurements.windows(3);

    let mut measures: u32 = 0;
    let mut p_window: &[u32] = &[];
    for (i, window) in windows.enumerate() {
        let p_window_sum: u32 = p_window.iter().sum();
        let window_sum: u32 = window.iter().sum();
        if window_sum > p_window_sum && i > 0 {
            measures += 1;
        }
        p_window = &window;
    }

    println!("{}", measures);
    Ok(())
}
