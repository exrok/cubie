use crate::{FaceMove, TileMap, MapError};

/// Represents a corner cubie on the 3x3 cube by position.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Corner {
    URF = 0,
    DRF = 1,
    URB = 2,
    DRB = 3,
    ULF = 4,
    DLF = 5,
    ULB = 6,
    DLB = 7,
}

use crate::Face;
use std::mem::transmute;
impl Corner {
    ///Get the face of the corner perpendicular to the y-axis, either `Face::Up` or `Face::Down`;
    #[inline]
    pub fn y(self) -> Face {
        unsafe { transmute(self as u8 & 1) }
    }
    ///Get the face of the corner perpendicular to the x-axis, either `Face::Right` or `Face::Left`;
    #[inline]
    pub fn x(self) -> Face {
        unsafe { transmute(((self as u8) >> 2) | (Face::Right as u8)) }
    }
    ///Get the face of the corner perpendicular to the z-axis, either `Face::Front` or `Face::Back`;
    #[inline]
    pub fn z(self) -> Face {
        unsafe { transmute((((self as u8) >> 1) & 1) | (Face::Front as u8)) }
    }
    #[inline]
    pub fn corners() -> impl Iterator<Item = Corner> {
        (0..8).map(|i| Corner::from(i))
    }
}

impl std::convert::From<u8> for Corner {
    fn from(a: u8) -> Corner {
        unsafe { std::mem::transmute::<u8, Corner>(a & 0b111) }
    }
}

#[derive(Clone, Copy, Debug, PartialEq,Eq)]
#[repr(u8)]
pub enum Twist {
    Identity,
    Cw,
    Ccw,
}

impl Twist {
    pub fn is_identity(self) -> bool {
        self == Twist::Identity
    }
    pub fn is_ccw(self) -> bool {
        self == Twist::Ccw
    }
    pub fn is_cw(self) -> bool {
        self == Twist::Cw
    }
    pub fn inverse(self) -> Twist {
        (&[Twist::Identity, Twist::Ccw, Twist::Cw])[self as usize]
    }
}

impl Mul for Twist {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        unsafe { transmute((rhs as u8 + self as u8) % 3) }
    }
}

impl MulAssign for Twist {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

/// Corner piece mapping for 3x3 cube: Corner -> (CornerPosition, Twist)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CornerMap {
    pub(crate) raw: u64,
}

impl Default for CornerMap {
    fn default() -> CornerMap {
        CornerMap {
            raw: 0x0706050403020100,
        }
    }
}
impl std::fmt::Debug for CornerMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CornerMap")?;
        f.debug_map().entries(self.iter()).finish()
    }
}

#[derive(Copy,Clone,PartialEq,Eq,Debug)]
#[repr(transparent)]
pub struct COIndex(pub u32);

impl COIndex {
    pub const SIZE: u32 = 2187; // 3^8 
}

#[derive(Copy,Clone,PartialEq,Eq,Debug)]
#[repr(transparent)]
pub struct CPIndex(pub u32);

impl CPIndex {
    pub const SIZE: u32 = 40320; // 12 factorial
}

impl CornerMap {

    pub fn get(&self, corner: Corner) -> (Corner, Twist) {
        let m = (self.raw >> (8 * corner as u64)) as u8;
        unsafe { (Corner::from(m), transmute(m >> 3)) }
    }
    pub fn iter(self) -> impl Iterator<Item = (Corner, (Corner, Twist))> + ExactSizeIterator {
        let mut set = self.raw;
        (0..8).map(move |i| unsafe {
            let corner: Corner = transmute(i as u8);
            let position: Corner = transmute((set & 0b0111) as u8);
            let ori: Twist = transmute(((set >> 3) & 0b11) as u8);
            set >>= 8;
            (corner, (position, ori))
        })
    }
    pub fn from_iter(iter: impl Iterator<Item = (Corner, (Corner, Twist))>) -> Result<CornerMap, MapError> {
        let mut res = 0x0706050403020100;
        for (corner, (pos, ori)) in iter {
            res &= !(0xffu64 << ((corner as u64) * 8));
            res |= ((pos as u64) | ((ori as u64) << 3)) << ((corner as u64) * 8);
        }
        CornerMap::from_raw(res)
    }

    pub fn inverse(self) -> CornerMap {
        let set = self.raw;
        let mut res = 0;

        for i in 0..8 {
            let offset = i * 8;
            let input = set >> offset;
            let dest = i | (input & 0b11000);
            let off = (input & 0b0111) * 8;
            res |= dest << off;
        }

        let mnx = (res | (res >> 1)) & 0x08080808_08080808;
        CornerMap {
            raw: res ^ mnx ^ (mnx << 1),
        }
    }

    pub fn inverse_multiply(self, b: CornerMap) -> CornerMap {
        CornerMap {
            raw: map_inv_mul(self.raw, b.raw),
        }
    }

    // pub fn multiply_slow(self, b: CornerMap) -> CornerMap {
    //     CornerMap::from_iter(self.iter().map(|(edge,(pos,twist))| {
    //         let (new_pos, other_twist) = b.get(pos);
    //         (edge, (new_pos, other_twist*twist))
    //     })).unwrap()
    // }

