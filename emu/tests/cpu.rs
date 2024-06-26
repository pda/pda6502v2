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
    ($cpu:expr, $bus:expr, $reg:ident, $val:expr, $stat:literal) => {
        $cpu.step($bus);
        println!("{:?}", $cpu);
        assert_eq_hex!($cpu.$reg, $val);
        assert_eq!(stat(&$cpu.p), $stat);
    };
}

macro_rules! step_and_assert_mem {
    ($cpu:expr, $bus:expr, $addr:expr, $val:expr, $stat:literal) => {
        $cpu.step($bus);
        println!("{:?}", $cpu);
        assert_eq_hex!($bus.read($addr), $val);
        assert_eq!(stat(&$cpu.p), $stat);
    };
}

#[test]
fn test_adc() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    cpu.a = 0x10; // starting value
    cpu.x = 0x21; // for testing X-indexed address modes
    cpu.y = 0x46; // for testing Y-indexed address modes
    bus.write(0x00F0, 0xEF); // for testing zero-page address mode
    bus.write(0x00F1, 0x70); // for testing zp,X address mode
    bus.write(0x00F2, 0x37); // for testing X,ind address mode (ptr LO)
    bus.write(0x00F3, 0x12); // for testing X,ind address mode (ptr HI)
    bus.write(0x00F4, 0xF2); // for testing ind,Y address mode (ptr LO)
    bus.write(0x00F5, 0x11); // for testing ind,Y address mode (ptr HI)
    bus.write(0x1234, 0x84); // for testing absolute address mode
    bus.write(0x1235, 0xFA); // for testing abs,X address mode
    bus.write(0x1236, 0x00); // for testing abs,X address mode
    bus.write(0x1237, 0x42); // for testing X,ind address mode (val)
    bus.write(0x1238, 0xBC); // for testing ind,Y address mode (val)

    use Operand::{Abs, AbsX, AbsY, Imm, IndY, XInd, Z, ZX};
    let mut asm = Assembler::new();
    bus.load(
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

    step_and_assert!(cpu, bus, a, 0x21, "nv-bdizc"); // ADC #$11
    step_and_assert!(cpu, bus, a, 0x10, "nv-bdizC"); // ADC $F0
    step_and_assert!(cpu, bus, a, 0x81, "NV-bdizc"); // ADC $D0,X
    step_and_assert!(cpu, bus, a, 0x05, "nV-bdizC"); // ADC $1234
    step_and_assert!(cpu, bus, a, 0x00, "nv-bdiZC"); // ADC $1214,X
    step_and_assert!(cpu, bus, a, 0x01, "nv-bdizc"); // ADC $12F0,Y
    step_and_assert!(cpu, bus, a, 0x43, "nv-bdizc"); // ADC ($D1,X)
    step_and_assert!(cpu, bus, a, 0xFF, "Nv-bdizc"); // ADC ($F4),Y
}

#[test]
fn test_and() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    cpu.a = 0b10011001; // starting value

    use Operand::Imm;
    let mut asm = Assembler::new();
    bus.load(
        cpu.pc,
        asm.and(Imm(0b11110000)).print_listing().assemble().unwrap(),
    );

    step_and_assert!(cpu, bus, a, 0b10010000, "Nv-bdizc"); // ADC #$A0
}

#[test]
fn test_asl() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    cpu.a = 0b01000000; // for testing Accumulator address mode
    cpu.x = 0x01; // for testing zp,X address mode
    bus.write(0xF1, 0b11011011); // for testing zp,X address mode

    let mut asm = Assembler::new();
    bus.load(
        cpu.pc,
        asm.asl(Operand::A)
            .asl(Operand::A)
            .asl(Operand::ZX(0xF0))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    use pda6502v2emu::cpu::StatusMask;

    cpu.set_p_bit(StatusMask::Carry, true);
    step_and_assert!(cpu, bus, a, 0b10000000, "Nv-bdizc"); // ASL A

    cpu.set_p_bit(StatusMask::Carry, true);
    step_and_assert!(cpu, bus, a, 0b00000000, "nv-bdiZC"); // ASL A

    cpu.step(bus); // ASL $F0,X
    println!("{:?}", cpu);
    let val = bus.read(0xF1);
    assert_eq_hex!(val, 0b10110110);
    assert_eq!(stat(&cpu.p), "Nv-bdizC");
}

#[test]
fn test_bcc() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();

    let mut asm = Assembler::new();
    bus.load(
        cpu.pc,
        asm.bcc(Operand::Rel(BranchTarget::Offset(0x10)))
            .bcc(Operand::Rel(BranchTarget::Offset(0x20)))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    use pda6502v2emu::cpu::StatusMask;

    cpu.set_p_bit(StatusMask::Carry, true);
    cpu.step(bus); // BCC 0x10 (don't branch)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x0002);
    assert_eq!(stat(&cpu.p), "nv-bdizC");

    cpu.set_p_bit(StatusMask::Carry, false);
    cpu.step(bus); // BCC 0x20 (do branch)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x0024);
    assert_eq!(stat(&cpu.p), "nv-bdizc");
}

