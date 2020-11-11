use crate::CenterMap;
use crate::{CornerMap, Cube, EdgeMap, Face, FixedCentersCube};
use std::convert::TryFrom;
use std::convert::TryInto;

use std::mem;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Move {
    Ucw,
    U2,
    Uccw,
    Dcw,
    D2,
    Dccw,
    Fcw,
    F2,
    Fccw,
    Bcw,
    B2,
    Bccw,
    Rcw,
    R2,
    Rccw,
    Lcw,
    L2,
    Lccw,

    Ecw,
    E2,
    Eccw,
    Scw,
    S2,
    Sccw,
    Mcw,
    M2,
    Mccw,

    Ycw,
    Y2,
    Yccw,
    Zcw,
    Z2,
    Zccw,
    Xcw,
    X2,
    Xccw,
}

impl std::convert::TryFrom<u8> for Move {
    type Error = &'static str;
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value >= 36 {
            return Err("Value too large (>36).");
        } else {
            unsafe { Ok(mem::transmute(value)) }
        }
    }
}

pub static ROTATION_TABLE: &[Cube; 24] = &[
    Cube::from_raw_unchecked(0x12080E140B31B74D, 0x0394869C612DE71A), //
    Cube::from_raw_unchecked(0x140E08120D37916B, 0x029CC48CE50CEF58), //
    Cube::from_raw_unchecked(0x15090F130C10768A, 0x0ADE96D637984A33), //
    Cube::from_raw_unchecked(0x130F09150A1650AC, 0x0BD6D4C6B3B94271), //
    Cube::from_raw_unchecked(0x0607020304458021, 0x000C41ADAF45250B), //
    Cube::from_raw_unchecked(0x030207060140A504, 0x018022B56974A968), //
    Cube::from_raw_unchecked(0x0915100C136F0AB6, 0x09C2515A149BD2D5), //
    Cube::from_raw_unchecked(0x0C101509166A2F93, 0x084E3242D2AA5EB6), //
    Cube::from_raw_unchecked(0x170D0B110E149248, 0x0218E584A71C6B79), //
    Cube::from_raw_unchecked(0x110B0D170812B46E, 0x0310A794233D633B), //
    Cube::from_raw_unchecked(0x0F13160A15490C90, 0x08CA134A90BADA97), //
    Cube::from_raw_unchecked(0x0A16130F104C29B5, 0x09467052568B56F4), //
    Cube::from_raw_unchecked(0x0E14170D12882B51, 0x0C677A10C2039885), //
    Cube::from_raw_unchecked(0x0D17140E118B0872, 0x0CE35B1880131CA4), //
    Cube::from_raw_unchecked(0x0100030205A46706, 0x04A16A290E608062), //
    Cube::from_raw_unchecked(0x0203000106A74425, 0x04254B214C700443), //
    Cube::from_raw_unchecked(0x160A0C100F335589, 0x0A5AB7DE7588CE12), //
    Cube::from_raw_unchecked(0x100C0A16093573AF, 0x0B52F5CEF1A9C650), //
    Cube::from_raw_unchecked(0x0706050403824100, 0x05A928398A418820), //
    Cube::from_raw_unchecked(0x0405060700816223, 0x052D0931C8510C01), //
    Cube::from_raw_unchecked(0x0504010007668302, 0x008860A5ED55A12A), //
    Cube::from_raw_unchecked(0x000104050263A627, 0x010403BD2B642D49), //
    Cube::from_raw_unchecked(0x0B11120817AD0E54, 0x0DEB1908043214E6), //
    Cube::from_raw_unchecked(0x0812110B14AE2D77, 0x0D6F3800462290C7),
];

