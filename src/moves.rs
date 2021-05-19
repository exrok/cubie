use crate::CenterMap;
use crate::{CornerMap, Cube, EdgeMap, Face, FixedCentersCube};

use std::mem;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Move {
    U1, U2, U3,
    D1, D2, D3,
    F1, F2, F3,
    B1, B2, B3,
    R1, R2, R3,
    L1, L2, L3,

    E1, E2, E3,
    S1, S2, S3,
    M1, M2, M3,

    Y1, Y2, Y3,
    Z1, Z2, Z3,
    X1, X2, X3,

    Uw1, Uw2, Uw3,
    Dw1, Dw2, Dw3,
    Fw1, Fw2, Fw3,
    Bw1, Bw2, Bw3,
    Rw1, Rw2, Rw3,
    Lw1, Lw2, Lw3,
}

impl std::convert::TryFrom<u8> for Move {
    type Error = &'static str;
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value >= 54 {
            return Err("Value too large (>54).");
        } else {
            unsafe { Ok(mem::transmute(value)) }
        }
    }
}

impl std::convert::TryFrom<FixedCentersCube> for FaceMove {
    type Error = ();
    fn try_from(cube: FixedCentersCube) -> Result<Self, ()> {
        use crate::FaceMove::*;
        let pftable = &[U1, U1, U1, F3, U1, D2, U1, R3,
                        U1, F1, L2, R1, U1, D1, L3, U1,
                        L1, B3, U1, U1, U2, D3, U1, U1,
                        B1, R2, U1, U3, B2, U1, F2, U1];
        let (a,b) = cube.raw();
        let index = (((a^b)*35) >> 38)&0x1f;
        let mv = pftable[index as usize];
        if mv.fc_cube() == cube {
            Ok(mv)
        } else {
            Err(())
        }
    }
}
impl std::convert::TryFrom<Cube> for Move {
    type Error = ();
    fn try_from(cube: Cube) -> Result<Self, ()> {
        use crate::Move::*;
        let phtable = &[Bw3, U3, Y1, Z1, Uw3, U1, Fw1, F1,
                        B3, B2, M1, X1, Z2, U1, X2, S1, L3,
                        Dw3, B1, Y3, U1, U1, U2, Rw2, U1, D1,
                        F3, Lw2, E1, F2, Lw1, U1, R3, U1, Dw2,
                        X3, M3, Bw1, R1, Bw2, Z3, D3, D2, M2,
                        S3, U1, Lw3, U1, E3, S2, U1, Dw1, Y2,
                        L2, Uw1, Fw3, L1, E2, Rw3, Rw1, R2,
                        U1, Fw2, Uw2];
        let (a,b) = cube.raw();
        let d = a^b;
        let hx1 = d * 18236133;
        let hx2 = (d * 4852774) & (0x20 << 40);
        let index = ((hx1 ^ hx2) >> 40) & 0x3f;
        let mv1 = phtable[index as usize];
        if mv1.cube() == cube {
            Ok(mv1)
        } else {
            Err(())
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

pub static MOVE_TABLE: &[Cube; 54] = &[
    Cube::from_raw_unchecked(0x0702050603804104, 0x058122398A41A828), // U1
    Cube::from_raw_unchecked(0x0700050203844106, 0x05A12A398A418022), // U2
    Cube::from_raw_unchecked(0x0704050003864102, 0x058920398A41A02A), // U3
    Cube::from_raw_unchecked(0x0506010407824300, 0x00A868398A458920), // D1
    Cube::from_raw_unchecked(0x0106030405824700, 0x04A968398A408860), // D2
    Cube::from_raw_unchecked(0x0306070401824500, 0x01A828398A448960), // D3
    Cube::from_raw_unchecked(0x07060C1003825509, 0x05DAA83E74418820), // F1
    Cube::from_raw_unchecked(0x0706000103824405, 0x05A548394C418820), // F2
    Cube::from_raw_unchecked(0x070609150382500C, 0x05D6C83EB2418820), // F3
    Cube::from_raw_unchecked(0x130F05040A964100, 0x0BA934C18BB18820), // B1
    Cube::from_raw_unchecked(0x0203050406874100, 0x04292B218A718820), // B2
    Cube::from_raw_unchecked(0x160A05040F934100, 0x0A2937D98B818820), // B3
    Cube::from_raw_unchecked(0x07060504118B4812, 0x05A928188A431C20), // R1
    Cube::from_raw_unchecked(0x0706050400814203, 0x05A92831CA410C20), // R2
    Cube::from_raw_unchecked(0x0706050412884B11, 0x05A92810CA439820), // R3
    Cube::from_raw_unchecked(0x0E14170D03824100, 0x05A9283982018885), // L1
    Cube::from_raw_unchecked(0x0405060703824100, 0x05A9283988518801), // L2
    Cube::from_raw_unchecked(0x0D17140E03824100, 0x05A92839801188A4), // L3
    Cube::from_raw_unchecked(0x0706050403628100, 0x05A928A5ED518820), // E1
    Cube::from_raw_unchecked(0x0706050403A26100, 0x05A928290E618820), // E2
    Cube::from_raw_unchecked(0x070605040342A100, 0x05A928B569718820), // E3
    Cube::from_raw_unchecked(0x0706050403224180, 0x05A928398A48CE12), // S1
    Cube::from_raw_unchecked(0x0706050403A24120, 0x05A928398A400443), // S2
    Cube::from_raw_unchecked(0x07060504030241A0, 0x05A928398A494271), // S3
    Cube::from_raw_unchecked(0x0706050403822140, 0x0C677A398A418820), // M1
    Cube::from_raw_unchecked(0x0706050403826120, 0x052D09398A418820), // M2
    Cube::from_raw_unchecked(0x0706050403820160, 0x0CE35B398A418820), // M3
    Cube::from_raw_unchecked(0x030207060140A504, 0x018022B56974A968), // Y1
    Cube::from_raw_unchecked(0x0100030205A46706, 0x04A16A290E608062), // Y2
    Cube::from_raw_unchecked(0x0504010007668302, 0x008860A5ED55A12A), // Y3
    Cube::from_raw_unchecked(0x160A0C100F335589, 0x0A5AB7DE7588CE12), // Z1
    Cube::from_raw_unchecked(0x0203000106A74425, 0x04254B214C700443), // Z2
    Cube::from_raw_unchecked(0x130F09150A1650AC, 0x0BD6D4C6B3B94271), // Z3
    Cube::from_raw_unchecked(0x0D17140E118B0872, 0x0CE35B1880131CA4), // X1
    Cube::from_raw_unchecked(0x0405060700816223, 0x052D0931C8510C01), // X2
    Cube::from_raw_unchecked(0x0E14170D12882B51, 0x0C677A10C2039885), // X3
    Cube::from_raw_unchecked(0x070205060340A104, 0x058122B56971A828), //Uw1
    Cube::from_raw_unchecked(0x0700050203A46106, 0x05A12A290E618022), //Uw2
    Cube::from_raw_unchecked(0x0704050003668102, 0x058920A5ED51A02A), //Uw3
    Cube::from_raw_unchecked(0x0506010407628300, 0x00A868A5ED558920), //Dw1
    Cube::from_raw_unchecked(0x0106030405A26700, 0x04A968290E608860), //Dw2
    Cube::from_raw_unchecked(0x030607040142A500, 0x01A828B569748960), //Dw3
    Cube::from_raw_unchecked(0x07060C1003225589, 0x05DAA83E7448CE12), //Fw1
    Cube::from_raw_unchecked(0x0706000103A24425, 0x05A548394C400443), //Fw2
    Cube::from_raw_unchecked(0x07060915030250AC, 0x05D6C83EB2494271), //Fw3
    Cube::from_raw_unchecked(0x130F05040A1641A0, 0x0BA934C18BB94271), //Bw1
    Cube::from_raw_unchecked(0x0203050406A74120, 0x04292B218A700443), //Bw2
    Cube::from_raw_unchecked(0x160A05040F334180, 0x0A2937D98B88CE12), //Bw3
    Cube::from_raw_unchecked(0x07060504118B0872, 0x0CE35B188A431C20), //Rw1
    Cube::from_raw_unchecked(0x0706050400816223, 0x052D0931CA410C20), //Rw2
    Cube::from_raw_unchecked(0x0706050412882B51, 0x0C677A10CA439820), //Rw3
    Cube::from_raw_unchecked(0x0E14170D03822140, 0x0C677A3982018885), //Lw1
    Cube::from_raw_unchecked(0x0405060703826120, 0x052D093988518801), //Lw2
    Cube::from_raw_unchecked(0x0D17140E03820160, 0x0CE35B39801188A4), //Lw3
];
use std::str::FromStr;
#[derive(Debug)]
pub enum MoveParseError {
    MissingTurnCharacter,
    UnexpectedWide,
    UnknownSymbol
}
impl FromStr for Move {
    type Err = MoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.chars();
        let mut base = match iter.next().ok_or(MoveParseError::MissingTurnCharacter)? {
            'U' => Move::U1,
            'D' => Move::D1,
            'F' => Move::F1,
            'B' => Move::B1,
            'R' => Move::R1,
            'L' => Move::L1,
            'E' => Move::E1,
            'M' => Move::M1,
            'S' => Move::S1,
            'X' => Move::X1,
            'Y' => Move::Y1,
            'Z' => Move::Z1,
            'u' => Move::Uw1,
            'd' => Move::Dw1,
            'f' => Move::Fw1,
            'b' => Move::Bw1,
            'r' => Move::Rw1,
            'l' => Move::Lw1,
            _ => return Err(MoveParseError::UnknownSymbol) 
        };
        let mut rem = iter.as_str().as_bytes();
        if let Some(b'w') = rem.first() {
            if base.kind() == MoveKind::Face {
                base = Move::new(MoveKind::Wide, base.face(), MoveAngle::Cw)
            }
            rem = &rem[1..];
        }
        match rem {
            b"" =>  Ok(base),
            b"'" => Ok(base.ccw()),
            b"2" => Ok(base.two()),
            b"w" => Ok(base.two()),
            _ =>  Err(MoveParseError::UnknownSymbol)
        }
    }
}
impl<T: AsRef<[Move]>> From<T> for Cube {
    #[inline]
    fn from(moves:T) -> Cube {
        let mut cube = Cube::default();
        for mv in moves.as_ref() {
            cube *= mv.cube();
        }
        cube
    }
}
impl From<Move> for Cube {
    #[inline]
    fn from(mv: Move) -> Cube {
        MOVE_TABLE[mv as usize]
    }
}

impl From<Move> for CenterMap {
    #[inline]
    fn from(mv: Move) -> CenterMap {
        MOVE_TABLE[mv as usize].centers()
    }
}

impl From<Move> for EdgeMap {
    #[inline]
    fn from(mv: Move) -> EdgeMap {
        MOVE_TABLE[mv as usize].edges()
    }
}

impl From<Move> for CornerMap {
    #[inline]
    fn from(mv: Move) -> CornerMap {
        MOVE_TABLE[mv as usize].corners()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum MoveAngle {
    Cw,
    Two,
    Ccw,
}
impl MoveAngle {
    pub fn radians(self) -> f32 {
        use std::f32::consts::PI;
        match self {
            MoveAngle::Cw => -PI / 2.0,
            MoveAngle::Two => -PI,
            MoveAngle::Ccw => PI / 2.0,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum MoveKind {
    Face,
    Slice,
    Rotation,
    Wide,
}

impl Move {
    // return the number of moves.
    pub const COUNT: u8 = 54;
    pub fn new(kind: MoveKind, face: Face, angle: MoveAngle) -> Move {
        use Move::*;
        match kind {
            MoveKind::Face => FaceMove::new(face, angle).into(),
            MoveKind::Wide => 
                unsafe { mem::transmute(face as u8 * 3 + angle as u8 +36)},
            MoveKind::Rotation => {
                // let rotation = 27 + ((face as u8)>>1)*3 + angle as u8;
                // let mv:Move = unsafe{ mem::transmute(rotation)};
                // if face.is_reverse()  {
                //     mv.inverse()
                // } else {
                //     mv
                // }
                let map: &[Move; 18] = &[
                    Y1, Y2, Y3, Y3, Y2, Y1, //
                    Z1, Z2, Z3, Z3, Z2, Z1, //
                    X1, X2, X3, X3, X2, X1, //
                ];
                // Safety: Face <= 5, Angle <= 3 thus face+3*angle<=18.
                unsafe { *map.get_unchecked(face as usize * 3 + angle as usize) }
            }
            MoveKind::Slice => {
                // let rotation = 18 + (((face as u8)>> 1)*3) + angle as u8;
                // let mv:Move = unsafe{ mem::transmute(rotation) };
                // let x = (face as u8) >>1;
                // if ((face as u8) ^ (0b1_01 >> x))&1 == 1 {
                //     mv.inverse()
                // } else {
                //     mv
                // }
                let map: &[Move; 18] = &[
                    E3, E2, E1, E1, E2, E3, //
                    S1, S2, S3, S3, S2, S1, //
                    M3, M2, M1, M1, M2, M3, //
                ];

                // Safety: Face <= 5, Angle <= 3 thus face+3*angle<=18.
                unsafe { *map.get_unchecked(face as usize * 3 + angle as usize) }
            }
        }
    }
    pub fn face(self) -> Face {
        use Face::*;
        (&[
            Up, Down, Front, Back, Right, Left,
            Down, Front, Left, Up, Front, Right,
            Up, Down, Front, Back, Right, Left,
        ])[self as usize / 3]
    }
    pub fn kind(self) -> MoveKind {
        // MoveKind::Face
        (&[
            MoveKind::Face,
            MoveKind::Face,
            MoveKind::Slice,
            MoveKind::Rotation,
            MoveKind::Wide,
            MoveKind::Wide,
        ])[self as usize / 9]
    }
    // An iterator over all the `Turn` variants.
    pub fn moves() -> impl Iterator<Item = Move> {
        unsafe { (0..54u8).map(|t| mem::transmute(t)) }
    }
    pub fn projection(self, centers: CenterMap) -> Move {
        Move::new(self.kind(), centers.get(self.face()), self.angle())
    }

    pub fn angle(self) -> MoveAngle {
        let v = self as u8;
        unsafe { mem::transmute(v % 3) }
    }

    #[inline]
    pub fn set_angle(self, angle: MoveAngle) -> Move {
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
        // (&[ U3, U2, U1, D3, D2, D1,F3, F2, F1,
        //     B3, B2, B1, R3, R2, R1, L3, L2, L1])[self as usize]
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
/// use cubie::{ moves::FaceMove, FixedCentersCube };
/// let mut cube = FixedCentersCube::default() * FaceMove::U1;
/// assert_eq!(cube, FaceMove::U1.fc_cube());
///
/// cube *= FaceMove::U1.inverse(); // FaceTurn::U3
/// assert!(cube.is_solved());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FaceMove {
    U1,
    U2,
    U3,
    D1,
    D2,
    D3,
    F1,
    F2,
    F3,
    B1,
    B2,
    B3,
    R1,
    R2,
    R3,
    L1,
    L2,
    L3,
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

    pub const COUNT: u8 = 18;
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
            U3, U2, U1, D3, D2, D1, F3, F2, F1, B3, B2, B1, R3, R2, R1, L3, L2,
            L1,
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
            assert_eq!(
                cube.corners().permutation_parity() ^ cube.edges().permutation_parity(),
                cube.centers().permutation_parity()
            );
        }
    }

    #[test]
    fn move_faceturn_cast() {
        use std::convert::TryFrom;
        let mut move_mask = (1u64 << FaceMove::COUNT) - 1;
        for i in 0..FaceMove::COUNT {
            move_mask ^= 1 << FaceMove::try_from(i as u8).unwrap() as u8
        }
        assert!(move_mask == 0);
        assert!(FaceMove::try_from(FaceMove::COUNT).is_err());

        assert_eq!(FaceMove::R1, FaceMove::R2.cw());
        assert_eq!(FaceMove::R3, FaceMove::R2.ccw());
        assert_eq!(FaceMove::B2, FaceMove::B2.two());
        assert_eq!(FaceMove::L2, FaceMove::L1.two());
        assert_eq!(FaceMove::R1, FaceMove::R1.cw());
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
        let mut move_mask = (1u64 << Move::COUNT) - 1;
        for i in 0..Move::COUNT {
            move_mask ^= 1 << Move::try_from(i as u8).unwrap() as u8
        }
        assert!(move_mask == 0);
        assert!(Move::try_from(Move::COUNT).is_err());
        assert_eq!(Move::R1, Move::R2.cw());
        assert_eq!(Move::R3, Move::R2.ccw());
        assert_eq!(Move::X2, Move::X2.two());
        assert_eq!(Move::L2, Move::L1.two());
        assert_eq!(Move::R1, Move::R1.cw());
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