#[test]
fn test_bcs() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    bus.load(
        cpu.pc,
        asm.bcs(Operand::Rel(BranchTarget::Offset(0x10)))
            .bcs(Operand::Rel(BranchTarget::Offset(0x20)))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    use pda6502v2emu::cpu::StatusMask;

    cpu.set_p_bit(StatusMask::Carry, false);
    cpu.step(bus); // BCS 0x10 (don't branch)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x0002);
    assert_eq!(stat(&cpu.p), "nv-bdizc");

    cpu.set_p_bit(StatusMask::Carry, true);
    cpu.step(bus); // BCS 0x20 (do branch)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x0024);
    assert_eq!(stat(&cpu.p), "nv-bdizC");
}

#[test]
fn test_beq() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    bus.load(
        cpu.pc,
        asm.beq(Operand::Rel(BranchTarget::Offset(0x10)))
            .beq(Operand::Rel(BranchTarget::Offset(0x20)))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    use pda6502v2emu::cpu::StatusMask;

    cpu.set_p_bit(StatusMask::Zero, false);
    cpu.step(bus); // BEQ 0x10 (don't branch)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x0002);
    assert_eq!(stat(&cpu.p), "nv-bdizc");

    cpu.set_p_bit(StatusMask::Zero, true);
    cpu.step(bus); // BEQ 0x20 (do branch)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x0024);
    assert_eq!(stat(&cpu.p), "nv-bdiZc");
}

#[test]
fn test_bit() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    bus.load(
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
    cpu.step(bus); // BIT $00 (#$FF)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x0004);
    assert_eq!(stat(&cpu.p), "NV-bdizc"); // 0b11111111 AND 0b11111111 = 0b11111111 = z

    cpu.a = 0x00;
    cpu.step(bus); // BIT $0001 (#$00)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x0007);
    assert_eq!(stat(&cpu.p), "nv-bdiZc"); // 0b00000000 AND 0b00000000 = 0b00000000 = Z
}

#[test]
fn test_bmi() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    bus.load(
        cpu.pc,
        asm.ldx(Operand::Imm(0xFF)) // P N=1
            .label("a")
            .bmi(Operand::Rel(BranchTarget::Label("b".to_string())))
            .nop()
            .label("b")
            .ldx(Operand::Imm(0x10)) // P N=0
            .bmi(Operand::Rel(BranchTarget::Label("a".to_string())))
            .nop()
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(bus); // LDX #$FF
    println!("{cpu:?}");
    cpu.step(bus); // BMI b
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x0005);
    cpu.step(bus); // LDX #$10
    cpu.step(bus); // BMI a
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x0009);
}

#[test]
fn test_bne() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    bus.load(
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

    cpu.step(bus); // LDX #$01
    cpu.step(bus); // BNE b
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x0005);

    cpu.step(bus); // LDX #$00
    cpu.step(bus); // BNE a
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x0009);
}

#[test]
fn test_bpl() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    bus.load(
        cpu.pc,
        asm.label("a")
            .ldx(Operand::Imm(0x10)) // P N=0
            .bpl(Operand::Rel(BranchTarget::Label("b".to_string())))
            .nop()
            .label("b")
            .ldx(Operand::Imm(0xF0)) // P N=1
            .bpl(Operand::Rel(BranchTarget::Label("a".to_string())))
            .nop()
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(bus); // LDX #$10
    cpu.step(bus); // BPL b
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x0005);
    cpu.step(bus); // LDX #$F0
    cpu.step(bus); // BPL a
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x0009);
}

#[test]
fn test_brk_rti() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    let mut asmirq = Assembler::new();

    cpu.pc = 0x1000;

    bus.load(
        cpu.pc,
        asm.org(cpu.pc)
            .brk()
            .data(vec![0xAA]) // “break mark: identify reason for break”
            .nop()
            .label("data")
            .print_listing()
            .assemble()
            .unwrap(),
    );

    // set the interrupt vector to 0x2000 which is the "irq" label
    bus.write(0xFFFE, 0x00); // IRQ vector (lo)
    bus.write(0xFFFF, 0x20); // IRQ vector (hi)

    bus.load(
        0x2000,
        asmirq
            .org(0x2000)
            .label("irq")
            .rti()
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.s = 0xF8;

    assert_eq!(stat(&cpu.p), "nv-bdizc");

    step_and_assert!(cpu, bus, s, 0xF5, "nv-bdIzc"); // BRK
    assert_eq_hex16!(cpu.pc, 0x2000);
    assert_eq_hex!(bus.read(0x01F8), 0x10); // S hi
    assert_eq_hex!(bus.read(0x01F7), 0x02); // S lo
    assert_eq!(stat(&bus.read(0x01F6)), "nv-Bdizc");

    step_and_assert!(cpu, bus, s, 0xF8, "nv-bdizc"); // RTI
    assert_eq_hex16!(cpu.pc, 0x1002);
}