pub static MOVE_TABLE: &[Cube; 36] = &[
    Cube::from_raw_unchecked(0x0702050603804104, 0x058122398A41A828), // Ucw
    Cube::from_raw_unchecked(0x0700050203844106, 0x05A12A398A418022), // U2
    Cube::from_raw_unchecked(0x0704050003864102, 0x058920398A41A02A), // Uccw
    Cube::from_raw_unchecked(0x0506010407824300, 0x00A868398A458920), // Dcw
    Cube::from_raw_unchecked(0x0106030405824700, 0x04A968398A408860), // D2
    Cube::from_raw_unchecked(0x0306070401824500, 0x01A828398A448960), // Dccw
    Cube::from_raw_unchecked(0x07060C1003825509, 0x05DAA83E74418820), // Fcw
    Cube::from_raw_unchecked(0x0706000103824405, 0x05A548394C418820), // F2
    Cube::from_raw_unchecked(0x070609150382500C, 0x05D6C83EB2418820), // Fccw
    Cube::from_raw_unchecked(0x130F05040A964100, 0x0BA934C18BB18820), // Bcw
    Cube::from_raw_unchecked(0x0203050406874100, 0x04292B218A718820), // B2
    Cube::from_raw_unchecked(0x160A05040F934100, 0x0A2937D98B818820), // Bccw
    Cube::from_raw_unchecked(0x07060504118B4812, 0x05A928188A431C20), // Rcw
    Cube::from_raw_unchecked(0x0706050400814203, 0x05A92831CA410C20), // R2
    Cube::from_raw_unchecked(0x0706050412884B11, 0x05A92810CA439820), // Rccw
    Cube::from_raw_unchecked(0x0E14170D03824100, 0x05A9283982018885), // Lcw
    Cube::from_raw_unchecked(0x0405060703824100, 0x05A9283988518801), // L2
    Cube::from_raw_unchecked(0x0D17140E03824100, 0x05A92839801188A4), // Lccw
    Cube::from_raw_unchecked(0x0706050403628100, 0x05A928A5ED518820), // Ecw
    Cube::from_raw_unchecked(0x0706050403A26100, 0x05A928290E618820), // E2
    Cube::from_raw_unchecked(0x070605040342A100, 0x05A928B569718820), // Eccw
    Cube::from_raw_unchecked(0x0706050403224180, 0x05A928398A48CE12), // Scw
    Cube::from_raw_unchecked(0x0706050403A24120, 0x05A928398A400443), // S2
    Cube::from_raw_unchecked(0x07060504030241A0, 0x05A928398A494271), // Sccw
    Cube::from_raw_unchecked(0x0706050403822140, 0x0C677A398A418820), // Mcw
    Cube::from_raw_unchecked(0x0706050403826120, 0x052D09398A418820), // M2
    Cube::from_raw_unchecked(0x0706050403820160, 0x0CE35B398A418820), // Mccw
    Cube::from_raw_unchecked(0x030207060140A504, 0x018022B56974A968), // Ycw
    Cube::from_raw_unchecked(0x0100030205A46706, 0x04A16A290E608062), // Y2
    Cube::from_raw_unchecked(0x0504010007668302, 0x008860A5ED55A12A), // Yccw
    Cube::from_raw_unchecked(0x160A0C100F335589, 0x0A5AB7DE7588CE12), // Zcw
    Cube::from_raw_unchecked(0x0203000106A74425, 0x04254B214C700443), // Z2
    Cube::from_raw_unchecked(0x130F09150A1650AC, 0x0BD6D4C6B3B94271), // Zccw
    Cube::from_raw_unchecked(0x0D17140E118B0872, 0x0CE35B1880131CA4), // Xcw
    Cube::from_raw_unchecked(0x0405060700816223, 0x052D0931C8510C01), // X2
    Cube::from_raw_unchecked(0x0E14170D12882B51, 0x0C677A10C2039885), // Xccw
];

impl From<Move> for Cube {
    fn from(mv: Move) -> Cube {
        MOVE_TABLE[mv as usize]
    }
}

impl From<Move> for CenterMap {
    fn from(mv: Move) -> CenterMap {
        MOVE_TABLE[mv as usize].centers()
    }
}

