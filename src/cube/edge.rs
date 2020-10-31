const E0:u64 = 0b00001_00001_00001_00001_00001_00001_00001_00001_00001_00001_00001_00001;
const E1:u64 = E0 << 1;
const E2:u64 = E0 << 2;
const E3:u64 = E0 << 3;
const E4:u64 = E0 << 4;
use crate::FaceMove;
use crate::Face;
use crate::Move;

/// Single edge orientation group. An edge is `Flipped` iff it cannot be solved using only <Rcw,Ucw,Lcw,Dcw,F2,B2> turns.
#[derive(Clone,Copy,PartialEq,Eq,Debug)]
#[repr(u8)]
pub enum Flip {
    Identity = 0,
    Flipped = 1,
}

/// An edge piece on the 3x3 cube, characterized by the faces of the two adjacent centres.
#[derive(Clone,Copy,PartialEq,Eq,Debug)]
#[repr(u8)]
pub enum Edge {
    /// The left & up edge.
    LU,
    /// The left & down edge.
    LD,
    /// The right & up edge.
    RU,
    /// The right & down edge.
    RD,
    /// The back & left edge.
    BL,
    /// The front & left edge.
    FL,
    /// The front & right edge. 
    FR,
    /// The back & right edge. 
    BR,
    /// The back & up edge. 
    BU,
    /// The front & down edge. 
    FD,
    /// The front & up edge. 
    FU,
    /// The back & down edge.
    BD,
}
impl Edge {
    pub fn faces(self) -> (Face, Face) {
        use Edge::*;
        use Face::*;
        match self {
            LU => (Left,Up),
            LD => (Left,Down),
            RU => (Right,Up),
            RD => (Right,Down),
            BL => (Back,Left),
            FL => (Front,Left),
            FR => (Front,Right),
            BR => (Back,Right),
            BU => (Back,Up),
            FD => (Front,Down),
            FU => (Front,Up),
            BD => (Back,Down),
        }
    }
}
impl From<FaceMove> for EdgeMap {
    fn from(turn: FaceMove) ->EdgeMap {
        use FaceMove::*;
        EdgeMap{set: match turn {
            Ucw => 0x058122398A41A828,
            U2 => 0x05A12A398A418022,
            Uccw => 0x058920398A41A02A,
            Dcw => 0x00A868398A458920,
            D2 => 0x04A968398A408860,
            Dccw => 0x01A828398A448960,
            Fcw => 0x05DAA83E74418820,
            F2 => 0x05A548394C418820,
            Fccw => 0x05D6C83EB2418820,
            Bcw => 0x0BA934C18BB18820,
            B2 => 0x04292B218A718820,
            Bccw => 0x0A2937D98B818820,
            Rcw => 0x05A928188A431C20,
            R2 => 0x05A92831CA410C20,
            Rccw => 0x05A92810CA439820,
            Lcw => 0x05A9283982018885,
            L2 => 0x05A9283988518801,
            Lccw => 0x05A92839801188A4
        }}
    }
}
#[inline]
fn fast_map(a:u64, mut map: impl FnMut(u64, u64)) {
    const ALT:u64 =0b00000_01111_00000_01111_00000_01111_00000_01111_00000_01111_00000_01111;
    let set1 = (a & ALT) *5;
    let set2 = ((a >> 5) & ALT)*5;
    let mut offset = 60;
    while offset != 0 {
        offset -= 10;
        let input = (set1 >> offset) & 0b111111;
        map(input, offset);
        let input = (set2 >> offset) & 0b111111;
        map(input, offset+5);
    }
}

fn map_mul(f:u64, b: u64) -> u64 {
    let mut res = f & E4;
    fast_map(f, |f_x, x| {
        res ^= ((b >> f_x)&0b11111) << x;
    });
    res
}

fn map_inverse_mul(a:u64, b: u64) -> u64 {
    let g = b ^ (a & E4);
    let mut res = 0;
    fast_map(a, |f_x, x| {
        res |= ((g >> x) & 0b11111) << f_x;
     });
    res
}

