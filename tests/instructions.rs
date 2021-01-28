//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate chip_8_emu;
use chip_8_emu::CPU;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn instruction_ret() {
    let mut cpu = CPU::new();
    // Set the RET instruction in memory
    cpu.set_memory(&[0x00, 0xEE]);
    // Add a value to the stack
    cpu.set_stack(&[0x400]);
    cpu.tick();
    assert_eq!(cpu.get_pc(), 0x400);
    assert_eq!(cpu.get_sp(), 0);
}

#[wasm_bindgen_test]
fn instruction_jp() {
    let mut cpu = CPU::new();
    // Set the JP instruction in memory targeting 0xABC
    cpu.set_memory(&[0x1A, 0xBC]);
    cpu.tick();
    assert_eq!(cpu.get_pc(), 0xABC);
}

#[wasm_bindgen_test]
fn instruction_call() {
    let mut cpu = CPU::new();
    // Store the current PC value to validate the stack entry
    let original_pc = cpu.get_pc();
    // Set the CALL instruction in memory targeting 0xABC
    cpu.set_memory(&[0x2A, 0xBC]);
    cpu.tick();
    // PC should equal the target 0xABC
    assert_eq!(cpu.get_pc(), 0xABC);
    // The first value on the stack should be the original PC value + 2
    assert_eq!(cpu.get_stack()[0], original_pc + 2);
}

#[wasm_bindgen_test]
fn instruction_se_vx_byte() {
    { // Skip
        let mut cpu = CPU::new();
        // Store the current PC value to validate the new PC
        let original_pc = cpu.get_pc();
        // Set se instruction, comparing register 1 to the value 0x00
        cpu.set_memory(&[0x31, 0x00]);
        cpu.tick();
        // PC increases by 2 (normal increment) + 2 (from skip instruction)
        assert_eq!(cpu.get_pc(), original_pc + 4);
    }
    { // No Skip
        let mut cpu = CPU::new();
        // Store the current PC value to validate the new PC
        let original_pc = cpu.get_pc();
        // Set se instruction, comparing register 1 to the value 0x00
        cpu.set_memory(&[0x31, 0x01]);
        cpu.tick();
        // PC only increased by 2 (normal increment)
        assert_eq!(cpu.get_pc(), original_pc + 2);
    }
}

#[wasm_bindgen_test]
fn instruction_sne_vx_byte() {
    { // Skip
        let mut cpu = CPU::new();
        // Store the current PC value to validate the new PC
        let original_pc = cpu.get_pc();
        // Set sne instruction, comparing register 1 to the value 0x01
        cpu.set_memory(&[0x41, 0x01]);
        cpu.tick();
        // PC increases by 2 (normal increment) + 2 (from skip instruction)
        assert_eq!(cpu.get_pc(), original_pc + 4);
    }
    { // No Skip
        let mut cpu = CPU::new();
        // Store the current PC value to validate the new PC
        let original_pc = cpu.get_pc();
        // Set sne instruction, comparing register 1 to the value 0x00
        cpu.set_memory(&[0x41, 0x00]);
        cpu.tick();
        // PC only increased by 2 (normal increment)
        assert_eq!(cpu.get_pc(), original_pc + 2);
    }
}

#[wasm_bindgen_test]
fn instruction_se_vx_vy() {
    { // Skip
        let mut cpu = CPU::new();
        // Store the current PC value to validate the new PC
        let original_pc = cpu.get_pc();
        // Set se vxvy instruction, comparing gpr0 to gpr1
        cpu.set_memory(&[0x50, 0x10]);
        cpu.tick();
        // PC increases by 2 (normal increment) + 2 (from skip instruction)
        assert_eq!(cpu.get_pc(), original_pc + 4);
    }
    { // No Skip
        let mut cpu = CPU::new();
        // Store the current PC value to validate the new PC
        let original_pc = cpu.get_pc();
        // Set se vxvy instruction, comparing gpr0 to gpr1
        cpu.set_memory(&[0x50, 0x10]);
        // Set the registers 0 and 1 to have non-matching values
        cpu.set_registers(&[0x0, 0x1]);
        cpu.tick();
        // PC only increased by 2 (normal increment)
        assert_eq!(cpu.get_pc(), original_pc + 2);
    }
}

