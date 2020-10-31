use crate::{ CornerMap, Face, Move };

/// Center piece mapping for 3x3 cube: Center -> CenterPosition
#[derive(Clone,Copy,Eq,PartialEq)]
pub struct CenterMap {
    pub map: u64,
}
impl Default for CenterMap {
    fn default() -> CenterMap {
        CenterMap{map: 0x04_02_00<<5}
    }
}

#[derive(Clone,Copy,Eq,PartialEq,Hash)]
pub struct CenteredCornerMap {
    pub raw: u64
}

impl Default for CenteredCornerMap {
    fn default() -> CenteredCornerMap {
        CenteredCornerMap{
            raw: CenterMap::default().map | CornerMap::default().set,
        }
    }
}

impl std::ops::Mul for CenteredCornerMap {
    type Output = Self;
    fn mul(self, rhs: CenteredCornerMap) -> Self {
        let corners = self.corners() * rhs.corners(); 
        let centers = self.centers() * rhs.centers(); 
        CenteredCornerMap{raw: centers.map | corners.set}
    }
}
impl CenteredCornerMap {
    #[inline]
    pub fn corners(self) -> CornerMap {
        CornerMap{set: self.raw & 0x1f1f1f1f_1f1f1f1f }
    }
    #[inline]
    pub fn centers(self) -> CenterMap {
        CenterMap{map: self.raw & 0xe0e0e0}
    }
}

impl From<CornerMap> for CenteredCornerMap {
    fn from(cm: CornerMap) -> CenteredCornerMap {
        CenteredCornerMap{raw: CenterMap::default().map |
                          cm.set
        }
    }
}


impl std::ops::MulAssign<Move> for CenterMap {
    #[inline]
    fn mul_assign(&mut self, mv: Move) {
        *self = *self * mv.centers();
    }
}
impl std::ops::Mul<Move> for CenterMap {
    type Output = Self;
    #[inline]
    fn mul(self, mv: Move) -> Self {
        self* mv.centers()
    }
}
impl std::ops::Mul for CenterMap {
    type Output = Self;
    #[inline]
    fn mul(self, centers: CenterMap) -> Self {
        let map = centers.map ;
        let input = ((self.map&0xc0c0c0)>>3) as u32;
        let output = (self.map & 0x202020)^
            (map.wrapping_shr(input) & 0xff) ^
            ((map.wrapping_shr(input >> 8) & 0xff) << 8) ^
            ((map.wrapping_shr(input >> 16) & 0xff) << 16) ;
        CenterMap{map: output}
    }
}


use std::convert::TryFrom;
use std::convert::TryInto;

impl std::fmt::Debug for CenterMap {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       f.write_str("CenterMap")?;
       f.debug_map()
           .entries(self.iter())
           .finish()
    }
}

impl CenterMap {
    pub fn inverse_multiply(self, rhs: CenterMap) -> CenterMap {
        let a = self.map;
        let b = rhs.map;
        let g = b ^ (a & 0x202020);
        let input = ((self.map&0xc0c0c0)>>3) as u32;
        let mut output = 0; 
        output |= (g& 0xff).wrapping_shl(input);
        output |= ((g>>8)& 0xff).wrapping_shl(input>>8);
        output |= ((g>>16)& 0xff).wrapping_shl(input>>16);

        CenterMap{map:output}
    }

    #[inline]
    pub fn iter(self) ->  impl Iterator<Item=(Face,Face)> + ExactSizeIterator {
        (0..3).map(move |i|{
            let face:Face = (i*2).try_into().unwrap();
            (face, self.get(face))
        })
    }

    pub fn index(self) -> u8 {
        let xx = self.map;
        // let index = (xx&0xe0_60_c0)*(3+(1<<8)*1 + (1<<16)*7);
        // let mut index = (index >> 21)&0b11111;
        // let mk = 0-((index + 0b1000)>>5);
        // index = index + ((((!index&0b11)<<3) - (index & 0b11011))&mk);
        // index as u8

        let mut index = (xx & 0xE0_E0_C0) * (10 + (1<<8)*1 + (1<<16)*7);
        index = (!index >> 21) & 0b11111;
        index ^= (index + 0b1000) >> 2;
        index as u8
    }

    pub fn get(self, face: Face) -> Face {
        let v = (self.map>>((((face as u8)&0b110)<<2)+5)) as u8;
        unsafe {
           ((v&0b111)^((face as u8)&1)).try_into().unwrap()
        }
    }
    pub fn inverse(self) -> CenterMap  {
        let input = ((self.map&0xc0c0c0)>>3) as u32;
        let output = 
            ((self.map&0b10_0000).wrapping_shl(input))|
            ((0b100_0000|((self.map>>8)&0b10_0000)).wrapping_shl(input >> 8))|
            ((0b1000_0000|((self.map>>16)&0b10_0000)).wrapping_shl(input >> 16));
        CenterMap{map: output}
    }
    // fn tilemap(self) -> TileMap {
    //     let mut tilemap = TileMap::default();
    //     //todo refact with get and opposite to avoid bound check on array
    //     let a = ((self.map>>5)&0b111) as usize;
    //     tilemap.map[a][4] = Some(Face::Up);
    //     tilemap.map[a^1][4] = Some(Face::Down);

    //     let b = ((self.map>>(5+8))&0b111) as usize;
    //     tilemap.map[b][4] = Some(Face::Front);
    //     tilemap.map[b^1][4] = Some(Face::Back);
        
    //     let c = ((self.map>>(5+16))&0b111) as usize;
    //     tilemap.map[c][4] = Some(Face::Right);
    //     tilemap.map[c^1][4] = Some(Face::Left);
    //     tilemap
    // }
}
