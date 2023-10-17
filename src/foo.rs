pub fn _add(rd: &mut i32, rs1: &i32, rs2: &i32) {
    *rd = rs1 + rs2;
}

pub fn _sub(rd: &mut i32, rs1: &i32, rs2: &i32) {
    *rd = rs1 - rs2;
}

pub fn xor(rd: &mut i32, rs1: &i32, rs2: &i32) {
    *rd = rs1 ^ rs2;
}

pub fn or(rd: &mut i32, rs1: &i32, rs2: &i32) {
    *rd = rs1 | rs2;
}

pub fn and(rd: &mut i32, rs1: &i32, rs2: &i32) {
    *rd = rs1 & rs2;
}

pub fn sll(rd: &mut i32, rs1: &i32, rs2: &i32) {
    *rd = rs1 << rs2;
}

pub fn sra(rd: &mut i32, rs1: &i32, rs2: &i32) {
    *rd = rs1 >> rs2;
}

//TODO: this needs testing
pub fn srl(rd: &mut i32, rs1: &i32, rs2: &i32) {
    *rd = ((*rs1 as u32) >> rs2) as i32
}

pub fn slt(rd: &mut i32, rs1: &i32, rs2: &i32) {
    *rd = if rs1 < rs2 { 1 } else { 0 }
}
