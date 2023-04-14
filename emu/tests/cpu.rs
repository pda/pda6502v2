use pda6502v2emu::asm::BranchTarget;
use pda6502v2emu::asm::{label, val};
use pda6502v2emu::asm::{Assembler, Operand};
use pda6502v2emu::bus::Bus;
use pda6502v2emu::cpu::stat;
use pda6502v2emu::cpu::Cpu;

macro_rules! assert_eq_hex {
    ($a:expr, $b:expr) => {
        assert_eq!(
            $a,
            $b,
            "{}:{}:{:#04X}:{:#010b} != {}:{:#04X}:{:#010b}",
            stringify!($a),
            $a,
            $a,
            $a,
            $b,
            $b,
            $b
        );
    };
}

macro_rules! assert_eq_hex16 {
    ($a:expr, $b:expr) => {
        assert_eq!($a, $b, "{}:{:#06X} != {:#06X}", stringify!($a), $a, $b);
    };
}

macro_rules! step_and_assert {
    ($cpu:expr, $reg:ident, $val:expr, $stat:literal) => {
        $cpu.step();
        println!("{:?}", $cpu);
        assert_eq_hex!($cpu.$reg, $val);
        assert_eq!(stat(&$cpu.sr), $stat);
    };
}

macro_rules! step_and_assert_mem {
    ($cpu:expr, $addr:expr, $val:expr, $stat:literal) => {
        $cpu.step();
        println!("{:?}", $cpu);
        assert_eq_hex!($cpu.bus.read($addr), $val);
        assert_eq!(stat(&$cpu.sr), $stat);
    };
}

#[test]
fn test_adc() {
    let mut cpu = Cpu::new(Bus::default());
    cpu.a = 0x10; // starting value
    cpu.x = 0x21; // for testing X-indexed address modes
    cpu.y = 0x46; // for testing Y-indexed address modes
    cpu.bus.write(0x00F0, 0xEF); // for testing zero-page address mode
    cpu.bus.write(0x00F1, 0x70); // for testing zp,X address mode
    cpu.bus.write(0x00F2, 0x37); // for testing X,ind address mode (ptr LO)
    cpu.bus.write(0x00F3, 0x12); // for testing X,ind address mode (ptr HI)
    cpu.bus.write(0x00F4, 0xF2); // for testing ind,Y address mode (ptr LO)
    cpu.bus.write(0x00F5, 0x11); // for testing ind,Y address mode (ptr HI)
    cpu.bus.write(0x1234, 0x84); // for testing absolute address mode
    cpu.bus.write(0x1235, 0xFA); // for testing abs,X address mode
    cpu.bus.write(0x1236, 0x00); // for testing abs,X address mode
    cpu.bus.write(0x1237, 0x42); // for testing X,ind address mode (val)
    cpu.bus.write(0x1238, 0xBC); // for testing ind,Y address mode (val)

    use Operand::{Abs, AbsX, AbsY, Imm, IndY, XInd, Z, ZX};
    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.adc(Imm(0x11)) //          C:0+A:$10+#$11                            =$21+C:0
            .adc(Z(0xF0)) //           C:0+A:$21+[$F0→#$EF]                      =$10+C:1
            .adc(ZX(0xD0)) //          C:1+A:$10+[$D0+X:$21→$F1→#$70]            =$81+C:0(V)
            .adc(Abs(val(0x1234))) //  C:0+A:$81+[$1234→#$84]                    =$05+C:1(V)
            .adc(AbsX(val(0x1214))) // C:1+A:$05+[$1214+X:$21→$1235→#$FA]        =$00+C:1
            .adc(AbsY(val(0x12F0))) // C:1+A:$00+[$12F0+Y:$46→$1236→#$00]        =$01+C:0
            .adc(XInd(0xD1)) //        C:0+A:$01+[($D1+X:$21)→($F2)→$1237→#$42]  =$43+C:0
            .adc(IndY(0xF4)) //        C:0+A:$43+[($F4)+Y→$11F2+Y:$46→$1238→#$BC]=$FF+C:0
            .print_listing()
            .assemble()
            .unwrap(),
    );

    step_and_assert!(cpu, a, 0x21, "nv-bdizc"); // ADC #$11
    step_and_assert!(cpu, a, 0x10, "nv-bdizC"); // ADC $F0
    step_and_assert!(cpu, a, 0x81, "NV-bdizc"); // ADC $D0,X
    step_and_assert!(cpu, a, 0x05, "nV-bdizC"); // ADC $1234
    step_and_assert!(cpu, a, 0x00, "nv-bdiZC"); // ADC $1214,X
    step_and_assert!(cpu, a, 0x01, "nv-bdizc"); // ADC $12F0,Y
    step_and_assert!(cpu, a, 0x43, "nv-bdizc"); // ADC ($D1,X)
    step_and_assert!(cpu, a, 0xFF, "Nv-bdizc"); // ADC ($F4),Y
}