#[wasm_bindgen_test]
fn instruction_ld_byte() {
    let mut cpu = CPU::new();
    // Set 4x load byte instructions targeting gpr0, gpr1, gpr2, gpr15
    cpu.set_memory(&[
        0x60, 0x00,
        0x61, 0x01,
        0x62, 0x02,
        0x6F, 0x0F
    ]);
    cpu.tick();
    cpu.tick();
    cpu.tick();
    cpu.tick();
    assert_eq!(cpu.get_registers()[0], 0x00);
    assert_eq!(cpu.get_registers()[1], 0x01);
    assert_eq!(cpu.get_registers()[2], 0x02);
    assert_eq!(cpu.get_registers()[15], 0x0F);
}

#[wasm_bindgen_test]
fn instruction_add_byte() {
    let mut cpu = CPU::new();
    cpu.set_memory(&[
        0x70, 0x01, // ADD 0x01 to gpr0
        0x70, 0x10, // ADD 0x10 to gpr0
        0x7F, 0x01, // ADD 0x01 to gpr15
        0x7F, 0x10, // ADD 0x10 to gpr15
    ]);
    assert_eq!(cpu.get_registers()[0], 0x00);
    assert_eq!(cpu.get_registers()[15], 0x00);
    cpu.tick();
    assert_eq!(cpu.get_registers()[0], 0x01);
    assert_eq!(cpu.get_registers()[15], 0x00);
    cpu.tick();
    assert_eq!(cpu.get_registers()[0], 0x11);
    assert_eq!(cpu.get_registers()[15], 0x00);
    cpu.tick();
    assert_eq!(cpu.get_registers()[0], 0x11);
    assert_eq!(cpu.get_registers()[15], 0x01);
    cpu.tick();
    assert_eq!(cpu.get_registers()[0], 0x11);
    assert_eq!(cpu.get_registers()[15], 0x11);
}

#[wasm_bindgen_test]
fn instruction_ld_register() {
    let mut cpu = CPU::new();
    cpu.set_registers(&[0x0, 0xF]);
    cpu.set_memory(&[
        0x80, 0x10, // LD gpr0, gpr1
    ]);
    assert_eq!(cpu.get_registers()[0], 0x00);
    assert_eq!(cpu.get_registers()[1], 0x0F);
    cpu.tick();
    assert_eq!(cpu.get_registers()[0], 0x0F);
    assert_eq!(cpu.get_registers()[1], 0x0F);
}

#[wasm_bindgen_test]
fn instruction_or_register() {
    let mut cpu = CPU::new();
    cpu.set_registers(&[0x00, 0xFF, 0xA0, 0x0A, 0x00]);
    cpu.set_memory(&[
        0x80, 0x11, // OR gpr0, gpr1
        0x82, 0x31, // OR gpr2, grp3
        0x84, 0x41, // OR gpr4, gpr4
    ]);
    cpu.tick();
    cpu.tick();
    cpu.tick();
    assert_eq!(cpu.get_registers()[0], 0xFF);
    assert_eq!(cpu.get_registers()[2], 0xAA);
    assert_eq!(cpu.get_registers()[4], 0x00);
}

#[wasm_bindgen_test]
fn instruction_and_register() {
    let mut cpu = CPU::new();
    cpu.set_registers(&[0x00, 0xFF, 0xA0, 0x0A, 0xFF]);
    cpu.set_memory(&[
        0x80, 0x12, // AND gpr0, gpr1
        0x82, 0x32, // AND gpr2, grp3
        0x84, 0x42, // AND gpr4, gpr4
    ]);
    cpu.tick();
    cpu.tick();
    cpu.tick();
    assert_eq!(cpu.get_registers()[0], 0x00);
    assert_eq!(cpu.get_registers()[2], 0x00);
    assert_eq!(cpu.get_registers()[4], 0xFF);
}

#[wasm_bindgen_test]
fn instruction_xor_register() {
    let mut cpu = CPU::new();
    cpu.set_registers(&[0x00, 0xFF, 0xA0, 0x0A, 0xFF]);
    cpu.set_memory(&[
        0x80, 0x13, // XOR gpr0, gpr1
        0x82, 0x33, // XOR gpr2, grp3
        0x84, 0x43, // XOR gpr4, gpr4
    ]);
    cpu.tick();
    cpu.tick();
    cpu.tick();
    assert_eq!(cpu.get_registers()[0], 0xFF);
    assert_eq!(cpu.get_registers()[2], 0xAA);
    assert_eq!(cpu.get_registers()[4], 0x00);
}