impl From<Move> for EdgeMap {
    fn from(mv: Move) -> EdgeMap {
        MOVE_TABLE[mv as usize].edges()
    }
}

impl From<Move> for CornerMap {
    fn from(mv: Move) -> CornerMap {
        MOVE_TABLE[mv as usize].corners()
    }
}

#[derive(Copy, Clone, PartialEq, Eq,Debug)]
#[repr(u8)]
pub enum MoveAngle {
    Cw,
    Two,
    Ccw,
}
impl MoveAngle {
    pub fn radians(self) -> f32 {
        use std::f32::consts::PI;
        match self  {
            MoveAngle::Cw => -PI/2.0,
            MoveAngle::Two => -PI,
            MoveAngle::Ccw => PI/2.0
        }
    }

}


#[derive(Copy, Clone, PartialEq, Eq,Debug)]
#[repr(u8)]
pub enum MoveKind {
    Face,
    Slice,
    Rotation,
}

impl Move {
    // return the number of moves.
    pub fn len() -> u8 {
        36
    }
    pub fn new(kind: MoveKind, face: Face, angle: MoveAngle) -> Move {
        use Move::*;
        match kind {
            MoveKind::Face => FaceMove::new(face,angle).into(),
            MoveKind::Rotation => {
                // let rotation = 27 + ((face as u8)>>1)*3 + angle as u8;
                // let mv:Move = unsafe{ mem::transmute(rotation)};
                // if face.is_reverse()  {
                //     mv.inverse()
                // } else {
                //     mv
                // }
                let map:&[Move;18] = &[
                    Ycw, Y2, Yccw, Yccw, Y2, Ycw,//
                    Zcw, Z2, Zccw, Zccw, Z2, Zcw,//
                    Xcw, X2, Xccw, Xccw, X2, Xcw,//
                ];
                // Safety: Face <= 5, Angle <= 3 thus face+3*angle<=18.
                unsafe{
                    *map.get_unchecked(face as usize*3+angle as usize)
                }
            },
            MoveKind::Slice => {
                // let rotation = 18 + (((face as u8)>> 1)*3) + angle as u8;
                // let mv:Move = unsafe{ mem::transmute(rotation) };
                // let x = (face as u8) >>1;
                // if ((face as u8) ^ (0b1_01 >> x))&1 == 1 {
                //     mv.inverse()
                // } else {
                //     mv
                // }
                let map: &[Move;18] = &[
                    Eccw, E2, Ecw, Ecw, E2, Eccw,//
                    Scw, S2, Sccw, Sccw, S2, Scw,//
                    Mccw, M2, Mcw, Mcw, M2, Mccw //
                ];

                // Safety: Face <= 5, Angle <= 3 thus face+3*angle<=18.
                unsafe{
                    *map.get_unchecked(face as usize*3+angle as usize)
                }
            }
        }
    }
    pub fn face(self) -> Face {
        use Face::*;
        (&[Up,Down,Front,
           Back,Right,Left,
           Down,Front,Left,
           Up,Front,Right])[self as usize /3]
    }
    pub fn kind(self) -> MoveKind {
// MoveKind::Face
        (&[MoveKind::Face,MoveKind::Face,MoveKind::Slice,MoveKind::Rotation])[self as usize /9]
    }
    // An iterator over all the `Turn` variants.
    pub fn moves() -> impl Iterator<Item = Move> {
        unsafe { (0..36u8).map(|t| mem::transmute(t)) }
    }
    pub fn projection(self, centers: CenterMap) -> Move {
        Move::new(self.kind(), centers.get(self.face()), self.angle())
    }

    pub fn angle(self) -> MoveAngle {
        let v = self as u8;
        unsafe { mem::transmute(v % 3) }
    }