#[test]
fn test_and() {
    let mut cpu = Cpu::new(Bus::default());
    cpu.a = 0b10011001; // starting value

    use Operand::Imm;
    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.and(Imm(0b11110000)).print_listing().assemble().unwrap(),
    );

    step_and_assert!(cpu, a, 0b10010000, "Nv-bdizc"); // ADC #$A0
}

#[test]
fn test_asl() {
    let mut cpu = Cpu::new(Bus::default());
    cpu.a = 0b01000000; // for testing Accumulator address mode
    cpu.x = 0x01; // for testing zp,X address mode
    cpu.bus.write(0xF1, 0b11011011); // for testing zp,X address mode

    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.asl(Operand::A)
            .asl(Operand::A)
            .asl(Operand::ZX(0xF0))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    use pda6502v2emu::cpu::StatusMask;

    cpu.set_sr_bit(StatusMask::Carry, true);
    step_and_assert!(cpu, a, 0b10000000, "Nv-bdizc"); // ASL A

    cpu.set_sr_bit(StatusMask::Carry, true);
    step_and_assert!(cpu, a, 0b00000000, "nv-bdiZC"); // ASL A

    cpu.step(); // ASL $F0,X
    println!("{:?}", cpu);
    let val = cpu.bus.read(0xF1);
    assert_eq_hex!(val, 0b10110110);
    assert_eq!(stat(&cpu.sr), "Nv-bdizC");
}

#[test]
fn test_bcc() {
    let mut cpu = Cpu::new(Bus::default());

    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.bcc(Operand::Rel(BranchTarget::Offset(0x10)))
            .bcc(Operand::Rel(BranchTarget::Offset(0x20)))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    use pda6502v2emu::cpu::StatusMask;

    cpu.set_sr_bit(StatusMask::Carry, true);
    cpu.step(); // BCC 0x10 (don't branch)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x0002);
    assert_eq!(stat(&cpu.sr), "nv-bdizC");

    cpu.set_sr_bit(StatusMask::Carry, false);
    cpu.step(); // BCC 0x20 (do branch)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x0024);
    assert_eq!(stat(&cpu.sr), "nv-bdizc");
}

#[test]
fn test_bcs() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.bcs(Operand::Rel(BranchTarget::Offset(0x10)))
            .bcs(Operand::Rel(BranchTarget::Offset(0x20)))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    use pda6502v2emu::cpu::StatusMask;

    cpu.set_sr_bit(StatusMask::Carry, false);
    cpu.step(); // BCS 0x10 (don't branch)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x0002);
    assert_eq!(stat(&cpu.sr), "nv-bdizc");

    cpu.set_sr_bit(StatusMask::Carry, true);
    cpu.step(); // BCS 0x20 (do branch)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x0024);
    assert_eq!(stat(&cpu.sr), "nv-bdizC");
}

#[test]
fn test_beq() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.beq(Operand::Rel(BranchTarget::Offset(0x10)))
            .beq(Operand::Rel(BranchTarget::Offset(0x20)))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    use pda6502v2emu::cpu::StatusMask;

    cpu.set_sr_bit(StatusMask::Zero, false);
    cpu.step(); // BEQ 0x10 (don't branch)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x0002);
    assert_eq!(stat(&cpu.sr), "nv-bdizc");

    cpu.set_sr_bit(StatusMask::Zero, true);
    cpu.step(); // BEQ 0x20 (do branch)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x0024);
    assert_eq!(stat(&cpu.sr), "nv-bdiZc");
}

