use std::io::ErrorKind;
use std::{collections::VecDeque, io::Write, net::UdpSocket};

pub const SIZE: usize = 16;

#[allow(dead_code)]
pub struct Uart {
    registers: [u8; SIZE],

    enable_tx: bool,
    enable_rx: bool,

    enable_tx_irq: bool,

    // mode registers (selectable via CRA commands)
    mra: [u8; 3],
    mrb: [u8; 3],
    // indexes into mra/mrb, as "MR pointers"
    mrai: usize,
    mrbi: usize,

    socket_a: UdpSocket,
    recv_a: VecDeque<u8>, // network receive buffer
}

#[allow(dead_code)]
impl Uart {
    // I wish I could express these as enums (ReadRegisters & WriteRegisters) and use them as patterns
    // to match a u8 reg adress, but that's not (cleanly) possible.
    const REG_MRA: u8 = 0x0; // read + write
    const REG_SRA: u8 = 0x1; // read
    const REG_CSRA: u8 = 0x1; // write
    const REG_CRA: u8 = 0x2; // write
    const REG_RXFIFOA: u8 = 0x3; // read
    const REG_TXFIFOA: u8 = 0x3; // write
    const REG_IPCR: u8 = 0x4; // read
    const REG_ACR: u8 = 0x4; // write
    const REG_ISR: u8 = 0x5; // read
    const REG_IMR: u8 = 0x5; // write
    const REG_CTU: u8 = 0x6; // read
    const REG_CTPU: u8 = 0x6; // write
    const REG_CTL: u8 = 0x7; // read
    const REG_CTPL: u8 = 0x7; // write
    const REG_MRB: u8 = 0x8; // read + write
    const REG_SRB: u8 = 0x9; // read
    const REG_CSRB: u8 = 0x9; // write
    const REG_CRB: u8 = 0xA; // write
    const REG_RXFIFOB: u8 = 0xB; // read
    const REG_TXFIFOB: u8 = 0xB; // write
    const REG_MISC: u8 = 0xC; // read + write
    const REG_IPR: u8 = 0xD; // read
    const REG_OPCR: u8 = 0xD; // write
    const REG_SOPR: u8 = 0xE; // write
    const REG_ROPR: u8 = 0xF; // write

    const IRQ_MASK_TXRDYA: u8 = 1 << 0;
    const IRQ_MASK_RXRDYA: u8 = 1 << 1;

    const SRA_BIT_RXRDYA: usize = 0;
    const SRA_BIT_RXFULLA: usize = 1;
    const SRA_BIT_TXRDYA: usize = 2;
    const SRA_BIT_TXEMTA: usize = 3;

    const REG_READ: [&'static str; SIZE] = [
        "MRA", "SRA", "", "RXFIFOA", "IPCR", "ISR", "CTU", "CTL", "MRB", "SRB", "", "RXFIFOB",
        "MISC", "IPR", "", "",
    ];

    const REG_WRITE: [&'static str; SIZE] = [
        "MRA", "CSRA", "CRA", "TXFIFOA", "ACR", "IMR", "CTPU", "CTPL", "MRB", "CSRB", "CRB",
        "TXFIFOB", "MISC", "OPCR", "SOPR", "ROPR",
    ];

    pub fn new() -> Self {
        let socket_a = UdpSocket::bind("127.0.0.1:0").unwrap();

        Self {
            registers: [0x00; SIZE],
            enable_tx: false,
            enable_rx: false,
            enable_tx_irq: false,
            mra: [0x00; 3],
            mrb: [0x00; 3],
            mrai: 0,
            mrbi: 0,
            socket_a,
            recv_a: VecDeque::new(),
        }
    }

    pub fn reset(&mut self) {
        self.registers[0x05] |= 0b00000001; // TxRDY
        self.mrai = 1;
        self.mrbi = 1;
        self.enable_tx = false;
        self.enable_rx = false;
        self.enable_tx_irq = false;
    }

