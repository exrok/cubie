
#[derive(Clone,Copy,Debug)]
#[repr(u8)]
pub enum FTurn {
    Ucw,
    U2,
    Uccw,
    Rcw,
    R2,
    Rccw,
    Fcw,
    F2,
    Fccw,
    Dcw,
    D2,
    Dccw,
    Lcw,
    L2,
    Lccw,
    Bcw,
    B2,
    Bccw,
    
}


#[derive(Clone,Copy)]
struct CornerSetField {
    pub offset: u64
}
impl CornerSetField {
    const fn mask(self) -> u64 {
        0b00001_00001_00001_00001_00001_00001_00001_00001
            << self.offset
    }
    const fn to(self, dest: CornerSetField) -> u64 {
        dest.offset - self.offset
    }
    const fn from(self, dest: CornerSetField) -> u64 {
        self.offset - dest.offset
    }
}
const ET:CornerSetField = CornerSetField{offset: 0};
const EF:CornerSetField = CornerSetField{offset: 1};
const ER:CornerSetField = CornerSetField{offset: 2};
const EO0:CornerSetField = CornerSetField{offset: 3};
const EO1:CornerSetField = CornerSetField{offset: 4};
const COLOR_YELLOW: u8 = 0;
const COLOR_WHITE: u8 = 1;
const COLOR_RED: u8 = 2;
const COLOR_ORANGE: u8 = 3;
const COLOR_BLUE: u8 = 4;
const COLOR_GREEN: u8 = 5;

#[cfg(target_feature = "avx2")]
fn sdfas() {
}
#[cfg(not(target_feature = "avx2"))]
fn sdfas() {

}
fn pe(edge:u64) {
    for i in 0..8 {
        eprint!("{:05b}_", (edge >> ((7-i)*5))& 0b11111);
    }
   eprintln!();
    for i in 0..8 {
        eprint!("{:5}|", (edge >> ((7-i)*5))& 0b11111);
    }
   eprintln!();
}
#[derive(Clone,Copy, Debug)]
pub struct CornerColor{pub y: u8, pub z:u8, pub x:u8}
impl CornerColor {
    fn from(pos: u64, corner: u64, orientation: u64) -> CornerColor {
        let mut c = match corner & 0b111 {
            0b111 => CornerColor{y: COLOR_YELLOW, z: COLOR_RED, x: COLOR_GREEN},
            0b101 => CornerColor{y: COLOR_YELLOW, z: COLOR_ORANGE, x: COLOR_GREEN},
            0b001 => CornerColor{y: COLOR_YELLOW, z: COLOR_ORANGE, x: COLOR_BLUE},
            0b011 => CornerColor{y: COLOR_YELLOW, z: COLOR_RED, x: COLOR_BLUE},
            0b110 => CornerColor{y: COLOR_WHITE, z: COLOR_RED, x: COLOR_GREEN},
            0b100 => CornerColor{y: COLOR_WHITE, z: COLOR_ORANGE, x: COLOR_GREEN},
            0b000 => CornerColor{y: COLOR_WHITE, z: COLOR_ORANGE, x: COLOR_BLUE},
            0b010 => CornerColor{y: COLOR_WHITE, z: COLOR_RED, x: COLOR_BLUE},
            _ => panic!("invalid corner"),
        };
        match orientation {
            0b00 => {
                let test =((pos & 0b110 )>> 1) ^ ((corner & 0b110 )>> 1);
                if  test != 0b11 && test != 0b00 {
                    if (pos ^ corner) &0b1 == 0 {
                        c.swapxz();
                    }
                } else if (pos ^ corner) &0b1 != 0 {
                    c.swapxz();
                }
                c
            },
            0b01 => {
                let test =((pos & 0b110 )>> 1) ^ ((corner & 0b110 )>> 1);
                if  test != 0b11 && test != 0b00 {
                    if (pos ^ corner) &0b1 == 0 {
                        c.swapxz();
                    }
                } else if (pos ^ corner) &0b1 != 0 {
                    c.swapxz();
                }
                CornerColor{x:c.z, y:c.x, z:c.y}
            },
            0b10 => {
                let test =((pos & 0b110 )>> 1) ^ ((corner & 0b110 )>> 1);
                if  test != 0b11 && test != 0b00 {
                    if (pos ^ corner) &0b1 == 0 {
                        c.swapxz();
                    }
                } else if (pos ^ corner) &0b1 != 0 {
                    c.swapxz();
                }
                CornerColor{x:c.y, y:c.z, z:c.x}
            } 
            _ => panic!("invalid corner"),
        }
    }
    fn swapxz(&mut self) {
        std::mem::swap(&mut self.x, &mut self.z);
    }
}
#[derive(Debug)]
pub struct CornerColoring( pub [CornerColor;8]);
use std::num::NonZeroU64 as NzU64;
#[derive(Clone,Copy,PartialEq,Eq)]
pub struct CornerSet {
    pub set: NzU64
}
impl Default for CornerSet {
    fn default() -> CornerSet {
        unsafe {
            CornerSet{set: NzU64::new_unchecked(0b00111_00110_00101_00100_00011_00010_00001_00000)}
        }
    }
}

