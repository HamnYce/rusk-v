struct fungi {
    rd: i32,
    rs1: i32,
    rs2: i32,
}

impl fungi {
    pub fn add(&mut self) {
        self.rd = self.rs1 + self.rs2;
    }

    pub fn sub(&mut self) {
        self.rd = self.rs1 - self.rs2;
    }

    pub fn xor(&mut self) {
        self.rd = self.rs1 ^ self.rs2;
    }

    pub fn or(&mut self) {
        self.rd = self.rs1 | self.rs2;
    }

    pub fn and(&mut self) {
        self.rd = self.rs1 & self.rs2;
    }

    pub fn sll(&mut self) {
        self.rd = self.rs1 << self.rs2;
    }

    pub fn sra(&mut self) {
        self.rd = self.rs1 >> self.rs2;
    }

    //TODO: this needs testing
    pub fn srl(&mut self) {
        self.rd = ((self.rs1 as u32) >> self.rs2) as i32
    }

    pub fn slt(&mut self) {
        self.rd = if self.rs1 < self.rs2 { 1 } else { 0 }
    }
}