#[test]
fn test_bit() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.org(cpu.pc)
            .label("data")
            .data(vec![0xFF, 0x00])
            .label("prog")
            .bit(Operand::Z(cpu.pc.try_into().unwrap()))
            .bit(Operand::Abs(val(cpu.pc)))
            .print_listing()
            .assemble()
            .unwrap(),
    );
    cpu.pc += 2; // skip the data

    cpu.a = 0xFF;
    cpu.step(); // BIT $00 (#$FF)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x0004);
    assert_eq!(stat(&cpu.sr), "NV-bdizc"); // 0b11111111 AND 0b11111111 = 0b11111111 = z

    cpu.a = 0x00;
    cpu.step(); // BIT $0001 (#$00)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x0007);
    assert_eq!(stat(&cpu.sr), "nv-bdiZc"); // 0b00000000 AND 0b00000000 = 0b00000000 = Z
}

#[test]
fn test_bmi() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.ldx(Operand::Imm(0xFF)) // SR N=1
            .label("a")
            .bmi(Operand::Rel(BranchTarget::Label("b".to_string())))
            .nop()
            .label("b")
            .ldx(Operand::Imm(0x10)) // SR N=0
            .bmi(Operand::Rel(BranchTarget::Label("a".to_string())))
            .nop()
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(); // LDX #$FF
    println!("{cpu:?}");
    cpu.step(); // BMI b
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x0005);
    cpu.step(); // LDX #$10
    cpu.step(); // BMI a
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x0009);
}

#[test]
fn test_bne() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.label("a")
            .ldx(Operand::Imm(0x01))
            .bne(Operand::Rel(BranchTarget::Label("b".to_string())))
            .nop()
            .label("b")
            .ldx(Operand::Imm(0x00))
            .bne(Operand::Rel(BranchTarget::Label("a".to_string())))
            .nop()
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(); // LDX #$01
    cpu.step(); // BNE b
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x0005);

    cpu.step(); // LDX #$00
    cpu.step(); // BNE a
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x0009);
}

#[test]
fn test_bpl() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.label("a")
            .ldx(Operand::Imm(0x10)) // SR N=0
            .bpl(Operand::Rel(BranchTarget::Label("b".to_string())))
            .nop()
            .label("b")
            .ldx(Operand::Imm(0xF0)) // SR N=1
            .bpl(Operand::Rel(BranchTarget::Label("a".to_string())))
            .nop()
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(); // LDX #$10
    cpu.step(); // BPL b
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x0005);
    cpu.step(); // LDX #$F0
    cpu.step(); // BPL a
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x0009);
}

#[test]
fn test_brk() {
    let mut cpu = Cpu::new(Bus::default());
    cpu.bus.write(0xFFFE, 0x68); // IRQ vector (lo)
    cpu.bus.write(0xFFFF, 0x24); // IRQ vector (hi)
    cpu.pc = 0x0400;
    cpu.sp = 0xFF;

    let mut asm = Assembler::new();
    cpu.bus
        .load(cpu.pc, asm.brk().print_listing().assemble().unwrap());

    assert_eq!(stat(&cpu.sr), "nv-bdizc");
    cpu.step(); // BRK
    println!("{cpu:?}");
    assert_eq!(stat(&cpu.sr), "nv-bdizc");
    assert_eq_hex16!(cpu.pc, 0x2468);
    assert_eq_hex!(cpu.sp, 0xFC);
    assert_eq!(cpu.bus.read(0x01FF), 0x04);
    assert_eq!(cpu.bus.read(0x01FE), 0x02);
    assert_eq!(stat(&cpu.bus.read(0x01FD)), "nv-Bdizc");
}

#[test]
fn test_bvc() {
    let mut cpu = Cpu::new(Bus::default());

    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.bvc(Operand::Rel(BranchTarget::Offset(0x10)))
            .bvc(Operand::Rel(BranchTarget::Offset(0x20)))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    use pda6502v2emu::cpu::StatusMask;

    cpu.set_sr_bit(StatusMask::Overflow, true);
    cpu.step(); // BVC 0x10 (don't branch)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x0002);
    assert_eq!(stat(&cpu.sr), "nV-bdizc");

    cpu.set_sr_bit(StatusMask::Overflow, false);
    cpu.step(); // BVC 0x20 (do branch)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x0024);
    assert_eq!(stat(&cpu.sr), "nv-bdizc");
}

#[test]
fn test_bvs() {
    let mut cpu = Cpu::new(Bus::default());

    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.bvs(Operand::Rel(BranchTarget::Offset(0x10)))
            .bvs(Operand::Rel(BranchTarget::Offset(0x20)))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    use pda6502v2emu::cpu::StatusMask;

    cpu.set_sr_bit(StatusMask::Overflow, false);
    cpu.step(); // BVS 0x10 (don't branch)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x0002);
    assert_eq!(stat(&cpu.sr), "nv-bdizc");

    cpu.set_sr_bit(StatusMask::Overflow, true);
    cpu.step(); // BVS 0x20 (do branch)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x0024);
    assert_eq!(stat(&cpu.sr), "nV-bdizc");
}

