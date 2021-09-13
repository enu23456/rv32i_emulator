mod rom;

#[derive(Debug, Copy, Clone)]
pub struct CpuOfRV32I {
    pub x: [u32; 32],
    pub program_counter: u32,
    pub pc_loaded: bool,
    pub rom: rom::Rom,
}
impl CpuOfRV32I {
    pub fn new() -> CpuOfRV32I {
        return CpuOfRV32I {
            x: [0; 32],
            program_counter: 0,
            pc_loaded: false,
            rom: rom::Rom::new(),
        };
    }
    pub fn next(&mut self) {
        let instruction = self.fetch();
        self.decode_and_execute(instruction);
        if self.pc_loaded == false {
            self.program_counter += 4;
        }
        self.pc_loaded = false;
    }
    pub fn fetch(&mut self) -> u32 {
        let temp3 = (self.rom.rom_data[(self.program_counter as usize) + 3] as u32) << 24;
        let temp2 = (self.rom.rom_data[(self.program_counter as usize) + 2] as u32) << 16;
        let temp1 = (self.rom.rom_data[(self.program_counter as usize) + 1] as u32) << 8;
        let temp0 = self.rom.rom_data[self.program_counter as usize] as u32;
        let ret: u32 = temp3 + temp2 + temp1 + temp0;
        print!("pc: {}, inst: {:0>32b}. ", self.program_counter, ret);
        return ret;
    }
    pub fn decode_and_execute(&mut self, instruction: u32) {
        let opcode: u8 = (instruction & 0x00_00_00_7F) as u8;
        match opcode {
            0b0110111 => {
                let immediate: u32 = instruction >> 12;
                let rd: u8 = ((instruction >> 7) & 0b11111) as u8;
                println!("LUI x[{:0>2}], {}", rd, immediate);
                self.x[rd as usize] = (immediate << 12) as u32;
            }
            0b0010111 => {
                let immediate: u32 = instruction >> 12;
                let rd: u8 = ((instruction >> 7) & 0b11111) as u8;
                println!("AUIPC x[{:0>2}], {}", rd, immediate);
                self.x[rd as usize] = self.program_counter + (immediate << 12) as u32;
            }
            0b1101111 => {
                let rd: u8 = ((instruction >> 7) & 0b11111) as u8;
                let mut offset: u32 = ((instruction >> 21) & 0b1111111111) << 1;
                offset += ((instruction >> 20) & 0b1) << (10 + 1);
                offset += ((instruction >> 12) & 0b11111111) << (11 + 1);
                offset += ((instruction >> 31) & 0b1) << (19 + 1);
                if ((instruction >> 31) & 0b1) == 1 {
                    offset = ((!offset + 1) & !(0b111111111111 << 20)) as u32;
                    println!("jal x[{:0>2}], -{}", rd, offset);
                } else {
                    println!("jal x[{:0>2}], {}", rd, offset);
                }

                if rd != 0 {
                    self.x[rd as usize] = self.program_counter + 4;
                }
                if ((instruction >> 31) & 0b1) == 1 {
                    self.program_counter -= offset as u32;
                } else {
                    self.program_counter += offset as u32;
                }
                self.pc_loaded = true;
            }
            0b1100011 => {
                let rs2: u8 = ((instruction >> 20) & 0b11111) as u8;
                let rs1: u8 = ((instruction >> 15) & 0b11111) as u8;
                let func3: u8 = ((instruction >> 12) & 0b111) as u8;
                let mut offset: u16 = (((instruction >> 8) & 0b1111) << 1) as u16;
                offset += (((instruction >> 25) & 0b111111) << (4 + 1)) as u16;
                offset += (((instruction >> 7) & 0b1) << (10 + 1)) as u16;
                offset += (((instruction >> 31) & 0b1) << (11 + 1)) as u16;
                if ((instruction >> 31) & 0b1) == 1 {
                    offset = ((!offset + 1) & !(0b1111 << 12)) as u16;
                }
                match func3 {
                    0b000 => {
                        println!("beq x[{:0>2}], x[{:0>2}], {}", rs1, rs2, offset);
                        if self.x[rs1 as usize] == self.x[rs2 as usize] {
                            if ((instruction >> 31) & 0b1) == 1 {
                                self.program_counter -= offset as u32;
                            } else {
                                self.program_counter += offset as u32;
                            }
                            self.pc_loaded = true;
                        }
                    }
                    0b001 => {
                        println!("bne x[{:0>2}], x[{:0>2}], {}", rs1, rs2, offset);
                        if self.x[rs1 as usize] != self.x[rs2 as usize] {
                            if ((instruction >> 31) & 0b1) == 1 {
                                self.program_counter -= offset as u32;
                            } else {
                                self.program_counter += offset as u32;
                            }
                            self.pc_loaded = true;
                        }
                    }
                    0b100 => {
                        println!("blt x[{:0>2}], x[{:0>2}], {}", rs1, rs2, offset);
                        if (self.x[rs1 as usize] as i32) < (self.x[rs2 as usize] as i32) {
                            if ((instruction >> 31) & 0b1) == 1 {
                                self.program_counter -= offset as u32;
                            } else {
                                self.program_counter += offset as u32;
                            }
                            self.pc_loaded = true;
                        }
                    }
                    0b101 => {
                        println!("bge x[{:0>2}], x[{:0>2}], {}", rs1, rs2, offset);
                        if (self.x[rs1 as usize] as i32) >= (self.x[rs2 as usize] as i32) {
                            if ((instruction >> 31) & 0b1) == 1 {
                                self.program_counter -= offset as u32;
                            } else {
                                self.program_counter += offset as u32;
                            }
                            self.pc_loaded = true;
                        }
                    }
                    0b110 => {
                        println!("bltu x[{:0>2}], x[{:0>2}], {}", rs1, rs2, offset);
                        if self.x[rs1 as usize] < self.x[rs2 as usize] {
                            if ((instruction >> 31) & 0b1) == 1 {
                                self.program_counter -= offset as u32;
                            } else {
                                self.program_counter += offset as u32;
                            }
                            self.pc_loaded = true;
                        }
                    }
                    0b111 => {
                        println!("bgeu x[{:0>2}], x[{:0>2}], {}", rs1, rs2, offset);
                        if self.x[rs1 as usize] >= self.x[rs2 as usize] {
                            if ((instruction >> 31) & 0b1) == 1 {
                                self.program_counter -= offset as u32;
                            } else {
                                self.program_counter += offset as u32;
                            }
                            self.pc_loaded = true;
                        }
                    }
                    _ => {
                        println!("this instruction is not specified");
                    }
                }
            }
            0b1100111 => {
                let mut offset: u16 = (instruction >> 20) as u16;
                if ((instruction >> 31) & 0b1) == 1 {
                    offset = ((!offset + 1) & !(0b1111 << 12)) as u16;
                }
                let rs1: u8 = ((instruction >> 15) & 0b11111) as u8;
                let rd: u8 = ((instruction >> 7) & 0b11111) as u8;
                println!("jalr x[{:0>2}], x[{:0>2}], {}", rd, rs1, offset);
                let t = self.program_counter + 4;
                if ((instruction >> 31) & 0b1) == 1 {
                    self.program_counter -= (offset as u32) & !1;
                } else {
                    self.program_counter += (offset as u32) & !1;
                }
                self.x[rd as usize] = t;
            }
            0b000011 => {
                let mut offset: u16 = (instruction >> 20) as u16;
                if ((instruction >> 31) & 0b1) == 1 {
                    offset = ((!offset + 1) & !(0b1111 << 12)) as u16;
                }
                let rs1: u8 = ((instruction >> 15) & 0b11111) as u8;
                let func3: u8 = ((instruction >> 12) & 0b111) as u8;
                let rd: u8 = ((instruction >> 7) & 0b11111) as u8;
                match func3 {
                    0b000 => {
                        let mut address = self.x[rs1 as usize];
                        if ((instruction >> 31) & 0b1) == 1 {
                            println!("lb x[{:0>2}], -{}(x[{:0>2}])", rd, offset, rs1);
                            address -= offset as u32;
                        } else {
                            println!("lb x[{:0>2}], {}(x[{:0>2}])", rd, offset, rs1);
                            address += offset as u32;
                        }
                        let temp0 = (self.rom.rom_data[address as usize] as i32) as u32;
                        self.x[rd as usize] = temp0;
                    }
                    0b001 => {
                        let mut address = self.x[rs1 as usize];
                        if ((instruction >> 31) & 0b1) == 1 {
                            println!("lh x[{:0>2}], -{}(x[{:0>2}])", rd, offset, rs1);
                            address -= offset as u32;
                        } else {
                            println!("lh x[{:0>2}], {}(x[{:0>2}])", rd, offset, rs1);
                            address += offset as u32;
                        }
                        let temp1 = (self.rom.rom_data[(address as usize) + 1] as u16) << 8;
                        let temp0 = self.rom.rom_data[address as usize] as u16;
                        let sum = temp1 + temp0;
                        self.x[rd as usize] = ((sum as i16) as i32) as u32;
                    }
                    0b010 => {
                        let mut address = self.x[rs1 as usize];
                        if ((instruction >> 31) & 0b1) == 1 {
                            println!("lw x[{:0>2}], -{}(x[{:0>2}])", rd, offset, rs1);
                            address -= offset as u32;
                        } else {
                            println!("lw x[{:0>2}], {}(x[{:0>2}])", rd, offset, rs1);
                            address += offset as u32;
                        }
                        let temp3 = (self.rom.rom_data[(address as usize) + 3] as u32) << 24;
                        let temp2 = (self.rom.rom_data[(address as usize) + 2] as u32) << 16;
                        let temp1 = (self.rom.rom_data[(address as usize) + 1] as u32) << 8;
                        let temp0 = self.rom.rom_data[address as usize] as u32;
                        self.x[rd as usize] = temp3 + temp2 + temp1 + temp0;
                    }
                    0b100 => {
                        println!("lbu x[{:0>2}], {}(x[{:0>2}])", rd, offset, rs1);
                        let mut address = self.x[rs1 as usize];
                        if ((instruction >> 31) & 0b1) == 1 {
                            address -= offset as u32;
                        } else {
                            address += offset as u32;
                        }
                        let temp0 = self.rom.rom_data[address as usize];
                        self.x[rd as usize] = temp0 as u32;
                    }
                    0b101 => {
                        println!("lb x[{:0>2}], {}(x[{:0>2}])", rd, offset, rs1);
                        let mut address = self.x[rs1 as usize];
                        if ((instruction >> 31) & 0b1) == 1 {
                            address -= offset as u32;
                        } else {
                            address += offset as u32;
                        }
                        let temp1 = (self.rom.rom_data[(address as usize) + 1] as u16) << 8;
                        let temp0 = self.rom.rom_data[address as usize] as u16;
                        let sum = temp1 + temp0;
                        self.x[rd as usize] = sum as u32;
                    }
                    0b110 => {
                        println!("lw x[{:0>2}], {}(x[{:0>2}])", rd, offset, rs1);
                        let mut address = self.x[rs1 as usize];
                        if ((instruction >> 31) & 0b1) == 1 {
                            address -= offset as u32;
                        } else {
                            address += offset as u32;
                        }
                        let temp3 = (self.rom.rom_data[(address as usize) + 3] as u32) << 24;
                        let temp2 = (self.rom.rom_data[(address as usize) + 2] as u32) << 16;
                        let temp1 = (self.rom.rom_data[(address as usize) + 1] as u32) << 8;
                        let temp0 = self.rom.rom_data[address as usize] as u32;
                        self.x[rd as usize] = temp3 + temp2 + temp1 + temp0;
                    }
                    _ => {
                        println!("this instruction is not specified");
                    }
                }
            }
            0b0010011 => {
                let mut immediate: u16 = (instruction >> 20) as u16;
                let rs1: u8 = ((instruction >> 15) & 0b11111) as u8;
                let func3: u8 = ((instruction >> 12) & 0b111) as u8;
                let rd: u8 = ((instruction >> 7) & 0b11111) as u8;
                match func3 {
                    0b000 => {
                        if ((instruction >> 31) & 0b1) == 1 {
                            immediate = ((!immediate + 1) & !(0b1111 << 12)) as u16;
                        }
                        if ((instruction >> 31) & 0b1) == 1 {
                            println!("addi x[{:0>2}], x[{:0>2}], -{}", rd, rs1, immediate);
                            self.x[rd as usize] = self.x[rs1 as usize] - immediate as u32;
                        } else {
                            println!("addi x[{:0>2}], x[{:0>2}], {}", rd, rs1, immediate);
                            self.x[rd as usize] = self.x[rs1 as usize] + immediate as u32;
                        }
                    }
                    0b010 => {
                        println!("slti x[{:0>2}], x[{:0>2}], {}", rd, rs1, immediate);
                        if ((instruction >> 31) & 0b1) == 1 {
                            immediate = ((!immediate + 1) & !(0b1111 << 12)) as u16;
                        }
                        let imm_is_minus = if (instruction >> 31) & 0b1 == 1 {
                            true
                        } else {
                            false
                        };
                        let rs_is_minus = if (self.x[rs1 as usize] >> 31) & 0b1 == 1 {
                            true
                        } else {
                            false
                        };

                        if imm_is_minus == true {
                            if rs_is_minus == true && self.x[rs1 as usize] > immediate as u32 {
                                self.x[rd as usize] = 1;
                            } else {
                                self.x[rd as usize] = 0;
                            }
                        } else {
                            if rs_is_minus == false && self.x[rs1 as usize] > immediate as u32 {
                                self.x[rd as usize] = 0;
                            } else {
                                self.x[rd as usize] = 1;
                            }
                        }
                    }
                    0b011 => {
                        println!("sltiu x[{:0>2}], x[{:0>2}], {}", rd, rs1, immediate);
                        self.x[rd as usize] = if self.x[rs1 as usize] < immediate as u32 {
                            1
                        } else {
                            0
                        };
                    }
                    0b100 => {
                        println!("xori x[{:0>2}], x[{:0>2}], {}", rd, rs1, immediate);
                        self.x[rd as usize] = self.x[rs1 as usize] ^ immediate as u32;
                    }
                    0b110 => {
                        println!("ori x[{:0>2}], x[{:0>2}], {}", rd, rs1, immediate);
                        self.x[rd as usize] = self.x[rs1 as usize] | immediate as u32;
                    }
                    0b111 => {
                        println!("andi x[{:0>2}], x[{:0>2}], {}", rd, rs1, immediate);
                        self.x[rd as usize] = self.x[rs1 as usize] & immediate as u32;
                    }
                    0b001 => {
                        println!("slli x[{:0>2}], x[{:0>2}], {}", rd, rs1, immediate & 0x1f);
                        self.x[rd as usize] = self.x[rs1 as usize] << immediate & 0x1f;
                    }
                    0b101 => {
                        if ((instruction >> 30) & 0b1) == 0 {
                            println!("srli x[{:0>2}], x[{:0>2}], {}", rd, rs1, immediate & 0x1f);
                            self.x[rd as usize] = self.x[rs1 as usize] >> immediate & 0x1f;
                        } else {
                            println!("srai x[{:0>2}], x[{:0>2}], {}", rd, rs1, immediate & 0x1f);
                            self.x[rd as usize] =
                                ((self.x[rs1 as usize] as i32) >> immediate & 0x1f) as u32;
                        }
                    }
                    _ => {
                        println!("this instruction is not specified");
                    }
                }
            }
            0b0100011 => {
                let mut offset: u16 =
                    ((instruction >> 25) << 5) as u16 + ((instruction >> 7) & 0b1111) as u16;
                if ((instruction >> 31) & 0b1) == 1 {
                    offset = ((!offset + 1) & !(0b1111 << 12)) as u16;
                }
                let rs2: u8 = ((instruction >> 20) & 0b11111) as u8;
                let rs1: u8 = ((instruction >> 15) & 0b11111) as u8;
                let func3: u8 = ((instruction >> 12) & 0b111) as u8;
                match func3 {
                    0b000 => {
                        println!("sb x[{:0>2}], {}(x[{:0>2}])", rs2, offset, rs1);
                        let mut address = self.x[rs1 as usize];
                        if ((instruction >> 31) & 0b1) == 1 {
                            address -= offset as u32;
                        } else {
                            address += offset as u32;
                        }
                        self.rom.rom_data[address as usize] = (self.x[rs2 as usize] & 0xFF) as u8;
                    }
                    0b001 => {
                        println!("sh x[{:0>2}], {}(x[{:0>2}])", rs2, offset, rs1);
                        let mut address = self.x[rs1 as usize];
                        if ((instruction >> 31) & 0b1) == 1 {
                            address -= offset as u32;
                        } else {
                            address += offset as u32;
                        }
                        self.rom.rom_data[(address as usize) + 0] =
                            (self.x[rs2 as usize] & 0xFF) as u8;
                        self.rom.rom_data[(address as usize) + 1] =
                            ((self.x[rs2 as usize] >> 8) & 0xFF) as u8;
                    }
                    0b010 => {
                        println!("sw x[{:0>2}], {}(x[{:0>2}])", rs2, offset, rs1);
                        let mut address = self.x[rs1 as usize];
                        if ((instruction >> 31) & 0b1) == 1 {
                            address -= offset as u32;
                        } else {
                            address += offset as u32;
                        }
                        self.rom.rom_data[(address as usize) + 0] =
                            (self.x[rs2 as usize] & 0xFF) as u8;
                        self.rom.rom_data[(address as usize) + 1] =
                            ((self.x[rs2 as usize] >> 8) & 0xFF) as u8;
                        self.rom.rom_data[(address as usize) + 2] =
                            ((self.x[rs2 as usize] >> 16) & 0xFF) as u8;
                        self.rom.rom_data[(address as usize) + 3] =
                            ((self.x[rs2 as usize] >> 24) & 0xFF) as u8;
                    }
                    _ => {
                        println!("this instruction is not specified");
                    }
                }
            }
            0b0110011 => {
                let func7: u8 = (instruction >> 25) as u8;
                let rs2: u8 = ((instruction >> 20) & 0b11111) as u8;
                let rs1: u8 = ((instruction >> 15) & 0b11111) as u8;
                let func3: u8 = ((instruction >> 12) & 0b111) as u8;
                let rd: u8 = ((instruction >> 7) & 0b11111) as u8;
                match (func3, func7) {
                    (0b000, 0b0000000) => {
                        println!("add x[{:0>2}], x[{:0>2}], x[{:0>2}]", rd, rs1, rs2);
                        self.x[rd as usize] = self.x[rs1 as usize] + self.x[rs2 as usize];
                    }
                    (0b000, 0b0100000) => {
                        println!("sub x[{:0>2}], x[{:0>2}], x[{:0>2}]", rd, rs1, rs2);
                        self.x[rd as usize] =
                            ((self.x[rs1 as usize] as i32) - (self.x[rs2 as usize] as i32)) as u32;
                    }
                    (0b001, 0b0000000) => {
                        println!("sll x[{:0>2}], x[{:0>2}], x[{:0>2}]", rd, rs1, rs2);
                        self.x[rd as usize] =
                            self.x[rs1 as usize] << (self.x[rs2 as usize] & 0x00_00_00_1F);
                    }
                    (0b010, 0b0000000) => {
                        println!("slt x[{:0>2}], x[{:0>2}], x[{:0>2}]", rd, rs1, rs2);
                        if (self.x[rs1 as usize] as i32) < (self.x[rs2 as usize] as i32) {
                            self.x[rd as usize] = 1;
                        } else {
                            self.x[rd as usize] = 0;
                        };
                    }
                    (0b011, 0b0000000) => {
                        println!("sltu x[{:0>2}], x[{:0>2}], x[{:0>2}]", rd, rs1, rs2);
                        if self.x[rs1 as usize] < self.x[rs2 as usize] {
                            self.x[rd as usize] = 1;
                        } else {
                            self.x[rd as usize] = 0;
                        };
                    }
                    (0b100, 0b0000000) => {
                        println!("xor x[{:0>2}], x[{:0>2}], x[{:0>2}]", rd, rs1, rs2);
                        self.x[rd as usize] = self.x[rs1 as usize] ^ self.x[rs2 as usize];
                    }
                    (0b101, 0b0000000) => {
                        println!("srl x[{:0>2}], x[{:0>2}], x[{:0>2}]", rd, rs1, rs2);
                        self.x[rd as usize] =
                            self.x[rs1 as usize] >> (self.x[rs2 as usize] & 0x00_00_00_1F);
                    }
                    (0b101, 0b0100000) => {
                        println!("sra x[{:0>2}], x[{:0>2}], x[{:0>2}]", rd, rs1, rs2);
                        self.x[rd as usize] = ((self.x[rs1 as usize] as i32)
                            >> (self.x[rs2 as usize] & 0x00_00_00_1F))
                            as u32;
                    }
                    (0b110, 0b0000000) => {
                        println!("or x[{:0>2}], x[{:0>2}], x[{:0>2}]", rd, rs1, rs2);
                        self.x[rd as usize] = self.x[rs1 as usize] | self.x[rs2 as usize];
                    }
                    (0b111, 0b0000000) => {
                        println!("and x[{:0>2}], x[{:0>2}], x[{:0>2}]", rd, rs1, rs2);
                        self.x[rd as usize] = self.x[rs1 as usize] & self.x[rs2 as usize];
                    }
                    _ => {
                        println!("this instruction is not specified");
                    }
                }
            }
            0b0001111 => {
                println!("?-type, FENCE or FENCEI");
                let func3: u8 = ((instruction >> 12) & 0b111) as u8;
                if func3 == 0b000 {
                    println!("fence...but execute nop in this emulator")
                } else {
                    println!("this instruction is not specified");
                }
            }
            0b1110011 => {
                println!("?-type, ECALL or EBREAK");
                let csr: u8 = (instruction >> 25) as u8;
                let rs1: u8 = ((instruction >> 15) & 0b11111) as u8;
                let func3: u8 = ((instruction >> 12) & 0b111) as u8;
                let rd: u8 = ((instruction >> 7) & 0b11111) as u8;
                if func3 == 0 {
                    if csr == 1 && rs1 == 0 && rd == 0 {
                        println!("ecall ...but execute nop in this emulator")
                    } else if csr == 0 && rs1 == 0 && rd == 0 {
                        println!("ebreak ...but execute nop in this emulator")
                    }
                }
            }
            _ => {
                println!("this instruction is not specified");
            }
        }
        return;
    }
    pub fn get_array(&mut self) -> [u32; 4] {
        let mut ret: [u32; 4] = [0; 4];
        for i in 0..ret.len() {
            let temp3 = (self.rom.rom_data[0x200 + (4 * i) + 3] as u32) << 24;
            let temp2 = (self.rom.rom_data[0x200 + (4 * i) + 2] as u32) << 16;
            let temp1 = (self.rom.rom_data[0x200 + (4 * i) + 1] as u32) << 8;
            let temp0 = (self.rom.rom_data[0x200 + (4 * i) + 0] as u32) << 0;
            ret[i] = temp3 + temp2 + temp1 + temp0;
        }
        return ret;
    }
}