use std::ops::{ Mul,MulAssign };

impl Mul<Move> for EdgeMap {
    type Output = Self;

    #[inline]
    fn mul(self, mv: Move) -> Self {
        self * mv.edges()
    }
}

impl Mul<FaceMove> for EdgeMap {
    type Output = Self;

    #[inline]
    fn mul(self, turn: FaceMove) -> Self {
        self.apply(turn)
    }
}

impl MulAssign<FaceMove> for EdgeMap {
    #[inline]
    fn mul_assign(&mut self, rhs: FaceMove) {
        *self = self.apply(rhs);
    }
}
impl Mul for EdgeMap {
    // The multiplication of rational numbers is a closed operation.

    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self.multiply(rhs)
    }
}
impl MulAssign for EdgeMap {
    fn mul_assign(&mut self, rhs: Self){
        *self = self.multiply(rhs);
    }
}
impl MulAssign<Move> for EdgeMap {
    fn mul_assign(&mut self, mv: Move){
        *self *= mv.edges();
    }
}



/// Edge piece mapping for 3x3 cube: Edge -> (EdgePosition, Flip)
///
/// The primary way to manipulate a mapping is by multiplying it by a turn or another mapping.
/// [EdgeMap](struct.EdgeMap.html), [CornerMap](struct.CornerMap.html) and [Cube](struct.Cube.html) work
/// similarly with multiplication which is equivalent to their respective group operations. See the
/// example below. 
///
/// Stores a mapping `Edge -> (Edge, Flipped)` where given `edge: Edge`, `map: EdgeMap`,
/// `let (position, orientation) = map.get(edge)` describes position of the given edge
/// on cube and where it is flipped. 
///
/// The mapping is represented by a bit-packed array of len 12 (an entry for each edge)
/// stored in a single u64.
///
/// Converting an `EdgeMap` to a `TileMap` can be used to visualize the mapping, see [TileMap::terminal_display](struct.TileMap.html#method.terminal_display).
///
/// # Example
/// ```
/// use speedcube::cube::edge::{ Edge, Flip, EdgeMap };
/// use speedcube::Move;
///
/// let mut edgemap = EdgeMap::default(); // solved edges
/// assert_eq!(edgemap.get(Edge::FU), (Edge::FU, Flip::Identity));
///
/// edgemap *= Move::Ucw;
/// assert_eq!(edgemap.get(Edge::FU), (Edge::LU, Flip::Identity));
///
/// {
///   use Move::*; 
///   let mut edges = edgemap * Rcw * Ucw * F2 * B2 * Dccw; // Scamble
///   assert!(!edges.is_solved());
///   edges *= edges.inverse(); // Multiplying by the inverse results in the identity 
///                            // map, the solved state.
///   assert!(edges.is_solved());
/// }
/// ```
#[derive(Clone,Copy,PartialEq,Eq,Hash)]
pub struct EdgeMap {
    pub set: u64
}

impl std::fmt::Debug for EdgeMap {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       f.write_str("EdgeMap")?;
       f.debug_map()
           .entries(self.iter())
           .finish()
    }
}
#[inline]
fn filter_mask(bit: u64) -> u64 {
    E4^(E4 - (bit & E0))
}
impl Default for EdgeMap {
    fn default() -> EdgeMap {
        EdgeMap{set: 0b01011_01010_01001_01000_00111_00110_00101_00100_00011_00010_00001_00000}
    }
}
use std::mem::transmute;