    fn multiply(self, b: CornerMap) -> CornerMap {
        CornerMap {
            raw: map_mul(self.raw, b.raw),
        }
    }

    // pub fn inverse_slow(self) -> CornerMap {
    //     CornerMap::from_iter(self.iter().map(|(edge,(pos,twist))| {
    //         (pos, (edge, twist.inverse()))
    //     })).unwrap()
    // }



    pub(crate) fn orientation_residue(self) -> Twist {
        let res = ((((self.raw&0x1818_1818_1818_1818)
        ).wrapping_mul(0x01010101_01010101))>>57) as u8;
        unsafe { transmute((res ) % 3) }
    }

    pub fn is_solved(self) -> bool {
        self == CornerMap::default()
    }

    /// Returns true if the Map represents a well-defined permutation in range.
    /// In safe user code this should ALWAYS return true. A valid cube may still 
    /// be unsolvable.
    pub(crate) fn validate(self) -> Result<(),MapError> {
        if (self.raw & 0xe0e0e0e0_e0e0e0e0) != 0 { 
            return Err(MapError::OutOfBounds); // extra bits
        }
        if (self.raw & (self.raw >> 1) & 0x08_08_08_08_08_08_08_08) != 0 { 
            return Err(MapError::OutOfBounds); // extra bits
        }
        // cannot use CornerMap::iter since it assumes validity
        let mut corners_mask = 0u32;
        for i in 0..8 {
            let v = (self.raw >> (i*8))&0b111;
            corners_mask |= 1 << v;
        }
        if corners_mask != 0b1111_1111 {
            return Err(MapError::Duplicate); //All corners appear exactly once
        }
        if !self.orientation_residue().is_identity() {
            return Err(MapError::Orientation); //All corners appear exactly once
        }
        Ok(())
    }
    pub fn permutation_parity(self) -> bool {
        let mut transc = false;
        { // WALK edge permutation in discrete cycle form
            let mut rem =  0b1111_1110u32;
            let mut at = Corner::URF;
            // print!("({:?}", at)
            while rem != 0 {
                at = self.get(at).0;
                let bit = 1u32 << (at as u8);
                if rem & bit != 0 {
                    rem ^= bit;
                    transc ^= true;
                    // print!(" {:?}", at);
                    continue;
                }
                at = unsafe{std::mem::transmute(rem.trailing_zeros() as u8)};
                rem ^= 1u32 << (at as u8);
                // print!(")({:?}", at);
            }
            // println!(")");
        }
        transc
    }
    pub fn permutation_index(self) -> CPIndex {
        const fn section(sh: u64) -> u64 {
            return 0b00111 << (sh * 8);
        }
        let x: u64 = self.raw & 0x0707070707070707;
        let dist = 0x0101_0101_0101;
        let s_mask = 0x0808_0808_0808;
        let x_cmp = s_mask - ((x >> 16) + dist); //Shift a CHEC CHANGE
        let acc = ((x & section(0)) as u32) * 5040;
        let x = x >> 8; //Shift allows s CHANGE
        CPIndex(acc + (((x_cmp + (x & section(0)).wrapping_mul(dist)) & s_mask).count_ones() * 720)
            + (((x_cmp + (x & section(1)).wrapping_mul(dist)) & s_mask).count_ones() * 120)
            + (((x_cmp + (x & section(2)).wrapping_mul(dist)) & s_mask).count_ones() * 24)
            + (((x_cmp + (x & section(3)).wrapping_mul(dist)) & s_mask).count_ones() * 6)
            + (((x_cmp + (x & section(4)).wrapping_mul(dist)) & s_mask).count_ones() * 2)
            + (((x_cmp + (x & section(5)).wrapping_mul(dist)) & s_mask).count_ones() * 1))
    }
    pub fn set_permutation_index(&mut self, index: CPIndex) {
        let mut idx = index.0 as u32;
        let mut val = 0xFEDCBA9876543210u64;
        let mut extract = 0u64;
        for p in 2..8 as u64 + 1 {
            extract = extract << 4 | (idx as u64) % p;
            idx /= p as u32;
        }
        let mut res = 0;
        for e in 0..7 {
            let v = ((extract & 0xf) << 2) as u32;
            extract >>= 4;
            res |= (((val >> v) & 0x7) as u64) << (e * 8);
            let m = (1u64 << v) - 1;
            val = val & m | (val >> 4) & !m; // TODO verfiy
        }
        res |= ((val & 0x7) as u64) << (8 * 7);
        self.raw &= 0x18_18_18_18_18_18_18_18;
        self.raw |= res;
    }

    pub fn orientation_index(self) -> COIndex {
        let mut b = (self.raw & 0x00_18_18_18__18_18_18_18) >> 3;
        b *= 1 + (1 << 8) * 3; // x3
        b &= 0x1f001f001f001f00;
        b = b.wrapping_mul(1 + (1 << 16) * 9); // x9
        let x = (b >> 24) & 0xff;
        let y = b >> (24 + 24 + 8);
        COIndex((y * 27 + x) as u32)
    }