impl CornerSet {
    pub unsafe fn new_unchecked(x:u64) -> CornerSet {
        CornerSet{set: NzU64::new_unchecked(x)}
    }
    // pub fn store_turn(self,turn:Turn) -> CornerSet {
    //     unsafe {
    //         CornerSet{set:NzU64::new_unchecked(((turn as u64) << 48) | (self.set.get() & (!(0b11111u64 << 48))))}}
            
    // }
    // pub fn get_turn(self) -> Turn {
    //     unsafe {std::mem::transmute::<u8,Turn>((self.set.get() >> 48) as u8)}
    // }

    pub fn invert(self) -> CornerSet {
        let set = self.set.get();
        let mut res = 0;
        for i in 0..8 {
            let offset = i *5;
            let input = set >> offset;
            let mut dest = i | (input & 0b11000);
            let parity = input ^ dest;
            let f_parity = (parity >> 1) ^ (parity >> 2) ^ parity;
            if (f_parity & 0b1) == 0 {
                if (dest & &0b11_000) != 0 {
                    dest ^= 0b11_000;
                }
            }
            res |= dest << ((input & 0b0111)*5);
        }
        let mask_sym = (res | ((res & EO1.mask()) >> 1) | ((res & EO0.mask()) << 1)) & (EO1.mask() | EO0.mask());
        unsafe{ CornerSet{set:NzU64::new_unchecked( res  )}}
    }
    fn get_corner( self, corner: u64)->u64 {
        (self.set.get() >> (5 *corner)) & 0b11111
    }
    pub fn coloring(self) -> CornerColoring {
        let map:[usize;8] = [6,2,7,3,5,1,4,0];
        let mut def = [CornerColor{x:0,y:0,z:0};8];
        for i in 0..8u64 {
            let corner = self.get_corner(i);
            let pos = map[(corner & 0b111) as usize];
            def[pos] = CornerColor::from(corner as u64, i as u64, corner >> 3 );
        }
        CornerColoring(def)
    }
    fn filter_front(self) -> u64 {
        EO1.mask()^(EO1.mask() - (((self.set.get() >> 1)) & ET.mask()))
    }
    fn filter_back(self) -> u64 {
        EO1.mask()^(EO1.mask() - ((!(self.set.get() >> 1)) & ET.mask()))
    }
    fn filter_down(self) -> u64 {
        EO1.mask()^(EO1.mask() - ((!(self.set.get() )) & ET.mask()))
    }
    fn filter_top(self) -> u64 {
        EO1.mask()^(EO1.mask() - (((self.set.get() )) & ET.mask()))
    }
    fn filter_left(self) -> u64 {
        EO1.mask()^(EO1.mask() - ((!(self.set.get() >> 2)) & ET.mask()))
    }
    pub fn filter_right(self) -> u64 {
        EO1.mask()^(EO1.mask() - ((self.set.get() >> 2) & ET.mask()))
    }
    pub fn right_cw(self) -> CornerSet {
        let e = self.set.get();
        let t = (e ^ (e >> 1)) & ET.mask();
        let f = (t ^ ET.mask()) << 1;
        let eo0 = (!(e >> 1)) & EO0.mask();
        let res =( t|f |eo0) & self.filter_right();
        unsafe{ CornerSet{set:NzU64::new_unchecked(e ^ res)}}
    }
    pub fn right_2(self) -> CornerSet {
        let res =(ET.mask()|EF.mask() ) & self.filter_right();
        unsafe{ CornerSet{set:NzU64::new_unchecked(self.set.get() ^ res)}}
    }
    pub fn right_ccw(self) -> CornerSet {
        let e = self.set.get();
        let t = (e ^ (e >> 1)) & ET.mask();
        let f = (t) << 1;
        let eo0 = (!(e >> 1)) & EO0.mask();
        let res =( (t^ET.mask()) |f|eo0) & self.filter_right();
        unsafe{ CornerSet{set:NzU64::new_unchecked(e ^ res)}}
    }
    pub fn left_ccw(self) -> CornerSet {
        let e = self.set.get();
        let t = (e ^ (e >> 1)) & ET.mask();
        let f = (t ^ ET.mask()) << 1;
        let eo0 = (!(e >> 1)) & EO0.mask();
        let res =( t|f |eo0) & self.filter_left();
        unsafe{ CornerSet{set:NzU64::new_unchecked(e ^ res)}}
    }
    pub fn left_cw(self) -> CornerSet {
        let e = self.set.get();
        let t = (e ^ (e >> 1)) & ET.mask();
        let f = (t) << 1;
        let eo0 = (!(e >> 1)) & EO0.mask();
        let res =( (t^ET.mask()) |f|eo0) & self.filter_left();
        unsafe{ CornerSet{set:NzU64::new_unchecked(e ^ res)}}
    }