impl EdgeMap {
    pub fn iter(self) -> impl Iterator<Item=(Edge,(Edge,Flip))> + ExactSizeIterator{
        let mut set = self.set;
        (0..12).map(move |i| unsafe {
            let edge: Edge = transmute(i as u8);
            let position: Edge = transmute((set&0b1111) as u8);
            let ori: Flip = transmute(((set>>4)&0b1) as u8);
            set >>= 5;
            (edge,(position,ori))
        })
    }
    pub fn get( self, edge: Edge)->(Edge,Flip) {
        let s = self.set >> (5 *(edge as u32));
        unsafe{
            (transmute((s&0b1111) as u8),
            transmute(((s>>4)&0b1) as u8))
        }
    }
    pub fn is_solved(self) -> bool {
        self == EdgeMap::default()
    }
    pub fn inverse(self) -> EdgeMap {
        // const E0:u64 = 0b00001_00001_00001_00001_00001_00001_00001_00001_00001_00001_00001_00001;
        // const E4:u64 = E0 << 4;
        let set = self.set;
        let mut res = 0;
        for i in 0..12 {
            let offset = i *5;
            let input = set >> offset;
            let dest = i | (input & 0b10000);
            //TODO this should be or not xro
            res ^= dest << ((input & 0b1111)*5);
        }
        EdgeMap{set:res}
    }
    fn multiply(self, other: EdgeMap) -> EdgeMap {
        EdgeMap{set: map_mul(self.set, other.set)}
    }

    /// Apply inverse then multiply. Provided as an optimization, twice as fast as the equivalent in example.
    ///
    /// # Example
    /// ```
    /// use speedcube::Move::*;
    /// let (a, b) = (Rcw.edges(), Fcw.edges());
    /// assert_eq!(a.inverse()*b, a.inverse_multiply(b));
    /// ```
    pub fn inverse_multiply(self, other: EdgeMap) -> EdgeMap {
        EdgeMap{set: map_inverse_mul(self.set, other.set)}
    }

}

// Pure bit-twiddling implementation, complex and unsafe but faster. However,
// using theses in-practice requires an branch in-direction. When the branch
// is predicted 1/3 cycles vs a multiply depending on the CPU. But, if the
// turns are random, and branch prediction fails, they are marginally slower.
//
// Previously, `CornerMap` had a similar bit-twiddling implementation and when
// only in-direction was used to switch for both the `CornerMap` & `EdgeMap` in
// `Cube`, the performance was universally better. However, the representation
// of the orientation for the corners was flawed, as orientation of corners
// relative to current positions (meaning the index's where not independent).
// Fixing the representation issue of `CornerMap` made the direct bit-twiddling
// approach less-efficient.
impl EdgeMap {
    fn filter_down(self) -> u64 {
        filter_mask(self.set & !(self.set >> 2))
    }
    fn filter_up(self) -> u64 {
        filter_mask(!(self.set | (self.set >> 2)))
    }
// Binary { op: ANDI, a: 1, b: 3 }
// Binary { op: NOR, a: 1, b: 3 }
    fn filter_right(self) -> u64 {
        filter_mask((self.set >> 1) & !(self.set >> 3))
    }
    fn filter_left(self) -> u64 {
        filter_mask(!((self.set >> 1) | (self.set >> 3)))
    }
// BinaryOffset { op: IAND, a: 1, b: 3, offset: 3 }
// Multi4 { op1: XNOR, op2: OR, a1: 0, b1: 1, a2: 2, b2: 3, join: 0 }
    fn filter_front(self) -> u64 {
        let e = self.set + (E0|E1);
        filter_mask(!(e >> 1) & (e >> 3))
    }
    fn filter_back(self) -> u64 {
        let e = self.set;
        filter_mask(!((e) ^ (e >> 1)) & ((e>>2) | (e>>3)))
    }

// ===== up_cw ====
// [(10, 2), (2, 8), (8, 0), (0, 10)]
// SCORE: 2
// BIT 0: Some(Zero) 
// BIT 1: Some(Unary { negate: true, a: -2 }) !(e >> 2)
// BIT 2: Some(Zero) 
// BIT 3: Some(One) 

// ===== up_ccw ====
// [(2, 10), (8, 2), (0, 8), (10, 0)]
// SCORE: 1
// BIT 0: Some(Zero) 
// BIT 1: Some(Unary { negate: false, a: -2 }) e >> 2
// BIT 2: Some(Zero) 
// BIT 3: Some(One) 
    #[inline]
    fn apply(self, turn: FaceMove) -> EdgeMap {
        use FaceMove::*;
        (match turn {
            R2 => EdgeMap::right_2,
            Rcw => EdgeMap::right_cw,
            Rccw => EdgeMap::right_ccw,
            L2 => EdgeMap::left_2,
            Lcw => EdgeMap::left_cw,
            Lccw => EdgeMap::left_ccw,
            U2 => EdgeMap::up_2,
            Ucw =>  EdgeMap::up_cw,
            Uccw => EdgeMap::up_ccw,
            D2 => EdgeMap::down_2,
            Dcw => EdgeMap::down_cw,
            Dccw =>EdgeMap::down_ccw,
            F2 => EdgeMap::front_2,
            Fcw => EdgeMap::front_cw,
            Fccw =>EdgeMap::front_ccw,
            B2 => EdgeMap::back_2,
            Bcw => EdgeMap::back_cw,
            Bccw =>EdgeMap::back_ccw,
            // M2 => self.right_cw().right_cw().left_cw().left_cw(),
            // Mcw => self.right_cw().left_ccw(),
            // Mccw => self.right_ccw().left_cw(),
            // S2 => self.front_cw().front_cw().back_cw().back_cw(),
            // Scw => self.front_ccw().back_cw(),
            // Sccw => self.front_cw().back_ccw(),
            // E2 => self.up_cw().up_cw().down_cw().down_cw(),
            // Ecw => self.up_ccw().down_cw(),
            // Eccw => self.up_cw().down_ccw(),
        })(self)
    }

