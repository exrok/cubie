#![doc(
    html_logo_url = "data:image/svg+xml,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 width=%22512%22 height=%22512%22%3E%3Cpath d=%22M251 0c-15 5-28 15-43 20-14 9-26 20-42 22-19 10-39 18-55 32-13 4-27 8-38 16-15 7-31 13-42 26-8 14-1 31-3 46 0 15-1 31 4 46-4 20-1 42-2 63 0 13 5 25 2 38-1 20 2 39 1 59-1 11 5 22 15 27 12 8 26 14 38 23 15 5 28 13 41 23 18 10 35 23 55 29 17 16 39 24 59 37 11 6 26 8 37-1 14-9 30-15 43-25 10-9 21-14 33-19 16-9 32-17 48-28 10-10 26-12 38-22 13-9 30-14 41-27 7-13 4-29 6-43 0-20-5-40 0-59 3-19 0-38 2-57-3-14-1-27 1-40 1-22 2-43 0-65-5-15-23-19-35-27-14-7-27-15-42-18-11-6-20-14-31-19l-37-17c-14 0-22-15-35-19-17-7-33-20-52-21h-7z%22 fill=%22%23181818%22/%3E%3Cpath d=%22M258 5c-20 3-37 17-55 25-7 16 18 16 26 24 13 7 29 16 43 8l43-22c8-15-19-16-27-23-9-5-19-12-30-12zm76 38c-17 2-30 13-45 20-9 2-20 14-5 19 17 6 32 18 50 23 15 1 27-12 40-18 12-1 30-16 11-22-17-7-31-18-49-22zm-151 0c-21 2-37 18-56 25-9 16 19 18 28 25 13 8 29 17 43 7 13-8 28-13 41-21 10-11-11-15-18-20-12-6-24-14-38-16zm228 39c-14 2-26 11-38 17-8 4-25 8-18 19l37 19c11 6 24 12 36 4l41-22c10-14-15-17-23-23-11-5-22-13-35-14zm-304 0c-19 2-34 17-52 23-15 6-3 16 6 20 15 8 29 19 46 20 18-4 32-16 49-23 16-4 6-18-5-20-14-7-26-16-41-20h-1zm153 80c-18 3-32 15-49 22-13 2-18 18-2 21 16 8 30 18 47 23 17 1 30-12 45-19 10-1 29-15 12-22l-43-22c-4-2-7-3-11-3z%22 fill=%22%23fff%22/%3E%3Cpath d=%22M81 91z%22 fill=%22%23a8a8a8%22/%3E%3Cpath d=%22M39 128c-13 9-2 30-5 44-1 14-2 32 11 40 14 8 29 14 42 23 17 3 10-21 11-30 0-15 0-29-3-43-8-16-26-18-39-27-6-2-11-7-17-7zm151 82c-10 7-3 24-5 35 1 14-3 30 4 43 10 12 27 15 39 25 10 9 29 9 24-8l1-45c1-15-10-25-23-30-13-7-25-15-39-20zM42 220c-12 5-6 23-6 34 0 13-2 27 3 39 10 15 29 17 42 28 11 10 24 2 19-12l-1-50c-3-16-21-20-34-28-7-4-15-9-23-11zm-1 90c-7 11-1 24-3 36 2 13-3 28 6 39 10 11 26 14 38 24 9 9 26 4 20-10-2-17 1-34-2-50-4-16-20-21-33-28-8-4-16-11-26-11zm75 41c-10 9-4 27-4 40 1 13-4 32 9 40 14 10 31 16 45 26 13 4 10-13 10-21 0-16 3-32-1-47-4-13-19-16-29-23-10-5-19-12-30-15zm75 43c-9 6-4 21-5 30 3 15-4 32 4 45 14 13 32 19 47 30 12 9 18-4 15-14 0-16 2-32 0-48-1-13-14-19-24-24-12-6-23-15-36-19z%22 fill=%22%23b03535%22/%3E%3Cpath d=%22M478 129c-16 6-31 17-47 24-12 7-11 23-10 35 0 14-2 28-1 42 12 13 27-9 40-12 13-5 27-16 24-32l2-51c-1-3-4-6-8-6zm-152 82c-17 7-33 17-50 26-15 8-7 28-9 42 2 13-8 35 7 41 12-3 22-13 34-18 10-5 26-11 25-26-1-20 3-41 0-61-2-3-4-4-7-4zm152 10c-16 3-28 15-42 21-15 4-18 18-17 31 0 13 1 27-1 40-4 14 14 16 21 7 12-9 27-15 39-25 9-13 2-29 4-44-1-9 6-25-4-30zm-3 90c-16 4-28 15-43 22-14 4-16 19-14 32l-1 44c8 14 22-5 32-8 12-8 31-14 31-31 0-16-1-32 2-48 0-4-2-10-7-11zm-73 41c-17 5-30 17-46 24-13 6-15 21-14 35 1 13 2 26 0 39 5 17 24-2 33-6 11-8 30-13 32-30v-52c-1-4 0-10-5-10zm-77 43c-14 5-26 15-40 22-14 4-22 17-19 31 1 16-1 33 1 49 9 10 21-3 29-8 13-8 32-13 36-30v-59c-1-3-4-5-7-5z%22 fill=%22%2365b4f2%22/%3E%3C/svg%3E"
)]
//! # Cubie: Rubiks Cube Library
//! A Fast 3x3x3 Rubiks cube library for rust.
//! # Features
//! - Extensive Move Set supporting 54 moves including: outer face, wide, slice and rotation moves. 
//! - Compact 16 Byte Representation of the entire cube state.
//! - Ergonomic Immutable & Copy based API.
//! - Performance 
//! - 2 Cube Renders: Svg and Ansi terminal.
//! - Fast solver via the cubie-solver crate
//! - Group Theoretic Representation and Methods:
//!   - Cube state is represented permutation maps.
//!   - Multiply Cube States group operation.
//!   - Inverse method.
//! - Identity element is the Default solved cube.
//! - Compact Component Indices
//! - Abstract Manipulation of Moves 
//!   - Inverse
//!   - Projection
//!   - Destructing to Axis, Rotation and Kind
//!   - Construction from Axis, Rotation and Kind
//! - Additional Intermediate Tile Based Reposition
//#![doc(html_logo_url = "path_to_logo", html_favicon_url = "path_to_favicon")]
pub mod cube;
mod cubedisplay;
pub mod moves;
pub mod tile;

#[doc(inline)]
pub use cube::corner::{Corner,CornerOrientation,COIndex,CPIndex};
#[doc(inline)]
pub use cube::edge::{Edge,EdgeOrientation,EOIndex,EPIndex};
#[doc(inline)]
pub use cubedisplay::CubeDisplay;
#[doc(inline)]
pub use moves::FaceMove;
#[doc(inline)]
pub use moves::Move;


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

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum MapError {
    Orientation,
    Duplicate,
    OutOfBounds,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum PieceKind {
    Edge,
    Corner,
    Center,
}
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
    pub fn is_reverse(self) -> bool {
        self as u8 & 1 == 1
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
