use super::util;
use super::db::*;

#[derive(Clone,Copy,Debug,Default,Eq,PartialEq)]
pub struct CubieEdges {
    pub raw: [u8;12],
}
impl CubieEdges {
    pub fn mult(&self, other: &CubieEdges) -> CubieEdges  {
        let mut prod = CubieEdges::default();
        for ed in 0..12 {
            prod.raw[ed] = self.raw[(other.raw[ed] >> 1) as usize] ^ (other.raw[ed] & 1) ;
        }
        prod
    }

}

fn esym2csym(idx:u32) -> u32 {
    idx ^ (0x00DDDD00 >> ((idx & 0xf) << 1) & 3)
}
#[derive(Clone,Copy,Debug,Default,Eq,PartialEq)]
pub struct CubieCorners {
    pub raw: [u8;8],
}
impl CubieCorners {
    // pub fn print(self)  {
    //     for i in 0..8 {
    //         eprint!("{:05b}_", (self.raw[i]));
    //     }
    //     eprintln!("");
    //     for i in 0..8 {
    //         eprint!(" {:1}:{:1}  ",i, (self.raw[i])& 0b111);
    //     }
    //     eprintln!("");
    // }
    pub fn mult(self, other: CubieCorners) -> CubieCorners {
        let mut prod = CubieCorners::default();
        for corn in 0..8 {
            let oria = self.raw[(other.raw[corn] & 7) as usize] >> 3;
            let orib = other.raw[corn] >> 3;
            prod.raw[corn] = self.raw[(other.raw[corn] & 7) as usize] & 7 | ((oria + orib) % 3) << 3 ;
        }
        prod
    }
    // pub fn mult_full(self, other: CubieCorners) -> CubieCorners {
    //     let mut prod = CubieCorners::default();
    //     for corn in 0..8 {
    //         let oria = self.raw[(other.raw[corn] & 7) as usize] >> 3;
    //         let orib = other.raw[corn] >> 3;
    //         let mut ori = oria + if oria < 3 { orib } else { 6 - orib};
    //         ori = ori % 3 + if(oria < 3) == (orib < 3) { 0 } else { 3 };
    //         prod.raw[corn] = (self.raw[(other.raw[corn] & 7) as usize] & 7 | ori << 3) as u8;
    //     }
    //     prod
    // }
    // pub fn is_solved(&self) -> bool  {
    //     self.raw == [0, 1, 2, 3, 4, 5, 6, 7] 
    // }

}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub struct CubieCube {
    pub corners: CubieCorners,
    pub edges: CubieEdges,
}


pub const URF1: CubieCube = CubieCube {
    edges: CubieEdges{raw:[3, 16, 11, 18, 7, 22, 15, 20, 1, 9, 13, 5]  },
    corners: CubieCorners{raw:[8, 20, 13, 17, 19, 15, 22, 10] },
};

pub const URF2: CubieCube = CubieCube {
    edges: CubieEdges{raw:[17, 1, 23, 9, 19, 5, 21, 13, 2, 6, 14, 10]  },
    corners: CubieCorners{raw:[16, 11, 23, 12, 9, 18, 14, 21] },
};

pub const URF_MOVE: &[[u8;18];6] = &[
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
    [6, 7, 8, 0, 1, 2, 3, 4, 5, 15, 16, 17, 9, 10, 11, 12, 13, 14],
    [3, 4, 5, 6, 7, 8, 0, 1, 2, 12, 13, 14, 15, 16, 17, 9, 10, 11],
    [2, 1, 0, 5, 4, 3, 8, 7, 6, 11, 10, 9, 14, 13, 12, 17, 16, 15],
    [8, 7, 6, 2, 1, 0, 5, 4, 3, 17, 16, 15, 11, 10, 9, 14, 13, 12],
    [5, 4, 3, 8, 7, 6, 2, 1, 0, 14, 13, 12, 17, 16, 15, 11, 10, 9]
];
impl Default for CubieCube {
   fn default() -> CubieCube{
        CubieCube{
            edges: CubieEdges{raw:[0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22]  },
            corners: CubieCorners{raw:[0, 1, 2, 3, 4, 5, 6, 7] },
        }
    }
}