    pub fn back_2(self) -> EdgeMap {
        let e = self.set;
        let k = (e >> 1) ^  e;
        let k = !k & (k >> 2) & E0;
        return EdgeMap{set: e ^ k ^ (k << 1)};
    }
    pub fn down_2(self) -> EdgeMap {
        let e = self.set;
        return EdgeMap{set: e^ (((self.set << 1) & !(self.set >> 1)) & E1)}
    }
    fn up_ccw(self) -> EdgeMap {
        let e = self.set;
        return EdgeMap{set: e ^ ((E3 | (!(e >> 2) & E1)) & self.filter_up())};
    }
    fn up_cw(self) -> EdgeMap {
        let e = self.set;
        return EdgeMap{set: e ^ ((E3 | ((e >> 2) & E1)) & self.filter_up())};
    }
    fn down_ccw(self) -> EdgeMap {
        let e = self.set;
        return EdgeMap{set: e ^ ((E3 | (!(e >> 2) & E1)) & self.filter_down())};
    }
     fn down_cw(self) -> EdgeMap {
        let e = self.set;
        return EdgeMap{set: e ^ ((E3 | ((e >> 2) & E1)) & self.filter_down())};
    }

    fn right_cw(self) -> EdgeMap {
        let e = self.set;
        return EdgeMap{set: e ^ (( E2 | (!(e >> 2) & E0)) & self.filter_right())};
    }
     fn right_ccw(self) -> EdgeMap {
        let e = self.set;
        return EdgeMap{set: e ^ (( E2 | ((e >> 2) & E0)) & self.filter_right())};
    }
    fn left_cw(self ) -> EdgeMap {
        let e = self.set;
        return EdgeMap{set: e ^ ((E2 | ((!(e >> 2)) & E0)) & self.filter_left())};
    }
    fn left_ccw(self) -> EdgeMap {
        let e = self.set;
        return EdgeMap{set: e ^ ((E2 | ((e >> 2) & E0)) & self.filter_left())};
    }
    fn front_ccw(self) -> EdgeMap {
        let e = self.set;
        return EdgeMap{set: e ^ (((E4|E3|E2) | ((e >> 3) & E0) | ((e >> 2) & E1))
                                 & self.filter_front())};
    }
    fn front_cw(self) -> EdgeMap {
        let e = self.set;
        return EdgeMap{set: e ^ (((E4|E3|E2) | ((e >> 2) & E0) | ((e >> 1) & E1))
                                 & self.filter_front())};
    }
    fn back_ccw(self) -> EdgeMap {
        let e = self.set;
        return EdgeMap{set: e ^ (((E4|E3|E2) | ((e >> 3) & E0) | ((e >> 2) & E1))
                                 & self.filter_back())};
    }
    fn back_cw(self) -> EdgeMap {
        let e = self.set;
        return EdgeMap{set: e ^ (((E4|E3|E2) | ((e >> 2) & E0) | ((e >> 1) & E1))
                                 & self.filter_back())};
    }