    #[inline]
    pub fn set_angle(self, angle:MoveAngle) -> Move {
        let v = self as u8;
        unsafe { mem::transmute((angle as u8) + v - (v % 3)) }
    }

    /// Counter clockwise Face of the `self` face.     
    #[inline]
    pub fn ccw(self) -> Move {
        let v = self as u8;
        unsafe { mem::transmute(2 + v - (v % 3)) }
    }

    pub fn pow(self) -> u8 {
        self as u8 % 3
    }

    /// Double turn of the `self` face.     
    #[inline]
    pub fn two(self) -> Move {
        let v = self as u8;
        unsafe { mem::transmute(1 + v - (v % 3)) }
    }

    /// Clockwise turn of the `self` face.     
    #[inline]
    pub fn cw(self) -> Move {
        let v = self as u8;
        unsafe { mem::transmute(v - (v % 3)) }
    }

    #[inline]
    pub fn centers(self) -> CenterMap {
        self.into()
    }

    #[inline]
    pub fn cube(self) -> Cube {
        self.into()
    }

    #[inline]
    pub fn corners(self) -> CornerMap {
        self.into()
    }

    #[inline]
    pub fn edges(self) -> EdgeMap {
        self.into()
    }

    #[inline]
    pub fn inverse(self) -> Move {
        let v = self as u8;
        unsafe { mem::transmute(v.wrapping_sub((v % 3).wrapping_sub(1) << 1)) }
        // (&[ Uccw, U2, Ucw, Dccw, D2, Dcw,Fccw, F2, Fcw,
        //     Bccw, B2, Bcw, Rccw, R2, Rcw, Lccw, L2, Lcw])[self as usize]
    }
}

/// A `FaceMove` respect a move of a outer face on the cube and is a subset of `Move`.
///
/// All face moves leave the centers of the cubes unchanged, this subset  
/// is provided to better model the capabilities of cubes without centers:
/// `FixedCenterCube`, `CornerMap` and `CenterMap`.
///
/// There is a marginal performance benefit to using `FaceMove` over `Move`.
///
/// `FaceMove`s are represented by an `u8` from 0 to 17 inclusive. `try_from` can
/// be cast an `u8` to a `FaceMove`. See example ... TODO
///
/// # Examples
///
/// ```
/// use speedcube::{ moves::FaceMove, FixedCentersCube };
/// let mut cube = FixedCentersCube::default() * FaceMove::Ucw;
/// assert_eq!(cube, FaceMove::Ucw.fc_cube());
///
/// cube *= FaceMove::Ucw.inverse(); // FaceTurn::Uccw
/// assert!(cube.is_solved());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FaceMove {
    Ucw,
    U2,
    Uccw,
    Dcw,
    D2,
    Dccw,
    Fcw,
    F2,
    Fccw,
    Bcw,
    B2,
    Bccw,
    Rcw,
    R2,
    Rccw,
    Lcw,
    L2,
    Lccw,
}

impl std::convert::TryFrom<u8> for FaceMove {
    type Error = &'static str;
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value >= 18 {
            return Err("Value too large (>17).");
        } else {
            // Safety: see above check, FaceMove is represented by u8 in 0..18.
            unsafe { Ok(mem::transmute(value)) }
        }
    }
}

impl From<FaceMove> for Move {
    fn from(fm: FaceMove) -> Move {
        // Safety: FaceMove (0..18) is a subset of Move (0..37).
        unsafe { mem::transmute(fm) }
    }
}

impl FaceMove {
    pub fn new(face: Face, angle: MoveAngle) -> FaceMove {
        unsafe { mem::transmute((face as u8 * 3) + (angle as u8)) }
    }

    // Returns the number of moves variants in this enum, FaceMove.
    pub const fn len() -> u8 {
        18
    }
    pub fn projection(self, centers: CenterMap) -> FaceMove {
        FaceMove::new(centers.get(self.face()), self.angle())
    }