#[test]
fn test_bvc() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();

    let mut asm = Assembler::new();
    bus.load(
        cpu.pc,
        asm.bvc(Operand::Rel(BranchTarget::Offset(0x10)))
            .bvc(Operand::Rel(BranchTarget::Offset(0x20)))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    use pda6502v2emu::cpu::StatusMask;

    cpu.set_p_bit(StatusMask::Overflow, true);
    cpu.step(bus); // BVC 0x10 (don't branch)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x0002);
    assert_eq!(stat(&cpu.p), "nV-bdizc");

    cpu.set_p_bit(StatusMask::Overflow, false);
    cpu.step(bus); // BVC 0x20 (do branch)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x0024);
    assert_eq!(stat(&cpu.p), "nv-bdizc");
}

#[test]
fn test_bvs() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();

    let mut asm = Assembler::new();
    bus.load(
        cpu.pc,
        asm.bvs(Operand::Rel(BranchTarget::Offset(0x10)))
            .bvs(Operand::Rel(BranchTarget::Offset(0x20)))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    use pda6502v2emu::cpu::StatusMask;

    cpu.set_p_bit(StatusMask::Overflow, false);
    cpu.step(bus); // BVS 0x10 (don't branch)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x0002);
    assert_eq!(stat(&cpu.p), "nv-bdizc");

    cpu.set_p_bit(StatusMask::Overflow, true);
    cpu.step(bus); // BVS 0x20 (do branch)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x0024);
    assert_eq!(stat(&cpu.p), "nV-bdizc");
}

#[test]
fn test_cmp() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    cpu.pc = 0x0200;
    cpu.a = 0xC0;
    cpu.x = 0x03;
    let mut asm = Assembler::new();
    bus.load(
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

    cpu.step(bus); // CMP data
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x0203);
    assert_eq!(stat(&cpu.p), "nv-bdizC");
    cpu.step(bus); // CMP data,X
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x0206);
    assert_eq!(stat(&cpu.p), "Nv-bdizc");
}

#[test]
fn test_cpx_and_cpy() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    cpu.pc = 0x0200;
    cpu.x = 0x04;
    cpu.y = 0x04;
    let mut asm = Assembler::new();
    bus.load(
        cpu.pc,
        asm.org(cpu.pc)
            .label("prog")
            .cpx(Operand::Imm(0x04))
            .cpy(Operand::Imm(0x08))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(bus); // CPX #$04
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x0202);
    assert_eq!(stat(&cpu.p), "nv-bdiZc");
    cpu.step(bus); // CPY #$08
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x0204);
    assert_eq!(stat(&cpu.p), "Nv-bdizC");
}

#[test]
fn test_dec() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    cpu.pc = 0x1000;
    cpu.x = 0x10;
    bus.write(0x0010, 100);
    bus.write(0x0020, 200);
    bus.write(0x2000, 1);
    bus.write(0x2010, 200);
    let mut asm = Assembler::new();
    bus.load(
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

    cpu.step(bus); // DEC 0x10
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x1002);
    assert_eq!(stat(&cpu.p), "nv-bdizc");
    assert_eq!(bus.read(0x0010), 99);

    cpu.step(bus); // DEC 0x10,X where X=10
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x1004);
    assert_eq!(stat(&cpu.p), "Nv-bdizc");
    assert_eq!(bus.read(0x0020), 199);

    cpu.step(bus); // DEC 0x2000
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x1007);
    assert_eq!(stat(&cpu.p), "nv-bdiZc");
    assert_eq!(bus.read(0x2000), 0);

    cpu.step(bus); // DEC 0x2000,X where X=10
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x100A);
    assert_eq!(stat(&cpu.p), "Nv-bdizc");
    assert_eq!(bus.read(0x2010), 199);
}

#[test]
fn test_dex_and_dey() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    cpu.pc = 0x2000;
    cpu.x = 1;
    cpu.y = 1;
    bus.load(
        cpu.pc,
        asm.dex()
            .dex()
            .dey()
            .dey()
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(bus); // DEX
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x2001);
    assert_eq!(cpu.x, 0x00);
    assert_eq!(stat(&cpu.p), "nv-bdiZc");

    cpu.step(bus); // DEX
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x2002);
    assert_eq!(cpu.x, 0xFF);
    assert_eq!(stat(&cpu.p), "Nv-bdizc");

    cpu.step(bus); // DEY
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x2003);
    assert_eq!(cpu.y, 0x00);
    assert_eq!(stat(&cpu.p), "nv-bdiZc");

    cpu.step(bus); // DEY
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x2004);
    assert_eq!(cpu.y, 0xFF);
    assert_eq!(stat(&cpu.p), "Nv-bdizc");
}