    pub fn left_2(self) -> CornerSet {
        let res =(ET.mask()|EF.mask() ) & self.filter_left();
        unsafe{ CornerSet{set:NzU64::new_unchecked(self.set.get() ^ res)}}
    }
    pub fn up_2(self) -> CornerSet {
        let res =(ER.mask()|EF.mask() ) & self.filter_top();
        unsafe{ CornerSet{set:NzU64::new_unchecked(self.set.get() ^ res)}}
    }
    pub fn up_cw(self) -> CornerSet {
        let e = self.set.get();
        let eo0 = ((e) ^ (e >> 1)) & (EO0.mask()|EF.mask());
        let eo1 = eo0 << 1 ;
        let res =( (eo0)|(eo1 ^ ER.mask())) & self.filter_top();
        unsafe{ CornerSet{set:NzU64::new_unchecked(e ^ res)}}
    }
    pub fn up_ccw(self) -> CornerSet {
        let e = self.set.get();
        let eo0 = ((e) ^ (e >> 1)) & (EO0.mask()|EF.mask());
        let eo1 = eo0 << 1 ;
        let res =( (eo0 ^ EF.mask())|eo1) & self.filter_top();
        unsafe{ CornerSet{set:NzU64::new_unchecked(e ^ res)}}
    }

    pub fn print_conv(self)  {
        let mn = self.set.get();
        let at = |a:u8|(self.set.get() >> (a*5))& 0b11111;
        let mut atlus = [0,0,0,0,0,0,0,0];
        for i in 0..8 {
            atlus[(at(i) & 0b111) as usize] = i | (at(i) as u8 & 0b11000);
        }
        // for i in 0..8 {
        //     eprintln!("F({}) := {}",i,at(i));
        //     eprintln!("F'({}) := {}",at(i),atlus[at(i) as usize]);
        // }
        let conv = [6,2,5,1,7,3,4,0];
        let mut conv_inv = [0,0,0,0,0,0,0,0];
        for i in 0..8 {
            conv_inv[conv[i] as usize] = i;//| (at(i) as u8 & 0b11000);
        }
        let mut corners = [6,2,5,1,7,3,4,0];
        let swapo = |o:u8| { ((o >> 1) & 0b1) | ( (o << 1) & 0b10) };
        for i in 0..8 {
            let bits = atlus[conv_inv[i] as usize] as u8; 
            let loc = conv[(bits & 0b111) as usize] as u8;
            let ori = match (i,bits >> 3) {
                (0,a) => swapo(a),
                (7,a) => swapo(a),
                (5,a) => swapo(a),
                (2,a) => swapo(a),
                (_,ori) => ori
            };
            corners[i] = loc | (ori << 3);
        }
        for i in 0..8 {
            eprint!("{:05b}_",corners[i]);
        }
        eprintln!("");
        for i in 0..8 {
            eprint!(" {:1}:{:1}  ", i,corners[i]&0b111);
        }
        // eprintln!("");
        // for i in (0..8) {
        //     eprint!("{:2}:{:2} ", i ,at(i));
        // }
        eprintln!("");
    }
    pub fn print(self)  {
        for i in (0..8).rev() {
            eprint!("{:05b}_", (self.set.get() >> ((7-i)*5))& 0b11111);
        }
        eprintln!("");
        for i in (0..8).rev() {
            eprint!("{:5}_", (self.set.get() >> ((7-i)*5))& 0b11111);
        }
        eprintln!("");
    }
    pub fn down_ccw(self) -> CornerSet {
        let e = self.set.get();
        let eo0 = ((e) ^ (e >> 1)) & (EO0.mask()|EF.mask());
        let eo1 = eo0 << 1 ;
        let res =( (eo0)|(eo1 ^ ER.mask())) & self.filter_down();
        unsafe{ CornerSet{set:NzU64::new_unchecked(e ^ res)}}
    }
    pub fn down_cw(self) -> CornerSet {
        let e = self.set.get();
        let eo0 = ((e) ^ (e >> 1)) & (EO0.mask()|EF.mask());
        let eo1 = eo0 << 1 ;
        let res =( (eo0 ^ EF.mask())|eo1) & self.filter_down();
        unsafe{ CornerSet{set:NzU64::new_unchecked(e ^ res)}}
    }
    
