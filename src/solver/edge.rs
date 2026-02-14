const E0:u64 = 0b00001_00001_00001_00001_00001_00001_00001_00001_00001_00001_00001_00001;
const E1:u64 = E0 << 1;
const E2:u64 = E0 << 2;
const E3:u64 = E0 << 3;
const E4:u64 = E0 << 4;
const COLOR_YELLOW: u8 = 0;
const COLOR_WHITE: u8 = 1;
const COLOR_RED: u8 = 2;
const COLOR_ORANGE: u8 = 3;
const COLOR_BLUE: u8 = 4;
const COLOR_GREEN: u8 = 5;
use crate::corner::FTurn;

pub fn map_mul(a:u64, b: u64) -> u64 {
    const E0:u64 = 0b00001_00001_00001_00001_00001_00001_00001_00001_00001_00001_00001_00001;
    const E4:u64 = E0 << 4;
    const ALT:u64 =0b00000_01111_00000_01111_00000_01111_00000_01111_00000_01111_00000_01111;
    let map = b;
    let mut res = a & E4;
    let set1 = (a & ALT) *5;
    let mut offset = 60;
    while offset != 0 {
        offset -= 10;
        let input = (set1 >> offset) & 0b111111;
        res ^= ((map >> input)&0b11111) << offset;
    }
    let set1 = (a & (ALT << 5))*5;
    let mut offset = 65;
    while offset != 5 {
        offset -= 10;
        let input = (set1 >> offset) & 0b111111;
        res ^= ((map >> input)&0b11111) << offset;
    }
    res
}

pub fn map_invert(set: u64) -> u64 {
    // Iterations:        100
    // Instructions:      11700
    // Total Cycles:      2961
    // Total uOps:        11900
    //TODO!: Optimize it.
    let mut res = 0;
    for i in 0..12 {
        let offset = i *5;
        let input = set >> offset;
        let dest = i | (input & 0b10000);
        res |= dest << ((input & 0b1111)*5);
    }
    res
}
pub fn map_mul_inverse(a: u64, set: u64 ) -> u64 {
    // Iterations:        100
    // Instructions:      12200
    // Total Cycles:      2812
    // Total uOps:        12500
    let mut buffer =[0u8;16];
    for i in 0..12 {
        let offset = i *5;
        let input = set >> offset;
        let dest = i | (input & 0b10000);
        buffer[(input & 0b1111) as usize] = dest as u8;
    }
    const E0:u64 = 0b00001_00001_00001_00001_00001_00001_00001_00001_00001_00001_00001_00001;
    const E4:u64 = E0 << 4;
    let mut res = a & E4;
    for i in 0..12 {
        let offset = i * 5;
        res ^= (buffer[((a >> offset) & 0b1111) as usize] as u64) << offset;
    }
    res
}
#[derive(Clone,Copy, Debug)]
pub struct EdgeColor{pub a: u8, pub b:u8}
impl EdgeColor {
    fn from(pos: u64,og:u64) -> EdgeColor {
        let color_map = &[
          EdgeColor{a: COLOR_BLUE, b: COLOR_YELLOW },
          EdgeColor{a: COLOR_BLUE, b: COLOR_WHITE },
          EdgeColor{a: COLOR_GREEN, b: COLOR_YELLOW},
          EdgeColor{a: COLOR_GREEN, b: COLOR_WHITE },
          EdgeColor{a: COLOR_ORANGE, b: COLOR_BLUE },
          EdgeColor{a: COLOR_RED, b: COLOR_BLUE },
          EdgeColor{a: COLOR_RED, b: COLOR_GREEN },
          EdgeColor{a: COLOR_ORANGE, b: COLOR_GREEN },
          EdgeColor{a: COLOR_ORANGE, b: COLOR_YELLOW },
          EdgeColor{a: COLOR_RED, b: COLOR_WHITE },
          EdgeColor{a: COLOR_RED, b: COLOR_YELLOW },
          EdgeColor{a: COLOR_ORANGE, b: COLOR_WHITE }
        ];
        // let nm1 = (og & !(og >> 2))|(!(og | (og >> 2)));
        // let nm2 = (pos & !(pos >> 2))|(!(pos | (pos >> 2)));
        let nm1 = pos >> 2;
        let nm2 = og >> 2;
        let nm3 = og >> 4;
        let mut c = color_map[pos as usize & 0b01111];
        if ((nm1 ^ nm2 ^ nm3) & 0b1) != 0 {
            std::mem::swap(&mut c.a, &mut c.b);
        }

        c 
    }
}
#[derive(Debug)]
pub struct EdgeColoring( pub [EdgeColor;12]);
#[derive(Clone,Copy)]
pub struct EdgeSet {
    pub set: u64
}
#[inline]
fn filter_mask(bit: u64) -> u64 {
    E4^(E4 - (bit & E0))
}
impl EdgeSet {

