use crate::cube::center::CenterMap;
use crate::Move;
use crate::{CornerMap, EdgeMap, FaceMove, MapError};
use std::ops::{Mul, MulAssign};

pub mod center;
pub mod corner;
pub mod edge;
mod fixed_centers_cube;
pub use self::edge::Edge;
pub use self::fixed_centers_cube::FixedCentersCube;

/// Stores both the centers and corners in a single u64. Used in
/// cube to optimize for size.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct CenteredCornerMap {
    pub raw: u64,
}

impl CenteredCornerMap {
    fn new(centers: CenterMap, corners: CornerMap) -> CenteredCornerMap {
        CenteredCornerMap {
            raw: centers.raw | corners.raw,
        }
    }
    fn corners(self) -> CornerMap {
        CornerMap {
            raw: self.raw & 0x1f1f1f1f_1f1f1f1f,
        }
    }
    fn centers(self) -> CenterMap {
        CenterMap {
            raw: self.raw & 0xe0e0e0,
        }
    }
}

/// 3x3 Puzzle Cube
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cube {
    pub centered_corners: CenteredCornerMap,
    pub edges: EdgeMap,
}

impl Default for Cube {
    fn default() -> Cube {
        Cube {
            centered_corners: CenteredCornerMap::new(CenterMap::default(), CornerMap::default()),
            edges: EdgeMap::default(),
        }
    }
}

/// # Components
impl Cube {
    pub fn new(centers: CenterMap, corners: CornerMap, edges: EdgeMap) -> Cube {
        Cube {
            centered_corners: CenteredCornerMap::new(centers, corners),
            edges,
        }
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
    pub fn set_corners(&mut self, corners: CornerMap) {
        self.centered_corners = CenteredCornerMap::new(self.centers(), corners);
    }
    pub fn set_edges(&mut self, edges: EdgeMap) {
        self.edges = edges;
    }
    pub fn set_centers(&mut self, centers: CenterMap) {
        self.centered_corners = CenteredCornerMap::new(centers, self.corners());
    }
}

impl Cube {
    pub const fn raw(self) -> (u64, u64) {
        (self.centered_corners.raw, self.edges.raw)
    }

    pub fn from_raw(centered_corners: u64, edges: u64) -> Result<Cube, MapError> {
        let cube = Cube {
            centered_corners: CenteredCornerMap {
                raw: centered_corners & 0x1f_1f1f1f1fffffff,
            },
            edges: EdgeMap { raw: edges },
        };
        cube.validate().map(|_| cube)
    }
    pub(crate) const fn from_raw_unchecked(centered_corners: u64, edges: u64) -> Cube {
        Cube {
            centered_corners: CenteredCornerMap {
                raw: centered_corners,
            },
            edges: EdgeMap { raw: edges },
        }
    }
    pub fn inverse_multiply(self, rhs: Cube) -> Cube {
        Cube {
            centered_corners: CenteredCornerMap::new(
                self.centers().inverse_multiply(rhs.centers()),
                self.corners().inverse_multiply(rhs.corners()),
            ),
            edges: self.edges.inverse_multiply(rhs.edges),
        }
    }
    pub fn inverse(self) -> Cube {
        Cube {
            centered_corners: CenteredCornerMap::new(
                self.centers().inverse(),
                self.corners().inverse(),
            ),
            edges: self.edges.inverse(),
        }
    }

    pub fn has_solution(self) -> bool {
        self.corners().orientation_residue().is_identity()
            && self.edges().orientation_residue().is_identity()
            && (self.edges.permutation_parity()
                ^ self.corners().permutation_parity()
                ^ self.centers().permutation_parity())
                == false
    }

