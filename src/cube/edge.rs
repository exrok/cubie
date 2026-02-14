use crate::Face;
use crate::FaceMove;
use crate::MapError;

const E0: u64 = 0b00001_00001_00001_00001_00001_00001_00001_00001_00001_00001_00001_00001;
const E4: u64 = E0 << 4;

/// Single edge orientation group. An edge is `Flipped` iff it cannot be solved using only <R1,U1,L1,D1,F2,B2> turns.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum EdgeOrientation {
    Identity = 0,
    Flipped = 1,
}
impl EdgeOrientation {
    pub fn is_flipped(self) -> bool {
        self == EdgeOrientation::Flipped
    }
    pub fn is_identity(self) -> bool {
        self == EdgeOrientation::Identity
    }
}

/// An edge piece on the 3x3 cube, characterized by the faces of the two adjacent centres.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
    pub fn edges() -> impl Iterator<Item = Edge> {
        unsafe { (0u8..12).map(|e| std::mem::transmute(e)) }
    }
    pub fn faces(self) -> (Face, Face) {
        use Edge::*;
        use Face::*;
        match self {
            LU => (Left, Up),
            LD => (Left, Down),
            RU => (Right, Up),
            RD => (Right, Down),
            BL => (Back, Left),
            FL => (Front, Left),
            FR => (Front, Right),
            BR => (Back, Right),
            BU => (Back, Up),
            FD => (Front, Down),
            FU => (Front, Up),
            BD => (Back, Down),
        }
    }
}
impl From<FaceMove> for EdgeMap {
    #[inline]
    fn from(turn: FaceMove) -> EdgeMap {
        use FaceMove::*;
        EdgeMap {
            raw: match turn {
                U1 => 0x058122398A41A828,
                U2 => 0x05A12A398A418022,
                U3 => 0x058920398A41A02A,
                D1 => 0x00A868398A458920,
                D2 => 0x04A968398A408860,
                D3 => 0x01A828398A448960,
                F1 => 0x05DAA83E74418820,
                F2 => 0x05A548394C418820,
                F3 => 0x05D6C83EB2418820,
                B1 => 0x0BA934C18BB18820,
                B2 => 0x04292B218A718820,
                B3 => 0x0A2937D98B818820,
                R1 => 0x05A928188A431C20,
                R2 => 0x05A92831CA410C20,
                R3 => 0x05A92810CA439820,
                L1 => 0x05A9283982018885,
                L2 => 0x05A9283988518801,
                L3 => 0x05A92839801188A4,
            },
        }
    }
}
#[inline]
fn fast_map(a: u64, mut map: impl FnMut(u64, u64)) {
    const ALT: u64 = 0b00000_01111_00000_01111_00000_01111_00000_01111_00000_01111_00000_01111;
    let set1 = (a & ALT) * 5;
    let set2 = ((a >> 5) & ALT) * 5;
    let mut offset = 60;
    while offset != 0 {
        offset -= 10;
        let input = (set1 >> offset) & 0b111111;
        map(input, offset);
        let input = (set2 >> offset) & 0b111111;
        map(input, offset + 5);
    }
}

fn map_mul(f: u64, b: u64) -> u64 {
    let mut res = f & E4;
    fast_map(f, |f_x, x| {
        res ^= ((b >> f_x) & 0b11111) << x;
    });
    res
}

fn map_inverse_mul(a: u64, b: u64) -> u64 {
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
        EdgeMap {
            raw: map_mul(self.raw, rhs.raw),
        }
    }
}
impl std::ops::MulAssign for EdgeMap {
    fn mul_assign(&mut self, rhs: Self) {
        self.raw = map_mul(self.raw, rhs.raw);
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
/// use cubie::cube::edge::{ Edge, Flip, EdgeMap };
/// use cubie::Move;
///
/// let mut edgemap = EdgeMap::default(); // solved edges
/// assert_eq!(edgemap.get(Edge::FU), (Edge::FU, Flip::Identity));
///
/// edgemap *= Move::U1;
/// assert_eq!(edgemap.get(Edge::FU), (Edge::LU, Flip::Identity));
///
/// {
///   use Move::*;
///   let mut edges = edgemap * R1 * U1 * F2 * B2 * D3; // Scamble
///   assert!(!edges.is_solved());
///   edges *= edges.inverse(); // Multiplying by the inverse results in the identity
///                            // map, the solved state.
///   assert!(edges.is_solved());
/// }
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeMap {
    pub raw: u64,
}

impl std::fmt::Debug for EdgeMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("EdgeMap")?;
        f.debug_map().entries(self.iter()).finish()
    }
}