    pub fn step(&mut self) {
        if self.recv_a.is_empty() {
            let mut buf = [0; 1024];
            self.socket_a.set_nonblocking(true).unwrap();
            match self.socket_a.recv(&mut buf) {
                Ok(amt) => {
                    self.recv_a.write(&buf[..amt]).unwrap();
                }
                Err(e) => match e.kind() {
                    ErrorKind::WouldBlock => {}
                    _ => {
                        panic!("{}", e);
                    }
                },
            }
        }
    }

    pub fn read(&mut self, reg: u8) -> u8 {
        match reg {
            Self::REG_SRA => self.read_sra(),
            Self::REG_RXFIFOA => self.read_fifo_a(),
            _ => self.registers[reg as usize],
        }
    }

    pub fn write(&mut self, reg: u8, data: u8) {
        match reg {
            Self::REG_MRA => self.write_mra(data),
            Self::REG_CRA => self.write_cra(data),
            Self::REG_TXFIFOA => self.tx_a(data),
            _ => {
                // eprintln!(
                //     "UART: {}/{reg:#X} <- {data:#04X}/{data}/{data:#010b}",
                //     Self::REG_WRITE[reg as usize]
                // )
            }
        }
        self.registers[reg as usize] = data;
    }

    pub fn is_interrupt(&self) -> bool {
        let imr = self.registers[Self::REG_IMR as usize];

        // for now we're assuming Tx buffer is always ready / never full.
        (self.enable_tx && imr & Self::IRQ_MASK_TXRDYA != 0)
            || (self.enable_rx && imr & Self::IRQ_MASK_RXRDYA != 0 && !self.recv_a.is_empty())
    }

    pub fn name_for_read(&mut self, reg: u8) -> String {
        format!("UART:{}", Self::REG_READ[reg as usize])
    }

    pub fn name_for_write(&self, reg: u8) -> String {
        format!("UART:{}", Self::REG_WRITE[reg as usize])
    }