    pub fn get_edge( self, corner: u64)->u64 {
        (self.set >> (5 *corner)) & 0b11111
    }
    pub fn coloring(self) -> EdgeColoring {
        let mut def = [EdgeColor{a:0,b:0};12];
        for i in 0..12u64 {
            let edge = self.get_edge(i);
            def[edge as usize & 0b1111] = EdgeColor::from(i as u64, edge);
        }
        EdgeColoring(def)
    }
//      og  rd |  bl  gr |  ye  wh
// 0000_0   0_ |  1   0 |  _1  0_
// 0001_0   0_ |  1   0 |  _0  1_
// 0010_0   0_ |  0   1 |  _1  0_
// 0011_0   0_ |  0   1 |  _0  1_
// 0100_1   0_ |  1   0 |  _0  0_
// 0101_0   1_ |  1   0 |  _0  0_
// 0110_0   1_ |  0   1 |  _0  0_
// 0111_1   0_ |  0   1 |  _0  0_
// 1000_1   0_ |  0   0 |  _1  0_
// 1001_0   1_ |  0   0 |  _0  1_
// 1010_0   1_ |  0   0 |  _1  0_
// 1011_1   0_ |  0  _0 |   0  1

    pub fn invert(self) -> EdgeSet {
        let set = self.set;
        let mut res = 0;
        for i in 0..12 {
            let offset = i *5;
            let input = set >> offset;
            let dest = i | (input & 0b10000);
            res |= dest << ((input & 0b1111)*5);
        }
        EdgeSet{set:res}
    }
    pub fn mul(self, other: EdgeSet) -> EdgeSet {
        EdgeSet{set: map_mul(self.set, other.set)}
    }

    pub fn mul_inverse(self, other: EdgeSet) -> EdgeSet {
        EdgeSet{set: map_mul_inverse(self.set, other.set)}
    }
    pub fn default() -> EdgeSet {
        //EdgeSet{set: 0b00010_01010_00000_01000_00111_00110_00101_00100_00011_01011_00001_01001}
        EdgeSet{set: 0b01011_01010_01001_01000_00111_00110_00101_00100_00011_00010_00001_00000}
    }
    pub fn filter_down(self) -> u64 {
        filter_mask(self.set & !(self.set >> 2))
    }
    pub fn filter_up(self) -> u64 {
        filter_mask(!(self.set | (self.set >> 2)))
    }
// Binary { op: ANDI, a: 1, b: 3 }
// Binary { op: NOR, a: 1, b: 3 }
    pub fn filter_right(self) -> u64 {
        filter_mask((self.set >> 1) & !(self.set >> 3))
    }
    pub fn filter_left(self) -> u64 {
        filter_mask(!((self.set >> 1) | (self.set >> 3)))
    }
// BinaryOffset { op: IAND, a: 1, b: 3, offset: 3 }
// Multi4 { op1: XNOR, op2: OR, a1: 0, b1: 1, a2: 2, b2: 3, join: 0 }
    pub fn filter_front(self) -> u64 {
        let e = self.set + (E0|E1);
        filter_mask(!(e >> 1) & (e >> 3))
    }
    pub fn filter_back(self) -> u64 {
        let e = self.set;
        filter_mask(!((e) ^ (e >> 1)) & ((e>>2) | (e>>3)))
    }

// ===== up_cw ====
// [(10, 2), (2, 8), (8, 0), (0, 10)]
// SCORE: 2
// BIT 0: Some(Zero) 
// BIT 1: Some(Unary { negate: true, a: -2 }) !(e >> 2)
// BIT 2: Some(Zero) 
// BIT 3: Some(One) 

// ===== up_ccw ====
// [(2, 10), (8, 2), (0, 8), (10, 0)]
// SCORE: 1
// BIT 0: Some(Zero) 
// BIT 1: Some(Unary { negate: false, a: -2 }) e >> 2
// BIT 2: Some(Zero) 
// BIT 3: Some(One) 

