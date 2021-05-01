use crate::cube::center::CenterMap;
use crate::cube::corner::Corner;
use crate::cube::edge::Edge;
use crate::Cube;
use crate::EdgeMap;
use crate::Face;
use crate::FixedCentersCube;
use crate::{CornerMap, MapError};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TileMapConversionError {
    InvalidPiece,
    MissingTile,
    Corner(MapError),
    Center(MapError),
    Edge(MapError),
}
#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub enum Tile {
    U1,U2,U3,U4,U5,U6,U7,U8,U9,
    D1,D2,D3,D4,D5,D6,D7,D8,D9,
    F1,F2,F3,F4,F5,F6,F7,F8,F9,
    B1,B2,B3,B4,B5,B6,B7,B8,B9,
    R1,R2,R3,R4,R5,R6,R7,R8,R9,
    L1,L2,L3,L4,L5,L6,L7,L8,L9,
}
use TileMapConversionError as TMErr;
use std::ops::{ Index,IndexMut };
impl Index<Tile> for TileMap {
    type Output = Option<Face>;

    fn index(&self, index: Tile) -> &Self::Output {
        &self.as_array()[index as usize]
    }
}

impl IndexMut<Tile> for TileMap {
    fn index_mut(&mut self, index: Tile) -> &mut Self::Output {
        &mut self.as_array_mut()[index as usize]
    }
}
#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct TileMap {
    pub(crate) map: [[Option<Face>; 9]; 6],
}
impl From<Cube> for TileMap {
    fn from(cube: Cube) -> TileMap {
        let mut tilemap: TileMap = Default::default();
        tilemap.store_centers(cube.centers());
        tilemap.store_edges(cube.edges(),0);
        tilemap.store_corners(cube.corners(),0);
        tilemap
    }
}
impl From<FixedCentersCube> for TileMap {
    fn from(cube: FixedCentersCube) -> TileMap {
        let mut tilemap: TileMap = Default::default();
        tilemap.store_identity_centers();
        tilemap.store_edges(cube.edges,0);
        tilemap.store_corners(cube.corners,0);
        tilemap
    }
}

impl From<CenterMap> for TileMap {
    fn from(centers: CenterMap) -> TileMap {
        let mut tilemap: TileMap = Default::default();
        tilemap.store_centers(centers);
        tilemap
    }
}
impl From<CornerMap> for TileMap {
    fn from(corners: CornerMap) -> TileMap {
        let mut tilemap: TileMap = Default::default();
        tilemap.store_identity_centers();
        tilemap.store_corners(corners,0);
        tilemap
    }
}

impl From<EdgeMap> for TileMap {
    fn from(edges: EdgeMap) -> TileMap {
        let mut tilemap: TileMap = Default::default();
        tilemap.store_identity_centers();
        tilemap.store_edges(edges,0);
        tilemap
    }
}