    pub fn down_2(self) -> CornerSet {
        let res =(ER.mask()|EF.mask() ) & self.filter_down();
        unsafe{ CornerSet{set:NzU64::new_unchecked(self.set.get() ^ res)}}
    }
    pub fn back_ccw(self) -> CornerSet {
        let e = self.set.get();
        let t = ((e) ^ (e >> 2)) & ET.mask();
        let r = (t ) << 2;

        let eo1 = (!(e<<1))  & EO1.mask();
        let res =( r|(t^ ET.mask())|eo1) & self.filter_back();
        unsafe{ CornerSet{set:NzU64::new_unchecked(e ^ res)}}
    }
    pub fn back(self) -> [CornerSet;3] {
        return [self.back_cw(), self.back_ccw(), self.back_2()]
    }
    pub fn back_cw(self) -> CornerSet {
        let e = self.set.get();
        let t = ((e) ^ (e >> 2)) & ET.mask();
        let r = (t ^ ET.mask()) << 2;

        let eo1 = (!(e<<1))  & EO1.mask();
        let res =( r|t|eo1) & self.filter_back();
        unsafe{ CornerSet{set:NzU64::new_unchecked(e ^ res)}}
    }

    pub fn back_2(self) -> CornerSet {
        let res =(ER.mask()|ET.mask() ) & self.filter_back();
        unsafe{ CornerSet{set:NzU64::new_unchecked(self.set.get() ^ res)}}
    }
    pub fn front_2(self) -> CornerSet {
        let res =(ER.mask()|ET.mask() ) & self.filter_front();
        unsafe{ CornerSet{set:NzU64::new_unchecked(self.set.get() ^ res)}}
    }
    pub fn front_cw(self) -> CornerSet {
        let e = self.set.get();
        let t = (e ^ (e >> 2)) & ET.mask();
        let r = (t << 2)|(t ^ET.mask());
        let eo1 = (!(e<<1))  & EO1.mask();
        let res =( r|eo1) & self.filter_front();
        unsafe{ CornerSet{set:NzU64::new_unchecked(e ^ res)}}
    }
    pub fn front_ccw(self) -> CornerSet {
        let e = self.set.get();
        let t = ((e) ^ (e >> 2)) & ET.mask();
        let r = (t ^ ET.mask()) << 2;

        let eo1 = (!(e<<1))  & EO1.mask();
        let res =( r|t|eo1) & self.filter_front();
        unsafe{ CornerSet{set:NzU64::new_unchecked(e ^ res)}}
    }
    pub fn index_orientation(self) -> u64 {
        let mut b = self.set.get() & 0b11000_11000_11000_11000_11000_11000_11000;

        b *= 1+32*3; // x3
        b &= 0b11111_00000_11111_00000_11111_00000_11111_00000_000;
  //      pe(b);
        b *=  1+1024*9; // x9
        b &= 0b11111_11111_00000_00000_11111_11111_00000_00000_00000_000;
 //       eprintln!("{} {}",b >> 35,(b >> 15) &0b1111_11111);
//        pe(b);
        b = b.wrapping_mul( 1*27+1048576); //x27
//       pe2(b);
      // eprintln!("{:0b}: {}",b>>35,b>>35);
      // eprintln!("{:0b}: {}",b>>36,b>>36);
        (b >>38) &0b11111_11111_11111
    }