impl CubieCube {
    pub const fn default()  -> CubieCube{
        CubieCube{
            edges: CubieEdges{raw:[0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22]  },
            corners: CubieCorners{raw:[0, 1, 2, 3, 4, 5, 6, 7] },
        }
    }
    const fn new_zero()  -> CubieCube {
        CubieCube{
            edges: CubieEdges{raw: [0;12] },
            corners: CubieCorners{raw: [0;8] }
        }
    }
    pub fn from_sym( cperm:u32,  twist:u32, eperm:u32, flip:u32) -> CubieCube {
        let mut ret = CubieCube::default();
        ret.set_c_perm(cperm);
        ret.set_twist(twist);
        util::set_perm(&mut ret.edges.raw, eperm, true);
        ret.set_flip(flip);
        ret
    }


    pub fn invert(&self) -> CubieCube{
        let mut inverted = CubieCube::new_zero();
        for edge in 0..12usize {
            inverted.edges.raw[(self.edges.raw[edge] >> 1) as usize] = (edge << 1 | self.edges.raw[edge] as usize & 1) as u8;
        }

        for corn in 0..8usize {
            inverted.corners.raw[(self.corners.raw[corn] & 0x7) as usize] =
                (corn | 0x20 >> (self.corners.raw[corn] >> 3) & 0x18) as u8;
        }
        inverted
    }
    pub fn corn_mult(&self, other: &CubieCube, prod: &mut CubieCube)  {
        for corn in 0..8 {
            let oria = self.corners.raw[(other.corners.raw[corn] & 7) as usize] >> 3;
            let orib = other.corners.raw[corn] >> 3;
            prod.corners.raw[corn] =  self.corners.raw[(other.corners.raw[corn] & 7) as usize] & 7 | ((oria + orib) % 3) << 3 ;
        }
    }
    pub fn corn_mult_full(&self, other: &CubieCube, prod: &mut CubieCube)  {
        for corn in 0..8 {
            let oria = self.corners.raw[(other.corners.raw[corn] & 7) as usize] >> 3;
            let orib = other.corners.raw[corn] >> 3;
            let mut ori = oria + if oria < 3 { orib } else { 6 - orib};
            ori = ori % 3 + if(oria < 3) == (orib < 3) { 0 } else { 3 };
            prod.corners.raw[corn] = self.corners.raw[(other.corners.raw[corn] & 7) as usize] & 7 | ori << 3 ;
        }
    }

   pub fn edge_mult(&self,  other: &CubieCube, prod: &mut CubieCube) {
        for ed in 0..12 {
            prod.edges.raw[ed] = self.edges.raw[(other.edges.raw[ed] >> 1) as usize] ^ (other.edges.raw[ed] & 1) ;
        }
    }

   pub fn corn_conjugate(&self, db: &CubeTable, idx: u32,  b: &mut CubieCube) {
        let sinv = db.cube_sym[db.sym_mult_inv[0][idx as usize] as usize];
        let s = db.cube_sym[idx as usize];
        for corn in 0..8 {
            let oria = sinv.corners.raw[(self.corners.raw[(s.corners.raw[corn] & 7) as usize] & 7) as usize] >> 3;
            let orib = self.corners.raw[(s.corners.raw[corn] & 7) as usize] >> 3;
            let ori = if oria < 3 {  orib  } else {(3 - orib) % 3};
            b.corners.raw[corn] =sinv.corners.raw[(self.corners.raw[(s.corners.raw[corn] & 7) as usize] & 7) as usize ] & 7 | ori << 3 ;
        }
    }
    pub fn edge_conjugate(self,db:&CubeTable, idx:u32, b: &mut CubieCube) {
        let sinv = db.cube_sym[db.sym_mult_inv[0][idx as usize] as usize];
        let s = db.cube_sym[idx as usize];
        for ed in 0..12 {
            b.edges.raw[ed] =sinv.edges.raw[(self.edges.raw[(s.edges.raw[ed] >> 1) as usize] >> 1) as usize] ^
                               (self.edges.raw[(s.edges.raw[ed] >> 1) as usize] & 1) ^ (s.edges.raw[ed] & 1) ;
        }
    }


    pub fn urf_conjugate(&mut self) {
        let mut temp = CubieCube::new_zero();
        URF2.corn_mult(self, &mut temp);
        temp.corn_mult(&URF1,  self);
        URF2.edge_mult(self, &mut temp);
        temp.edge_mult(&URF1,  self);
    }
    pub fn get_flip(&self) -> u32 {
        let mut idx = 0;
        for i in 0..11 {
            idx = (idx << 1) | (self.edges.raw[i] as u32 & 1);
        }
        idx
    }