#[test]
fn test_cmp() {
    let mut cpu = Cpu::new(Bus::default());
    cpu.pc = 0x0200;
    cpu.a = 0xC0;
    cpu.x = 0x03;
    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.org(cpu.pc)
            .label("prog")
            .cmp(Operand::Abs(label("data")))
            .cmp(Operand::AbsX(label("data")))
            .label("data")
            .data(vec![0xAA, 0xBB, 0xCC, 0xDD])
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(); // CMP data
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x0203);
    assert_eq!(stat(&cpu.sr), "nv-bdizc");
    cpu.step(); // CMP data,X
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x0206);
    assert_eq!(stat(&cpu.sr), "Nv-bdizC");
}

#[test]
fn test_cpx_and_cpy() {
    let mut cpu = Cpu::new(Bus::default());
    cpu.pc = 0x0200;
    cpu.x = 0x04;
    cpu.y = 0x04;
    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.org(cpu.pc)
            .label("prog")
            .cpx(Operand::Imm(0x04))
            .cpy(Operand::Imm(0x08))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(); // CPX #$04
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x0202);
    assert_eq!(stat(&cpu.sr), "nv-bdiZc");
    cpu.step(); // CPY #$08
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x0204);
    assert_eq!(stat(&cpu.sr), "Nv-bdizC");
}

#[test]
fn test_dec() {
    let mut cpu = Cpu::new(Bus::default());
    cpu.pc = 0x1000;
    cpu.x = 0x10;
    cpu.bus.write(0x0010, 100);
    cpu.bus.write(0x0020, 200);
    cpu.bus.write(0x2000, 1);
    cpu.bus.write(0x2010, 200);
    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.org(cpu.pc)
            .dec(Operand::Z(0x10))
            .dec(Operand::ZX(0x10))
            .dec(Operand::Abs(val(0x2000)))
            .dec(Operand::AbsX(val(0x2000)))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(); // DEC 0x10
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x1002);
    assert_eq!(stat(&cpu.sr), "nv-bdizc");
    assert_eq!(cpu.bus.read(0x0010), 99);

    cpu.step(); // DEC 0x10,X where X=10
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x1004);
    assert_eq!(stat(&cpu.sr), "Nv-bdizc");
    assert_eq!(cpu.bus.read(0x0020), 199);

    cpu.step(); // DEC 0x2000
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x1007);
    assert_eq!(stat(&cpu.sr), "nv-bdiZc");
    assert_eq!(cpu.bus.read(0x2000), 0);

    cpu.step(); // DEC 0x2000,X where X=10
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x100A);
    assert_eq!(stat(&cpu.sr), "Nv-bdizc");
    assert_eq!(cpu.bus.read(0x2010), 199);
}

#[test]
fn test_dex_and_dey() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.pc = 0x2000;
    cpu.x = 1;
    cpu.y = 1;
    cpu.bus.load(
        cpu.pc,
        asm.dex()
            .dex()
            .dey()
            .dey()
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(); // DEX
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x2001);
    assert_eq!(cpu.x, 0x00);
    assert_eq!(stat(&cpu.sr), "nv-bdiZc");

    cpu.step(); // DEX
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x2002);
    assert_eq!(cpu.x, 0xFF);
    assert_eq!(stat(&cpu.sr), "Nv-bdizc");

    cpu.step(); // DEY
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x2003);
    assert_eq!(cpu.y, 0x00);
    assert_eq!(stat(&cpu.sr), "nv-bdiZc");

    cpu.step(); // DEY
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x2004);
    assert_eq!(cpu.y, 0xFF);
    assert_eq!(stat(&cpu.sr), "Nv-bdizc");
}