    pub fn angle(self) -> MoveAngle {
        let v = self as u8;
        unsafe { mem::transmute(v % 3) }
    }
    // An iterator over all the `Turn` variants.
    pub fn moves() -> impl Iterator<Item = FaceMove> {
        unsafe { (0..18u8).map(|t| mem::transmute(t)) }
    }

    #[inline]
    pub fn inverse(self) -> FaceMove {
        use FaceMove::*;
        (&[
            Uccw, U2, Ucw, Dccw, D2, Dcw, Fccw, F2, Fcw, Bccw, B2, Bcw, Rccw, R2, Rcw, Lccw, L2,
            Lcw,
        ])[self as usize]
    }

    /// Counter clockwise Face of the `self` face.     
    #[inline]
    pub fn face(self) -> crate::Face {
        let v = self as u8;
        unsafe { mem::transmute(v / 3) }
    }
    /// Counter clockwise Face of the `self` face.     
    #[inline]
    pub fn ccw(self) -> FaceMove {
        let v = self as u8;
        unsafe { mem::transmute(2 + v - (v % 3)) }
    }

    /// Double turn of the `self` face.     
    #[inline]
    pub fn two(self) -> FaceMove {
        let v = self as u8;
        unsafe { mem::transmute(1 + v - (v % 3)) }
    }
    /// Clockwise turn of the `self` face.     
    #[inline]
    pub fn cw(self) -> FaceMove {
        let v = self as u8;
        unsafe { mem::transmute(v - (v % 3)) }
    }

    #[inline]
    pub fn cube(self) -> Cube {
        Move::from(self).into()
    }
    #[inline]
    pub fn fc_cube(self) -> FixedCentersCube {
        self.into()
    }
    #[inline]
    pub fn corners(self) -> CornerMap {
        self.into()
    }
    #[inline]
    pub fn edges(self) -> EdgeMap {
        self.into()
    }
}

macro_rules! impl_mul {
    ($move:tt for $map:tt) => {
        impl std::ops::Mul<$move> for $map {
            type Output = Self;
            #[inline]
            fn mul(self, mv: $move) -> Self {
                self * <$map>::from(mv)
            }
        }
    };
}

macro_rules! impl_mul_assign {
    ($move:tt for $map:tt) => {
        impl std::ops::MulAssign<$move> for $map {
            #[inline]
            fn mul_assign(&mut self, mv: $move) {
                *self *= <$map>::from(mv);
            }
        }
    };
}

impl_mul!(Move for EdgeMap);
impl_mul_assign!(Move for EdgeMap);
impl_mul!(FaceMove for EdgeMap);
impl_mul_assign!(FaceMove for EdgeMap);

impl_mul!(Move for CornerMap);
impl_mul_assign!(Move for CornerMap);
impl_mul!(FaceMove for CornerMap);
impl_mul_assign!(FaceMove for CornerMap);

impl_mul!(Move for CenterMap);
impl_mul_assign!(Move for CenterMap);

impl std::ops::Mul<FaceMove> for CenterMap {
    type Output = Self;
    #[inline]
    fn mul(self, _: FaceMove) -> Self {
        self
    }
}
impl std::ops::MulAssign<FaceMove> for CenterMap {
    #[inline]
    fn mul_assign(&mut self, _: FaceMove) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn center_index_rotation_table() {
        for (index, cube) in ROTATION_TABLE.iter().enumerate() {
            assert_eq!(index, cube.centers().index() as usize);
            assert_eq!(cube.corners().permutation_parity() ^ cube.edges().permutation_parity(),
                       cube.centers().permutation_parity());
        }
    }

