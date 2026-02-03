# Rust RISC-V Simulator Library (rrs-lib)

This crate contains components to implement a RISC-V instruction set simulator.
It is designed to be modular so should prove useful for any application that
needs to decode or otherwise work with RISC-V instructions.

rrs-lib supports RV32IM, there is no privileged support, CSR instructions or
exceptions/interrupts (though these are planned).

## Usage

```toml
[dependencies]
rrs-lib = "0.1"
```

A key trait of rrs-lib is InstructionProcessor. It contains one function per
RISC-V instruction. The process_instruction function decodes an instruction and
calls the appropriate function in an InstructionProcessor. An
InstructionProcessor could execute an instruction, disassemble instruction or
something else entirely (e.g. compute statistical information on instructions
executed). rrs-lib provides InstructionProcessor implementations to execute
RISC-V code and produce instruction disassembly in a string. This example
demonstrates both:

```rust
use rrs_lib::{HartState, MemAccessSize, Memory};
use rrs_lib::memories::VecMemory;
use rrs_lib::instruction_executor::{InstructionExecutor, InstructionException};
use rrs_lib::instruction_string_outputter::InstructionStringOutputter;

fn simulate_riscv() {
  let mut hart = HartState::new();
  // Memory contains these instructions:
  // lui x2, 0x1234b
  // lui x3, 0xf387e
  // add x1, x2, x3
  let mut mem = VecMemory::new(vec![0x1234b137, 0xf387e1b7, 0x003100b3]);

  hart.pc = 0;

  // InstructionExecutor implements IntructionProcessor. The step function calls
  // process_instruction internally and handles things like updating the PC.
  let mut executor = InstructionExecutor {
      hart_state: &mut hart,
      mem: &mut mem,
  };

  // Execute first instruction
  output_disass(&mut executor);
  assert_eq!(executor.step(), Ok(()));
  assert_eq!(executor.hart_state.registers[2], 0x1234b000);

  // Execute second instruction
  output_disass(&mut executor);
  assert_eq!(executor.step(), Ok(()));
  assert_eq!(executor.hart_state.registers[3], 0xf387e000);

  // Execute third instruction
  output_disass(&mut executor);
  assert_eq!(executor.step(), Ok(()));
  assert_eq!(executor.hart_state.registers[1], 0x05bc9000);

  // Memory only contains three instructions so next step will produce a fetch error
  assert_eq!(executor.step(), Err(InstructionException::FetchError(0xc)));
}

fn output_disass<M: Memory>(executor: &mut InstructionExecutor<M>) {
  let mut outputter = InstructionStringOutputter { insn_pc: executor.hart_state.pc };
  let insn_bits = executor.mem.read_mem(executor.hart_state.pc, MemAccessSize::Word).unwrap();
  println!("{}", rrs_lib::process_instruction(&mut outputter, insn_bits).unwrap());
}
```