#[test]
fn test_eor() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.pc = 0x1000;

    //     00000000      [0]
    // EOR 00000001      [1]
    //   = 00000001      [1a]
    // EOR 11111111      [2]
    //   = 11111110 (N)  [2a]
    // EOR 10101010      [3]
    //   = 01010100      [3a]
    // EOR 01010100      [4]
    //   = 00000000 (Z)  [4a]
    // EOR 11110000      [5]
    //   = 11110000 (N)  [5a]
    // EOR 00001111      [6]
    //   = 11111111 (N)  [6a]
    // EOR 00111100      [7]
    //   = 11000011 (N)  [7a]
    // EOR 11000011      [8]
    //   = 00000000 (Z)  [8a]

    cpu.a = 0b00000000; // [0]
    cpu.x = 0x02; // [3, 5, 7]
    cpu.y = 0x04; // [6, 8]
    cpu.bus.write(0x0010, 0b11111111); // [2]
    cpu.bus.write(0x12, 0b10101010); // [3]
    cpu.bus.write(0x22, 0x80); // [7] ptr LL
    cpu.bus.write(0x23, 0x20); // [7] ptr HH
    cpu.bus.write(0x2080, 0b00111100); // [7]
    cpu.bus.write(0x20, 0x81); // [8] ptr LL
    cpu.bus.write(0x21, 0x20); // [8] ptr HH
    cpu.bus.write(0x2085, 0b11000011); // [8]

    cpu.bus.load(
        cpu.pc,
        asm.org(cpu.pc)
            .eor(Operand::Imm(0x01)) // [1]
            .eor(Operand::Z(0x10)) // [2]
            .eor(Operand::ZX(0x10)) // [3]
            .eor(Operand::Abs(label("data"))) // [4]
            .eor(Operand::AbsX(label("data"))) // [5]
            .eor(Operand::AbsY(label("data"))) // [6]
            .eor(Operand::XInd(0x20)) // [7]
            .eor(Operand::IndY(0x20)) // [8]
            .label("data")
            .data(vec![0b01010100, 0, 0b11110000, 0, 0b00001111]) // [4], [5], [6]
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(); // EOR immediate
    println!("{cpu:?}");
    assert_eq_hex!(cpu.a, 0b00000001); // [1a]
    assert_eq!(stat(&cpu.sr), "nv-bdizc");

    cpu.step(); // EOR zeropage
    println!("{cpu:?}");
    assert_eq_hex!(cpu.a, 0b11111110); // [2a]
    assert_eq!(stat(&cpu.sr), "Nv-bdizc");

    cpu.step(); // EOR zeropage,X
    println!("{cpu:?}");
    assert_eq_hex!(cpu.a, 0b01010100); // [3a]
    assert_eq!(stat(&cpu.sr), "nv-bdizc");

    cpu.step(); // EOR absolute
    println!("{cpu:?}");
    assert_eq_hex!(cpu.a, 0b00000000); // [4a]
    assert_eq!(stat(&cpu.sr), "nv-bdiZc");

    cpu.step(); // EOR absolute,X
    println!("{cpu:?}");
    assert_eq_hex!(cpu.a, 0b11110000); // [5a]
    assert_eq!(stat(&cpu.sr), "Nv-bdizc");

    cpu.step(); // EOR absolute,Y
    println!("{cpu:?}");
    assert_eq_hex!(cpu.a, 0b11111111); // [6a]
    assert_eq!(stat(&cpu.sr), "Nv-bdizc");

    cpu.step(); // EOR (indirect,X)
    println!("{cpu:?}");
    assert_eq_hex!(cpu.a, 0b11000011); // [7a]
    assert_eq!(stat(&cpu.sr), "Nv-bdizc");

    cpu.step(); // EOR (indirect),Y
    println!("{cpu:?}");
    assert_eq_hex!(cpu.a, 0b00000000); // [8a]
    assert_eq!(stat(&cpu.sr), "nv-bdiZc");
}

#[test]
fn test_inc() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.x = 0x10;
    cpu.bus.write(0x0040, 0x00);
    cpu.bus.write(0x0050, 0xFF);
    cpu.bus.write(0x8000, 0x7F);
    cpu.bus.write(0x8010, 0x80);
    cpu.bus.load(
        cpu.pc,
        asm.inc(Operand::Z(0x40))
            .inc(Operand::ZX(0x40))
            .inc(Operand::Abs(val(0x8000)))
            .inc(Operand::AbsX(val(0x8000)))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(); // INC zeropage
    println!("{cpu:?}");
    assert_eq_hex!(cpu.bus.read(0x0040), 0x01);
    assert_eq!(stat(&cpu.sr), "nv-bdizc");

    cpu.step(); // INC zeropage,X
    println!("{cpu:?}");
    assert_eq_hex!(cpu.bus.read(0x0050), 0x00);
    assert_eq!(stat(&cpu.sr), "nv-bdiZc");

    cpu.step(); // INC absolute
    println!("{cpu:?}");
    assert_eq_hex!(cpu.bus.read(0x8000), 0x80);
    assert_eq!(stat(&cpu.sr), "Nv-bdizc");

    cpu.step(); // INC absolute,X
    println!("{cpu:?}");
    assert_eq_hex!(cpu.bus.read(0x8010), 0x81);
    assert_eq!(stat(&cpu.sr), "Nv-bdizc");
}

