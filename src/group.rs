use crate::{ Move, Cube, CornerMap, EdgeMap, CenterMap, FixedCentersCube, FaceMove };
use std::ops::{ Mul,MulAssign };

pub trait CubeGroupExt {
    fn inverse(self) -> Self;
    fn identity() -> Self;
}

impl CubeGroupExt for Cube {
    #[inline] fn inverse(self) -> Self { Cube::inverse(self) }
    #[inline] fn identity() -> Self { Self::default() }
}

impl CubeGroupExt for CornerMap {
    #[inline] fn inverse(self) -> Self { CornerMap::inverse(self) }
    #[inline] fn identity() -> Self { Self::default() }
}

impl CubeGroupExt for EdgeMap {
    #[inline] fn inverse(self) -> Self { EdgeMap::inverse(self) }
    #[inline] fn identity() -> Self { Self::default() }
}

impl CubeGroupExt for CenterMap {
    #[inline] fn inverse(self) -> Self { CenterMap::inverse(self) }
    #[inline] fn identity() -> Self { Self::default() }
}

impl CubeGroupExt for FixedCentersCube {
    #[inline] fn inverse(self) -> Self { FixedCentersCube::inverse(self) }
    #[inline] fn identity() -> Self { Self::default() }
}

pub trait CubeGroup: Eq + Sized + CubeGroupExt +
    Mul<Self,Output=Self> + MulAssign<Self> +
    Mul<Move,Output=Self> + MulAssign<Move> +
    Mul<FaceMove,Output=Self> + MulAssign<FaceMove> {}

impl<T: Eq + Sized + CubeGroupExt +
    Mul<Self,Output=Self> + MulAssign<Self> +
    Mul<Move,Output=Self> + MulAssign<Move> +
    Mul<FaceMove,Output=Self> + MulAssign<FaceMove>> CubeGroup for T {}