#[test]
fn test_eor() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
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
    bus.write(0x0010, 0b11111111); // [2]
    bus.write(0x12, 0b10101010); // [3]
    bus.write(0x22, 0x80); // [7] ptr LL
    bus.write(0x23, 0x20); // [7] ptr HH
    bus.write(0x2080, 0b00111100); // [7]
    bus.write(0x20, 0x81); // [8] ptr LL
    bus.write(0x21, 0x20); // [8] ptr HH
    bus.write(0x2085, 0b11000011); // [8]

    bus.load(
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

    cpu.step(bus); // EOR immediate
    println!("{cpu:?}");
    assert_eq_hex!(cpu.a, 0b00000001); // [1a]
    assert_eq!(stat(&cpu.p), "nv-bdizc");

    cpu.step(bus); // EOR zeropage
    println!("{cpu:?}");
    assert_eq_hex!(cpu.a, 0b11111110); // [2a]
    assert_eq!(stat(&cpu.p), "Nv-bdizc");

    cpu.step(bus); // EOR zeropage,X
    println!("{cpu:?}");
    assert_eq_hex!(cpu.a, 0b01010100); // [3a]
    assert_eq!(stat(&cpu.p), "nv-bdizc");

    cpu.step(bus); // EOR absolute
    println!("{cpu:?}");
    assert_eq_hex!(cpu.a, 0b00000000); // [4a]
    assert_eq!(stat(&cpu.p), "nv-bdiZc");

    cpu.step(bus); // EOR absolute,X
    println!("{cpu:?}");
    assert_eq_hex!(cpu.a, 0b11110000); // [5a]
    assert_eq!(stat(&cpu.p), "Nv-bdizc");

    cpu.step(bus); // EOR absolute,Y
    println!("{cpu:?}");
    assert_eq_hex!(cpu.a, 0b11111111); // [6a]
    assert_eq!(stat(&cpu.p), "Nv-bdizc");

    cpu.step(bus); // EOR (indirect,X)
    println!("{cpu:?}");
    assert_eq_hex!(cpu.a, 0b11000011); // [7a]
    assert_eq!(stat(&cpu.p), "Nv-bdizc");

    cpu.step(bus); // EOR (indirect),Y
    println!("{cpu:?}");
    assert_eq_hex!(cpu.a, 0b00000000); // [8a]
    assert_eq!(stat(&cpu.p), "nv-bdiZc");
}

#[test]
fn test_inc() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    cpu.x = 0x10;
    bus.write(0x0040, 0x00);
    bus.write(0x0050, 0xFF);
    bus.write(0x8000, 0x7F);
    bus.write(0x8010, 0x80);
    bus.load(
        cpu.pc,
        asm.inc(Operand::Z(0x40))
            .inc(Operand::ZX(0x40))
            .inc(Operand::Abs(val(0x8000)))
            .inc(Operand::AbsX(val(0x8000)))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(bus); // INC zeropage
    println!("{cpu:?}");
    assert_eq_hex!(bus.read(0x0040), 0x01);
    assert_eq!(stat(&cpu.p), "nv-bdizc");

    cpu.step(bus); // INC zeropage,X
    println!("{cpu:?}");
    assert_eq_hex!(bus.read(0x0050), 0x00);
    assert_eq!(stat(&cpu.p), "nv-bdiZc");

    cpu.step(bus); // INC absolute
    println!("{cpu:?}");
    assert_eq_hex!(bus.read(0x8000), 0x80);
    assert_eq!(stat(&cpu.p), "Nv-bdizc");

    cpu.step(bus); // INC absolute,X
    println!("{cpu:?}");
    assert_eq_hex!(bus.read(0x8010), 0x81);
    assert_eq!(stat(&cpu.p), "Nv-bdizc");
}

#[test]
fn test_inx_and_iny() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    cpu.x = 0xFE;
    cpu.y = 0xFE;
    bus.load(
        cpu.pc,
        asm.inx()
            .inx()
            .iny()
            .iny()
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.step(bus); // INX
    assert_eq!(cpu.x, 0xFF);
    assert_eq!(stat(&cpu.p), "Nv-bdizc");

    cpu.step(bus); // INX
    assert_eq!(cpu.x, 0x00);
    assert_eq!(stat(&cpu.p), "nv-bdiZc");

    cpu.step(bus); // INY
    assert_eq!(cpu.y, 0xFF);
    assert_eq!(stat(&cpu.p), "Nv-bdizc");

    cpu.step(bus); // INY
    assert_eq!(cpu.y, 0x00);
    assert_eq!(stat(&cpu.p), "nv-bdiZc");
}

#[test]
fn test_jmp() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    bus.write(0xFFFC, 0x00);
    bus.write(0xFFFD, 0x80);
    cpu.reset(bus); // reset vector 0x8000

    let mut asm = Assembler::new();
    bus.load(
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

    cpu.step(bus); // JMP testlabel
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x8004);
    assert_eq!(stat(&cpu.p), "nv-BdIzc"); // unchanged

    cpu.p = !cpu.p;
    cpu.step(bus); // JMP ($FFFC)
    println!("{:?}", cpu);
    assert_eq_hex16!(cpu.pc, 0x8000);
    assert_eq!(stat(&cpu.p), "NV-bDiZC"); // unchanged
}

#[test]
fn test_jsr_and_rts() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    cpu.pc = 0x4000;
    cpu.s = 0xFF;
    bus.load(
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
    cpu.step(bus); // JP first
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x4017);
    assert_eq_hex!(cpu.s, 0xFD);
    assert_eq_hex!(bus.read(0x01FF), 0x40); // HH
    assert_eq_hex!(bus.read(0x01FE), 0x03); // LL
    cpu.step(bus); // JP second
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x401B);
    assert_eq_hex!(cpu.s, 0xFB);
    assert_eq_hex!(bus.read(0x01FD), 0x40); // HH
    assert_eq_hex!(bus.read(0x01FC), 0x1A); // LL (TODO: wrong?)
    cpu.step(bus); // RTS (from second)
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x401A);
    assert_eq_hex!(cpu.s, 0xFD);
    cpu.step(bus); // RTS (from first)
    println!("{cpu:?}");
    assert_eq_hex16!(cpu.pc, 0x4003);
    assert_eq_hex!(cpu.s, 0xFF);
}

