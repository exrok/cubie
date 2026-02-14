use super::db::CubeTable;
use super::db;
use super::util;
use super::cube::CubieCube;
use std::cmp::{max};

#[derive(Clone, Copy,Default)]
#[repr(transparent)]
pub struct Turn(pub u8);

impl Turn {
    pub fn conjugate(self, db: &CubeTable) -> Turn {
        unsafe {Turn(*db.sym_move.get_unchecked(3).get_unchecked(self.0 as usize) )}
    }
}
impl From<UDTurn> for Turn {
    fn from(turn: UDTurn) -> Turn {
        Turn(util::UD2STD[turn.0 as usize ])
    }
}
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct UDTurn(pub u8);

impl UDTurn {
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Clone, Copy,Debug, Default)]
#[repr(transparent)]
pub struct SymCPerm(pub u16);

#[derive(Clone, Copy,Debug, Default)]
#[repr(transparent)]
pub struct SymEPerm(pub u16);

#[derive(Clone, Copy,Debug, Default)]
#[repr(transparent)]
pub struct MPerm(pub u16);

#[derive(Clone, Copy,Debug, Default)]
#[repr(transparent)]
pub struct UDSlice(u16);

#[derive(Clone, Copy,Debug, Default)]
#[repr(transparent)]
pub struct SymFlip(u16);

#[derive(Clone, Copy,Debug, Default)]
#[repr(transparent)]
pub struct SymTwist(u16);

impl MPerm {
    pub fn from(_: &CubeTable, cc: &CubieCube) -> MPerm {
        MPerm(cc.get_m_perm() as u16)
    }

    #[inline]
    pub fn turn_inverse(self, db: &CubeTable, turn: UDTurn) -> MPerm {
        //Dbebuge aser size
        unsafe {
            MPerm(*db.m_perm_inv
                  .get_unchecked(*db.m_perm_move
                                 .get_unchecked(*db.m_perm_inv.get_unchecked(self.0 as usize) as usize)
                                 .get_unchecked(turn.index()) as usize) as u16)
        }
    }
}

impl Coord for SymCPerm {
    const SIZE:usize = 0;
    type TurnSet = UDTurn;
    fn index(self) -> usize {
        self.0 as usize
    }
    #[inline]
    fn turn(self, db: &CubeTable, turn: UDTurn) -> SymCPerm {
        //Dbebuge aser size
        unsafe {
            let sym_turn = *db.sym_move_ud.get_unchecked((self.0 & 0xf) as usize)
                .get_unchecked(turn.index()) as usize;
            let mut raw = *db.c_perm_move.get_unchecked((self.0 >> 4) as usize)
                .get_unchecked(sym_turn);
            raw ^= *db.sym_mult.get_unchecked((raw & 0xf) as usize)
                .get_unchecked((self.0 & 0xf) as usize) as u16;
            SymCPerm(raw)
        }
    }
}

impl Coord for SymEPerm {
    const SIZE:usize = 0;
    type TurnSet = UDTurn;
    fn index(self) -> usize {
        self.0 as usize
    }

    #[inline]
    fn turn(self, db: &CubeTable, turn: UDTurn) -> SymEPerm {
        //Dbebuge aser size
        unsafe {
            let sym_turn = *db.sym_move_ud.get_unchecked((self.0 & 0xf) as usize)
                .get_unchecked(turn.index()) as usize;
            let mut raw = *db.e_perm_move.get_unchecked((self.0 >> 4) as usize)
                .get_unchecked(sym_turn);
            raw ^= *db.sym_mult.get_unchecked((raw & 0xf) as usize)
                .get_unchecked((self.0 & 0xf) as usize) as u16;
            SymEPerm(raw)
        }
    }
}
impl Coord for MPerm {
    const SIZE:usize = 0;
    type TurnSet = UDTurn;
    fn index(self) -> usize {
        self.0 as usize
    }
    #[inline]
    fn turn(self, db: &CubeTable, turn: UDTurn) -> MPerm {
        //Dbebuge aser size
        unsafe {
            MPerm(*db.m_perm_move.get_unchecked(self.0 as usize)
                .get_unchecked(turn.index()) )
        }
    }
}

impl SymEPerm {
    #[inline]
   pub fn inverse(self, db:&CubeTable) -> SymEPerm {
        SymEPerm(db.get_perm_sym_inv(self.0 >> 4, self.0 & 0xf, false))
    }
    pub fn from(db: &CubeTable, cc: &CubieCube) -> SymEPerm {
        SymEPerm(cc.get_e_perm_sym(db))
    }
    // pub fn sym(self) -> u16 {
    //     self.0 &0xf
    // }
    // pub fn raw(self) -> u16 {
    //     self.0 >>4
    // }
}

impl SymCPerm {
    #[inline]
    pub fn inverse(self, db:&CubeTable) -> SymCPerm {
        SymCPerm(db.get_perm_sym_inv(self.0 >> 4, self.0 & 0xf, true))
    }
    pub fn from(db: &CubeTable, cc: &CubieCube) -> SymCPerm {
        SymCPerm(cc.get_c_perm_sym(db) as u16)
    }
    pub fn sym(self) -> u16 {
        self.0 &0xf
    }
    pub fn raw(self) -> u16 {
        self.0 >>4
    }
}

impl SymTwist {
    pub fn from(db: &CubeTable, cc: &CubieCube) -> SymTwist {
        SymTwist(cc.get_twist_sym(db))
    }
    // pub fn sym(self) -> u16 {
    //     self.0 &7
    // }
    // pub fn raw(self) -> u16 {
    //     self.0 >>3
    // }
}

