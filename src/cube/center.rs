use crate::{Cube, Face, MapError};
use std::convert::TryInto;

/// Center piece mapping for 3x3 cube: Center -> CenterPosition
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct CenterMap {
    pub raw: u64,
}
impl Default for CenterMap {
    fn default() -> CenterMap {
        CenterMap {
            raw: 0x04_02_00 << 5,
        }
    }
}

impl std::ops::Mul for CenterMap {
    type Output = Self;
    #[inline]
    fn mul(self, centers: CenterMap) -> Self {
        let map = centers.raw;
        let input = ((self.raw & 0xc0c0c0) >> 3) as u32;
        let output = (self.raw & 0x202020)
            ^ (map.wrapping_shr(input) & 0xff)
            ^ ((map.wrapping_shr(input >> 8) & 0xff) << 8)
            ^ ((map.wrapping_shr(input >> 16) & 0xff) << 16);
        CenterMap { raw: output }
    }
}

impl std::ops::MulAssign for CenterMap {
    #[inline]
    fn mul_assign(&mut self, centers: CenterMap) {
        *self = *self * centers;
    }
}

impl std::fmt::Debug for CenterMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CenterMap")?;
        f.debug_map().entries(self.iter()).finish()
    }
}

impl CenterMap {
    #[inline]
    pub fn cube(self) -> Cube {
        crate::moves::ROTATION_TABLE[self.index() as usize]
    }
    #[inline]
    pub fn validate(self) -> Result<(), MapError> {
        if let Some(cube) = crate::moves::ROTATION_TABLE.get(self.index() as usize) {
            if cube.centers() == self {
                Ok(())
            } else {
                Err(MapError::OutOfBounds)
            }
        } else {
            Err(MapError::OutOfBounds)
        }
    }

    pub fn inverse_multiply(self, rhs: CenterMap) -> CenterMap {
        let a = self.raw;
        let b = rhs.raw;
        let g = b ^ (a & 0x202020);
        let input = ((self.raw & 0xc0c0c0) >> 3) as u32;
        let mut output = 0;
        output |= (g & 0xff).wrapping_shl(input);
        output |= ((g >> 8) & 0xff).wrapping_shl(input >> 8);
        output |= ((g >> 16) & 0xff).wrapping_shl(input >> 16);

        CenterMap { raw: output }
    }

    #[inline]
    pub fn iter(self) -> impl Iterator<Item = (Face, Face)> + ExactSizeIterator {
        (0..3).map(move |i| {
            let face: Face = (i * 2).try_into().unwrap();
            (face, self.get(face))
        })
    }
    pub fn permutation_parity(self) -> bool {
        // let mut parity_mask = 0u32;
        // for (i, rot) in cubie::moves::ROTATION_TABLE.iter().enumerate() {
        //     eprintln!("{}=> {}",i, rot.corners().permutation_parity() ^rot.edges().permutation_parity() );
        //     let parity = rot.corners().permutation_parity() ^rot.edges().permutation_parity() ;
        //     parity_mask |= (parity as u32) << i;
        // }
        // eprintln!("{:0b}",parity_mask);
        let parity_mask = 0b111100110011000000111100;
        (parity_mask >> self.index()) & 0b1 == 1
    }
    pub fn index(self) -> u8 {
        let mut index = (self.raw & 0xE0_E0_C0) * (10 + (1 << 8) * 1 + (1 << 16) * 7);
        index = (!index >> 21) & 0b11111;
        index ^= (index + 0b1000) >> 2;
        index as u8
    }

    pub fn get(self, face: Face) -> Face {
        let v = (self.raw >> ((((face as u8) & 0b110) << 2) + 5)) as u8;
        unsafe { std::mem::transmute((v & 0b111) ^ ((face as u8) & 1)) }
    }

    pub fn inverse(self) -> CenterMap {
        let input = ((self.raw & 0xc0c0c0) >> 3) as u32;
        let output = ((self.raw & 0b10_0000).wrapping_shl(input))
            | ((0b100_0000 | ((self.raw >> 8) & 0b10_0000)).wrapping_shl(input >> 8))
            | ((0b1000_0000 | ((self.raw >> 16) & 0b10_0000)).wrapping_shl(input >> 16));
        CenterMap { raw: output }
    }
}

/// #Raw Interface
/// todo document
impl CenterMap {
    pub fn from_raw(raw: u64) -> Result<CenterMap, MapError> {
        let cm = CenterMap { raw };
        cm.validate().map(|_| cm)
    }

    pub unsafe fn from_raw_unchecked(raw: u64) -> CenterMap {
        CenterMap { raw }
    }

    pub fn raw(self) -> u64 {
        self.raw
    }
}