#[test]
fn test_inx_and_iny() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.x = 0xFE;
    cpu.y = 0xFE;
    cpu.bus.load(
        cpu.pc,
        asm.inx()
            .inx()
            .iny()
            .iny()
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(); // INX
    assert_eq!(cpu.x, 0xFF);
    assert_eq!(stat(&cpu.sr), "Nv-bdizc");

    cpu.step(); // INX
    assert_eq!(cpu.x, 0x00);
    assert_eq!(stat(&cpu.sr), "nv-bdiZc");

    cpu.step(); // INY
    assert_eq!(cpu.y, 0xFF);
    assert_eq!(stat(&cpu.sr), "Nv-bdizc");

    cpu.step(); // INY
    assert_eq!(cpu.y, 0x00);
    assert_eq!(stat(&cpu.sr), "nv-bdiZc");
}

#[test]
fn test_jmp() {
    let mut cpu = Cpu::new(Bus::default());
    cpu.bus.write(0xFFFC, 0x00);
    cpu.bus.write(0xFFFD, 0x80);
    cpu.reset(); // reset vector 0x8000

    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.org(cpu.pc)
            .jmp(Operand::Abs(label("testlabel")))
            .nop()
            .label("testlabel")
            .jmp(Operand::Ind(val(0xFFFC))) // back to start via reset vector
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(); // JMP testlabel
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x8004);
    assert_eq!(stat(&cpu.sr), "nv-BdIzc"); // unchanged

    cpu.sr = !cpu.sr;
    cpu.step(); // JMP ($FFFC)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x8000);
    assert_eq!(stat(&cpu.sr), "NV-bDiZC"); // unchanged
}

#[test]
fn test_jsr_and_rts() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.pc = 0x4000;
    cpu.sp = 0xFF;
    cpu.bus.load(
        cpu.pc,
        asm.org(cpu.pc)
            .jsr(Operand::Abs(label("first")))
            .data("something in the way".into())
            .label("first")
            .jsr(Operand::Abs(label("second")))
            .rts()
            .label("second")
            .rts()
            .print_listing()
            .assemble()
            .unwrap(),
    );

    println!("{cpu:?}");
    cpu.step(); // JSR first
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x4017);
    assert_eq_hex!(cpu.sp, 0xFD);
    assert_eq_hex!(cpu.bus.read(0x01FF), 0x40); // HH
    assert_eq_hex!(cpu.bus.read(0x01FE), 0x03); // LL
    cpu.step(); // JSR second
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x401B);
    assert_eq_hex!(cpu.sp, 0xFB);
    assert_eq_hex!(cpu.bus.read(0x01FD), 0x40); // HH
    assert_eq_hex!(cpu.bus.read(0x01FC), 0x1A); // LL (TODO: wrong?)
    cpu.step(); // RTS (from second)
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x401A);
    assert_eq_hex!(cpu.sp, 0xFD);
    cpu.step(); // RTS (from first)
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x4003);
    assert_eq_hex!(cpu.sp, 0xFF);
}

