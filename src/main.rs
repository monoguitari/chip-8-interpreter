use std::env;

use chip_8_interpreter::Chip8;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    //let file = fs::read_to_string(file_path);
    
    let mut interpreter = Chip8::new(); 

    //println!("{}", file.unwrap());
    _ = interpreter.load_rom(file_path);
}