#[test]
fn test_lda() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    cpu.pc = 0x1000;
    bus.load(
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
    bus.write(0x00B0, 0x22);
    bus.write(0x00B4, 0x44);

    // (indirect,X): operand 0xC0 + x=0x04 = (0xC4) -> 0x00C6 = 0xCC
    bus.write(0x00C4, 0xC6); // LL
    bus.write(0x00C5, 0x00); // HH
    bus.write(0x00C6, 0xCC);

    // (indirect),Y: operand 0xC0 -> 0x00BD + y=0x05 = 0x00C2 = 0xEE
    bus.write(0x00C0, 0xBD); // LL
    bus.write(0x00C1, 0x00); // HH
    bus.write(0x00C2, 0xEE);

    step_and_assert!(cpu, bus, a, 0x00, "nv-bdiZc");
    step_and_assert!(cpu, bus, a, 0x22, "nv-bdizc");
    step_and_assert!(cpu, bus, a, 0x44, "nv-bdizc");
    step_and_assert!(cpu, bus, a, 0x66, "nv-bdizc");
    step_and_assert!(cpu, bus, a, 0x88, "Nv-bdizc");
    step_and_assert!(cpu, bus, a, 0xAA, "Nv-bdizc");
    step_and_assert!(cpu, bus, a, 0xCC, "Nv-bdizc");
    step_and_assert!(cpu, bus, a, 0xEE, "Nv-bdizc");
}

#[test]
fn test_ldx() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    cpu.y = 0x02; // for testing AddressMode::ZeropageY & AddressMode::AbsoluteY
    bus.write(0x01FF, 0x11); // for testing AddressMode::Absolute
    bus.write(0x0201, 0x22); // for testing AddressMode::AbsoluteY

    let mut asm = Assembler::new();
    bus.load(
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

    step_and_assert!(cpu, bus, x, 0xAA, "Nv-bdizc"); // LDX #$AA
    step_and_assert!(cpu, bus, x, 0x00, "nv-bdiZc"); // LDX #$00
    step_and_assert!(cpu, bus, x, 0xA6, "Nv-bdizc"); // LDX $04
    step_and_assert!(cpu, bus, x, 0xB6, "Nv-bdizc"); // LDX $04,Y
    step_and_assert!(cpu, bus, x, 0x11, "nv-bdizc"); // LDX $01FF ; Y=2
    step_and_assert!(cpu, bus, x, 0x22, "nv-bdizc"); // LDX $01FF,Y ; Y=2
}

#[test]
fn test_ldy() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    cpu.pc = 0x1000;
    bus.load(
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
    bus.write(0x00B0, 0x22);
    bus.write(0x00B4, 0x44);

    step_and_assert!(cpu, bus, y, 0x00, "nv-bdiZc");
    step_and_assert!(cpu, bus, y, 0x22, "nv-bdizc");
    step_and_assert!(cpu, bus, y, 0x44, "nv-bdizc");
    step_and_assert!(cpu, bus, y, 0x66, "nv-bdizc");
    step_and_assert!(cpu, bus, y, 0x88, "Nv-bdizc");
}

#[test]
fn test_lsr() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    cpu.pc = 0x1000;
    bus.load(
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

    cpu.a = 0b11110001;
    cpu.x = 0xAA;
    bus.write(0x0000, 0b11111111);
    bus.write(0x00AA, 0b00000000);
    bus.write(0x2000, 0b10101010);
    bus.write(0x20AA, 0b01010101);

    step_and_assert!(cpu, bus, a, 0b01111000, "nv-bdizC");
    step_and_assert_mem!(cpu, bus, 0x0000, 0b01111111, "nv-bdizC");
    step_and_assert_mem!(cpu, bus, 0x00AA, 0b00000000, "nv-bdiZc");
    step_and_assert_mem!(cpu, bus, 0x2000, 0b01010101, "nv-bdizc");
    step_and_assert_mem!(cpu, bus, 0x20AA, 0b00101010, "nv-bdizC");
}

#[test]
fn test_ora() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    cpu.pc = 0x1000;
    bus.load(
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
    bus.write(0x00A0, 0b00000001);
    bus.write(0x00A2, 0b00000100);

    // XInd(0x00+x:0x02)
    bus.write(0x0002, 0x10); // LL
    bus.write(0x0003, 0x32); // HH
    bus.write(0x3210, 0b01000000);

    // IndY(0x00)+y:0x04 = (0x3224)
    bus.write(0x0000, 0x20); // LL
    bus.write(0x0001, 0x32); // HH
    bus.write(0x3224, 0b11111111);

    step_and_assert!(cpu, bus, a, 0b00000000, "nv-bdiZc");
    step_and_assert!(cpu, bus, a, 0b00000001, "nv-bdizc");
    step_and_assert!(cpu, bus, a, 0b00000101, "nv-bdizc");
    step_and_assert!(cpu, bus, a, 0b00010101, "nv-bdizc");
    step_and_assert!(cpu, bus, a, 0b00111101, "nv-bdizc");
    step_and_assert!(cpu, bus, a, 0b10111101, "Nv-bdizc");
    step_and_assert!(cpu, bus, a, 0b11111101, "Nv-bdizc");
    step_and_assert!(cpu, bus, a, 0b11111111, "Nv-bdizc");
}

