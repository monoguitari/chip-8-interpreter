use std::fs::File;
use std::io::{self, Read, Error};

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const RAM_SIZE: usize = 4096;
const VARIABLE_REGISTER_SIZE: usize = 16;
const STACK_SIZE: usize = 2; // OG interpreter holds 16 2-byte entries (we have 2 16 bytes)
const KEYPAD_SIZE: usize = 16; // 4x4 keypad

const _FONT_TABLE: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

#[derive(Debug)]
pub struct Chip8 {
    pub memory: [u8; RAM_SIZE],
    pub display: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    pub program_counter: u16,
    pub index_register: u16,
    pub stack: [u16; STACK_SIZE], // will be list of u8
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub variable_registers: [u8; VARIABLE_REGISTER_SIZE],
    pub keypad: [bool; KEYPAD_SIZE],
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            memory: [0; RAM_SIZE],
            display: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            program_counter: 0x200,
            index_register: 0,
            stack: [0; STACK_SIZE],
            delay_timer: 0,
            sound_timer: 0,
            variable_registers: [0; VARIABLE_REGISTER_SIZE],
            keypad: [false; KEYPAD_SIZE],

        }
    }

    pub fn load_rom(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?; // Open the ROM file

        // Read the file into a buffer
        let mut file_buffer = Vec::new();
        file.read_to_end(&mut file_buffer)?;

        // Define the memory range for loading the ROM
        let start_index = 512; // CHIP-8 programs start at 0x200 (512)
        let end_index = (start_index + file_buffer.len()).min(4096); // Prevent overflow

        if file_buffer.len() > 4096 - start_index {
            return Err(Box::new(Error::new(
                io::ErrorKind::InvalidData,
                "ROM size exceeds available memory",
            )));
        }

        // Copy ROM data into memory
        self.memory[start_index..end_index].copy_from_slice(&file_buffer[..(end_index - start_index)]);

        println!("ROM loaded into memory!");
        //println!("Memory: {:?}", self.memory); //debugging statement for memory at this point

        Ok(())
    }
}

impl Default for Chip8 {
    /// Provides a default implementation for the CHIP-8 interpreter.
    fn default() -> Self {
        Self::new()
    }
}