    pub fn set_flip(&mut self, mut idx:u32) {
        let mut parity = 0;
        for i in (0..11).rev() {
            let val = idx & 1;
            parity ^= val;
            self.edges.raw[i] = (self.edges.raw[i] & !1) | val as u8 ;
            idx >>=1;
        }
        self.edges.raw[11] =self.edges.raw[11] & !1 | parity as u8 ;
    }

    pub fn get_flip_sym(&self, db: &CubeTable) -> u32 {
        db.flip_r2s[self.get_flip() as usize] as u32
    }
    pub fn get_ud_slice(&self) -> u32 {
        494 - util::get_comb(&self.edges.raw, 8, true)
    } 
    pub fn get_twist(&self) -> u32 {
        let mut idx = 0;
        for i in 0..7 {
            idx += (idx << 1) + (self.corners.raw[i] >> 3) as u32;
        }
        idx
    }

    pub fn set_twist(&mut self, mut idx: u32) {
        let mut twst = 15;
        for i in (0..7).rev() {
            let val = idx % 3;
            twst -= val;
            self.corners.raw[i] = (self.corners.raw[i] & 0x7 )| (val << 3) as u8;
            idx /= 3;
        }
        self.corners.raw[7] = (self.corners.raw[7] & 0x7) | ((twst%3) << 3) as u8;
    }

    pub fn get_twist_sym(&self, db: &CubeTable) -> u16 {
        db.twist_r2s[self.get_twist() as usize]
    }

    pub fn set_ud_slice(&mut self, idx: u32)  {
        util::set_comb(&mut self.edges.raw, 494 -idx, 8, true);
    } 

    pub fn  get_c_perm(&self) -> u32 {
        util::get_perm(&self.corners.raw, false)
    }

    pub fn  set_c_perm(&mut self, idx: u32)  {
        util::set_perm(&mut self.corners.raw, idx, false);
    }

    pub fn get_c_perm_sym(&self, db: &CubeTable) -> u32 {
        // eprintln!("cp:{} epr2s: {}",self.get_c_perm(), db.eperm_r2s[self.get_c_perm() as usize]);
        esym2csym(db.eperm_r2s[self.get_c_perm() as usize] as u32)
    }

    pub fn get_e_perm(&self) -> u32 {
        util::get_perm(&self.edges.raw[..8],true)
    }
    pub fn set_e_perm(&mut self, idx:u32) {
        util::set_perm(&mut self.edges.raw[..8],idx,true);
    }

    pub fn get_e_perm_sym(&self, db: &CubeTable) -> u16 {
        db.eperm_r2s[self.get_e_perm() as usize] 
    }
    pub fn get_m_perm(&self) ->u32 {
        //optimize this
        util::get_perm(&self.edges.raw, true) % 24
    }

    pub fn set_m_perm(&mut self, idx:u32) {
        util::set_perm(&mut self.edges.raw[..12],idx,true);
    }
    pub fn get_c_comb(&self) -> u32 {
        util::get_comb(&self.corners.raw, 0, false)
    } 

    pub fn set_c_comb(&mut self, idx: u32)  {
        util::set_comb(&mut self.corners.raw, idx, 0, false);
    } 

    pub fn self_symmetry(&self, db: &CubeTable) -> u64 {
        let mut c = *self;
        let mut d = CubieCube::default();
        let cperm = c.get_c_perm_sym(db) >> 4;
        let mut sym = 0u64;
        for urf_inv in 0..6 {
            let cpermx = c.get_c_perm_sym(db) >> 4;
            if cperm == cpermx {
                for i in 0..16 {
                    c.corn_conjugate(db, db.sym_mult_inv[0][i] as u32, &mut d);
                    if d.corners.raw == self.corners.raw {
                        c.edge_conjugate(db, db.sym_mult_inv[0][i] as  u32, &mut d);
                        if d.edges.raw == self.edges.raw {
                            sym |= 1u64 << std::cmp::min((urf_inv << 4) | i, 48);
                        }
                    }
                }
            }
            c.urf_conjugate();
            if urf_inv % 3 == 2 {
                c = c.invert();
            }
        }
        sym
    }
}