#[test]
fn test_pha_pla() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    bus.load(0, asm.pha().pla().print_listing().assemble().unwrap());

    cpu.s = 0xA8;
    cpu.a = 0xF0;

    step_and_assert!(cpu, bus, s, 0xA7, "nv-bdizc"); // PHA
    assert_eq_hex!(bus.read(0x01A8), 0xF0);

    cpu.a = 0xAA;

    step_and_assert!(cpu, bus, s, 0xA8, "Nv-bdizc"); // PLA
    assert_eq_hex!(cpu.a, 0xF0);
}

#[test]
fn test_php_plp() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    bus.load(0, asm.php().plp().print_listing().assemble().unwrap());

    cpu.p = 0b00000100;
    cpu.s = 0xA8;

    step_and_assert!(cpu, bus, s, 0xA7, "nv-bdIzc"); // PHP
    assert_eq_hex!(bus.read(0x01A8), 0b00110100);

    cpu.p = 0b11111111;
    assert_eq!(stat(&cpu.p), "NV-BDIZC");

    step_and_assert!(cpu, bus, s, 0xA8, "nv-bdIzc"); // PLP
}

#[test]
fn test_rol_ror() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    cpu.pc = 0x1000;
    bus.load(
        cpu.pc,
        asm.rol(Operand::A)
            .rol(Operand::A)
            .rol(Operand::Z(0x80))
            .rol(Operand::ZX(0x80))
            .rol(Operand::Abs(val(0x2000)))
            .rol(Operand::AbsX(val(0x2000)))
            .ror(Operand::AbsX(val(0x2000)))
            .ror(Operand::Abs(val(0x2000)))
            .ror(Operand::ZX(0x80))
            .ror(Operand::Z(0x80))
            .ror(Operand::A)
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.a = 0b10000000;
    cpu.x = 0x42;
    bus.write(0x0080, 0b11110000);
    bus.write(0x00C2, 0b01010101);
    bus.write(0x2000, 0b11001100);
    bus.write(0x2042, 0b10101010);

    step_and_assert!(cpu, bus, a, 0b00000000, "nv-bdiZC"); // ROL A
    step_and_assert!(cpu, bus, a, 0b00000001, "nv-bdizc"); // ROL A
    step_and_assert_mem!(cpu, bus, 0x0080, 0b11100000, "Nv-bdizC"); // ROL $80
    step_and_assert_mem!(cpu, bus, 0x00C2, 0b10101011, "Nv-bdizc"); // ROL $80,X
    step_and_assert_mem!(cpu, bus, 0x2000, 0b10011000, "Nv-bdizC"); // ROL $2000
    step_and_assert_mem!(cpu, bus, 0x2042, 0b01010101, "nv-bdizC"); // ROL $2000,X

    step_and_assert_mem!(cpu, bus, 0x2042, 0b10101010, "Nv-bdizC"); // ROR $2000,X
    step_and_assert_mem!(cpu, bus, 0x2000, 0b11001100, "Nv-bdizc"); // ROR $2000
    step_and_assert_mem!(cpu, bus, 0x00C2, 0b01010101, "nv-bdizC"); // ROR $80,X
    step_and_assert_mem!(cpu, bus, 0x0080, 0b11110000, "Nv-bdizc"); // ROR $80
    step_and_assert!(cpu, bus, a, 0b00000000, "nv-bdiZC"); // ROR A
}

