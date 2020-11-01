//#![doc(html_logo_url = "path_to_logo", html_favicon_url = "path_to_favicon")]
pub mod cube;
pub mod group;
pub mod moves;
pub mod tile;

#[doc(inline)]
pub use moves::FaceMove;
#[doc(inline)]
pub use moves::Move;

#[doc(inline)]
pub use cube::corner::Corner;

#[doc(inline)]
pub use cube::edge::Edge;

#[doc(inline)]
pub use cube::corner::CornerMap;
#[doc(inline)]
pub use cube::edge::EdgeMap;

#[doc(inline)]
pub use cube::center::CenterMap;
#[doc(inline)]
pub use cube::Cube;
#[doc(inline)]
pub use cube::FixedCentersCube;
use std::mem;
#[doc(inline)]
pub use tile::TileMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Face {
    Up = 0,
    Down = 1,
    Front = 2,
    Back = 3,
    Right = 4,
    Left = 5,
}

impl std::convert::TryFrom<u8> for Face {
    type Error = &'static str;
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 5 {
            return Err("Value too large (>5).");
        } else {
            unsafe { Ok(mem::transmute(value)) }
        }
    }
}
impl Face {
    /// An iterator of the 6 faces.
    pub fn faces() -> impl Iterator<Item = Face> {
        unsafe { (0..6u8).map(|t| mem::transmute(t)) }
    }

    /// The face on the opposite of the cube
    pub fn opposite(self) -> Face {
        unsafe { mem::transmute(1 ^ self as u8) }
    }

    /// Counter clockwise Face of the `self` face.     
    #[inline]
    pub fn ccw(self) -> FaceMove {
        unsafe { mem::transmute(2 + self as u8 * 3) }
    }

    /// Double turn of the `self` face.     
    #[inline]
    pub fn two(self) -> FaceMove {
        unsafe { mem::transmute(1 + self as u8 * 3) }
    }
    /// Clockwise turn of the `self` face.     
    #[inline]
    pub fn cw(self) -> FaceMove {
        unsafe { mem::transmute(self as u8 * 3) }
    }
}