    pub fn set_orientation_index(&mut self, index: COIndex) {
        let mut index = index.0 as u64;
        let mut sum = 0;
        let mut res = 0;
        let mut hp = |i: u64| {
            let m = index % 3;
            sum += m;
            index /= 3;
            res |= m << (i * 8 + 3);
        };

        hp(3);
        hp(2);
        hp(1);
        hp(0);
        hp(6);
        hp(5);
        hp(4);
        res |= ((3 * 10 - sum) % 3) << (7 * 8 + 3);
        self.raw &= !0x00_18_18_18__18_18_18_18;
        self.raw |= res;
        // self.fix_orientation();
    }

}

#[inline]
fn fast_map(a: u64, mut map: impl FnMut(u64, u64)) {
    let set1 = a << 3;
    for i in 0..8 {
        //this will unroll
        let offset = i * 8;
        let input = (set1 >> offset) & 0b111_000;
        map(input, offset);
    }
}

pub fn map_mul(a: u64, b: u64) -> u64 {
    let mut ret = 0;
    fast_map(a, |f_x, x| {
        ret |= ((b >> f_x) & 0xff) << x;
    });
    multiply_orientation(a, ret)
}

#[inline]
fn multiply_orientation(a: u64, ret: u64) -> u64 {
    const SELECT: u64 = 0x18_18_18_18_18_18_18_18;

    let mut xm = a & SELECT;
    xm = (ret & SELECT) + (xm); //
    xm = ((xm >> 2) + xm) & SELECT; // orientation MOD 3
    let three_bit = (xm >> 1) & xm; //

    let p2 = ret ^ ((ret ^ xm ^ (three_bit | (three_bit << 1))) & SELECT);
    p2
}

pub fn map_inv_mul(a: u64, b: u64) -> u64 {
    let b = multiply_orientation(a ^ 0x18_18_18_18_18_18_18_18, b);
    let mut ret = 0;
    fast_map(a, |f_x, x| {
        ret |= ((b >> x) & 0xff) << f_x;
    });
    ret
}
use std::ops::{Mul, MulAssign};

impl std::ops::Mul for CornerMap {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self.multiply(rhs)
    }
}

impl std::ops::MulAssign for CornerMap {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.multiply(rhs);
    }
}

impl std::convert::From<FaceMove> for CornerMap {
    #[inline]
    fn from(turn: FaceMove) -> Self {
        let turns = &[
            CornerMap {
                raw: 0x0702050603000104,
            }, //Ucw
            CornerMap {
                raw: 0x0700050203040106,
            }, //U2
            CornerMap {
                raw: 0x0704050003060102,
            }, //Uccw
            CornerMap {
                raw: 0x0506010407020300,
            }, //Dcw
            CornerMap {
                raw: 0x0106030405020700,
            }, //D2
            CornerMap {
                raw: 0x0306070401020500,
            }, //Dccw
            CornerMap {
                raw: 0x07060c1003021509,
            }, //Fcw
            CornerMap {
                raw: 0x0706000103020405,
            }, //F2
            CornerMap {
                raw: 0x070609150302100c,
            }, //Fccw
            CornerMap {
                raw: 0x130f05040a160100,
            }, //Bcw
            CornerMap {
                raw: 0x0203050406070100,
            }, //B2
            CornerMap {
                raw: 0x160a05040f130100,
            }, //Bccw
            CornerMap {
                raw: 0x07060504110b0812,
            }, //Rcw
            CornerMap {
                raw: 0x0706050400010203,
            }, //R2
            CornerMap {
                raw: 0x0706050412080b11,
            }, //Rccw
            CornerMap {
                raw: 0x0e14170d03020100,
            }, //Lcw
            CornerMap {
                raw: 0x0405060703020100,
            }, //L2
            CornerMap {
                raw: 0x0d17140e03020100,
            }, //Lccw
        ];
        turns[turn as usize]
    }
}


/// #Raw Interface
/// todo document
impl CornerMap {
    pub fn from_raw(raw: u64) -> Result<CornerMap, MapError> {
        let cm = CornerMap{raw};
        cm.validate().map(|_| cm)
    }
    
    pub unsafe fn from_raw_unchecked(raw: u64) -> CornerMap {
        CornerMap{raw}
    }

    pub fn raw(self) -> u64 {
        self.raw
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn corner_orientation_index_mapping() {
        for index in 0..2187 {
            let mut cube = CornerMap::default();
            cube.set_orientation_index(COIndex(index));
            assert_eq!(index, cube.orientation_index().0);
            assert_eq!(cube.validate(), Ok(()));
        }
    }
    #[test]
    fn corner_premutation_index_mapping() {
        for index in 0..40320 {
            let mut cube = CornerMap::default();
            cube.set_permutation_index(CPIndex(index));
            assert_eq!(index, cube.permutation_index().0);
            assert_eq!(cube.validate(), Ok(()));
        }
    }
}
