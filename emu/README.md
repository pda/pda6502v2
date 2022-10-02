pda6502v2 / emu
===============

A work-in-progress emulator for the work-in-progress pda6502v2 computer.

- 6502 CPU emulator (specifically: W65C02)
- 6502 assembler (API driven; non-parsing)
- …

Similar to https://github.com/pda/go6502 but:

- for [pda6502v2](https://github.com/pda/pda6502v2) not [pda6502](https://github.com/pda/pda6502).
- written in Rust, not Go.
- even less finished.


Development
-----------

Run the tests:

```shell-session
$ cargo test
```

Verbose tests (assembly and CPU state shown):

```shell-session
$ cargo test -- --nocapture
```

Run the emulator:

```shell-session
$ cargo run
```