#[test]
fn test_lda() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.pc = 0x1000;
    cpu.bus.load(
        cpu.pc,
        asm.org(cpu.pc)
            .lda(Operand::Imm(0x00)) // 0x00
            .lda(Operand::Z(0xB0)) // 0x22
            .lda(Operand::ZX(0xB0)) // 0x44
            .lda(Operand::Abs(label("data"))) // 0x66
            .lda(Operand::AbsX(label("data"))) // 0x88
            .lda(Operand::AbsY(label("data"))) // 0xAA
            .lda(Operand::XInd(0xC0)) // 0xCC
            .lda(Operand::IndY(0xC0)) // 0xEE
            .label("data")
            .data(vec![0x66, 0, 0, 0, 0x88, 0xAA])
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.x = 0x04;
    cpu.y = 0x05;
    cpu.bus.write(0x00B0, 0x22);
    cpu.bus.write(0x00B4, 0x44);

    // (indirect,X): operand 0xC0 + x=0x04 = (0xC4) -> 0x00C6 = 0xCC
    cpu.bus.write(0x00C4, 0xC6); // LL
    cpu.bus.write(0x00C5, 0x00); // HH
    cpu.bus.write(0x00C6, 0xCC);

    // (indirect),Y: operand 0xC0 -> 0x00BD + y=0x05 = 0x00C2 = 0xEE
    cpu.bus.write(0x00C0, 0xBD); // LL
    cpu.bus.write(0x00C1, 0x00); // HH
    cpu.bus.write(0x00C2, 0xEE);

    step_and_assert!(cpu, a, 0x00, "nv-bdiZc");
    step_and_assert!(cpu, a, 0x22, "nv-bdizc");
    step_and_assert!(cpu, a, 0x44, "nv-bdizc");
    step_and_assert!(cpu, a, 0x66, "nv-bdizc");
    step_and_assert!(cpu, a, 0x88, "Nv-bdizc");
    step_and_assert!(cpu, a, 0xAA, "Nv-bdizc");
    step_and_assert!(cpu, a, 0xCC, "Nv-bdizc");
    step_and_assert!(cpu, a, 0xEE, "Nv-bdizc");
}

#[test]
fn test_ldx() {
    let mut cpu = Cpu::new(Bus::default());
    cpu.y = 0x02; // for testing AddressMode::ZeropageY & AddressMode::AbsoluteY
    cpu.bus.write(0x01FF, 0x11); // for testing AddressMode::Absolute
    cpu.bus.write(0x0201, 0x22); // for testing AddressMode::AbsoluteY

    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.ldx(Operand::Imm(0xAA))
            .ldx(Operand::Imm(0x00))
            .ldx(Operand::Z(0x04))
            .ldx(Operand::ZY(0x04))
            .ldx(Operand::Abs(val(0x01FF)))
            .ldx(Operand::AbsY(val(0x01FF)))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    step_and_assert!(cpu, x, 0xAA, "Nv-bdizc"); // LDX #$AA
    step_and_assert!(cpu, x, 0x00, "nv-bdiZc"); // LDX #$00
    step_and_assert!(cpu, x, 0xA6, "Nv-bdizc"); // LDX $04
    step_and_assert!(cpu, x, 0xB6, "Nv-bdizc"); // LDX $04,Y
    step_and_assert!(cpu, x, 0x11, "nv-bdizc"); // LDX $01FF ; Y=2
    step_and_assert!(cpu, x, 0x22, "nv-bdizc"); // LDX $01FF,Y ; Y=2
}

#[test]
fn test_ldy() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.pc = 0x1000;
    cpu.bus.load(
        cpu.pc,
        asm.org(cpu.pc)
            .ldy(Operand::Imm(0x00)) // 0x00
            .ldy(Operand::Z(0xB0)) // 0x22
            .ldy(Operand::ZX(0xB0)) // 0x44
            .ldy(Operand::Abs(label("data"))) // 0x66
            .ldy(Operand::AbsX(label("data"))) // 0x88
            .label("data")
            .data(vec![0x66, 0, 0, 0, 0x88])
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.x = 0x04;
    cpu.bus.write(0x00B0, 0x22);
    cpu.bus.write(0x00B4, 0x44);

    step_and_assert!(cpu, y, 0x00, "nv-bdiZc");
    step_and_assert!(cpu, y, 0x22, "nv-bdizc");
    step_and_assert!(cpu, y, 0x44, "nv-bdizc");
    step_and_assert!(cpu, y, 0x66, "nv-bdizc");
    step_and_assert!(cpu, y, 0x88, "Nv-bdizc");
}