use crate::cube::edge::Flip;
fn edge_from_faces(a: Face, b: Face) -> Option<(Edge, Flip)> {
    use Edge::*;
    use Flip::*;

    // for edge in Edge::edges() {
    //     let (a,b) = edge.faces();
    //     let ai = a as usize;
    //     let bi = b as usize;
    //     bff[ai*6+bi] = Some((edge, Flip::Identity));
    //     bff[bi*6+ai] = Some((edge, Flip::Flipped));
    //     // eprintln!("{:?} edge", edge.faces())
    // }
    let map = [
        None,
        None,
        Some((FU, Flipped)),
        Some((BU, Flipped)),
        Some((RU, Flipped)),
        Some((LU, Flipped)),
        None,
        None,
        Some((FD, Flipped)),
        Some((BD, Flipped)),
        Some((RD, Flipped)),
        Some((LD, Flipped)),
        Some((FU, Identity)),
        Some((FD, Identity)),
        None,
        None,
        Some((FR, Identity)),
        Some((FL, Identity)),
        Some((BU, Identity)),
        Some((BD, Identity)),
        None,
        None,
        Some((BR, Identity)),
        Some((BL, Identity)),
        Some((RU, Identity)),
        Some((RD, Identity)),
        Some((FR, Flipped)),
        Some((BR, Flipped)),
        None,
        None,
        Some((LU, Identity)),
        Some((LD, Identity)),
        Some((FL, Flipped)),
        Some((BL, Flipped)),
        None,
        None,
    ];
    map[a as usize * 6 + b as usize]
}
pub fn tiles_from_edge(edge: Edge) -> (usize, usize) {
    match edge {
        Edge::LU => (46, 3),
        Edge::LD => (52, 12),
        Edge::RU => (37, 5),
        Edge::RD => (43, 14),
        Edge::BL => (32, 48),
        Edge::FL => (21, 50),
        Edge::FR => (23, 39),
        Edge::BR => (30, 41),
        Edge::BU => (28, 1),
        Edge::FD => (25, 10),
        Edge::FU => (19, 7),
        Edge::BD => (34, 16),
    }
}
pub fn tiles_from_corner(corner: Corner) -> [usize; 3] {
    match corner {
        Corner::ULF => [47, 6, 18],
        Corner::DLF => [53, 9, 24],
        Corner::ULB => [45, 0, 29],
        Corner::URF => [36, 8, 20],
        Corner::URB => [38, 2, 27],
        Corner::DLB => [51, 15, 35],
        Corner::DRF => [42, 11, 26],
        Corner::DRB => [44, 17, 33],
    }
}
struct SrcTileMask {

}
use crate::cube::corner::Twist;
impl TileMap {
#[doc(hidden)]
    pub fn as_array(&self) -> &[Option<Face>; 54] {
        unsafe { std::mem::transmute(&self.map) }
    }
#[doc(hidden)]
    pub fn as_array_mut(&mut self) -> &mut [Option<Face>; 54] {
        unsafe { std::mem::transmute(&mut self.map) }
    }
    pub fn cube(&self) -> Result<Cube, TMErr> {
        Ok(Cube::new(self.centers()?, self.corners()?, self.edges()?))
    }
    pub fn fc_cube(&self) -> Result<FixedCentersCube, TMErr> {
        Ok(self.cube()?.into())
    }
    pub fn autofill(&mut self) {
        let tiles = self.as_array_mut();

        let mut corner_acc = 0;
        let mut twist_acc = Twist::Identity;
        let mut unset_acc = None;
        let mut set_cnt = 0;
        for pos in Corner::corners() {
            let map = tiles_from_corner(pos);
            let mut unset = 0;
            let mut cnt = 0;
            let mut f = [0, 0, 0];
            for i in 0..3 {
                if let Some(face) = tiles[map[i]] {
                    f[i] = face as u64;
                    cnt += 1;
                } else {
                    unset = i;
                }
            }
            if cnt == 2 {
                f[unset] = (f[0] ^ f[1] ^ f[2] ^ 0b110) & 0b110;
                if f[unset] & 0b110 == 0b110 {
                    //two same faces
                    continue;
                }
                let corner = ((f[0] & 1) << (f[0] >> 1))
                    | ((f[1] & 1) << (f[1] >> 1))
                    | ((f[2] & 1) << (f[2] >> 1));
                let y_axis = if (f[0] & 0b110) == 0 {
                    f[2]
                } else if (f[1] & 0b110) == 0 {
                    f[0]
                } else {
                    f[1]
                };

                if (((0b01101001_00) >> ((pos as u64) ^ (corner as u64))) ^ y_axis) & 0b100 != 0 {
                    f[unset] ^= 1;
                }
                tiles[map[unset]] = Some(unsafe { std::mem::transmute(f[unset] as u8) });
            }
            if cnt < 2 {
                unset_acc = Some(pos);
                continue;
            }
            let [f1, f2, f3] = f;

            if (f1 ^ f2 ^ f3) & 0b110 != 0b110 {
                continue;
            }
            // corner may still be wrong if 2-cycle in above faces thus test y_axis consistancty
            let corner =
                ((f1 & 1) << (f1 >> 1)) | ((f2 & 1) << (f2 >> 1)) | ((f3 & 1) << (f3 >> 1));
            let (mut twist, y_axis) = if (f1 & 0b110) == 0 {
                (Twist::C1, f3)
            } else if (f2 & 0b110) == 0 {
                (Twist::Identity, f1)
            } else if (f3 & 0b110) == 0 {
                (Twist::Cw, f2)
            } else {
                continue;
            };

            if (((0b01101001_00) >> ((pos as u64) ^ (corner as u64))) ^ y_axis) & 0b100 != 0 {
                continue;
            }

            let a = pos as u64;
            if (a ^ (a >> 1) ^ (a >> 2)) & 0b1 == 1 {
                twist = twist.inverse();
            }

            corner_acc ^= corner as u64;
            twist_acc = twist_acc * twist;
            set_cnt += 1;
        }
        if set_cnt == 7 {
            if let Some(unset_acc) = unset_acc {
                let c: Corner = unsafe { std::mem::transmute(corner_acc as u8) };
                let map = tiles_from_corner(unset_acc);
                let a = unset_acc as u32;
                if (a ^ (a >> 1) ^ (a >> 2)) & 0b1 == 1 {
                    eprintln!("inversed");
                    twist_acc = twist_acc.inverse();
                }
                eprintln!("AUTOFILE: {:?}", twist_acc);
                let new = match twist_acc.inverse() {
                    Twist::Identity => [c.x(), c.y(), c.z()],
                    Twist::Cw => [c.z(), c.x(), c.y()],
                    Twist::C1 => [c.y(), c.z(), c.x()],
                };
                if map
                    .iter()
                    .map(|m| tiles[*m])
                    .zip(new.iter())
                    .all(|(o, n)| o.map_or(true, |a| a == *n))
                {
                    for i in 0..3 {
                        tiles[map[i]] = Some(new[i]);
                    }
                }
            }
        }
        let mut edge_masks = [
            0b111111 ^ (1 << Face::Up as u8) ^ (1 << Face::Down as u8),
            0b111111 ^ (1 << Face::Up as u8) ^ (1 << Face::Down as u8),
            0b111111 ^ (1 << Face::Front as u8) ^ (1 << Face::Back as u8),
            0b111111 ^ (1 << Face::Front as u8) ^ (1 << Face::Back as u8),
            0b111111 ^ (1 << Face::Right as u8) ^ (1 << Face::Left as u8),
            0b111111 ^ (1 << Face::Right as u8) ^ (1 << Face::Left as u8),
        ];
        let mut face_cnts = [0, 0, 0, 0, 0, 0];

        for pos in Edge::edges() {
            let (t1, t2) = tiles_from_edge(pos);
            let (f1, f2) = (tiles[t1], tiles[t2]);
            if let Some(face) = f1 {
                face_cnts[face as usize] += 1;
                if let Some(other_face) = f2 {
                    edge_masks[face as usize] &= !(1 << (other_face as u8));
                }
            }
            if let Some(face) = f2 {
                face_cnts[face as usize] += 1;
                if let Some(other_face) = f1 {
                    edge_masks[face as usize] &= !(1 << (other_face as u8));
                }
            }
        }
        let mut t_mask = 0;
        for (face, &cnt) in face_cnts.iter().enumerate() {
            if cnt < 4 {
                t_mask |= 1 << face;
            }
        }
        for _ in 0..3 {
            for pos in Edge::edges() {
                let (t1, t2) = tiles_from_edge(pos);
                match ((tiles[t1], t1), (tiles[t2], t2)) {
                    ((Some(face), _), (None, tile)) | ((None, tile), (Some(face), _)) => {
                        let opt = (t_mask & edge_masks[face as usize]) as u8;
                        if opt.count_ones() != 1 {
                            continue;
                        }
                        let fc: Face = unsafe { std::mem::transmute(opt.trailing_zeros() as u8) };
                        tiles[tile] = Some(fc);
                        face_cnts[fc as usize] += 1;
                        edge_masks[fc as usize] &= !(1 << (face as u8));
                        edge_masks[face as usize] &= !(1 << (fc as u8));
                        if face_cnts[fc as usize] >= 4 {
                            t_mask &= !(1 << (fc as u8))
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    pub fn edges(&self) -> Result<EdgeMap, TMErr> {
        let tiles = self.as_array();
        let mut raw = 0;
        for pos in Edge::edges() {
            let (t1, t2) = tiles_from_edge(pos);
            let (edge, flip) = edge_from_faces(
                tiles[t1].ok_or(TMErr::MissingTile)?,
                tiles[t2].ok_or(TMErr::MissingTile)?,
            )
            .ok_or(TMErr::InvalidPiece)?;
            let parity = flip as u64 ^ (((edge as u64 ^ pos as u64) >> 2) & 1);
            let offset = edge as u32;
            raw |= parity << (4 + offset * 5);
            raw |= (pos as u64) << (offset * 5);
        }
        EdgeMap::from_raw(raw).map_err(|err| TMErr::Edge(err))
    }

    pub fn centers(&self) -> Result<CenterMap, TMErr> {
        let y1 = self.map[Face::Up as usize][4].ok_or(TMErr::MissingTile)? as u64;
        let y2 = self.map[Face::Down as usize][4].ok_or(TMErr::MissingTile)? as u64;
        if (y1 ^ y2) != 1 {
            return Err(TMErr::Center(MapError::Orientation));
        }
        let x1 = self.map[Face::Front as usize][4].ok_or(TMErr::MissingTile)? as u64;
        let x2 = self.map[Face::Back as usize][4].ok_or(TMErr::MissingTile)? as u64;
        if (x1 ^ x2) != 1 {
            return Err(TMErr::Center(MapError::Orientation));
        }
        let z1 = self.map[Face::Right as usize][4].ok_or(TMErr::MissingTile)? as u64;
        let z2 = self.map[Face::Left as usize][4].ok_or(TMErr::MissingTile)? as u64;
        if (z1 ^ z2) != 1 {
            return Err(TMErr::Center(MapError::Orientation));
        }
        CenterMap::from_raw((y1 << 5) | (x1 << (8 + 5)) | (z1 << (16 + 5)))
            .map(|centers| centers.inverse())
            .map_err(|err| TMErr::Center(err))
    }
    pub fn corners(&self) -> Result<CornerMap, TMErr> {
        let tiles = self.as_array();
        let mut raw = 0;
        for pos in Corner::corners() {
            let [t1, t2, t3] = tiles_from_corner(pos);
            let (f1, f2, f3) = (
                tiles[t1].ok_or(TMErr::MissingTile)? as u64,
                tiles[t2].ok_or(TMErr::MissingTile)? as u64,
                tiles[t3].ok_or(TMErr::MissingTile)? as u64,
            );
            if (f1 ^ f2 ^ f3) & 0b110 != 0b110 {
                return Err(TMErr::InvalidPiece); // sanity
            }
            // corner may still be wrong if 2-cycle in above faces thus test y_axis consistancty
            let corner =
                ((f1 & 1) << (f1 >> 1)) | ((f2 & 1) << (f2 >> 1)) | ((f3 & 1) << (f3 >> 1));
            let (mut twist, y_axis) = if (f1 & 0b110) == 0 {
                (Twist::C1, f3)
            } else if (f2 & 0b110) == 0 {
                (Twist::Identity, f1)
            } else if (f3 & 0b110) == 0 {
                (Twist::Cw, f2)
            } else {
                return Err(TMErr::InvalidPiece); // sanity
            };

            if (((0b01101001_00) >> ((pos as u64) ^ (corner as u64))) ^ y_axis) & 0b100 != 0 {
                return Err(TMErr::InvalidPiece); // sanity
            }

            let a = pos as u64;
            if (a ^ (a >> 1) ^ (a >> 2)) & 0b1 == 1 {
                twist = twist.inverse();
            }
            raw |= (twist as u64) << (corner as u32 * 8 + 3);
            raw |= (pos as u64) << (corner as u32 * 8);
        }
        CornerMap::from_raw(raw).map_err(|err| TMErr::Corner(err))
    }

#[doc(hidden)]
    pub fn store_identity_centers(&mut self) {
        for face in Face::faces() {
            self.map[face as usize][4] = Some(face);
        }
    }
#[doc(hidden)]
    pub fn store_centers(&mut self, centers: CenterMap) {
        for face in Face::faces() {
            self.map[centers.get(face) as usize][4] = Some(face);
        }
    }
#[doc(hidden)]
    pub fn store_edges(&mut self, edges: EdgeMap, tile_mask:u64) {
        let fm: &mut [Option<Face>; 54] = unsafe { std::mem::transmute(&mut self.map) };
        let mut tmask = (tile_mask & 0xf0f0_f0f0) | ((tile_mask >> 36) & 0x0f0f_0f0f);
        for (edge, (pos, flipped)) in edges.iter() {
            let (af, bf) = edge.faces();
            let mut a = if tmask & 1 == 0 {Some(af)} else {None};
            tmask >>= 1;
            let mut b = if tmask & 1 == 0 {Some(bf)} else {None};
            tmask >>= 1;
            let position_parity = ((edge as u8 ^ pos as u8) >> 2) & 1;
            if position_parity != (flipped as u8) {
                std::mem::swap(&mut a, &mut b);
            }
            let (a_index, b_index) = tiles_from_edge(pos);
            fm[a_index as usize] = a;
            fm[b_index as usize] = b;
        }
    }
#[doc(hidden)]
    pub fn store_corners(&mut self, cm: CornerMap, tile_mask:u64) {
        let mut xm = 0x05050505_04040404;
        let mut zm = 0x03030202_03030202;
        let mut ym = 0x01000100_01000100;
        {
            let xm_sel = 0x01010101_01010101u64;
            xm |= (tile_mask & xm_sel) * 7;
            ym |= ((tile_mask>>1) & xm_sel) * 7;
            zm |= ((tile_mask>>2) & xm_sel) * 7;
        }
        // for i in 0..8 {
        //     tk
        // }

        // let mut xm = 0x5050505_04040404;
        // let mut zm = 0x3030202_03030202;
        // let ym = 0x01000100_01000100;
        let pm = 0x08000008_00080800;
        const MASK: u64 = 0x08080808_08080808;

        let pfilter = |x| (MASK - x) | x;

        let parity = (!((cm.raw << 3) ^ (cm.raw << 2) ^ (cm.raw << 1))) & MASK;

        let b_flip = (!pfilter((parity ^ pm) >> 3)) & (xm ^ zm); // adjust side facelet
        xm ^= b_flip;
        zm ^= b_flip;

        let mnx = (cm.raw | (cm.raw >> 1)) & parity;
        let cm = cm.raw ^ mnx ^ (mnx << 1);

        let cw = pfilter((cm & MASK) >> 3);
        let ccw = pfilter(((cm >> 1) & MASK) >> 3);
        let nil = !(cw | ccw);

        let xmj = nil & xm | ccw & zm | cw & ym; // apply orientation
        let ymj = nil & ym | ccw & xm | cw & zm;
        let zmj = nil & zm | ccw & ym | cw & xm;
        let fm: &mut [Option<Face>; 54] = unsafe { std::mem::transmute(&mut self.map) };
        let fnx = |a| -> Option<Face> {
            let k = a as u8;
            if k > 5 {
                None
            } else {
                Some(unsafe { std::mem::transmute(k) })
            }
        };
        for _i in 0..8 {
            let i = _i * 8;
            use Corner::*;
            let (xp, yp, zp) = match Corner::from((cm >> i) as u8) {
                ULF => (47, 6, 18),
                DLF => (53, 9, 24),
                ULB => (45, 0, 29),
                URF => (36, 8, 20),
                URB => (38, 2, 27),
                DLB => (51, 15, 35),
                DRF => (42, 11, 26),
                DRB => (44, 17, 33),
            };
            fm[xp] =fnx(xmj >> i);
            fm[yp] =fnx(ymj >> i);
            fm[zp] =fnx(zmj >> i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn center_extraction() {
        for rotation_cube in crate::moves::ROTATION_TABLE {
            let tm: TileMap = rotation_cube.centers().into();
            assert_eq!(tm.centers(), Ok(rotation_cube.centers()));
        }
    }
    #[test]
    fn corner_extraction() {
        use crate::Move;
        let tilemap: TileMap = (CornerMap::default() * Move::F2).into();
        assert_eq!(tilemap.corners(), Ok(CornerMap::default() * Move::F2));

        let tilemap: TileMap = (CornerMap::default() * Move::F1).into();
        assert_eq!(tilemap.corners(), Ok(CornerMap::default() * Move::F1));

        let cube = Move::F1.corners() * Move::L1 * Move::B1 * Move::U2 * Move::D3;
        let tilemap: TileMap = (cube).into();
        assert_eq!(tilemap.corners(), Ok(cube));
    }
    #[test]
    fn edge_extraction() {
        use crate::Move;
        let tilemap: TileMap = (EdgeMap::default() * Move::R1 * Move::L1).into();
        assert_eq!(
            tilemap.edges(),
            Ok(EdgeMap::default() * Move::R1 * Move::L1)
        )
    }
    #[test]
    fn tile_rotations_conversions() {
        let mut map_table: [TileMap; 24] = Default::default();
        fn assert_equal_filled_tiles(a: &TileMap, b: &TileMap, skip_centers: bool) {
            for (face_a, face_b) in a.map.iter().zip(b.map.iter()) {
                for (index, x) in face_a.iter().zip(face_b.iter()).enumerate() {
                    if index == 4 && skip_centers {
                        continue;
                    } // skip centers
                    if let (Some(face_a), Some(face_b)) = x {
                        assert_eq!(face_a, face_b);
                    }
                }
            }
        }
        for (rotation_cube, entry) in crate::moves::ROTATION_TABLE
            .iter()
            .zip(map_table.iter_mut())
        {
            *entry = TileMap::from(*rotation_cube);
            assert_equal_filled_tiles(entry, &rotation_cube.corners().into(), true);
            assert_equal_filled_tiles(entry, &rotation_cube.centers().into(), false);
            assert_equal_filled_tiles(entry, &rotation_cube.edges().into(), true);

            //rotations cubes should be all solved and specify every piece
            for face in entry.map.iter() {
                let center_tile = face[0].unwrap();
                for tile in face.iter() {
                    assert_eq!(tile.unwrap(), center_tile);
                }
            }
        }
        //All rotations cubes are distinct, the tile maps should be as well
        for x in 0..24usize {
            for y in (x + 1)..24 {
                assert_ne!(&map_table[x], &map_table[y]);
            }
        }
    }
}
