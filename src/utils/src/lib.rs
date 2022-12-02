use std::{env, fs::read_to_string, path::Path};

pub fn read_input() -> String {
    let cargo_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let input_path = Path::new(&cargo_dir).join("input.txt");
    read_to_string(input_path).unwrap()
}