#[wasm_bindgen_test]
fn instruction_add_register() {
    let mut cpu = CPU::new();
    cpu.set_registers(&[0x00, 0xFF, 0xFF, 0xFF, 0xAA, 0x11]);
    cpu.set_memory(&[
        0x80, 0x14, // ADD gpr0, gpr1
        0x82, 0x34, // ADD gpr2, gpr3
        0x84, 0x54, // ADD gpr4, gpr5
    ]);
    cpu.tick();
    assert_eq!(cpu.get_registers()[0], 0xFF);
    assert_eq!(cpu.get_vf(), 0);
    cpu.tick();
    assert_eq!(cpu.get_registers()[2], 0xFE);
    assert_eq!(cpu.get_vf(), 1);
    cpu.tick();
    assert_eq!(cpu.get_registers()[4], 0xBB);
    assert_eq!(cpu.get_vf(), 0);
}

#[wasm_bindgen_test]
fn instruction_sub_register() {
    let mut cpu = CPU::new();
    cpu.set_registers(&[0x00, 0xFF, 0xFF, 0xFF, 0xAA, 0x11]);
    cpu.set_memory(&[
        0x80, 0x15, // SUB gpr0, gpr1
        0x82, 0x35, // SUB gpr2, gpr3
        0x84, 0x55, // SUB gpr4, gpr5
    ]);
    cpu.tick();
    assert_eq!(cpu.get_registers()[0], 0x01);
    assert_eq!(cpu.get_vf(), 0);
    cpu.tick();
    assert_eq!(cpu.get_registers()[2], 0x00);
    assert_eq!(cpu.get_vf(), 1);
    cpu.tick();
    assert_eq!(cpu.get_registers()[4], 0x99);
    assert_eq!(cpu.get_vf(), 1);
}

#[wasm_bindgen_test]
fn instruction_shr_register() {
    let mut cpu = CPU::new();
    cpu.set_registers(&[0x00, 0x01, 0xFF]);
    cpu.set_memory(&[
        0x80, 0x06, // SHR gpr0
        0x81, 0x06, // SHR gpr1
        0x82, 0x06, // SHR gpr2
    ]);
    cpu.tick();
    assert_eq!(cpu.get_registers()[0], 0x00);
    assert_eq!(cpu.get_vf(), 0);
    cpu.tick();
    assert_eq!(cpu.get_registers()[1], 0x00);
    assert_eq!(cpu.get_vf(), 1);
    cpu.tick();
    assert_eq!(cpu.get_registers()[2], 0x7F);
    assert_eq!(cpu.get_vf(), 1);
}

#[wasm_bindgen_test]
fn instruction_subn_register() {
    let mut cpu = CPU::new();
    cpu.set_registers(&[0x01, 0x00, 0xFF, 0xFF, 0x11, 0xAA]);
    cpu.set_memory(&[
        0x80, 0x17, // SUBN gpr0, gpr1
        0x82, 0x37, // SUBN gpr2, gpr3
        0x84, 0x57, // SUBN gpr4, gpr5
    ]);
    cpu.tick();
    assert_eq!(cpu.get_registers()[0], 0xFF);
    assert_eq!(cpu.get_vf(), 0);
    cpu.tick();
    assert_eq!(cpu.get_registers()[2], 0x00);
    assert_eq!(cpu.get_vf(), 1);
    cpu.tick();
    assert_eq!(cpu.get_registers()[4], 0x99);
    assert_eq!(cpu.get_vf(), 1);
}

#[wasm_bindgen_test]
fn instruction_shl_register() {
    let mut cpu = CPU::new();
    cpu.set_registers(&[0x00, 0x0F, 0xFF]);
    cpu.set_memory(&[
        0x80, 0x0E, // SHL gpr0
        0x81, 0x0E, // SHL gpr1
        0x82, 0x0E, // SHL gpr2
    ]);
    cpu.tick();
    assert_eq!(cpu.get_registers()[0], 0x00);
    assert_eq!(cpu.get_vf(), 0);
    cpu.tick();
    assert_eq!(cpu.get_registers()[1], 0x1E);
    assert_eq!(cpu.get_vf(), 0);
    cpu.tick();
    assert_eq!(cpu.get_registers()[2], 0xFE);
    assert_eq!(cpu.get_vf(), 1);
}