#[test]
fn test_sbc() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    cpu.pc = 0x1000;
    bus.load(
        cpu.pc,
        asm.org(cpu.pc)
            .sec()
            .sbc(Operand::Imm(0x10))
            .sbc(Operand::Z(0x00))
            .sbc(Operand::ZX(0x00))
            .sbc(Operand::Abs(val(0x2000)))
            .sbc(Operand::AbsX(val(0x2000)))
            .sbc(Operand::AbsY(val(0x2000)))
            .sbc(Operand::XInd(0x10))
            .sbc(Operand::IndY(0x20))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.a = 0x00;
    cpu.x = 0x02;
    cpu.y = 0x04;

    bus.write(0x0000, 0x32);
    bus.write(0x0002, 0x20);
    bus.write(0x2000, 0xFF);
    bus.write(0x2002, 0x1D);
    bus.write(0x2004, 0x01);

    // X=2; ($10,X) -> ($12) -> $3210 -> #$40
    bus.write(0x0012, 0x10); // LL
    bus.write(0x0013, 0x32); // HH
    bus.write(0x3210, 0x40);

    // Y=4; ($20),Y -> $4321,Y -> $4325 -> #$3F
    bus.write(0x0020, 0x21); // LL
    bus.write(0x0021, 0x43); // HH
    bus.write(0x4325, 0x3F);

    step_and_assert!(cpu, bus, a, 0x00, "nv-bdizC"); // SEC
    step_and_assert!(cpu, bus, a, 0xF0, "Nv-bdizc"); // SBC #$10     ; 0x00 - 0x10     = 0xF0 (c)
    step_and_assert!(cpu, bus, a, 0xBD, "Nv-bdizC"); // SBC $00      ; 0xF0 - 0x32 - 1 = 0xBD
    step_and_assert!(cpu, bus, a, 0x9D, "Nv-bdizC"); // SBC $00,X    ; 0xBD - 0x20     = 0x9D
    step_and_assert!(cpu, bus, a, 0x9E, "Nv-bdizc"); // SBC $2000    ; 0x9D - 0xFF     = 0x9E (c)
    step_and_assert!(cpu, bus, a, 0x80, "Nv-bdizC"); // SBC $2000,X  ; 0x9E - 0x1D - 1 = 0x80
    step_and_assert!(cpu, bus, a, 0x7F, "nV-bdizC"); // SBC $2000,Y  ; 0x80 - 0x01     = 0x7F
    step_and_assert!(cpu, bus, a, 0x3F, "nv-bdizC"); // SBC ($10,X)  ; 0x7F - 0x40     = 0x3F
    step_and_assert!(cpu, bus, a, 0x00, "nv-bdiZC"); // SBC ($20),Y  ; 0x3F - 0x3F     = 0x00
}

#[test]
fn test_nop() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    bus.load(cpu.pc, asm.nop().assemble().unwrap());
    cpu.step(bus);
    assert_eq_hex16!(cpu.pc, 0x0001);
}

#[test]
fn test_set_and_clear_flags() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    bus.load(
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
    cpu.set_p_bit(StatusMask::Overflow, true);

    assert_eq!(stat(&cpu.p), "nV-bdizc");

    cpu.step(bus); // SEC
    println!("{cpu:?}");
    assert_eq!(stat(&cpu.p), "nV-bdizC");
    cpu.step(bus); // SED
    println!("{cpu:?}");
    assert_eq!(stat(&cpu.p), "nV-bDizC");
    cpu.step(bus); // SEI
    println!("{cpu:?}");
    assert_eq!(stat(&cpu.p), "nV-bDIzC");
    cpu.step(bus); // CLC
    println!("{cpu:?}");
    assert_eq!(stat(&cpu.p), "nV-bDIzc");
    cpu.step(bus); // CLD
    println!("{cpu:?}");
    assert_eq!(stat(&cpu.p), "nV-bdIzc");
    cpu.step(bus); // CLI
    println!("{cpu:?}");
    assert_eq!(stat(&cpu.p), "nV-bdizc");
    cpu.step(bus); // CLV
    println!("{cpu:?}");
    assert_eq!(stat(&cpu.p), "nv-bdizc");
}

#[test]
fn test_sta_stx_sty() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    cpu.pc = 0x4000;
    bus.load(
        cpu.pc,
        asm.sta(Operand::Z(0x00))
            .sta(Operand::ZX(0x00))
            .sta(Operand::Abs(val(0x2000)))
            .sta(Operand::AbsX(val(0x2000)))
            .sta(Operand::AbsY(val(0x2000)))
            .sta(Operand::XInd(0x10))
            .sta(Operand::IndY(0x20))
            .stx(Operand::Z(0x30))
            .stx(Operand::ZY(0x30))
            .stx(Operand::Abs(val(0x3000)))
            .sty(Operand::Z(0x40))
            .sty(Operand::ZX(0x40))
            .sty(Operand::Abs(val(0x5000)))
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.a = 0xAA;
    cpu.x = 0x02;
    cpu.y = 0x04;

    // ($10,X) -> $1234
    bus.write(0x0012, 0x34);
    bus.write(0x0013, 0x12);

    // ($20),Y -> $5678
    bus.write(0x0020, 0x74);
    bus.write(0x0021, 0x56);

    step_and_assert_mem!(cpu, bus, 0x0000, 0xAA, "nv-bdizc"); // STA $00
    step_and_assert_mem!(cpu, bus, 0x0002, 0xAA, "nv-bdizc"); // STA $00,X
    step_and_assert_mem!(cpu, bus, 0x2000, 0xAA, "nv-bdizc"); // STA $2000
    step_and_assert_mem!(cpu, bus, 0x2002, 0xAA, "nv-bdizc"); // STA $2000,X
    step_and_assert_mem!(cpu, bus, 0x2004, 0xAA, "nv-bdizc"); // STA $2000,Y
    step_and_assert_mem!(cpu, bus, 0x1234, 0xAA, "nv-bdizc"); // STA ($10,X)
    step_and_assert_mem!(cpu, bus, 0x5678, 0xAA, "nv-bdizc"); // STA ($20),Y

    step_and_assert_mem!(cpu, bus, 0x0030, 0x02, "nv-bdizc"); // STX $30
    step_and_assert_mem!(cpu, bus, 0x0034, 0x02, "nv-bdizc"); // STX $30,Y
    step_and_assert_mem!(cpu, bus, 0x3000, 0x02, "nv-bdizc"); // STX $3000
                                                              //
    step_and_assert_mem!(cpu, bus, 0x0040, 0x04, "nv-bdizc"); // STY $40
    step_and_assert_mem!(cpu, bus, 0x0042, 0x04, "nv-bdizc"); // STY $40,X
    step_and_assert_mem!(cpu, bus, 0x5000, 0x04, "nv-bdizc"); // STY $5000
}