impl Default for EdgeMap {
    fn default() -> EdgeMap {
        EdgeMap {
            raw: 0b01011_01010_01001_01000_00111_00110_00101_00100_00011_00010_00001_00000,
        }
    }
}
use std::mem::transmute;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct EOIndex(pub u32);

impl EOIndex {
    pub const SIZE: u32 = 2048; // 2^11
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct EPIndex(pub u32);

impl EPIndex {
    pub const SIZE: u32 = 479001600; // 12 factorial
}

impl EdgeMap {
    pub fn get(self, edge: Edge) -> (Edge, EdgeOrientation) {
        let s = self.raw >> (5 * (edge as u32));
        unsafe {
            (
                transmute((s & 0b1111) as u8),
                transmute(((s >> 4) & 0b1) as u8),
            )
        }
    }
    pub fn iter(self) -> impl Iterator<Item = (Edge, (Edge, EdgeOrientation))> + ExactSizeIterator {
        let mut set = self.raw;
        (0..12).map(move |i| unsafe {
            let edge: Edge = transmute(i as u8);
            let position: Edge = transmute((set & 0b1111) as u8);
            let ori: EdgeOrientation = transmute(((set >> 4) & 0b1) as u8);
            set >>= 5;
            (edge, (position, ori))
        })
    }
    pub fn from_iter(iter: impl Iterator<Item = (Edge, (Edge, EdgeOrientation))>) -> Option<EdgeMap> {
        let mut res = EdgeMap::default().raw;
        for (edge, (pos, ori)) in iter {
            res &= !(0b11111u64 << ((edge as u64) * 5));
            res |= ((pos as u64) | ((ori as u64) << 4)) << ((edge as u64) * 5);
        }
        let mapping = EdgeMap { raw: res };
        if mapping.validate().is_ok() {
            Some(mapping)
        } else {
            None
        }
    }

    pub(crate) fn validate(self) -> Result<(), MapError> {
        let get = |edge| {
            let s = self.raw >> (5 * (edge as u32));
            ((s & 0b1111), ((s >> 4) & 0b1))
        };
        let mut edge_mask = 0u32;
        let mut flip_sum = 0;
        for edge in Edge::edges() {
            let (pos, flip) = get(edge);
            if pos > 11 {
                return Err(MapError::OutOfBounds);
            }
            edge_mask |= 1 << (edge as u32);
            flip_sum ^= flip as u32;
        }
        if edge_mask != 0b1111_1111_1111 {
            Err(MapError::Duplicate)
        } else if flip_sum != 0 {
            Err(MapError::Orientation)
        } else {
            Ok(())
        }
    }

    pub fn inverse(self) -> EdgeMap {
        // const E0:u64 = 0b00001_00001_00001_00001_00001_00001_00001_00001_00001_00001_00001_00001;
        // const E4:u64 = E0 << 4;
        let set = self.raw;
        let mut res = 0;
        for i in 0..12 {
            let offset = i * 5;
            let input = set >> offset;
            let dest = i | (input & 0b10000);
            //TODO this should be or not xro
            res ^= dest << ((input & 0b1111) * 5);
        }
        EdgeMap { raw: res }
    }

    /// Apply inverse then multiply. Provided as an optimization, twice as fast as the equivalent in example.
    ///
    /// # Example
    /// ```
    /// use cubie::Move::*;
    /// let (a, b) = (R1.edges(), F1.edges());
    /// assert_eq!(a.inverse()*b, a.inverse_multiply(b));
    /// ```
    pub fn inverse_multiply(self, other: EdgeMap) -> EdgeMap {
        EdgeMap {
            raw: map_inverse_mul(self.raw, other.raw),
        }
    }

    pub fn is_solved(self) -> bool {
        self == EdgeMap::default()
    }
}

/// # Indices
impl EdgeMap {
    pub fn orientation_index(self) -> EOIndex {
        let mut b = 0;
        for i in 0..11 {
            b |= ((self.raw >> (i * 5 + 4)) & 0b1) << i;
        }
        EOIndex(b as u32)
    }

