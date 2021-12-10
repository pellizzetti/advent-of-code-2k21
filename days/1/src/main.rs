use std::fs::read_to_string;

fn main() {
    const FILENAME: &str = "input";

    let mut measures = -1;
    read_to_string(FILENAME)
        .expect("file not found!")
        .lines()
        .flat_map(|depth| depth.parse())
        .fold(0, |p_depth, depth| {
            if depth > p_depth {
                measures += 1;
            }
            depth
        });

    println!("{}", measures);
}
