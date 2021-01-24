//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate chip_8_emu;
use chip_8_emu::CPU;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn hello_world() {
    let mut cpu = CPU::new();
    cpu.set_memory(&[
        0x61, 0x01, // Set v1 to 1
        0x62, 0x02, // Set v2 to 2
        0x63, 0x03, // Set v3 to 3
        0x64, 0x0F, // Set v3 to 3
        0xF1, 0x29, // Load address for sprite "1" to I
        0xD0, 0x15, // Draw "1" to v0,v1
        0xF2, 0x29, // Load address for sprite "2" to I
        0x60, 0x06, // Set v0 to 6
        0xD0, 0x15, // Draw "2" to v0,v1
        0xF3, 0x29, // Load address for sprite "3" to I
        0x60, 0x0C, // Set v0 to 12
        0xD0, 0x15, // Draw "1" to v0,v1
        0xF4, 0x29, // Load address for sprite "4" to I
        0x61, 0x07, // Set v1 to 7
        0xD0, 0x15, // Draw "4" to v0,v1
    ]);
    cpu.init_hex_sprites();
    for i in 0..15 {
        cpu.tick();
    }
    //panic!("{:?}", cpu.get_display());
    cpu.log_ascii_display();
}