#[test]
fn test_tax_tay_tsx_txa_txs_tya() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    cpu.pc = 0x4000;
    bus.load(
        cpu.pc,
        asm.tax()
            .tay()
            .tsx()
            .txa()
            .txs()
            .tya()
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.a = 0x00;
    cpu.x = 0x7F;
    cpu.y = 0xBF;
    cpu.s = 0xFF;
    step_and_assert!(cpu, bus, x, 0x00, "nv-bdiZc"); // TAX
    step_and_assert!(cpu, bus, y, 0x00, "nv-bdiZc"); // TAY
    step_and_assert!(cpu, bus, x, 0xFF, "Nv-bdizc"); // TSX
    cpu.a = 0x00;
    cpu.x = 0x7F;
    cpu.y = 0xBF;
    cpu.s = 0xFF;
    step_and_assert!(cpu, bus, a, 0x7F, "nv-bdizc"); // TXA
    step_and_assert!(cpu, bus, s, 0x7F, "nv-bdizc"); // TXS
    step_and_assert!(cpu, bus, a, 0xBF, "Nv-bdizc"); // TYA
}

#[test]
fn test_address_modes_at_page_boundaries() {
    let bus = &mut Bus::new();
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    cpu.pc = 0x10FE; // opcode @ $10FE, LL @ 0x10FF, operand:HH @ 0x1100 (next page)
    bus.load(
        cpu.pc,
        asm.org(cpu.pc)
            .lda(Operand::Abs(val(0x2000))) // absolute operand crosses page
            .lda(Operand::AbsX(val(0x20FF))) // increment by X crosses page
            .lda(Operand::AbsY(val(0x20FF))) // increment by Y crosses page
            .jmp(Operand::Ind(val(0x30FF))) // indirect pointer crosses page
            .lda(Operand::XInd(0xFD)) // indirect wraps after indexing ZP by X
            .lda(Operand::XInd(0xFF)) // indirect wraps before indexing ZP by X
            .lda(Operand::IndY(0xFF)) // indirect wraps before indexing ptr by Y
            .lda(Operand::ZX(0xFF)) // ZP wraps incrementing by X
            .ldx(Operand::ZY(0xFF)) // ZP wraps incrementing by Y
            .print_listing()
            .assemble()
            .unwrap(),
    );

    cpu.x = 0x02;
    cpu.y = 0x04;

    bus.write(0x2000, 0x11);
    step_and_assert!(cpu, bus, a, 0x11, "nv-bdizc"); // LDA $2000

    bus.write(0x2101, 0x22); // $20FF + X
    step_and_assert!(cpu, bus, a, 0x22, "nv-bdizc"); // LDA $20FF,X
                                                     //
    bus.write(0x2103, 0x33); // $20FF + Y
    step_and_assert!(cpu, bus, a, 0x33, "nv-bdizc"); // LDA $20FF,Y

    // JMP is the only (non-indexed) indirect addressing instruction,
    // so use that to test a page-crossing indirect pointer.
    bus.write(0x30FF, 0x01); // indirect:LL
    bus.write(0x3100, 0x31); // indirect:HH
    bus.load(
        0x3101,
        Assembler::new()
            .org(0x3101)
            .jmp(Operand::Abs(val(0x110A))) // after the JMP that took us here
            .print_listing()
            .assemble()
            .unwrap(),
    );
    step_and_assert!(cpu, bus, pc, 0x3101, "nv-bdizc"); // JMP ($30FF)
    step_and_assert!(cpu, bus, pc, 0x110A, "nv-bdizc"); // JMP $110A (back to where we were)

    // ($FD,X) where X=0x02
    bus.write(0x00FF, 0x21); // LL
    bus.write(0x0000, 0x43); // HH
    bus.write(0x4321, 0x44);
    step_and_assert!(cpu, bus, a, 0x44, "nv-bdizc"); // LDA ($FF,X)

    // ($FF,X) where X=0x02
    bus.write(0x0001, 0x21); // LL
    bus.write(0x0002, 0x43); // HH
    bus.write(0x4321, 0x55);
    step_and_assert!(cpu, bus, a, 0x55, "nv-bdizc"); // LDA ($FF,X)

    bus.write(0x00FF, 0x32); // LL
    bus.write(0x0000, 0x54); // HH
    bus.write(0x5436, 0x66); // 0x5432 + Y:4
    step_and_assert!(cpu, bus, a, 0x66, "nv-bdizc"); // LDA ($FF),Y

    bus.write(0x0001, 0x77);
    step_and_assert!(cpu, bus, a, 0x77, "nv-bdizc"); // LDA $FF,X

    bus.write(0x0003, 0x88);
    step_and_assert!(cpu, bus, x, 0x88, "Nv-bdizc"); // LDX $FF,Y
}
