use crate::FaceMove;
use crate::Face;

const E0:u64 = 0b00001_00001_00001_00001_00001_00001_00001_00001_00001_00001_00001_00001;
const E4:u64 = E0 << 4;

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


impl std::ops::Mul for EdgeMap {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self.multiply(rhs)
    }
}
impl std::ops::MulAssign for EdgeMap {
    fn mul_assign(&mut self, rhs: Self){
        *self = self.multiply(rhs);
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

