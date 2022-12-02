use std::{
    env,
    fs::File,
    io::{BufReader, Read},
};

pub fn read_input() -> String {
    let input_path = env::var("INPUT_PATH").expect("INPUT_PATH not defined");
    let file = File::open(&input_path).expect(&format!("Input file {} not found", input_path));

    let mut reader = BufReader::new(file);
    let mut input = String::new();

    reader
        .read_to_string(&mut input)
        .expect("Input file was empty");

    input
}
