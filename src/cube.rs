pub mod center;
pub mod edge;
pub mod corner;

use crate::{CornerMap,EdgeMap,FaceMove,TileMap};
use crate::cube::center::CenteredCornerMap;
use crate::cube::center::CenterMap;
use crate::Move;
/// 3x3 Puzzle Cube
#[derive(Default,Clone,Copy,PartialEq,Eq,Hash)]
pub struct Cube {
    pub centered_corners: CenteredCornerMap,
    pub edges: EdgeMap,
}

impl Cube {
    pub fn inverse_multiply(self, rhs: Cube) -> Cube {
        Cube {
            centered_corners: CenteredCornerMap{raw:
                self.corners().inverse_multiply(rhs.corners()).set |
                self.centers().inverse_multiply(rhs.centers()).map 
            },
            edges: self.edges.inverse_multiply(rhs.edges)
        }
    }
    pub fn inverse(self) -> Cube {
        Cube {
            centered_corners: CenteredCornerMap{raw:
                self.corners().inverse().set |
                self.centers().inverse().map 
            },
            edges: self.edges.inverse()
        }
    }
    pub fn tilemap(self) -> TileMap {
        self.into()
    }
    pub fn corners(&self) -> CornerMap {
        self.centered_corners.corners()
    }
    pub fn edges(&self) -> EdgeMap {
        self.edges
    }
    pub fn centers(&self) -> CenterMap {
        self.centered_corners.centers()
    }
    pub fn is_solved(self) -> bool {
        self == crate::moves::ROTATION_TABLE[self.centers().index() as usize] 
    }
}
impl std::fmt::Debug for Cube {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       f.debug_struct("Cube")
           .field("centers", &self.centers())
           .field("corners", &self.corners())
           .field("edges", &self.edges)
           .finish()
    }
}
impl From<FixedCentersCube> for Cube {
    fn from(cube: FixedCentersCube) ->Cube {
        Cube{
            centered_corners: cube.corners.into(),
            edges: cube.edges
        }
    }
}
impl Mul for Cube {
    // The multiplication of rational numbers is a closed operation.

    type Output = Self;
    fn mul(self, cube: Self) -> Self {
        Cube{
            centered_corners: self.centered_corners*cube.centered_corners,
            edges: self.edges*cube.edges,
        }
    }
}

impl MulAssign for Cube {
    // The multiplication of rational numbers is a closed operation.

    fn mul_assign(&mut self, cube: Self)  {
        self.centered_corners = self.centered_corners*cube.centered_corners;
        self.edges*=cube.edges;
        // Cube{
        //     centered_corners: self.centered_corners*cube.centered_corners,
        //     edges: self.edges*cube.edges,
        // }
    }
}

impl MulAssign<Move> for Cube {
    // The multiplication of rational numbers is a closed operation.

    fn mul_assign(&mut self, mv: Move) {
        *self *= Cube::from(mv);
    }
}
impl Mul<Move> for Cube {
    // The multiplication of rational numbers is a closed operation.

    type Output = Self;
    fn mul(self, mv: Move) -> Self {
        self * Cube::from(mv)
    }
}


/// 3x3 Puzzle Cube, with centers fixed in space.
#[derive(Default,Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub struct FixedCentersCube {
    pub corners: CornerMap,
    pub edges: EdgeMap,
}
impl FixedCentersCube {
    pub fn is_solved(&self) ->bool {
        self.corners.is_solved() && self.edges.is_solved() 
    }
    pub fn tilemap(self) -> TileMap {
        self.into()
    }
}
impl From<FaceMove> for FixedCentersCube {
    fn from(turn: FaceMove) ->FixedCentersCube {
        FixedCentersCube {
            corners: turn.into(),
            edges: turn.into(),
        }
    }
}

use std::ops::{ Mul,MulAssign };

