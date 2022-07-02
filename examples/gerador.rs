use rand::distributions::{Distribution, Uniform};
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let between_width = Uniform::from(0..42);
    let between_height = Uniform::from(0..42);
    let mut rng = rand::thread_rng();
    let mut idx = 1;

    while Path::new(&("inputs/tools_".to_owned() + &idx.to_string())).exists() {
        idx += 1;
    }

    let mut tools_input =
        (fs::File::create(&("inputs/tools_".to_owned() + &idx.to_string()))).unwrap();
    let mut factories_input =
        (fs::File::create(&("inputs/factories_".to_owned() + &idx.to_string()))).unwrap();
    let mut agent_input =
        (fs::File::create(&("inputs/agent_".to_owned() + &idx.to_string()))).unwrap();

    let mut count = 0;
    while count < 5000 {
        let x = between_width.sample(&mut rng);
        let y = between_height.sample(&mut rng);
        writeln!(tools_input, "{} {}", x, y).unwrap();
        count += 1;
    }
    count = 0;
    while count < 1000 {
        let x = between_width.sample(&mut rng);
        let y = between_height.sample(&mut rng);
        writeln!(factories_input, "{} {}", x, y).unwrap();
        count += 1;
    }

    count = 0;
    while count < 50 {
        let x = between_width.sample(&mut rng);
        let y = between_height.sample(&mut rng);
        writeln!(agent_input, "{} {}", x, y).unwrap();
        count += 1;
    }
}
