use crate::{CornerMap, EdgeMap, FaceMove, TileMap,MapError};

use std::ops::{Mul, MulAssign};
/// 3x3 Puzzle Cube, with centers fixed in space.
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FixedCentersCube {
    pub(crate) corners: CornerMap, //We use setters/getters to be consistent with cube
    pub(crate) edges: EdgeMap,  
}
/// # Components
impl FixedCentersCube {
    #[inline]
    pub fn new(corners: CornerMap, edges: EdgeMap) -> FixedCentersCube {
        FixedCentersCube{
            corners,
            edges
        }
    }
    #[inline]
    pub fn corners(&self) -> CornerMap {
        self.corners
    }
    #[inline]
    pub fn set_corners(&mut self, corners: CornerMap) {
        self.corners = corners;
    }
    #[inline]
    pub fn edges(&self) -> EdgeMap {
        self.edges
    }
    #[inline]
    pub fn set_edges(&mut self, edges: EdgeMap)  {
        self.edges = edges;
    }

}
impl FixedCentersCube {
    pub fn has_solution(self) -> bool  {
        self.edges.permutation_parity() == self.corners.permutation_parity() &&
            self.edges.orientation_residue().is_identity()&&
            self.corners.orientation_residue().is_identity()
    }
    pub fn validate(self) -> Result<(),MapError>  {
        self.corners.validate().and_then(|_| self.edges.validate())
    }
    #[inline]
    pub fn inverse(self) -> FixedCentersCube {
        FixedCentersCube {
            corners: self.corners.inverse(),
            edges: self.edges.inverse(),
        }
    }
    #[inline]
    pub fn is_solved(self) -> bool {
        self.corners.is_solved() && self.edges.is_solved()
    }
}
impl From<FaceMove> for FixedCentersCube {
    fn from(turn: FaceMove) -> FixedCentersCube {
        FixedCentersCube {
            corners: turn.into(),
            edges: turn.into(),
        }
    }
}

impl Mul<FaceMove> for FixedCentersCube {
    type Output = Self;
    #[inline]
    fn mul(self, turn: FaceMove) -> Self {
        FixedCentersCube {
            corners: self.corners * turn,
            edges: self.edges * turn,
        }
    }
}

impl MulAssign<FaceMove> for FixedCentersCube {
    #[inline]
    fn mul_assign(&mut self, turn: FaceMove) {
        self.corners *= turn;
        self.edges *= turn;
    }
}
impl Mul for FixedCentersCube {
    // The multiplication of rational numbers is a closed operation.

    type Output = Self;
    fn mul(self, cube: Self) -> Self {
        FixedCentersCube {
            corners: self.corners * cube.corners,
            edges: self.edges * cube.edges,
        }
    }
}
impl MulAssign for FixedCentersCube {
    fn mul_assign(&mut self, cube: Self) {
        self.corners *= cube.corners;
        self.edges *= cube.edges;
    }
}
