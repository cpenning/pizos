extern {
    fn PUT32 (x: u32, y: u32);
    fn GET32 (x: u32) -> u32;
    fn GETPC () -> u32;
    fn GETCPUID() -> u32;
}

pub fn get_hw() -> Hardware {
    let id: u32;
    unsafe {
        id = GETCPUID();
    }
    let pbase: u32 = get_pbase(id);
    Hardware {
        id: id,
        pbase: pbase,
        gpfsel1: pbase | Hardware::GPFSEL1,
        aux_enables: pbase | Hardware::AUX_ENABLES,
        aux_mu_io_reg: pbase | Hardware::AUX_MU_IO_REG,
        aux_mu_ier_reg: pbase | Hardware::AUX_MU_IER_REG,
        aux_mu_iir_reg: pbase | Hardware::AUX_MU_IIR_REG,
        aux_mu_lcr_reg: pbase | Hardware::AUX_MU_LCR_REG,
        aux_mu_mcr_reg: pbase | Hardware::AUX_MU_MCR_REG,
        aux_mu_lsr_reg: pbase | Hardware::AUX_MU_LSR_REG,
        aux_mu_cntl_reg: pbase | Hardware::AUX_MU_CNTL_REG,
        aux_mu_baud_reg: pbase | Hardware::AUX_MU_BAUD_REG
    }
}

pub struct Hardware {
    id: u32,
    pbase: u32,
    gpfsel1: u32,
    aux_enables: u32,
    aux_mu_io_reg: u32,
    aux_mu_ier_reg: u32,
    aux_mu_iir_reg: u32,
    aux_mu_lcr_reg: u32,
    aux_mu_mcr_reg: u32,
    aux_mu_lsr_reg: u32,
    aux_mu_cntl_reg: u32,
    aux_mu_baud_reg: u32
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
    //const GPSET0: u32          = 0x0020001C;
    //const GPCLR0: u32          = 0x00200028;
    //const UART_DR: u32         = 0x00201000;
    //const UART_FR: u32         = 0x00201018;
    const AUX_ENABLES: u32     = 0x00215004;
    const AUX_MU_IO_REG: u32   = 0x00215040;
    const AUX_MU_IER_REG: u32  = 0x00215044;
    const AUX_MU_IIR_REG: u32  = 0x00215048;
    const AUX_MU_LCR_REG: u32  = 0x0021504C;
    const AUX_MU_MCR_REG: u32  = 0x00215050;
    const AUX_MU_LSR_REG: u32  = 0x00215054;
    //const AUX_MU_MSR_REG: u32  = 0x00215058;
    //const AUX_MU_SCRATCH: u32  = 0x0021505C;
    const AUX_MU_CNTL_REG: u32 = 0x00215060;
    //const AUX_MU_STAT_REG: u32 = 0x00215064;
    const AUX_MU_BAUD_REG: u32 = 0x00215068;

    pub const PI1: &'static str = "Raspberry Pi 1/zero\r\n";
    pub const PI2: &'static str = "Raspberry Pi 2\r\n";
    pub const PI3: &'static str = "Raspberry Pi 3\r\n";
    pub const UNK: &'static str = "UNKNOWN\r\n";

    pub fn uart_send(&self, c: u32) {
        unsafe {
            loop {
                let chk: u32 = GET32(self.aux_mu_lsr_reg)&0x20;
                if chk != 0 {
                    break;
                }
            }
            PUT32(self.aux_mu_io_reg,c);
        }
    }

	pub fn uart_recv(&self) -> u32 {
        loop {
            let chk: u32;
            unsafe {
                chk = GET32(self.aux_mu_lsr_reg)&0x01;
            }
    	    if chk > 0 {
                break;
            }
        }
        unsafe {
			GET32(self.aux_mu_io_reg)
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
            PUT32(self.aux_enables,1);
            PUT32(self.aux_mu_ier_reg,0);
            PUT32(self.aux_mu_cntl_reg,0);
            PUT32(self.aux_mu_lcr_reg,3);
            PUT32(self.aux_mu_mcr_reg,0);
            PUT32(self.aux_mu_ier_reg,0);
            PUT32(self.aux_mu_iir_reg,0xC6);
            PUT32(self.aux_mu_baud_reg,270);
            ra = GET32(self.gpfsel1);
        }

        ra&=!(7<<12); //gpio14
        ra|=2<<12;    //alt5
        ra&=!(7<<15); //gpio15
        ra|=2<<15;    //alt5

        unsafe {
            PUT32(self.gpfsel1,ra);
            PUT32(self.aux_mu_cntl_reg,3);
        }
    }

    pub fn send_string (&self,s: &str) {
        for c in s.chars() {
            self.uart_send(c as u32);
        }
    }

    pub fn send_hwstr(&self) {
        match self.id {
            0x410FB767 => self.send_string(Hardware::PI1),
            0x410FC075 => self.send_string(Hardware::PI2),
            0x410FD034 => self.send_string(Hardware::PI3),
            _ => self.send_string(Hardware::UNK)
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_pc(&self) -> u32 {
        unsafe {
            GETPC()
        }
    }
}