#[wasm_bindgen_test]
fn instruction_sne_register() {
    { // Skip
        let mut cpu = CPU::new();
        cpu.set_registers(&[0x00, 0x00]);
        cpu.set_memory(&[
            0x90, 0x10, // SNE gpr0, gpr1
        ]);
        let mut original_pc = cpu.get_pc();
        cpu.tick();
        // Normal increment of 2 without skip
        assert_eq!(cpu.get_pc(), original_pc + 2);
    }
    { // No Skip
        let mut cpu = CPU::new();
        cpu.set_registers(&[0x00, 0xFF]);
        cpu.set_memory(&[
            0x90, 0x10, // SNE gpr0, gpr1
        ]);
        let mut original_pc = cpu.get_pc();
        cpu.tick();
        // Skip = pc has been increased by 4
        assert_eq!(cpu.get_pc(), original_pc + 4);
    }
}

#[wasm_bindgen_test]
fn instruction_ldi() {
    let mut cpu = CPU::new();
    cpu.set_memory(&[
        0xA1, 0x23, // LD I 0x123
    ]);
    cpu.tick();
    assert_eq!(cpu.get_i(), 0x123);
}

#[wasm_bindgen_test]
fn instruction_jpv0() {
    let mut cpu = CPU::new();
    cpu.set_memory(&[
        0xB0, 0x03, // JP V0 0x3
    ]);
    cpu.set_i(0x120);
    cpu.tick();
    assert_eq!(cpu.get_pc(), 0x123);
}

#[wasm_bindgen_test]
fn instruction_rnd() {
    let mut cpu = CPU::new();
    cpu.set_memory(&[
        0xC0, 0xF0, // RND V0, 0xFF
        0xC0, 0x0F, // RND V0, 0x0F
        0xC0, 0x00, // RND V0, 0x00
    ]);
    cpu.tick();
    assert!(cpu.get_registers()[0] <= 0xF0);
    cpu.tick();
    assert!(cpu.get_registers()[0] <= 0x0F);
    cpu.tick();
    assert!(cpu.get_registers()[0] == 0x00);
}

#[wasm_bindgen_test]
fn instruction_drw() {
    // TODO : Implement testing of the DRW instruction
}

#[wasm_bindgen_test]
fn instruction_skp() {
    { // Skip
        let mut cpu = CPU::new();
        cpu.set_memory(&[
            0xE0, 0x9E, // RND V0, 0xFF
        ]);
        cpu.set_registers(&[
            0xA // Check if "A" key is pressed
        ]);
        // A and 0 keys are pressed
        cpu.set_keyboard(0x0401);
        let original_pc = cpu.get_pc();
        cpu.tick();
        // Should skip because the key is pressed
        assert_eq!(original_pc + 4, cpu.get_pc());
    }
    { // No Skip
        let mut cpu = CPU::new();
        cpu.set_memory(&[
            0xE0, 0x9E, // RND V0, 0xFF
        ]);
        cpu.set_registers(&[
            0xA // Check if A key is pressed
        ]);
        // All keys except A are pressed
        cpu.set_keyboard(0xFBFF);
        let original_pc = cpu.get_pc();
        cpu.tick();
        // Should not skip because the key is pressed
        assert_eq!(original_pc + 2, cpu.get_pc());
    }
}

#[wasm_bindgen_test]
fn instruction_sknp() {
    { // Skip
        let mut cpu = CPU::new();
        cpu.set_memory(&[
            0xE0, 0xA1, // RND V0, 0xFF
        ]);
        cpu.set_registers(&[
            0xA // Check if "A" key is not pressed
        ]);
        // A and 0 keys are pressed
        cpu.set_keyboard(0x0401);
        let original_pc = cpu.get_pc();
        cpu.tick();
        // Should not skip because the key is pressed
        assert_eq!(original_pc + 2, cpu.get_pc());
    }
    { // No Skip
        let mut cpu = CPU::new();
        cpu.set_memory(&[
            0xE0, 0xA1, // RND V0, 0xFF
        ]);
        cpu.set_registers(&[
            0xA // Check if A key is not pressed
        ]);
        // All keys except A are pressed
        cpu.set_keyboard(0xFBFF);
        let original_pc = cpu.get_pc();
        cpu.tick();
        // Should skip because the key is not pressed
        assert_eq!(original_pc + 4, cpu.get_pc());
    }
}

#[wasm_bindgen_test]
fn instruction_ld_gpr_dt() {
    let mut cpu = CPU::new();
    cpu.set_memory(&[
        0xF0, 0x07, // LD V0, DT
    ]);
    cpu.set_dt(0xAB);
    cpu.tick();
    assert_eq!(cpu.get_registers()[0], 0xAB);
}