    pub fn validate(self) -> Result<(), MapError> {
        self.edges
            .validate()
            .and_then(|_| self.corners().validate())
            .and_then(|_| self.centers().validate())
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

impl Mul for Cube {
    type Output = Self;
    fn mul(self, cube: Self) -> Self {
        Cube {
            centered_corners: CenteredCornerMap::new(
                self.centers() * cube.centers(),
                self.corners() * cube.corners(),
            ),
            edges: self.edges * cube.edges,
        }
    }
}

impl MulAssign for Cube {
    fn mul_assign(&mut self, cube: Self) {
        self.centered_corners = CenteredCornerMap::new(
            self.centers() * cube.centers(),
            self.corners() * cube.corners(),
        );
        self.edges *= cube.edges;
    }
}

impl MulAssign<FaceMove> for Cube {
    #[inline]
    fn mul_assign(&mut self, mv: FaceMove) {
        self.centered_corners =
            CenteredCornerMap::new(self.centers(), self.corners() * mv.corners());
        self.edges *= mv.edges();
    }
}

impl Mul<FaceMove> for Cube {
    type Output = Self;
    #[inline]
    fn mul(self, mv: FaceMove) -> Self {
        Cube {
            centered_corners: CenteredCornerMap::new(self.centers(), self.corners() * mv.corners()),
            edges: self.edges * mv.edges(),
        }
    }
}

impl MulAssign<Move> for Cube {
    #[inline]
    fn mul_assign(&mut self, mv: Move) {
        *self *= Cube::from(mv);
    }
}

impl Mul<Move> for Cube {
    type Output = Self;
    #[inline]
    fn mul(self, mv: Move) -> Self {
        self * Cube::from(mv)
    }
}

impl From<FixedCentersCube> for Cube {
    fn from(cube: FixedCentersCube) -> Cube {
        Cube {
            centered_corners: CenteredCornerMap::new(CenterMap::default(), cube.corners),
            edges: cube.edges,
        }
    }
}

impl From<Cube> for FixedCentersCube {
    fn from(cube: Cube) -> FixedCentersCube {
        let inv_rotation = cube.centers().inverse().cube();

        FixedCentersCube {
            corners: cube.corners() * inv_rotation.corners(),
            edges: cube.edges() * inv_rotation.edges(),
        }
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
        fn extend_cw(mv: Move, cw: Cube) -> CubeGroup {
            assert_eq!(cw, mv.cw().cube(), "Failed Bootstrap: Move {:?}", mv.cw());
            let two = cw * cw;
            assert_eq!(
                two,
                mv.two().cube(),
                "Failed Bootstrap: Move {:?}",
                mv.two()
            );
            let ccw = two * cw;
            assert_eq!(
                ccw,
                mv.ccw().cube(),
                "Failed Bootstrap: Move {:?}",
                mv.ccw()
            );
            CubeGroup { cw, two, ccw }
        }
        let u_cw = Move::U1.cube(); //generating set
        let y_cw = Move::Y1.cube();
        let x_cw = Move::X1.cube();

        let u = extend_cw(Move::U1, u_cw);
        let y = extend_cw(Move::Y1, y_cw);
        let x = extend_cw(Move::X1, x_cw);

        let z = extend_cw(Move::Z1, y.cw * x.ccw * y.ccw);
        let r = extend_cw(Move::R1, z.ccw * u.cw * z.cw);

        let l = extend_cw(Move::L1, y.two * r.cw * y.two);
        let b = extend_cw(Move::B1, y.cw * r.cw * y.ccw);
        let f = extend_cw(Move::F1, y.ccw * r.cw * y.cw);

        // and for fun and testing...
        let d_ccw = r.two * l.two * u.cw * f.two * b.two * u.cw * f.two * r.two * f.two * b.two
            * u.two * l.two * u.two * l.two * r.two * u.two * r.two * u.two * r.two * f.two * u.ccw
            * r.two * b.two * r.two * l.two * f.two * l.two * u.cw * b.two * f.two * u.cw;

        let d = extend_cw(Move::D1, d_ccw * d_ccw * d_ccw);

        let e = extend_cw(Move::E1, y.ccw * u.cw * d.ccw);
        let s = extend_cw(Move::S1, z.cw * f.ccw * b.cw);
        let m = extend_cw(Move::M1, x.ccw * r.cw * l.ccw);

        extend_cw(Move::Uw1, d.cw*y.cw);
        extend_cw(Move::Dw1, u.cw*y.ccw);

        extend_cw(Move::Fw1, b.cw*z.cw);
        extend_cw(Move::Bw1, f.cw*z.ccw);

        extend_cw(Move::Rw1, l.cw*x.cw);
        extend_cw(Move::Lw1, r.cw*x.ccw);
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
        let mut random_move = || -> Move { Move::try_from((rng1.rand_u32() % 36) as u8).unwrap() };
        let mut cube = Cube::default();
        let mut corners = CornerMap::default();
        let mut edges = EdgeMap::default();
        let mut centers = CenterMap::default();
        for _ in 0..400 {
            let mv = random_move();
            cube *= mv;
            corners *= mv;
            edges *= mv;
            centers *= mv;
            assert_eq!(cube.centers(), centers);
            assert_eq!(cube.edges(), edges);
            assert_eq!(cube.corners(), corners);
        }
    }

    #[test]
    fn cube_inverse_multiply() {
        use std::convert::TryFrom;
        let mut rng1 = oorandom::Rand32::new(0xdeadbeef);
        let mut random_move = || -> Move { Move::try_from((rng1.rand_u32() % 36) as u8).unwrap() };
        let mut x = Cube::default();
        for _ in 0..200 {
            x *= random_move();
            assert!((x * x.inverse()).is_solved());
            assert!((x.inverse() * x).is_solved());
            assert!((x.inverse_multiply(x)).is_solved());
            let mut y = Cube::default();
            for _ in 0..20 {
                y *= random_move();
            }
            assert_eq!(x.inverse() * y, x.inverse_multiply(y));
        }
    }

    #[test]
    fn move_vs_facemove() {
        use std::convert::TryFrom;
        let mut rng1 = oorandom::Rand32::new(0xdeadbeef);
        let mut random_move = || -> Move { Move::try_from((rng1.rand_u32() % 36) as u8).unwrap() };
        let mut cube = Cube::default();
        for _ in 0..20 {
            for fm in FaceMove::moves() {
                assert_eq!(cube.corners() * fm, cube.corners() * Move::from(fm));
                assert_eq!(cube.edges() * fm, cube.edges() * Move::from(fm));
            }
            cube *= random_move();
        }
    }
    #[test]
    fn is_rotated_cube_solved() {
        use std::convert::TryFrom;
        let mut rng1 = oorandom::Rand32::new(0xdeadbeef);
        let mut rng2 = oorandom::Rand32::new(0xeeff2101);
        let mut random_rotation =
            || -> Move { Move::try_from((rng1.rand_u32() % 9 + 27) as u8).unwrap() };
        let mut random_turn = || -> Move { Move::try_from((rng2.rand_u32() % 27) as u8).unwrap() };
        let mut cube = Cube::default();
        let mut bitset = 0u32;
        for _ in 0..500 {
            assert!(cube.is_solved());
            bitset |= 1 << cube.centers().index();
            cube *= random_rotation();
            assert!((cube * random_turn()).is_solved() == false);
        }
        assert!(
            bitset.count_ones() == 24,
            "Not all rotations checked, try increasing iteration count."
        )
    }
}
