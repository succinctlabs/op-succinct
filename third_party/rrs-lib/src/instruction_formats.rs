// Copyright 2021 Gregory Chadwick <mail@gregchadwick.co.uk>
// Licensed under the Apache License Version 2.0, with LLVM Exceptions, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Structures and constants for instruction decoding
//!
//! The structures directly relate to the RISC-V instruction formats described in the
//! specification. See the [RISC-V specification](https://riscv.org/technical/specifications/) for
//! further details

pub const OPCODE_LOAD: u32 = 0x03;
pub const OPCODE_MISC_MEM: u32 = 0x0f;
pub const OPCODE_OP_IMM: u32 = 0x13;
pub const OPCODE_AUIPC: u32 = 0x17;
pub const OPCODE_STORE: u32 = 0x23;
pub const OPCODE_OP: u32 = 0x33;
pub const OPCODE_LUI: u32 = 0x37;
pub const OPCODE_BRANCH: u32 = 0x63;
pub const OPCODE_JALR: u32 = 0x67;
pub const OPCODE_JAL: u32 = 0x6f;
pub const OPCODE_SYSTEM: u32 = 0x73;
pub const OPCODE_OP_IMM_32: u32 = 0x1b;
pub const OPCODE_OP_32: u32 = 0x3b;

#[derive(Debug, PartialEq)]
pub struct RType {
    pub funct7: u32,
    pub rs2: u64,
    pub rs1: u64,
    pub funct3: u32,
    pub rd: u64,
}