#[test]
fn test_lsr() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.pc = 0x1000;
    cpu.bus.load(
        cpu.pc,
        asm.org(cpu.pc)
            .lsr(Operand::A)
            .lsr(Operand::Z(0x00))
            .lsr(Operand::ZX(0x00))
            .lsr(Operand::Abs(val(0x2000)))
            .lsr(Operand::AbsX(val(0x2000)))
            .label("data")
            .data(vec![0b10101010, 0, 0, 0, 0b01010101])
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.a = 0b11110000;
    cpu.x = 0xAA;
    cpu.bus.write(0x0000, 0b11111111);
    cpu.bus.write(0x00AA, 0b00000000);
    cpu.bus.write(0x2000, 0b10101010);
    cpu.bus.write(0x20AA, 0b01010101);

    step_and_assert!(cpu, a, 0b01111000, "nv-bdizc");
    step_and_assert_mem!(cpu, 0x0000, 0b01111111, "nv-bdizC");
    step_and_assert_mem!(cpu, 0x00AA, 0b00000000, "nv-bdiZc");
    step_and_assert_mem!(cpu, 0x2000, 0b01010101, "nv-bdizc");
    step_and_assert_mem!(cpu, 0x20AA, 0b00101010, "nv-bdizC");
}

#[test]
fn test_ora() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.pc = 0x1000;
    cpu.bus.load(
        cpu.pc,
        asm.org(cpu.pc)
            .ora(Operand::Imm(0x00))
            .ora(Operand::Z(0xA0))
            .ora(Operand::ZX(0xA0))
            .ora(Operand::Abs(label("data")))
            .ora(Operand::AbsX(label("data")))
            .ora(Operand::AbsY(label("data")))
            .ora(Operand::XInd(0x00))
            .ora(Operand::IndY(0x00))
            .label("data")
            .data(vec![0b00010000, 0, 0b00111100, 0, 0b10000000])
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.a = 0b00000000;
    cpu.x = 0x02;
    cpu.y = 0x04;
    cpu.bus.write(0x00A0, 0b00000001);
    cpu.bus.write(0x00A2, 0b00000100);

    // XInd(0x00+x:0x02)
    cpu.bus.write(0x0002, 0x10); // LL
    cpu.bus.write(0x0003, 0x32); // HH
    cpu.bus.write(0x3210, 0b01000000);

    // IndY(0x00)+y:0x04 = (0x3224)
    cpu.bus.write(0x0000, 0x20); // LL
    cpu.bus.write(0x0001, 0x32); // HH
    cpu.bus.write(0x3224, 0b11111111);

    step_and_assert!(cpu, a, 0b00000000, "nv-bdiZc");
    step_and_assert!(cpu, a, 0b00000001, "nv-bdizc");
    step_and_assert!(cpu, a, 0b00000101, "nv-bdizc");
    step_and_assert!(cpu, a, 0b00010101, "nv-bdizc");
    step_and_assert!(cpu, a, 0b00111101, "nv-bdizc");
    step_and_assert!(cpu, a, 0b10111101, "Nv-bdizc");
    step_and_assert!(cpu, a, 0b11111101, "Nv-bdizc");
    step_and_assert!(cpu, a, 0b11111111, "Nv-bdizc");
}

#[test]
fn test_nop() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.bus.load(cpu.pc, asm.nop().assemble().unwrap());
    cpu.step();
    assert_eq_hex16!(cpu.pc, 0x0001);
}

#[test]
fn test_set_and_clear_flags() {
    let mut cpu = Cpu::new(Bus::default());
    let mut asm = Assembler::new();
    cpu.bus.load(
        cpu.pc,
        asm.sec()
            .sed()
            .sei()
            .clc()
            .cld()
            .cli()
            .clv()
            .print_listing()
            .assemble()
            .unwrap(),
    );

    use pda6502v2emu::cpu::StatusMask;
    cpu.set_sr_bit(StatusMask::Overflow, true);

    assert_eq!(stat(&cpu.sr), "nV-bdizc");

    cpu.step(); // SEC
    println!("{cpu:?}");
    assert_eq!(stat(&cpu.sr), "nV-bdizC");
    cpu.step(); // SED
    println!("{cpu:?}");
    assert_eq!(stat(&cpu.sr), "nV-bDizC");
    cpu.step(); // SEI
    println!("{cpu:?}");
    assert_eq!(stat(&cpu.sr), "nV-bDIzC");
    cpu.step(); // CLC
    println!("{cpu:?}");
    assert_eq!(stat(&cpu.sr), "nV-bDIzc");
    cpu.step(); // CLD
    println!("{cpu:?}");
    assert_eq!(stat(&cpu.sr), "nV-bdIzc");
    cpu.step(); // CLI
    println!("{cpu:?}");
    assert_eq!(stat(&cpu.sr), "nV-bdizc");
    cpu.step(); // CLV
    println!("{cpu:?}");
    assert_eq!(stat(&cpu.sr), "nv-bdizc");
}