    pub fn permutation_index(self) -> EPIndex {
        let mut idx = 0;
        let mut val = 0xFEDCBA9876543210u64;
        for (edge, (pos, _)) in self.iter().take(11) {
            let v = (pos as u8) << 2;
            idx = (12 - edge as u32) * idx + ((val >> v) & 0xf) as u32;
            val -= 0x1111111111111110 << v;
        }
        EPIndex(idx as u32)
    }
    pub fn set_orientation_index(&mut self, index: EOIndex) {
        // if BMI2 is avaibled should use pdep TODO
        let mut b = 0;
        let e = index.0 as u64;
        for i in 0..11 {
            b |= (e & (0b1 << i)) << ((i * 5 + 4) - i);
        }
        b |= ((index.0.count_ones() as u64) & 1) << (5 * 11 + 4);
        self.raw &= !E4;
        self.raw |= b;
    }

    pub fn set_permutation_index(&mut self, index: EPIndex) {
        let mut idx = index.0 as u32;
        let mut val = 0xFEDCBA9876543210u64;
        let mut extract = 0u64;
        for p in 2..12 as u64 + 1 {
            extract = extract << 4 | (idx as u64) % p;
            idx /= p as u32;
        }
        let mut res = 0;
        for e in 0..11 {
            let v = ((extract & 0xf) << 2) as u32;
            extract >>= 4;
            res |= (((val >> v) & 0xf) as u64) << (e * 5);
            let m = (1u64 << v) - 1;
            val = val & m | (val >> 4) & !m; // TODO verfiy
        }
        res |= ((val & 0xf) as u64) << (5 * 11);
        self.raw &= E4;
        self.raw |= res;
    }

    pub fn orientation_residue(self) -> EdgeOrientation {
        //TODO optimized
        let mut flip_parity = 0;
        for (_, (_, flip)) in self.iter() {
            flip_parity ^= flip as u32;
        }
        if flip_parity == 0 {
            EdgeOrientation::Identity
        } else {
            EdgeOrientation::Flipped
        }
    }

    pub fn permutation_parity(self) -> bool {
        let mut transc = false;
        {
            // WALK edge permutation in discrete cycle form
            let mut rem = 0b1111_1111_1110u32;
            let mut at = Edge::LU;
            // print!("({:?}", at);
            while rem != 0 {
                at = self.get(at).0;
                let bit = 1u32 << (at as u8);
                if rem & bit != 0 {
                    rem ^= bit;
                    transc ^= true;
                    // print!(" {:?}", at);
                    continue;
                }
                at = unsafe { std::mem::transmute(rem.trailing_zeros() as u8) };
                rem ^= 1u32 << (at as u8);
                // print!(")({:?}", at);
            }
            // println!(")");
        }
        transc
    }
}

/// #Raw Interface
/// todo document
impl EdgeMap {
    pub fn from_raw(raw: u64) -> Result<EdgeMap, MapError> {
        let cm = EdgeMap { raw };
        cm.validate().map(|_| cm)
    }

    pub unsafe fn from_raw_unchecked(raw: u64) -> EdgeMap {
        EdgeMap { raw }
    }

    pub fn raw(self) -> u64 {
        self.raw
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::Move;
    use std::convert::TryFrom;
    #[test]
    fn edge_orientation_index_mapping() {
        let mut rng1 = oorandom::Rand32::new(0xdeadbeef);
        let mut random_move = || -> Move { Move::try_from((rng1.rand_u32() % 18) as u8).unwrap() };
        let mut rng = oorandom::Rand32::new(0xdeadbeef);
        let mut mcube = EdgeMap::default();
        for _ in 0..3000 {
            let mut cube = mcube;
            mcube *= random_move();
            let index = EOIndex(rng.rand_u32() % EOIndex::SIZE);
            cube.set_orientation_index(index);
            assert_eq!(index, cube.orientation_index());
            assert_eq!(cube.validate(), Ok(()));
        }
    }
    #[test]
    fn edge_premutation_index_mapping() {
        let mut rng1 = oorandom::Rand32::new(0xdeadbeef);
        let mut random_move = || -> Move { Move::try_from((rng1.rand_u32() % 18) as u8).unwrap() };
        let mut rng = oorandom::Rand32::new(0xdeadbeef);
        let mut mcube = EdgeMap::default();
        for _ in 0..3000 {
            let mut cube = mcube;
            mcube *= random_move();
            let index = EPIndex(rng.rand_u32() % EPIndex::SIZE);
            cube.set_permutation_index(index);
            assert_eq!(index, cube.permutation_index());
            assert_eq!(cube.validate(), Ok(()));
        }
    }
}