    pub fn apply(self, turn: FTurn) -> EdgeSet {
        use FTurn::*;
        match turn {
            R2 => self.right_cw().right_cw(),
            Rcw => self.right_cw(),
            Rccw => self.right_ccw(),
            L2 => self.left_cw().left_cw(),
            Lcw => self.left_cw(),
            Lccw => self.left_ccw(),
            U2 => self.up_cw().up_cw(),
            Ucw => self.up_cw(),
            Uccw => self.up_ccw(),
            D2 => self.down_cw().down_cw(),
            Dcw => self.down_cw(),
            Dccw => self.down_ccw(),
            F2 => self.front_cw().front_cw(),
            Fcw => self.front_cw(),
            Fccw => self.front_ccw(),
            B2 => self.back_cw().back_cw(),
            Bcw => self.back_cw(),
            Bccw => self.back_ccw(),
            // M2 => self.right_cw().right_cw().left_cw().left_cw(),
            // Mcw => self.right_cw().left_ccw(),
            // Mccw => self.right_ccw().left_cw(),
            // S2 => self.front_cw().front_cw().back_cw().back_cw(),
            // Scw => self.front_ccw().back_cw(),
            // Sccw => self.front_cw().back_ccw(),
            // E2 => self.up_cw().up_cw().down_cw().down_cw(),
            // Ecw => self.up_ccw().down_cw(),
            // Eccw => self.up_cw().down_ccw(),
        }
    }
    pub fn print(self)  {
        for i in 0..12 {
            eprint!("{:05b}_", (self.set >> ((11-i)*5))& 0b11111);
        }
        eprintln!("");
        for i in 0..12 {
            eprint!("{:5}_", (self.set >> ((11-i)*5))& 0b11111);
        }
        eprintln!("");
    }
    pub fn thing_cw(self) -> EdgeSet { //Testing Virtual Slice Rotatings
        let e = self.set;
        let a1 = (e >> 1);
        let a2 = E1;
        let a3 =  0;
        let a4 =  0; 
        return EdgeSet{set: e ^ ((E4|(E0 & a1) | (E1 & a2) | (E2 & a3) | (E3 & a4)) &!(self.filter_left() | self.filter_right())) };
    }
    pub fn up_ccw(self) -> EdgeSet {
        let e = self.set;
        return EdgeSet{set: e ^ ((E3 | (!(e >> 2) & E1)) & self.filter_up())};
    }
    pub fn up_cw(self) -> EdgeSet {
        let e = self.set;
        return EdgeSet{set: e ^ ((E3 | ((e >> 2) & E1)) & self.filter_up())};
    }
    pub fn down_ccw(self) -> EdgeSet {
        let e = self.set;
        return EdgeSet{set: e ^ ((E3 | (!(e >> 2) & E1)) & self.filter_down())};
    }
    pub fn down_cw(self) -> EdgeSet {
        let e = self.set;
        return EdgeSet{set: e ^ ((E3 | ((e >> 2) & E1)) & self.filter_down())};
    }

    pub fn right_cw(self) -> EdgeSet {
        let e = self.set;
        return EdgeSet{set: e ^ (( E2 | (!(e >> 2) & E0)) & self.filter_right())};
    }
    pub fn right_ccw(self) -> EdgeSet {
        let e = self.set;
        return EdgeSet{set: e ^ (( E2 | ((e >> 2) & E0)) & self.filter_right())};
    }
    pub fn left_cw(self) -> EdgeSet {
        let e = self.set;
        return EdgeSet{set: e ^ ((E2 | ((!(e >> 2)) & E0)) & self.filter_left())};
    }
    pub fn left_ccw(self) -> EdgeSet {
        let e = self.set;
        return EdgeSet{set: e ^ ((E2 | ((e >> 2) & E0)) & self.filter_left())};
    }
    pub fn front_ccw(self) -> EdgeSet {
        let e = self.set;
        return EdgeSet{set: e ^ (((E4|E3|E2) | ((e >> 3) & E0) | ((e >> 2) & E1))
                                 & self.filter_front())};
    }
    pub fn front_cw(self) -> EdgeSet {
        let e = self.set;
        return EdgeSet{set: e ^ (((E4|E3|E2) | ((e >> 2) & E0) | ((e >> 1) & E1))
                                 & self.filter_front())};
    }
    pub fn back_ccw(self) -> EdgeSet {
        let e = self.set;
        return EdgeSet{set: e ^ (((E4|E3|E2) | ((e >> 3) & E0) | ((e >> 2) & E1))
                                 & self.filter_back())};
    }
    pub fn back_cw(self) -> EdgeSet {
        let e = self.set;
        return EdgeSet{set: e ^ (((E4|E3|E2) | ((e >> 2) & E0) | ((e >> 1) & E1))
                                 & self.filter_back())};
    }
}