impl Mul<FaceMove> for FixedCentersCube {
    type Output = Self;
    #[inline]
    fn mul(self, turn: FaceMove) -> Self {
        FixedCentersCube{
            corners: self.corners*turn,
            edges: self.edges*turn,
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
        FixedCentersCube{
            corners: self.corners*cube.corners,
            edges: self.edges*cube.edges,
        }
    }
}
impl MulAssign for FixedCentersCube {
    fn mul_assign(&mut self, cube: Self){
        self.corners *= cube.corners;
        self.edges *= cube.edges;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn bootstrap_from_generating_set() {
        struct CubeGroup {
            cw: Cube,
            two: Cube,
            ccw: Cube,
        }
        fn extend_cw(mv: Move, cw: Cube) -> CubeGroup  {
            assert_eq!(cw, mv.cw().cube(), "Failed Bootstrap: Move {:?}", mv.cw());
            let two = cw*cw;
            assert_eq!(two, mv.two().cube(), "Failed Bootstrap: Move {:?}", mv.two());
            let ccw = two*cw;
            assert_eq!(ccw, mv.ccw().cube(), "Failed Bootstrap: Move {:?}", mv.ccw());
            CubeGroup{cw, two, ccw}
        }
        let u_cw = Move::Ucw.cube(); //generating set
        let y_cw = Move::Ycw.cube();
        let x_cw = Move::Xcw.cube();

        let u = extend_cw(Move::Ucw, u_cw);
        let y = extend_cw(Move::Ycw, y_cw);
        let x = extend_cw(Move::Xcw, x_cw);

        let z = extend_cw(Move::Zcw, y.cw * x.ccw * y.ccw);
        let r = extend_cw(Move::Rcw, z.ccw * u.cw * z.cw);

        let l = extend_cw(Move::Lcw, y.two * r.cw * y.two);
        let b = extend_cw(Move::Bcw, y.cw * r.cw * y.ccw);
        let f = extend_cw(Move::Fcw, y.ccw * r.cw * y.cw);

        // and for fun and testing...
        let d_ccw = r.two * l.two * u.cw * f.two * b.two * u.cw * f.two * r.two * f.two * b.two
            * u.two * l.two * u.two * l.two * r.two * u.two * r.two * u.two * r.two * f.two * u.ccw * 
            r.two * b.two * r.two * l.two * f.two * l.two * u.cw * b.two * f.two * u.cw;
        let d = extend_cw(Move::Dcw, d_ccw * d_ccw * d_ccw);

        let e = extend_cw(Move::Ecw, y.ccw * u.cw * d.ccw);
        let s = extend_cw(Move::Scw, z.cw * f.ccw * b.cw);
        let m = extend_cw(Move::Mcw, x.ccw * r.cw * l.ccw);

        // ensure equality is actually working.
        assert_ne!(e.cw, u.ccw);
        assert_ne!(s.cw, s.ccw);
        assert_ne!(m.cw, s.cw);
        assert_ne!(l.cw, l.ccw);
        assert_ne!(x.cw, y.ccw);
        assert_ne!(x.cw, x.ccw);
        assert_ne!(x.two, z.two);
    }

    #[test]
    fn cube_is_direct_product() {
        use std::convert::TryFrom;
        let mut rng1 = oorandom::Rand32::new(0xdeadbeef);
        let mut random_move = || -> Move {
            Move::try_from((rng1.rand_u32()%36) as u8).unwrap()
        };
        let mut cube = Cube::default();
        let mut corners = CornerMap::default();
        let mut edges = EdgeMap::default();
        let mut centers = CenterMap::default();
        // TODO check inverser_multiply and congute
        for _ in 0..400 {
            let mv = random_move();
            cube *= mv;
            corners *= mv;
            edges *= mv;
            centers *= mv;
            assert_eq!(cube.centers(),centers);
            assert_eq!(cube.edges(), edges);
            assert_eq!(cube.corners(),corners);
        }
    }

    #[test]
    fn cube_inverse_multiply() {
        use std::convert::TryFrom;
        let mut rng1 = oorandom::Rand32::new(0xdeadbeef);
        let mut random_move = || -> Move {
            Move::try_from((rng1.rand_u32()%36) as u8).unwrap()
        };
        let mut x = Cube::default();
        for _ in 0..200 {
            x *= random_move();
            assert!((x*x.inverse()).is_solved());
            assert!((x.inverse()*x).is_solved());
            assert!((x.inverse_multiply(x)).is_solved());
            let mut y = Cube::default();
            for _ in 0..20 {
                y *= random_move();
            }
            assert_eq!(x.inverse()*y, x.inverse_multiply(y));
        }
    }

    #[test]
    fn move_vs_facemove() {

        use std::convert::TryFrom;
        let mut rng1 = oorandom::Rand32::new(0xdeadbeef);
        let mut random_move = || -> Move {
            Move::try_from((rng1.rand_u32()%36) as u8).unwrap()
        };
        let mut cube = Cube::default();
        for _ in 0..20 {
            for fm in FaceMove::moves() {
                assert_eq!(cube.corners()*fm, cube.corners()*Move::from(fm));
                assert_eq!(cube.edges()*fm, cube.edges()*Move::from(fm));
            }
            cube *= random_move();
        }
    }
    #[test]
    fn is_rotated_cube_solved() {
        use std::convert::TryFrom;
        let mut rng1 = oorandom::Rand32::new(0xdeadbeef);
        let mut rng2 = oorandom::Rand32::new(0xeeff2101);
        let mut random_rotation = || -> Move {
            Move::try_from((rng1.rand_u32()%9 + 27) as u8).unwrap()
        };
        let mut random_turn = || -> Move {
            Move::try_from((rng2.rand_u32()%27) as u8).unwrap()
        };
        let mut cube = Cube::default();
        let mut bitset = 0u32;
        for _ in 0..500 {
            assert!(cube.is_solved());
            bitset |= 1 << cube.centers().index(); 
            cube *= random_rotation();
            assert!((cube*random_turn()).is_solved() == false);
        }
        assert!(bitset.count_ones() == 24,
                "Not all rotations checked, try increasing iteration count.") 
    }

}
