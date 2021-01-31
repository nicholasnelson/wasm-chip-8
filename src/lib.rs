use std::cmp;

#[macro_use]
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const PIXEL_ON: [u8; 3] = [102, 255, 102];
const PIXEL_OFF: [u8; 3] = [0, 0, 0];

#[wasm_bindgen]
#[repr(C)]
pub struct CPU {
    memory: [u8; 4096], // RAM
    gpr: [u8; 16usize], // GP registers 0x0 through 0xF
    stack: [u16; 16], // The stack
    i: u16, // I register
    pc: u16, // Program Counter
    sp: u8, // Stack Pointer
    dt: u8, // Delay Timer
    st: u8, // Sound Timer
    display: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT * 3], // Display memory
    keyboard: u16, // Keyboard memory
    last_tick_time: u64, // Unix time of the last tick
}

#[wasm_bindgen]
impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory: [0u8; 4096],
            gpr: [0u8; 16],
            stack: [0u16; 16],
            i: 0u16,
            pc: 0x200u16,
            sp: 0u8,
            dt: 0u8,
            st: 0u8,
            display: [100u8; DISPLAY_WIDTH * DISPLAY_HEIGHT * 3],
            keyboard: 0u16,
            last_tick_time: 0u64,
        }
    }

    pub fn get_display_pointer(&self) -> *const u8 {
        self.display.as_ptr()
    }

    pub fn get_memory_pointer(&self) -> *const u8 {
        self.memory.as_ptr()
    }

    pub fn get_stack_pointer(&self) -> *const u16 {
        self.stack.as_ptr()
    }

    pub fn get_gpr_pointer(&self) -> *const u8 {
        self.gpr.as_ptr()
    }

    pub fn init_hex_sprites(&mut self) {
        let hex_sprites = [
            0xF0,0x90,0x90,0x90,0xF0, // 0
            0x20,0x60,0x20,0x20,0x70, // 1
            0xF0,0x10,0xF0,0x80,0xF0, // 2
            0xF0,0x10,0xF0,0x10,0xF0, // 3
            0x90,0x90,0xF0,0x10,0x10, // 4
            0xF0,0x80,0xF0,0x10,0xF0, // 5
            0xF0,0x80,0xF0,0x90,0xF0, // 6
            0xF0,0x10,0x20,0x40,0x40, // 7
            0xF0,0x90,0xF0,0x90,0xF0, // 8
            0xF0,0x90,0xF0,0x10,0xF0, // 9
            0xF0,0x90,0xF0,0x90,0x90, // A
            0xE0,0x90,0xE0,0x90,0xE0, // B
            0xF0,0x80,0x80,0x80,0xF0, // C
            0xE0,0x90,0x90,0x90,0xE0, // D
            0xF0,0x80,0xF0,0x80,0xF0, // E
            0xF0,0x80,0xF0,0x80,0x80, // F
        ];
        let target_slice = &mut self.memory[0x000..hex_sprites.len()];
        target_slice.clone_from_slice(&hex_sprites);
    }

    pub fn load_program_memory(&mut self, memory: Vec<u8>) {
        let target_slice = &mut self.memory[self.pc as usize..self.pc as usize + memory.len()];
        target_slice.clone_from_slice(memory.as_slice());
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn get_dt(&self) -> u8 {
        self.dt
    }

    pub fn get_st(&self) -> u8 {
        self.st
    }

    pub fn get_i(&self) -> u16 {
        self.i
    }

    pub fn get_sp(&self) -> u8 {
        self.sp
    }

    pub fn set_key_down(&mut self, key_code: u32) {
        self.keyboard |= 0b000001 << key_code;
    }

    pub fn set_key_up(&mut self, key_code: u32) {
        self.keyboard &= !(0b000001 << key_code);
    }

    pub fn reset(&mut self) {
        self.memory.iter_mut().for_each(|m| *m = 0);
        self.gpr.iter_mut().for_each(|m| *m = 0);
        self.stack.iter_mut().for_each(|m| *m = 0);
        self.i = 0;
        self.pc = 0x200;
        self.sp = 0;
        self.dt = 0;
        self.st = 0;
        self.display.iter_mut().for_each(|m| *m = 0);
        self.keyboard = 0;    
    }
        
    // Decrement timers at ~60Hz based on provide unix time
    pub fn tick_timers(&mut self, time: u64) {
        let decrement = (time - self.last_tick_time) / 17;
        self.last_tick_time = time - (time - self.last_tick_time) % 17;
        self.dt -= cmp::min(self.dt as u64, decrement) as u8;
        self.st -= cmp::min(self.st as u64, decrement) as u8;
    }
        
    pub fn tick(&mut self) {
        // Get the 4 nibbles of the instruction, most significant first
        let instruction_nibbles = [
            self.memory[self.pc as usize] >> 4,
            self.memory[self.pc as usize] & 0x0F,
            self.memory[self.pc as usize + 1] >> 4,
            self.memory[self.pc as usize + 1] & 0x0F,
        ];
        // Now that we have the instruction, increment PC
        self.pc += 2;
        // Execute the current instruction
        match instruction_nibbles {
            [0x0, 0x0, 0xE, 0x0] => self.instruction_cls(),
            [0x0, 0x0, 0xE, 0xE] => self.instruction_ret(),
            [0x1, n1, n2, n3] => self.instruction_jp(n1, n2, n3),
            [0x2, n1, n2, n3] => self.instruction_call(n1, n2, n3),
            [0x3, n1, n2, n3] => self.instruction_se_byte(n1, n2, n3),
            [0x4, n1, n2, n3] => self.instruction_sne_byte(n1, n2, n3),
            [0x5, n1, n2, 0x0] => self.instruction_se_gpr(n1, n2),
            [0x6, n1, n2, n3] => self.instruction_ld_byte(n1, n2, n3),
            [0x7, n1, n2, n3] => self.instruction_add_byte(n1, n2, n3),
            [0x8, n1, n2, 0x0] => self.instruction_ld_gpr(n1, n2),
            [0x8, n1, n2, 0x1] => self.instruction_or_gpr(n1, n2),
            [0x8, n1, n2, 0x2] => self.instruction_and_gpr(n1, n2),
            [0x8, n1, n2, 0x3] => self.instruction_xor_gpr(n1, n2),
            [0x8, n1, n2, 0x4] => self.instruction_add_gpr(n1, n2),
            [0x8, n1, n2, 0x5] => self.instruction_sub_gpr(n1, n2),
            [0x8, n1, _, 0x6] => self.instruction_shr_gpr(n1),
            [0x8, n1, n2, 0x7] => self.instruction_subn_gpr(n1, n2),
            [0x8, n1, _, 0xE] => self.instruction_shl_gpr(n1),
            [0x9, n1, n2, 0x0] => self.instruction_sne_gpr(n1, n2),
            [0xA, n1, n2, n3] => self.instruction_ldi(n1, n2, n3),
            [0xB, n1, n2, n3] => self.instruction_jpv0(n1, n2, n3),
            [0xC, n1, n2, n3] => self.instruction_rnd(n1, n2, n3),
            [0xD, n1, n2, n3] => self.instruction_drw(n1, n2, n3),
            [0xE, n1, 0x9, 0xE] => self.instruction_skp(n1),
            [0xE, n1, 0xA, 0x1] => self.instruction_sknp(n1),
            [0xF, n1, 0x0, 0x7] => self.instruction_ld_gpr_dt(n1),
            [0xF, n1, 0x0, 0xA] => self.instruction_ld_kp(n1),
            [0xF, n1, 0x1, 0x5] => self.instruction_ld_dt_gpr(n1),
            [0xF, n1, 0x1, 0x8] => self.instruction_ld_st_gpr(n1),
            [0xF, n1, 0x1, 0xE] => self.instruction_add_i_gpr(n1),
            [0xF, n1, 0x2, 0x9] => self.instruction_ld_i_font(n1),
            [0xF, n1, 0x3, 0x3] => self.instruction_bcd(n1),
            [0xF, n1, 0x5, 0x5] => self.instruction_ld_i_vx(n1),
            [0xF, n1, 0x6, 0x5] => self.instruction_ld_vx_i(n1),
            _ => self.reset(),
        };
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl CPU {
    pub fn set_memory(&mut self, new_memory: &[u8]) {
        self.memory = [0u8; 4096];
        let target_slice = &mut self.memory[0x200..0x200 + new_memory.len()];
        target_slice.clone_from_slice(new_memory);
    }

    pub fn set_display(&mut self, new_display: &[u8]) {
        let target_slice = &mut self.display[..new_display.len()];
        target_slice.clone_from_slice(new_display);
    }

    // Set the stack values and pad with 0s, set the stack pointer accordingly
    pub fn set_stack(&mut self, new_stack: &[u16]) {
        self.stack = [0u16; 16];
        let target_slice = &mut self.stack[0..new_stack.len()];
        target_slice.clone_from_slice(new_stack);
        self.sp = new_stack.len() as u8;
    }

    // Set the register contents and pad with 0s
    pub fn set_registers(&mut self, new_registers: &[u8]) {
        self.gpr = [0u8; 16];
        let target_slice = &mut self.gpr[0..new_registers.len()];
        target_slice.clone_from_slice(new_registers);
    }

    pub fn set_i(&mut self, i: u16) {
        self.i = i;
    }

    pub fn set_keyboard(&mut self, keyboard: u16) {
        self.keyboard = keyboard;
    }

    pub fn set_dt(&mut self, dt: u8) {
        self.dt = dt;
    }

    pub fn get_stack(&self) -> [u16; 16] {
        self.stack
    }

    pub fn get_registers(&self) -> [u8; 16] {
        self.gpr
    }

    pub fn get_display(&self) -> [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT * 3] {
        self.display
    }

    pub fn get_memory(&self) -> [u8; 4096] {
        self.memory
    }

    // 00E0 - CLS
    // Clear the display.
    fn instruction_cls(&mut self) {
        self.display.clone_from_slice(
            &[0x0; DISPLAY_WIDTH * DISPLAY_HEIGHT * 3]);
    }

    // 00EE - RET
    // Return from a subroutine.
    //
    // The interpreter sets the program counter to the address at the top of the
    // stack, then subtracts 1 from the stack pointer.
    fn instruction_ret(&mut self) {
        if self.sp == 0 {
            panic!("Stack underflow!");
        }
        self.sp -= 1;
        self.pc =  self.stack[self.sp as usize];
    }

    // 1nnn - JP addr
    // Jump to location nnn.
    //
    // The interpreter sets the program counter to nnn.
    fn instruction_jp(&mut self, n1: u8, n2: u8, n3: u8) {
        self.pc = ((n1 as u16) << 8) | ((n2 as u16) << 4) | (n3 as u16);
    }

    // 2nnn - CALL addr
    // Call subroutine at nnn.
    //
    // The interpreter increments the stack pointer, then puts the current PC on
    // the top of the stack. The PC is then set to nnn.
    fn instruction_call(&mut self, n1: u8, n2: u8, n3: u8) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = ((n1 as u16) << 8) | ((n2 as u16) << 4) | (n3 as u16);
    }

    // 3xkk - SE Vx, byte
    // Skip next instruction if Vx = kk.
    //
    // The interpreter compares register Vx to kk, and if they are equal,
    // increments the program counter by 2.
    fn instruction_se_byte(&mut self, n1: u8, n2: u8, n3: u8) {
        if self.gpr[n1 as usize] == n2 << 4 | n3 {
            self.pc += 2;
        }
    }

    // 4xkk - SNE Vx, byte
    // Skip next instruction if Vx != kk.
    //
    // The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    fn instruction_sne_byte(&mut self, n1: u8, n2: u8, n3: u8) {
        if self.gpr[n1 as usize] != n2 << 4 | n3 {
            self.pc += 2;
        }
    }

    // 5xy0 - SE Vx, Vy
    // Skip next instruction if Vx = Vy.
    //
    // The interpreter compares register Vx to register Vy, and if they are
    // equal, increments the program counter by 2.
    fn instruction_se_gpr(&mut self, n1: u8, n2: u8) {
        if self.gpr[n1 as usize] == self.gpr[n2 as usize] {
            self.pc += 2;
        }
    }

    // 6xkk - LD Vx, byte
    // Set Vx = kk.
    //
    // The interpreter puts the value kk into register Vx.
    fn instruction_ld_byte(&mut self, n1: u8, n2: u8, n3: u8) {
        self.gpr[n1 as usize] = n2 << 4 | n3;
    }

    // 7xkk - ADD Vx, byte
    // Set Vx = Vx + kk.
    //
    // Adds the value kk to the value of register Vx, then stores the result in
    // Vx.
    fn instruction_add_byte(&mut self, n1: u8, n2: u8, n3: u8) {
        self.gpr[n1 as usize] += n2 << 4 | n3;
    }

    // 8xy0 - LD Vx, Vy
    // Set Vx = Vy.
    //
    // Stores the value of register Vy in register Vx.
    fn instruction_ld_gpr(&mut self, n1: u8, n2: u8) {
        self.gpr[n1 as usize] = self.gpr[n2 as usize];
    }

    // 8xy1 - OR Vx, Vy
    // Set Vx = Vx OR Vy.
    //
    // Performs a bitwise OR on the values of Vx and Vy, then stores the result
    // in Vx. A bitwise OR compares the corrseponding bits from two values, and
    // if either bit is 1, then the same bit in the result is also 1. Otherwise,
    // it is 0.
    fn instruction_or_gpr(&mut self, n1: u8, n2: u8) {
        self.gpr[n1 as usize] |= self.gpr[n2 as usize];
    }


    // 8xy2 - AND Vx, Vy
    // Set Vx = Vx AND Vy.
    //
    // Performs a bitwise AND on the values of Vx and Vy, then stores the result
    // in Vx. A bitwise AND compares the corrseponding bits from two values, and
    // if both bits are 1, then the same bit in the result is also 1.
    // Otherwise, it is 0.
    fn instruction_and_gpr(&mut self, n1: u8, n2: u8) {
        self.gpr[n1 as usize] &= self.gpr[n2 as usize];
    }

    // 8xy3 - XOR Vx, Vy
    // Set Vx = Vx XOR Vy.
    //
    // Performs a bitwise exclusive OR on the values of Vx and Vy, then stores
    // the result in Vx. An exclusive OR compares the corrseponding bits from
    // two values, and if the bits are not both the same, then the corresponding
    // bit in the result is set to 1. Otherwise, it is 0.
    fn instruction_xor_gpr(&mut self, n1: u8, n2: u8) {
        self.gpr[n1 as usize] ^= self.gpr[n2 as usize];
    }

    // 8xy4 - ADD Vx, Vy
    // Set Vx = Vx + Vy, set VF = carry.
    //
    // The values of Vx and Vy are added together. If the result is greater than
    // 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits
    // of the result are kept, and stored in Vx.
    fn instruction_add_gpr(&mut self, n1: u8, n2: u8) {
        let add_result = self.gpr[n1 as usize]
            .overflowing_add(self.gpr[n2 as usize]);
        self.gpr[n1 as usize] = add_result.0;
        self.gpr[0xF] = if add_result.1 { 1 } else { 0 };
    }

    // 8xy5 - SUB Vx, Vy
    // Set Vx = Vx - Vy, set VF = NOT borrow.
    //
    // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from
    // Vx, and the results stored in Vx.
    fn instruction_sub_gpr(&mut self, n1: u8, n2: u8) {
        let sub_result = self.gpr[n1 as usize]
            .overflowing_sub(self.gpr[n2 as usize]);
        self.gpr[n1 as usize] = sub_result.0;
        self.gpr[0xF] = if sub_result.1 { 0 } else { 1 };
    }

    // 8xy6 - SHR Vx {, Vy}
    // Set Vx = Vx SHR 1.
    //
    // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise
    // 0. Then Vx is divided by 2.
    fn instruction_shr_gpr(&mut self, n1: u8) {
        self.gpr[0xF] = self.gpr[n1 as usize] & 0b00000001;
        self.gpr[n1 as usize] >>= 1;
    }

    // 8xy7 - SUBN Vx, Vy
    // Set Vx = Vy - Vx, set VF = NOT borrow.
    //
    // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from
    // Vy, and the results stored in Vx.
    fn instruction_subn_gpr(&mut self, n1: u8, n2: u8) {
        let sub_result = self.gpr[n2 as usize]
            .overflowing_sub(self.gpr[n1 as usize]);
        self.gpr[n1 as usize] = sub_result.0;
        self.gpr[0xF] = if sub_result.1 { 0 } else { 1 };
    }

    // 8xyE - SHL Vx {, Vy}
    // Set Vx = Vx SHL 1.
    //
    // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
    fn instruction_shl_gpr(&mut self, n1: u8) {
        self.gpr[0xF] = if self.gpr[n1 as usize] & 0b10000000 == 0 { 0 } else { 1 };
        self.gpr[n1 as usize] <<= 1;
    }

    // 9xy0 - SNE Vx, Vy
    // Skip next instruction if Vx != Vy.
    //
    // The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
    fn instruction_sne_gpr(&mut self, n1: u8, n2: u8) {
        if self.gpr[n1 as usize] != self.gpr[n2 as usize] {
            self.pc += 2;
        }
    }

    // Annn - LD I, addr
    // Set I = nnn.
    //
    // The value of register I is set to nnn.
    fn instruction_ldi(&mut self, n1: u8, n2: u8, n3: u8) {
        self.i = (n1 as u16) << 8 | (n2 as u16) << 4 | n3 as u16;
    }

    // Bnnn - JP V0, addr
    // Jump to location nnn + V0.
    //
    // The program counter is set to nnn plus the value of V0.
    fn instruction_jpv0(&mut self, n1: u8, n2: u8, n3: u8) {
        self.pc = self.gpr[0] as u16
            + ((n1 as u16) << 8 | (n2 as u16) << 4 | n3 as u16);
    }

    // Cxkk - RND Vx, byte
    // Set Vx = random byte AND kk.
    //
    // The interpreter generates a random number from 0 to 255, which is then
    // ANDed with the value kk. The results are stored in Vx. See instruction
    // 8xy2 for more information on AND.
    fn instruction_rnd(&mut self, n1: u8, n2: u8, n3: u8) {
        self.gpr[n1 as usize] = (js_sys::Math::random() * 256f64).floor() as u8
            & (n2 << 4 | n3);
    }

    // Dxyn - DRW Vx, Vy, nibble
    // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF
    // = collision.
    //
    // The interpreter reads n bytes from memory, starting at the address stored
    // in I. These bytes are then displayed as sprites on screen at coordinates
    // (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any
    // pixels to be erased, VF is set to 1, otherwise it is set to 0. If the
    // sprite is positioned so part of it is outside the coordinates of the
    // display, it wraps around to the opposite side of the screen.
    // See instruction 8xy3 for more information on XOR, and section 2.4,
    // Display, for more information on the Chip-8 screen and sprites.
    fn instruction_drw(&mut self, n1: u8, n2: u8, n3: u8) {
        self.gpr[0xF] = 0;
        let x_origin = self.gpr[n1 as usize] as usize;
        let y_origin = self.gpr[n2 as usize] as usize;
        // Find the sprite
        let sprite_start = self.i as usize;
        let sprite_end = sprite_start + n3 as usize;
        let sprite = self.memory[sprite_start..sprite_end].iter().enumerate();
        // Draw the sprite
        for (y, value) in sprite {
            for x in 0..8 {
                // If the value is 0, we don't need to do anything
                if value & 0b10000000 >> x == 0 { continue }
                // Get the (possibly wrapped) coords
                let x_pos = (x_origin + x) % DISPLAY_WIDTH;
                let y_pos = (y_origin + y) % DISPLAY_HEIGHT;
                // Get the index of the pixel
                let pixel_index = (y_pos * DISPLAY_WIDTH + x_pos) * 3;
                let pixel = &mut self.display[pixel_index..pixel_index + 3];
                // If the value of the targeted pixel is not zero set off + VF
                if pixel[0] == 0 {
                    pixel.clone_from_slice(&PIXEL_ON);
                } else {
                    self.gpr[0xF] = 1;
                    pixel.clone_from_slice(&PIXEL_OFF);
                }
            }
        }
    }

    // Ex9E - SKP Vx
    // Skip next instruction if key with the value of Vx is pressed.
    //
    // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
    fn instruction_skp(&mut self, n1: u8) {
        if 1u16 << self.gpr[n1 as usize] & self.keyboard > 0 {
            self.pc += 2;
        }
    }

    // ExA1 - SKNP Vx
    // Skip next instruction if key with the value of Vx is not pressed.
    //
    // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
    fn instruction_sknp(&mut self, n1: u8) {
        if 1u16 << self.gpr[n1 as usize] & self.keyboard == 0 {
            self.pc += 2;
        }
    }

    // Fx07 - LD Vx, DT
    // Set Vx = delay timer value.
    //
    // The value of DT is placed into Vx.
    fn instruction_ld_gpr_dt(&mut self, n1: u8) {
        self.gpr[n1 as usize] = self.dt;
    }


    // Fx0A - LD Vx, K
    // Wait for a key press, store the value of the key in Vx.
    //
    // All execution stops until a key is pressed, then the value of that key is stored in Vx.
    fn instruction_ld_kp(&mut self, n1: u8) {
        if self.keyboard == 0 {
            self.pc -= 2;
        } else {
            for i in 0..16 {
                if (self.keyboard >> i) & 0x1 == 0x1 {
                    self.gpr[n1 as usize] = i;
                    break;
                }
            }
        }
    }

    // Fx15 - LD DT, Vx
    // Set delay timer = Vx.
    //
    // DT is set equal to the value of Vx.
    fn instruction_ld_dt_gpr(&mut self, n1: u8) {
        self.dt = self.gpr[n1 as usize];
    }


    // Fx18 - LD ST, Vx
    // Set sound timer = Vx.
    //
    // ST is set equal to the value of Vx.
    fn instruction_ld_st_gpr(&mut self, n1: u8) {
        self.st = self.gpr[n1 as usize];
    }

    // Fx1E - ADD I, Vx
    // Set I = I + Vx.
    //
    // The values of I and Vx are added, and the results are stored in I.
    fn instruction_add_i_gpr(&mut self, n1: u8) {
        self.i += self.gpr[n1 as usize] as u16;
    }


    // Fx29 - LD F, Vx
    // Set I = location of sprite for digit Vx.
    //
    // The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
    fn instruction_ld_i_font(&mut self, n1: u8) {
        self.i = self.gpr[n1 as usize] as u16 * 5;
    }

    // Fx33 - LD B, Vx
    // Store BCD representation of Vx in memory locations I, I+1, and I+2.
    //
    // The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
    fn instruction_bcd(&mut self, n1: u8) {
        let value = self.gpr[n1 as usize];
        let output_address = self.i as usize;
        self.memory[output_address] = value / 100;
        self.memory[output_address + 1] = value % 100 / 10;
        self.memory[output_address + 2] = value % 10;
    }

    // Fx55 - LD [I], Vx
    // Store registers V0 through Vx in memory starting at location I.
    //
    // The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
    fn instruction_ld_i_vx(&mut self, n1: u8) {
        for register_index in 0..=n1 as usize {
            self.memory[self.i as usize + register_index] =
                self.gpr[register_index as usize];
        }
    }

    // Fx65 - LD Vx, [I]
    // Read registers V0 through Vx from memory starting at location I.
    //
    // The interpreter reads values from memory starting at location I into registers V0 through Vx.
    fn instruction_ld_vx_i(&mut self, n1: u8) {
        for register_index in 0..=n1 as usize {
            self.gpr[register_index] =
                self.memory[self.i as usize + register_index];
        }
    }
}