    pub fn index_permutation(self) -> u64 {
        const fn section(sh: u64) -> u64 {
            return 0b00111 << (sh*5);
        }
        let x:u64 = self.set.get() & 0b00111_00111_00111_00111_00111_00111_00111_00111;
        let dist = 0b00000_00000_00001_00001_00001_00001_00001_00001u64;
        let s_mask = 0b00000_00000_01000_01000_01000_01000_01000_01000u64;
        let x_cmp = s_mask - ((x>>10) + dist); //Shift allows dist to fit in 32bit imediate
        let mut acc = ((x & section(0)) as u32) * 5040;
        let x = x>>5; //Shift allows s_mask to fit in 32bit imediate
        (acc + (((x_cmp + (x & section(0)).wrapping_mul(dist)) & s_mask).count_ones() * 720)
        + (((x_cmp + (x & section(1)).wrapping_mul(dist)) & s_mask).count_ones() * 120)
        + (((x_cmp + (x & section(2)).wrapping_mul(dist)) & s_mask).count_ones() * 24)
        + (((x_cmp + (x & section(3)).wrapping_mul(dist)) & s_mask).count_ones() * 6)
        + (((x_cmp + (x & section(4)).wrapping_mul(dist)) & s_mask).count_ones() * 2)
        + (((x_cmp + (x & section(5)).wrapping_mul(dist)) & s_mask).count_ones() * 1)) as u64
    }

    pub fn apply(self, turn:FTurn) -> CornerSet {
        use FTurn::*;
         match turn {
            R2 => self.right_2(),
            Rcw => self.right_cw(),
            Rccw => self.right_ccw(),
            L2 => self.left_2(),
            Lcw => self.left_cw(),
            Lccw => self.left_ccw(),
            U2 => self.up_2(),
            Ucw => self.up_cw(),
            Uccw => self.up_ccw(),
            D2 => self.down_2(),
            Dcw => self.down_cw(),
            Dccw => self.down_ccw(),
            F2 => self.front_2(),
            Fcw => self.front_cw(),
            Fccw => self.front_ccw(),
            B2 => self.back_2(),
            Bcw => self.back_cw(),
            Bccw => self.back_ccw(),
            // M2 => self.right_2().left_2(),
            // Mcw => self.right_cw().left_ccw(),
            // Mccw => self.right_ccw().left_cw(),
            // S2 => self.front_2().back_2(),
            // Scw => self.front_ccw().back_cw(),
            // Sccw => self.front_cw().back_ccw(),
            // E2 => self.up_2().down_2(),
            // Ecw => self.up_ccw().down_cw(),
            // Eccw => self.up_cw().down_ccw(),
        }
    }
    pub fn index(self) -> u64 {
        self.index_permutation() + self.index_orientation()*40320
    }
}