    fn front_2(self) -> EdgeMap {
        let e = self.set;
        let k = (e >> 1) ^  e;
        let k = k & (k >> 2) & E0;
        return EdgeMap{set: e ^ k ^ (k << 1)};
    }
    fn e_cw(self) -> EdgeMap {
        let e = self.set;
        let f = (!e << 1) & E3; //Selection
        return EdgeMap{set: e ^ f ^ ((f & ((!e << 3)^e)) >> 2) };
    }
    fn e_2(self) -> EdgeMap {
        let e = self.set;
        let f = (!e >>1) & E1; //Selection
        return EdgeMap{set: e ^ f };
    }
    fn e_ccw(self) -> EdgeMap {
        let e = self.set;
        let f = (!e << 1) & E3; //Selection
        return EdgeMap{set: e ^ f ^ ((f & ((e << 3)^e)) >> 2) };
    }

    pub fn m_cw(self) -> EdgeMap {
        let e = self.set;
        let f = (!e >> 1) & E2; //Selection
        return EdgeMap{set: e ^ f ^ ((f & ((e << 1)^e)) >> 2) };
    }
    fn m_ccw(self) -> EdgeMap {
        let e = self.set;
        let f = (!e >> 1) & E2; //Selection
        return EdgeMap{set: e ^ f ^ ((f & ((!e << 1)^e)) >> 2) };
    }
    fn m_2(self) -> EdgeMap {
        let e = self.set;
        let f = (!e >>3) & E0; //Selection
        return EdgeMap{set: e ^ f };
    }

    pub fn up_2(self) -> EdgeMap {
        let e = self.set;
        return EdgeMap{set: e^ (!((e << 1) | (e >> 1)) & E1)}
    }
   fn right_2(self) -> EdgeMap {
        let e = self.set;
        EdgeMap{set: e ^ ((e >> 1) & !(e >> 3) & E0)}
    }
    fn left_2(self) -> EdgeMap{
        let e = self.set;
        EdgeMap{set: e ^ (!((self.set >> 1) | (self.set >> 3)) & E0)}
    }
    fn s_ccw(self) -> EdgeMap {
        let e = self.set;
        let k = (e >> 1) ^ e;
        let l = ((e >> 3) ^k) &E0; 
        let l =(l+l) | l; 
//        let l = !(( (e & E0) << 1) | e);
        EdgeMap{set: e ^ (((E4|E3|E2) | (l & (E0 | E1)))
                          & filter_mask(k >> 2))}
    }
    fn s_2(self) -> EdgeMap {
        let e = self.set;
        let k = (e >> 1) ^  e;
        let k = (k >> 2) & E0;
        return EdgeMap{set: e ^ k ^ (k << 1)};
    }
    fn s_cw(self) -> EdgeMap {
        let e = self.set;
        let k = (e >> 1) ^ e;
        let l = ((e >> 2) ^k) &E0; 
        let l =(l+l) | l; 
//        let l = !(( (e & E0) << 1) | e);
        EdgeMap{set: e ^ (((E4|E3|E2) | (l & (E0 | E1)))
                          & filter_mask(k >> 2))}
    }
}