#[wasm_bindgen_test]
fn instruction_ld_kp() {
    let mut cpu = CPU::new();
    cpu.set_memory(&[
        0xF0, 0x0A, // LD V0, Next key
    ]);
    let original_pc = cpu.get_pc();
    cpu.tick();
    cpu.tick();
    cpu.tick();
    assert_eq!(cpu.get_pc(), original_pc);
    cpu.set_keyboard(0x0400); // A key is pressed
    cpu.tick();
    assert_eq!(cpu.get_pc(), original_pc + 2);
    assert_eq!(cpu.get_registers()[0], 0xA);
}

#[wasm_bindgen_test]
fn instruction_ld_dt_gpr() {
    let mut cpu = CPU::new();
    cpu.set_memory(&[
        0xF0, 0x15, // LD DT, V0
    ]);
    cpu.set_registers(&[0xAB]);
    cpu.tick();
    assert_eq!(cpu.get_dt(), 0xAB);
}

#[wasm_bindgen_test]
fn instruction_ld_st_gpr() {
    let mut cpu = CPU::new();
    cpu.set_memory(&[
        0xF0, 0x18, // LD ST, V0
    ]);
    cpu.set_registers(&[0xAB]);
    cpu.tick();
    assert_eq!(cpu.get_st(), 0xAB);
}

#[wasm_bindgen_test]
fn instruction_add_i_gpr() {
    let mut cpu = CPU::new();
    cpu.set_memory(&[
        0xF0, 0x1E, // ADD I, V0
    ]);
    cpu.set_i(0x11);
    cpu.set_registers(&[0x11]);
    cpu.tick();
    assert_eq!(cpu.get_i(), 0x22);
}

#[wasm_bindgen_test]
fn instruction_ld_i_font() {
    let mut cpu = CPU::new();
    cpu.set_memory(&[
        0xF0, 0x29, // LD I, FONT
        0xF1, 0x29, // LD I, FONT
    ]);
    cpu.set_registers(&[0x00, 0x01]);
    cpu.tick();
    assert_eq!(cpu.get_i(), 0x00);
    cpu.tick();
    assert_eq!(cpu.get_i(), 0x05);
}

#[wasm_bindgen_test]
fn instruction_bcd() {
    let mut cpu = CPU::new();
    cpu.set_memory(&[
        0xF0, 0x33, // LD B, V0
    ]);
    cpu.set_registers(&[123]);
    let output_address = cpu.get_pc() + 2;
    cpu.set_i(output_address);
    cpu.tick();
    assert_eq!(cpu.get_memory()[output_address as usize], 1);
    assert_eq!(cpu.get_memory()[output_address as usize] + 1, 2);
    assert_eq!(cpu.get_memory()[output_address as usize] + 2, 3);
}

#[wasm_bindgen_test]
fn instruction_ld_i_vx() {
    let mut cpu = CPU::new();
    cpu.set_memory(&[
        0xF0, 0x55, // LD [I], V0
        0xF7, 0x55, // LD [I], V0-7
        0xFF, 0x55, // LD [I], V0-F
    ]);
    cpu.set_registers(&[0xFF; 16]);
    let output_address = cpu.get_pc() + 6;
    cpu.set_i(output_address);
    cpu.tick();
    assert_eq!(cpu.get_memory()
        [output_address as usize..output_address as usize + 16],
        [
            0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
    cpu.tick();
    assert_eq!(cpu.get_memory()
        [output_address as usize..output_address as usize + 16],
        [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
    cpu.tick();
    assert_eq!(cpu.get_memory()
        [output_address as usize..output_address as usize + 16],
        [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        ]);
}

#[wasm_bindgen_test]
fn instruction_ld_vx_i() {
    let mut cpu = CPU::new();
    cpu.set_memory(&[
        0xF0, 0x65, // LD [I], V0
        0xF7, 0x65, // LD [I], V0-7
        0xFF, 0x65, // LD [I], V0-F
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // Load target
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // Load target
    ]);
    let output_address = cpu.get_pc() + 6;
    cpu.set_i(output_address);
    cpu.tick();
    assert_eq!(cpu.get_registers(),
        [
            0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
    cpu.tick();
    assert_eq!(cpu.get_registers(),
        [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
    cpu.tick();
    assert_eq!(cpu.get_registers(),
        [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        ]);
}

#[wasm_bindgen_test]
fn instruction_cls() {
    let mut cpu = CPU::new();
    cpu.set_memory(&[
        0x00, 0xE0, // CLS
    ]);
    cpu.set_display(&[
        0xFF, 0xFF,
    ]);
    cpu.tick();
    assert_eq!(cpu.get_display()[0..2], [0x00, 0x00]);
}