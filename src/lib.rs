#![crate_type = "staticlib"]
#![no_std]
#![feature(core,core_intrinsics, lang_items, compiler_builtins_lib)]
#[macro_use] extern crate fixedvec;

extern crate rlibc;
extern crate compiler_builtins;

pub mod hal;

use core::intrinsics::abort;
use core::str::from_utf8;
use fixedvec::FixedVec;

enum CharType {
    Control,
    Printable
}

const BUFFERLEN: usize = 0x00000400;

fn get_char_type(ch: u8) -> CharType {
    if ch < 0x20 { // Consider TAB, RET etc control characters
        CharType::Control
    }
    else {
        if ch < 0x7F {
            CharType::Printable
        } else {
            CharType::Control
        }
    }
}

fn repl(hw: &hal::Hardware, buffer: &fixedvec::FixedVec<u8>) {
    match from_utf8(buffer.as_slice()) {
        Ok(val) => {
            if val == "xyzzy" {
                hw.send_string("Nothing happens.")
            } else {
                hw.send_string(val)
            }
        },
        Err(_doh) => hw.send_string("PANIC: Decode Error!")
    }
}

#[no_mangle]
pub extern fn notmain() {
    let hw = hal::get_hw();
    hw.uart_init();

    let pc: u32 = hw.get_pc();
    hw.hexstring(pc);

    let id: u32 = hw.get_id();
    hw.hexstring(id);

    hw.send_hwstr();

    hw.uart_send(0x3E);
    hw.uart_send(0x20);

    let mut input_memory = alloc_stack!([u8; BUFFERLEN]);
    let mut input_buffer = FixedVec::new(&mut input_memory);

    let mut c: u32;
    loop {
        c = hw.uart_recv();
        match get_char_type(c as u8) {
            CharType::Control => {
                match c {
                    0x0000000D => {
                        // Do some stuff
                        hw.uart_send(0x0A);
                        hw.uart_send(0x0D);

                        repl(&hw,&input_buffer);
                        input_buffer.clear();

                        hw.uart_send(0x0A);
                        hw.uart_send(0x0D);
                        hw.uart_send(0x3E);
                        hw.uart_send(0x20);
                    },
                    0x0000007F => { // backspace
                        if input_buffer.len() > 0 {
                            input_buffer.pop();
                            hw.uart_send(0x00000008);
                        }
                    }
                    _ => {
                        hw.send_string("\r\n\nControl: 0x");
                        hw.hexstring(c);
                    }
                }
            }
            CharType::Printable => {
                if input_buffer.len() < BUFFERLEN { // Ignore typing past the length of the buffer
                    hw.uart_send(c);
                    match input_buffer.push(c as u8) {
                        Ok(_val) => (),
                        Err(_doh) => ()
                    }
                }
            }
        }
    }
}

#[lang = "panic_fmt"]
#[no_mangle]
pub fn panic_fmt() -> ! { unsafe { abort() } }