impl RType {
    pub fn new(insn: u32) -> RType {
        RType {
            funct7: (insn >> 25) & 0x7f,
            rs2: ((insn >> 20) & 0x1f) as u64,
            rs1: ((insn >> 15) & 0x1f) as u64,
            funct3: (insn >> 12) & 0x7,
            rd: ((insn >> 7) & 0x1f) as u64,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct IType {
    pub imm: i64,
    pub rs1: u64,
    pub funct3: u32,
    pub rd: u64,
}

impl IType {
    pub fn new(insn: u32) -> IType {
        IType {
            imm: (match insn & 0x80000000 {
                0x80000000 => 0xfffff800,
                _ => 0,
            } | ((insn >> 20) & 0x000007ff)) as i32 as i64,
            rs1: ((insn >> 15) & 0x1f) as u64,
            funct3: (insn >> 12) & 0x7,
            rd: ((insn >> 7) & 0x1f) as u64,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ITypeShamt {
    pub funct7: u32,
    pub shamt: u64,
    pub rs1: u64,
    pub funct3: u32,
    pub rd: u64,
}

impl ITypeShamt {
    pub fn new(insn: u32) -> ITypeShamt {
        let itype = IType::new(insn);
        let shamt = (itype.imm & 0x3f) as u64;

        ITypeShamt {
            funct7: (insn >> 25) & 0x7f,
            shamt,
            rs1: itype.rs1,
            funct3: itype.funct3,
            rd: itype.rd,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ITypeShamtW {
    pub funct7: u32,
    pub shamt: u64,
    pub rs1: u64,
    pub funct3: u32,
    pub rd: u64,
}

impl ITypeShamtW {
    pub fn new(insn: u32) -> ITypeShamtW {
        let itype = IType::new(insn);
        let shamt = (itype.imm & 0x1f) as u64;

        ITypeShamtW {
            funct7: (insn >> 25) & 0x7f,
            shamt,
            rs1: itype.rs1,
            funct3: itype.funct3,
            rd: itype.rd,
        }
    }
}

pub struct ITypeCSR {
    pub csr: u64,
    pub rs1: u64,
    pub funct3: u32,
    pub rd: u64,
}

impl ITypeCSR {
    pub fn new(insn: u32) -> ITypeCSR {
        ITypeCSR {
            csr: ((insn >> 20) & 0xfff) as u64,
            rs1: ((insn >> 15) & 0x1f) as u64,
            funct3: (insn >> 12) & 0x7,
            rd: ((insn >> 7) & 0x1f) as u64,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SType {
    pub imm: i64,
    pub rs2: u64,
    pub rs1: u64,
    pub funct3: u32,
}

impl SType {
    pub fn new(insn: u32) -> SType {
        SType {
            imm: (
                match insn & 0x80000000 {
                    0x80000000 => 0xfffff000,
                    _ => 0
                } | // imm[31:12] = [31]
                ((insn >> 20) & 0xfe0) | // imm[11:5] = [31:25]
                ((insn >> 7) & 0x1f)
                // imm[4:0] = [11:7]
            ) as i32 as i64,
            rs2: ((insn >> 20) & 0x1f) as u64,
            rs1: ((insn >> 15) & 0x1f) as u64,
            funct3: (insn >> 12) & 0x7,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BType {
    pub imm: i64,
    pub rs2: u64,
    pub rs1: u64,
    pub funct3: u32,
}

impl BType {
    pub fn new(insn: u32) -> BType {
        BType {
            rs1: ((insn >> 15) & 0x1f) as u64,
            rs2: ((insn >> 20) & 0x1f) as u64,
            imm: (
                match insn & 0x80000000 { // imm[31:12] = [31]
                    0x80000000 => 0xfffff000,
                    _ => 0
                } |
                ((insn << 4) & 0x00000800) | // imm[11] = [7]
                ((insn >> 20) & 0x000007e0) | // imm[10:5] = [30:25]
                ((insn >> 7) & 0x0000001e)
                // imm[4:1] = [11:8]
            ) as i32 as i64,
            funct3: (insn >> 12) & 0x7,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct UType {
    pub imm: u64,
    pub rd: u64,
}

impl UType {
    pub fn new(insn: u32) -> UType {
        UType {
            rd: ((insn >> 7) & 0x1f) as u64, // [11:7]
            imm: (
                match insn & 0x80000000 {
                    0x80000000 => 0xffffffff00000000,
                    _ => 0
                } | // imm[63:32] = [31]
                ((insn as u64) & 0xfffff000)
                // imm[31:12] = [31:12]
            ),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct JType {
    pub imm: i64,
    pub rd: u64,
}

impl JType {
    pub fn new(insn: u32) -> JType {
        JType {
            rd: ((insn >> 7) & 0x1f) as u64, // [11:7]
            imm: (
                match insn & 0x80000000 { // imm[31:20] = [31]
                    0x80000000 => 0xfff00000,
                    _ => 0
                } |
                (insn & 0x000ff000) | // imm[19:12] = [19:12]
                ((insn & 0x00100000) >> 9) | // imm[11] = [20]
                ((insn & 0x7fe00000) >> 20)
                // imm[10:1] = [30:21]
            ) as i32 as i64,
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_rtype() {
        assert_eq!(
            RType::new(0x0),
            RType {
                funct7: 0,
                rs2: 0,
                rs1: 0,
                funct3: 0,
                rd: 0
            }
        )
    }

    #[test]
    fn test_itype() {
        // addi x23, x31, 2047
        assert_eq!(
            IType::new(0x7fff8b93),
            IType {
                imm: 2047,
                rs1: 31,
                funct3: 0,
                rd: 23
            }
        );

        // addi x23, x31, -1
        assert_eq!(
            IType::new(0xffff8b93),
            IType {
                imm: -1,
                rs1: 31,
                funct3: 0,
                rd: 23
            }
        );

        // addi x23, x31, -2
        assert_eq!(
            IType::new(0xffef8b93),
            IType {
                imm: -2,
                rs1: 31,
                funct3: 0,
                rd: 23
            }
        );

        // ori x13, x7, 4
        assert_eq!(
            IType::new(0x8003e693),
            IType {
                imm: -2048,
                rs1: 7,
                funct3: 0b110,
                rd: 13
            }
        );
    }

    #[test]
    fn test_itype_shamt() {
        // slli x12, x5, 13
        assert_eq!(
            ITypeShamtW::new(0x00d29613),
            ITypeShamtW {
                funct7: 0,
                shamt: 13,
                rs1: 5,
                funct3: 0b001,
                rd: 12
            }
        );

        // srli x30, x19, 31
        assert_eq!(
            ITypeShamtW::new(0x01f9df13),
            ITypeShamtW {
                funct7: 0,
                shamt: 31,
                rs1: 19,
                funct3: 0b101,
                rd: 30
            }
        );

        // srai x7, x23, 0
        assert_eq!(
            ITypeShamtW::new(0x400bd393),
            ITypeShamtW {
                funct7: 0b0100000,
                shamt: 0,
                rs1: 23,
                funct3: 0b101,
                rd: 7
            }
        );
    }

    #[test]
    fn test_stype() {
        // sb x31, -2048(x15)
        assert_eq!(
            SType::new(0x81f78023),
            SType {
                imm: -2048,
                rs2: 31,
                rs1: 15,
                funct3: 0,
            }
        );

        // sh x18, 2047(x3)
        assert_eq!(
            SType::new(0x7f219fa3),
            SType {
                imm: 2047,
                rs2: 18,
                rs1: 3,
                funct3: 1,
            }
        );

        // sw x8, 1(x23)
        assert_eq!(
            SType::new(0x008ba0a3),
            SType {
                imm: 1,
                rs2: 8,
                rs1: 23,
                funct3: 2,
            }
        );

        // sw x5, -1(x25)
        assert_eq!(
            SType::new(0xfe5cafa3),
            SType {
                imm: -1,
                rs2: 5,
                rs1: 25,
                funct3: 2,
            }
        );

        // sw x13, 7(x12)
        assert_eq!(
            SType::new(0x00d623a3),
            SType {
                imm: 7,
                rs2: 13,
                rs1: 12,
                funct3: 2,
            }
        );

        // sw x13, -7(x12)
        assert_eq!(
            SType::new(0xfed62ca3),
            SType {
                imm: -7,
                rs2: 13,
                rs1: 12,
                funct3: 2,
            }
        );
    }

    #[test]
    fn test_btype() {
        // beq x10, x14, .-4096
        assert_eq!(
            BType::new(0x80e50063),
            BType {
                imm: -4096,
                rs1: 10,
                rs2: 14,
                funct3: 0b000
            }
        );

        // blt x3, x21, .+4094
        assert_eq!(
            BType::new(0x7f51cfe3),
            BType {
                imm: 4094,
                rs1: 3,
                rs2: 21,
                funct3: 0b100
            }
        );

        // bge x18, x0, .-2
        assert_eq!(
            BType::new(0xfe095fe3),
            BType {
                imm: -2,
                rs1: 18,
                rs2: 0,
                funct3: 0b101
            }
        );

        // bne x15, x16, .+2
        assert_eq!(
            BType::new(0x01079163),
            BType {
                imm: 2,
                rs1: 15,
                rs2: 16,
                funct3: 0b001
            }
        );

        // bgeu x31, x8, .+18
        assert_eq!(
            BType::new(0x008ff963),
            BType {
                imm: 18,
                rs1: 31,
                rs2: 8,
                funct3: 0b111
            }
        );

        // bgeu x31, x8, .-18
        assert_eq!(
            BType::new(0xfe8ff7e3),
            BType {
                imm: -18,
                rs1: 31,
                rs2: 8,
                funct3: 0b111
            }
        );
    }

    #[test]
    fn test_utype() {
        // lui x0, 0xfffff
        assert_eq!(
            UType::new(0xfffff037),
            UType {
                imm: 0xfffffffffffff000 as u64,
                rd: 0,
            }
        );
        assert_eq!(0xfffff000 as u32, (0xfffffffffffff000 as u64) as u32);

        // lui x31, 0x0
        assert_eq!(UType::new(0x00000fb7), UType { imm: 0x0, rd: 31 });

        // lui x17, 0x123ab
        assert_eq!(
            UType::new(0x123ab8b7),
            UType {
                imm: 0x123ab000,
                rd: 17,
            }
        );
    }

    #[test]
    fn test_jtype() {
        // jal x0, .+0xffffe
        assert_eq!(
            JType::new(0x7ffff06f),
            JType {
                imm: 0xffffe,
                rd: 0,
            }
        );

        // jal x31, .-0x100000
        assert_eq!(
            JType::new(0x80000fef),
            JType {
                imm: -0x100000,
                rd: 31,
            }
        );

        // jal x13, .-2
        assert_eq!(JType::new(0xfffff6ef), JType { imm: -2, rd: 13 });

        // jal x13, .+2
        assert_eq!(JType::new(0x002006ef), JType { imm: 2, rd: 13 });

        // jal x26, .-46
        assert_eq!(JType::new(0xfd3ffd6f), JType { imm: -46, rd: 26 });

        // jal x26, .+46
        assert_eq!(JType::new(0x02e00d6f), JType { imm: 46, rd: 26 });
    }
}
