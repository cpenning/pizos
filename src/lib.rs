#![no_std]
#![feature(core_intrinsics, lang_items)]

pub mod hal;

use core::intrinsics::abort;

#[no_mangle]
pub extern fn notmain() {
    let hw = hal::get_hw();
    hw.uart_init();

    hw.hexstring(0x12345678);

    let pc: u32 = hw.get_pc();
    hw.hexstring(pc);

    let id: u32 = hw.get_id();
    hw.hexstring(id);

    match id {
        0x410FB767 => hw.send_string(hal::Hardware::PI1),
        0x410FC075 => hw.send_string(hal::Hardware::PI2),
        0x410FD034 => hw.send_string(hal::Hardware::PI3),
        _ => hw.send_string(hal::Hardware::UNK)
    }
    hw.hexstring(0x11223344);

    let mut c: u32;
    loop {
        hw.uart_send(0x3E);
        hw.uart_send(0x20);
        c = hw.uart_recv();
        hw.uart_send(c);
        hw.uart_send(0x0A);
        hw.uart_send(0x0D);
        hw.hexstring(c);
    }
}

// TODO(cpenning) Track down the cause of this!
// Use nm to get the symbol name
// $ arm-none-eabi-nm target/arm-none-eabihf/debug/libpirustbarecpuid.rlib 2>/dev/null  | grep ' U .*panicking.*panic'
//          U _ZN4core9panicking5panic17h93f04452fe9c978cE
#[no_mangle]
pub extern fn _ZN4core9panicking5panic17h93f04452fe9c978cE() -> ! { unsafe { abort() } }
