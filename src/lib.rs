#![no_std]
#![feature(core_intrinsics, lang_items)]

use core::intrinsics::abort;

extern {
    fn PUT32 (x: u32, y: u32);
    fn GET32 (x: u32) -> u32;
    fn GETPC () -> u32;
    fn GETCPUID() -> u32;
    fn dummy (x: u32);
}

struct Hardware {
    pbase: u32
}

// Get the pbase from the cpuid
fn get_pbase(id: u32) -> u32 {
    match id {
        0x410FB767 => 0x20000000,
        _ => 0x3F000000
    }
}

impl Hardware {
    // Offsets from pbase
    const GPFSEL1: u32         = 0x00200004;
    const GPSET0: u32          = 0x0020001C;
    const GPCLR0: u32          = 0x00200028;
    const AUX_ENABLES: u32     = 0x00215004;
    const AUX_MU_IO_REG: u32   = 0x00215040;
    const AUX_MU_IER_REG: u32  = 0x00215044;
    const AUX_MU_IIR_REG: u32  = 0x00215048;
    const AUX_MU_LCR_REG: u32  = 0x0021504C;
    const AUX_MU_MCR_REG: u32  = 0x00215050;
    const AUX_MU_LSR_REG: u32  = 0x00215054;
    const AUX_MU_MSR_REG: u32  = 0x00215058;
    const AUX_MU_SCRATCH: u32  = 0x0021505C;
    const AUX_MU_CNTL_REG: u32 = 0x00215060;
    const AUX_MU_STAT_REG: u32 = 0x00215064;
    const AUX_MU_BAUD_REG: u32 = 0x00215068;

    const PI1: &'static str = "Raspberry Pi 1/zero\r\n";
    const PI2: &'static str = "Raspberry Pi 2\r\n";
    const PI3: &'static str = "Raspberry Pi 3\r\n";
    const UNK: &'static str = "UNKNOWN\r\n";

    pub fn uart_send(&self, c: u32) {
        unsafe {
            loop {
                let chk: u32 = GET32(self.pbase + Hardware::AUX_MU_LSR_REG)&0x20;
                if chk != 0 {
                    break;
                }
            }
            PUT32(self.pbase + Hardware::AUX_MU_IO_REG,c);
        }
    }

    pub fn hexstrings(&self, d: u32) {
        let mut rb: u32;
        let mut rc: u32;

        rb=32;
        loop {
            rb-=4;
            rc=(d>>rb)&0xF;
            if rc>9 {
                rc+=0x37;
            } else {
                rc+=0x30;
            }
            self.uart_send(rc);
            if rb==0 {
                break;
            }
        }
        self.uart_send(0x20);
    }

    pub fn hexstring(&self, d: u32) {
        self.hexstrings(d);
        self.uart_send(0x0D);
        self.uart_send(0x0A);
    }

    pub fn uart_init (&self) {
        let mut ra: u32;

        unsafe {
            PUT32(self.pbase + Hardware::AUX_ENABLES,1);
            PUT32(self.pbase + Hardware::AUX_MU_IER_REG,0);
            PUT32(self.pbase + Hardware::AUX_MU_CNTL_REG,0);
            PUT32(self.pbase + Hardware::AUX_MU_LCR_REG,3);
            PUT32(self.pbase + Hardware::AUX_MU_MCR_REG,0);
            PUT32(self.pbase + Hardware::AUX_MU_IER_REG,0);
            PUT32(self.pbase + Hardware::AUX_MU_IIR_REG,0xC6);
            PUT32(self.pbase + Hardware::AUX_MU_BAUD_REG,270);
            ra = GET32(self.pbase + Hardware::GPFSEL1);
        }

        ra&=!(7<<12); //gpio14
        ra|=2<<12;    //alt5
        ra&=!(7<<15); //gpio15
        ra|=2<<15;    //alt5

        unsafe {
            PUT32(self.pbase + Hardware::GPFSEL1,ra);
            PUT32(self.pbase + Hardware::AUX_MU_CNTL_REG,3);
        }
    }

    pub fn send_string (&self,s: &str) {
        for c in s.chars() {
            self.uart_send(c as u32);
        }
    }
}

#[no_mangle]
pub extern fn notmain() {
    let id: u32;
    unsafe {
        id = GETCPUID();
    }
    let hw = Hardware { pbase: get_pbase(id) };
    hw.uart_init();
    hw.hexstring(0x12345678);
    let pc: u32;
    unsafe {
        pc = GETPC();
    }
    hw.hexstring(pc);
    let id: u32;
    unsafe {
        id = GETCPUID();
    }
    hw.hexstring(id);
    let id: u32;
    unsafe {
        id = GETCPUID();
    }
    match id {
        0x410FB767 => hw.send_string(Hardware::PI1),
        0x410FC075 => hw.send_string(Hardware::PI2),
        0x410FD034 => hw.send_string(Hardware::PI3),
        _ => hw.send_string(Hardware::UNK)
    }
    hw.hexstring(0x11223344);
}

// TODO(cpenning) Track down the cause of this!
// Use nm to get the symbol name
// $ arm-none-eabi-nm target/arm-none-eabihf/debug/libpirustbarecpuid.rlib 2>/dev/null  | grep ' U .*panicking.*panic'
//          U _ZN4core9panicking5panic17h93f04452fe9c978cE
#[no_mangle]
pub extern fn _ZN4core9panicking5panic17h93f04452fe9c978cE() -> ! { unsafe { abort() } }
