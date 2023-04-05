use std::fs;

pub fn simulate(file_name: &str) {
    let bytes = &fs::read(file_name).unwrap();
    let bytes_len = bytes.len();
    let mut current: usize = 0;

    // let state

    // let mut output: String = Default::default();

    // output.push_str("bits 16\n\n");

    // println!("Simulator started with {}", file_name);
}