impl SymFlip {
    pub fn from(db: &CubeTable, cc: &CubieCube) -> SymFlip {
        SymFlip(cc.get_flip_sym(db) as u16)
    }
    // pub fn sym(self) -> u16 {
    //     self.0 &7
    // }
    // pub fn raw(self) -> u16 {
    //     self.0 >>3
    // }
}

impl UDSlice {
    pub fn from(_: &CubeTable, cc: &CubieCube) -> UDSlice {
        UDSlice(cc.get_ud_slice() as u16)
    }
}

pub trait Coord {
    const SIZE: usize;
    type TurnSet;
    fn index(self) -> usize; 
    fn turn(self, db: &CubeTable, turn: Self::TurnSet) -> Self; 
}

impl Coord for SymTwist {
    const SIZE:usize = 0;
    type TurnSet = Turn;
    fn index(self) -> usize {
        self.0 as usize
    }
    fn turn(self, db: &CubeTable, turn: Self::TurnSet) -> SymTwist {
        SymTwist(db.twist_sym_move(self.0, turn.0))
    }
}

impl Coord for SymFlip {
    const SIZE:usize = 0;
    type TurnSet = Turn;
    fn index(self) -> usize {
        self.0 as usize
    }
    fn turn(self, db: &CubeTable, turn: Turn) -> SymFlip {
        SymFlip(db.flip_sym_move(self.0, turn.0))
    }
}

impl Coord for UDSlice {
    const SIZE:usize = db::N_SLICE;
    type TurnSet = Turn;
    fn index(self) -> usize {
        self.0 as usize
    }
    fn turn(self, db: &CubeTable, turn: Turn) -> UDSlice {
        unsafe {
            UDSlice(*db.ud_slice_move.get_unchecked(self.index()).get_unchecked(turn.0 as usize))
        }
    }
}

impl PruningValue for CoordTuple<SymTwist,SymFlip> {
    #[inline]
    fn pruning_value(self, db: &CubeTable) -> u32 {
        unsafe {
            let index = ((self.sym.0) ^ (self.base.0 & 7)) as usize;
            let twist_flip_rf =  *db.flip_s2rf.get_unchecked(index) as u32;
            
            db.un_prune_twist_flip(((self.base.0 as u32 >> 3) << 11) | twist_flip_rf)
        }
    }
}

impl PruningValue for CoordTuple<MPerm, SymCPerm> {
    #[inline]
    fn pruning_value(self, db: &CubeTable) -> u32 {
        //nocheckin
        db.prune_cperm((self.sym.raw() as u32) * db::N_MPERM as u32 +
                       db.m_perm_conj[self.base.index()][self.sym.sym() as usize] as u32)
    }
}

impl PruningValue for CoordTuple<SymCPerm, SymEPerm> {
    #[inline]
    fn pruning_value(self, db: &CubeTable) -> u32 {
        //nocheckin
        db.prune_eperm((self.sym.0 >> 4) as u32 * db::N_COMB as u32 + (db.ccomb_p_conj[
            db.perm2combp[(self.base.0 >> 4) as usize] as usize
        ][
            db.sym_mult_inv[(self.sym.0 & 0xf) as usize][(self.base.0 & 0xf) as usize] as usize
        ]) as u32)
    }
}
impl PruningValue for CoordTuple<UDSlice,SymFlip> {
    #[inline]
    fn pruning_value(self, db: &CubeTable) -> u32 {
        db.prune_ud_flip(self.base.0 , self.sym.0 )
    }
}
impl PruningValue for CoordTuple<UDSlice,SymTwist> {
    #[inline]
    fn pruning_value(self, db: &CubeTable) -> u32 {
        db.prune_ud_twist(self.base.0 , self.sym.0 )
    }
}

pub trait PruningValue {
    fn pruning_value(self, db: &CubeTable) -> u32;
}
pub struct CoordTuple<X: Coord, Y: Coord> {
    pub base: X,
    pub sym: Y,
}


#[inline]
pub fn prune_value<X:Coord, Y:Coord>(db: &CubeTable, base: X, sym: Y) -> i8
where CoordTuple<X,Y>: PruningValue {
    CoordTuple{base,sym}.pruning_value(db) as i8
}
#[derive(Clone,Copy,Debug,Default)]
#[repr(C)]
pub struct Phase1Cube {
    pub twist: SymTwist,
    pub twistc: SymTwist,
    pub flip: SymFlip,
    pub flipc: SymFlip,
    pub slice: UDSlice,
    pub mv: u8,
    pub prun: i8,
}
impl Phase1Cube {
    #[inline]
    pub fn p1(&mut self, db: &CubeTable) -> i32 {
        prune_value(db, self.slice, self.flip) as i32
    }

    #[inline]
    pub fn p2(&mut self, db: &CubeTable) -> i32 {
        prune_value(db, self.slice, self.twist) as i32
    }
    #[inline]
    pub fn set_with_prun(&mut self, db: &CubeTable, cc: &CubieCube, depth: u32 ) -> bool {
        let depth = depth as i8;
        self.twist = SymTwist::from(db, cc);
        self.flip = SymFlip::from(db,cc);

        self.prun = prune_value(db, self.twist, self.flip);

        if self.prun > depth { return false; }

        self.slice = UDSlice::from(db, cc);

        self.prun = max(self.prun, max(prune_value(db, self.slice, self.flip),
                                       prune_value(db, self.slice, self.twist)));

        if self.prun > depth { return false; }

        let mut pc = CubieCube::default();
        cc.corn_conjugate(db, 1, &mut pc);
        cc.edge_conjugate(db, 1, &mut pc);

        self.twistc = SymTwist::from(db, &pc);
        self.flipc = SymFlip::from(db, &pc);

        self.prun = max(self.prun, prune_value(db, self.twistc, self.flipc));

        self.prun <= depth
    }
}
