use std::fs::File;
use std::io::{self, Error, Read};

// NOTE for self: u8 = 1byte, u16 = 2bytes
// TODO: assign scancodes for keypress

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const RAM_SIZE: usize = 4096; // 0x1FF = 4096 bytes
const VARIABLE_REGISTER_SIZE: usize = 16;
const STACK_SIZE: usize = 16; // OG interpreter holds 16 2-byte entries (we have 2 16 bytes)
const KEYPAD_SIZE: usize = 16; // 4x4 keypad
const ROM_START: usize = 0x200;
const FONT_TABLE: [u8; 80] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[derive(Debug)]
pub struct Chip8 {
    pub memory: [u8; RAM_SIZE],
    pub display: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    pub program_counter: u16,
    pub index_register: u16,
    pub stack: [u16; STACK_SIZE], // u16 = 2 bytes
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub variable_registers: [u8; VARIABLE_REGISTER_SIZE],
    pub keypad: [bool; KEYPAD_SIZE],
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Chip8 {
            memory: [0; RAM_SIZE],
            display: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            program_counter: 0x200,
            index_register: 0,
            stack: [0; STACK_SIZE],
            delay_timer: 0,
            sound_timer: 0,
            variable_registers: [0; VARIABLE_REGISTER_SIZE],
            keypad: [false; KEYPAD_SIZE],
        };
        chip8.load_font_table();
        chip8
    }

    pub fn load_font_table(&mut self) {
        const FONT_START_ADDRESS: usize = 0x050;
        const FONT_END_ADDRESS: usize = 0x0A0; // NOTE: want up to 0x09F, but end index is exclusive, so
                                               // we make it 0x0A0

        // Ensure the font table fits within the specified range
        assert_eq!(FONT_TABLE.len(), FONT_END_ADDRESS - FONT_START_ADDRESS);

        self.memory[FONT_START_ADDRESS..FONT_END_ADDRESS].copy_from_slice(&FONT_TABLE);

        println!("Font table loaded into memory!");
    }

    pub fn load_rom(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?; // Open the ROM file

        // Read the file into a buffer
        let mut file_buffer = Vec::new();
        file.read_to_end(&mut file_buffer)?;

        // Define the memory range for loading the ROM
        let start_index = ROM_START; // CHIP-8 programs start at 0x200 (512)
        let end_index = (start_index + file_buffer.len()).min(RAM_SIZE); // Prevent overflow

        assert!(end_index <= RAM_SIZE, "End index exceeds memory size!");
        assert!(
            end_index - start_index == file_buffer.len().min(RAM_SIZE - start_index),
            "Mismatch between memory range and ROM size!"
        );

        if file_buffer.len() > RAM_SIZE - start_index {
            return Err(Box::new(Error::new(
                io::ErrorKind::InvalidData,
                "ROM size exceeds available memory",
            )));
        }

        // Copy ROM data into memory
        self.memory[start_index..end_index]
            .copy_from_slice(&file_buffer[..(end_index - start_index)]);

        println!("ROM loaded into memory!");
        //println!("Memory: {:?}", self.memory); //debugging statement for memory at this point
        Ok(())
    }

    pub fn run(&mut self) {
        loop {
            let instruction = self.fetch();
            self.decode(instruction);
            //println!("Instruction: {}", instruction);
            //break;
        }
    }

    // Uses program counter to retreive next instruction
    fn fetch(&mut self) -> String {
        // TODO: fetch the next instruction from memory
        let high_byte = self.memory[self.program_counter as usize] as u16;
        let low_byte = self.memory[(self.program_counter + 1) as usize] as u16;

        // Combine the two bytes into a single 16-bit instruction
        let instruction = (high_byte << 8) | low_byte;
        self.program_counter += 2;
        format!("{:04X}", instruction)
    }

    fn decode(&mut self, instruction: String) {
        let opcode = u16::from_str_radix(&instruction, 16).unwrap();
        //println!("Instruction: {:?}", instruction);
        let first_nibble = (opcode & 0xF000) >> 12; 
        let x = ((opcode & 0x0F00) >> 8) as usize; // Second nibble
        let y = ((opcode & 0x00F0) >> 4) as usize; // Third nibble
        let n = (opcode & 0x000F) as u8; // Fourth nibble
        let nn = (opcode & 0x00FF) as u8; // Last byte
        let nnn = opcode & 0x0FFF;

        match first_nibble {
            0x0 => {
                println!("Sys Instruction")
            },
            0x1 => {
                println!("Jump instruction")
            },
            0x6 => {
                println!("set VX instructoin")
            },
            0xD => {
                println!("Draw instructoin")
            },
            _ => {
                println!("Unknown instruction")
            }
        }
        // TODO: decode instruction based on nub
    }
}

impl Default for Chip8 {
    /// Provides a default implementation for the CHIP-8 interpreter.
    fn default() -> Self {
        Self::new()
    }
}