    fn write_cra(&mut self, data: u8) {
        if data & 1 << 3 != 0 {
            eprintln!("UART CRA TODO: Disable channel A transmitter. This command terminates transmitter operation and reset the TxDRY and TxEMT status bits. However, if a character is being transmitted or if a character is in the Tx FIFO when the transmitter is disabled, the transmission of the character(s) is completed before assuming the inactive state.");
        }

        if data & 1 << 2 != 0 {
            // Enable channel A transmitter. Enables operation of the channel A transmitter.
            // The TxRDY and TxEMT status bits will be asserted if the transmitter is idle.
            // eprintln!("UART CRA: Tx enable, setting SRA TxRDY & TxEMPT");
            self.enable_tx = true;
            self.registers[Self::REG_SRA as usize] |= 0b00001100;
        }

        if data & 1 << 1 != 0 {
            eprintln!("UART CRA TODO: Disable channel A receiver. This command terminates operation of the receiver immediately-a character being received will be lost. The command has no effect on the receiver status bits or any other control registers. If the special multi-drop mode is programmed, the receiver operates even if it is disabled.");
        }

        if data & 1 << 0 != 0 {
            // Enable channel A receiver. Enables operation of the channel A receiver.
            // If not in the special wake-up mode, this also forces the receiver into the search for start-bit state.
            // eprintln!("UART CRA: Rx enable");
            self.enable_rx = true;
        }

        match data >> 4 {
            0b0001 => {
                self.mrai = 1;
            }
            0b0010 => {
                eprintln!("UART TODO: Reset receiver. Resets the channel A receiver as if a hardware reset had been applied. The receiver is disabled and the FIFO is flushed.")
            }
            0b0011 => {
                eprintln!("UART TODO: Reset transmitter. Resets the channel A transmitter as if a hardware reset had been applied")
            }
            0b0100 => {
                eprintln!("UART TODO: Reset error status. Clears the channel A received break, parity error, and overrun error bits in the status register (SRA[7:4]). Used in character mode to clear OE status (although RB, PE and FE bits will also be cleared) and in block mode to clear all error status after a block of data has been received.")
            }
            0b0101 => {
                eprintln!("UART TODO: Reset channel A break change interrupt. Causes the channel A break detect change bit in the interrupt status register (ISR[2]) to be cleared to zero.")
            }
            0b0110 => {
                eprintln!("UART TODO: Start break. Forces the TxDA output LOW (spacing). If the transmitter is empty the start of the break condition will be delayed up to two bit times. If the transmitter is active the break begins when transmission of the character is completed. If a character is in the Tx FIFO, the start of the break will be delayed until that character, or any other loaded subsequently are transmitted. The transmitter must be enabled for this command to be accepted.")
            }
            0b0111 => {
                eprintln!("UART TODO: Stop break. The TxDA line will go HIGH (marking) within two bit times. TxDA will remain HIGH for one bit time before the next character, if any, is transmitted.")
            }
            0b1000 => {
                eprintln!("UART TODO: Assert RTSN. Causes the RTSN output to be asserted (LOW)")
            }
            0b1001 => {
                eprintln!("UART TODO: Negate RTSN. Causes the RTSN output to be negated (HIGH).")
            }
            0b1010 => {
                eprintln!("UART TODO: Set time-out mode on. The receiver in this channel will restart the C/T as each receive character is transferred from the shift register to the Rx FIFO. The C/T is placed in the counter mode, the start counter or stop counter commands are disabled, the counter is stopped, and the counter ready bit, ISR[3], is reset.")
            }
            0b1011 => {
                self.mrai = 0;
            }
            0b1100 => {
                eprintln!("UART TODO: Disable time-out mode. This command returns control of the C/T to the regular start counter or stop counter commands. It does not stop the counter, or clear any pending interrupts. After disabling the time-out mode, a stop counter command should be issued to force a reset of the ISR[3] bit.")
            }
            0b1101 => {
                eprintln!("UART TODO: Not used.")
            }
            0b1110 => {
                eprintln!("UART TODO: Power-down mode on. In this mode, the DUART oscillator is stopped and all functions requiring this clock are suspended. The execution of commands other than disable Power-down mode (1111) requires a X1/CLK. While in the Power-down mode, do not issue any commands to the CR except the disable Power-down mode command. The contents of all registers will be saved while in this mode. It is recommended that the transmitter and receiver be disabled prior to placing the DUART into Power-down mode. This command is in CRA only.")
            }
            0b1111 => {
                eprintln!("Disable Power-down mode. This command restarts the oscillator. After invoking this command, wait for the oscillator to start up before writing further commands to the CR. This command is in CRA only. For maximum power reduction input pins should be at Vss or Vdd.")
            }
            _ => (),
        }
    }

    fn write_mra(&mut self, data: u8) {
        eprintln!("UART TODO: MRA{} <- {data:#010b}", self.mrai);
        self.mra[self.mrai] = data;
        if self.mrai <= 2 {
            self.mrai += 1
        }
    }

    fn tx_a(&self, data: u8) {
        let buf = [data];
        self.socket_a.send_to(&buf, "127.0.0.1:6502").unwrap();
    }

    fn read_sra(&self) -> u8 {
        let mut value: u8 = 0x00;

        if !self.recv_a.is_empty() {
            value |= 1 << Self::SRA_BIT_RXRDYA;
        }

        // For now, Rx FIFO is never considered to be full.
        value |= 0 << Self::SRA_BIT_RXFULLA;

        // For now, Tx is always ready, Tx FIFO buffer is never considered to be full.
        value |= 1 << Self::SRA_BIT_TXRDYA;

        // For now, Tx FIFO is always considered empty.
        value |= 1 << Self::SRA_BIT_TXEMTA;

        // TODO: bits 4..7 represent error flags for the byte at the top of the FIFO.

        value
    }

    fn read_fifo_a(&mut self) -> u8 {
        self.recv_a.pop_front().unwrap_or(0x00)
    }
}
