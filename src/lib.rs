use std::{
    env,
    fs::File,
    io::{self, BufReader, Read},
};

pub fn read_input() -> io::Result<String> {
    let input_path = env::var("INPUT_PATH").expect("INPUT_PATH not defined");
    let file = File::open(&input_path).expect(&format!("Input file {} not found", input_path));

    let mut reader = BufReader::new(file);
    let mut input = String::new();

    reader.read_to_string(&mut input)?;

    Ok(input)
}