    #[test]
    fn move_faceturn_cast() {
        use std::convert::TryFrom;
        let mut move_mask = (1u64 << FaceMove::len()) - 1;
        for i in 0..FaceMove::len() {
            move_mask ^= 1 << FaceMove::try_from(i as u8).unwrap() as u8
        }
        assert!(move_mask == 0);
        assert!(FaceMove::try_from(FaceMove::len()).is_err());

        assert_eq!(FaceMove::Rcw, FaceMove::R2.cw());
        assert_eq!(FaceMove::Rccw, FaceMove::R2.ccw());
        assert_eq!(FaceMove::B2, FaceMove::B2.two());
        assert_eq!(FaceMove::L2, FaceMove::Lcw.two());
        assert_eq!(FaceMove::Rcw, FaceMove::Rcw.cw());
        for mv in FaceMove::moves() {
            let (cw, two, ccw) = (mv.cw(), mv.two(), mv.ccw());
            if mv == cw {
                assert_eq!(mv, ccw.inverse(), "{:?} != {:?}.inverse()", mv, ccw);
                assert!((mv.fc_cube() * cw * two).is_solved());
            } else if mv == two {
                assert_eq!(mv, two.inverse(), "{:?} != {:?}.inverse()", mv, ccw);
                assert!((mv.fc_cube() * two).is_solved());
            } else if mv == ccw {
                assert_eq!(mv, cw.inverse(), "{:?} != {:?}.inverse()", mv, ccw);
                assert!((mv.fc_cube() * ccw * two).is_solved());
            } else {
                panic!(
                    "mv = Move::{:?} not equal to any of (mv.cw(), mv.two(), mv.ccw()) = {:?}",
                    mv,
                    (cw, two, ccw)
                );
            }
        }
    }
    #[test]
    fn move_turn_cast() {
        use std::convert::TryFrom;
        let mut move_mask = (1u64 << Move::len()) - 1;
        for i in 0..Move::len() {
            move_mask ^= 1 << Move::try_from(i as u8).unwrap() as u8
        }
        assert!(move_mask == 0);
        assert!(Move::try_from(Move::len()).is_err());
        assert_eq!(Move::Rcw, Move::R2.cw());
        assert_eq!(Move::Rccw, Move::R2.ccw());
        assert_eq!(Move::X2, Move::X2.two());
        assert_eq!(Move::L2, Move::Lcw.two());
        assert_eq!(Move::Rcw, Move::Rcw.cw());
        for mv in Move::moves() {
            let (cw, two, ccw) = (mv.cw(), mv.two(), mv.ccw());
            if mv == cw {
                assert_eq!(mv, ccw.inverse(), "{:?} != {:?}.inverse()", mv, ccw);
                assert!((mv.cube() * cw * two).is_solved());
            } else if mv == two {
                assert_eq!(mv, two.inverse(), "{:?} != {:?}.inverse()", mv, ccw);
                assert!((mv.cube() * two).is_solved());
            } else if mv == ccw {
                assert_eq!(mv, cw.inverse(), "{:?} != {:?}.inverse()", mv, ccw);
                assert!((mv.cube() * ccw * two).is_solved());
            } else {
                panic!(
                    "mv = Move::{:?} not equal to any of (mv.cw(), mv.two(), mv.ccw()) = {:?}",
                    mv,
                    (cw, two, ccw)
                );
            }
        }
    }

    #[test]
    fn move_projection() {
        for mv in Move::moves() {
            for (i, &rotation_cube) in crate::moves::ROTATION_TABLE.iter().enumerate() {
                let centers = rotation_cube.centers();
                let mv_mapped = mv.projection(centers);
                let with_rot = rotation_cube * mv_mapped * rotation_cube.inverse();
                assert_eq!(with_rot, mv.cube(), "[{:?}] {:?} -> {:?}", i, mv, mv_mapped);
            }
        }
    }
    #[test]
    fn facemove_projection() {
        for mv in FaceMove::moves() {
            for &rotation_cube in crate::moves::ROTATION_TABLE.iter() {
                let centers = rotation_cube.centers();
                let with_rot = rotation_cube * mv.projection(centers) * rotation_cube.inverse();
                assert_eq!(with_rot, mv.cube());
            }
        }
    }
}
