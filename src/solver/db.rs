pub const N_MOVES: usize = 18;
pub const N_MOVES2: usize = 10;
pub const N_SLICE: usize = 495;
pub const N_TWIST: usize = 2187;
pub const N_TWIST_SYM: usize = 324;
pub const N_FLIP: usize = 2048;
pub const N_FLIP_SYM: usize = 336;
pub const N_PERM: usize = 40320;
pub const N_PERM_SYM: usize = 2768;
pub const N_MPERM: usize = 24;
pub const N_COMB: usize = 140; //Search.USE_COMBP_PRUN
pub const P2_PARITY_MOVE: usize = 0xA5; //Search.USE_COMBP_PRUN

use std::mem::swap;

use super::coord::*;
use super::cube::*;
use super::util;

macro_rules! unsafe_index {
    ( $k:ident [ $y:expr ][$z:expr]) => {
        unsafe {
            debug_assert!($k.len() > ($y as usize));
            debug_assert!($k[($y) as usize].len() > ($z as usize));
            $k.get_unchecked(($y) as usize).get_unchecked(($z) as usize)
            //&$k[($y) as usize][($z) as usize]
        }
    };
    ( $k:ident [ $y:expr ]) => {
        unsafe {
            debug_assert!($k.len() > ($y as usize));
            $k.get_unchecked(($y) as usize)
        }
    };
}
fn fill_64xmask4(mask: u64) -> u64 {
    (((0x7777_7777_7777_7777 & mask) + 0x11111111_11111111) & mask) & 0x88888888_88888888
}

/// Precomputed pruning and move tables used by the two-phase solver.
///
/// Contains symmetry-reduced coordinate move tables and pruning tables
/// for both phases of the Kociemba algorithm. Constructed via
/// [`CubeTableEventedBuilder`] or [`CubeTable::new`].
pub struct CubeTable {
    //PHASE 1
    pub ud_slice_twist_prun: [u64; (N_SLICE * N_TWIST_SYM) / 16 + 1],
    pub ud_slice_flip_prun: [u64; (N_SLICE * N_FLIP_SYM) / 16 + 1],
    pub mc_perm_prun: [u64; N_MPERM * N_PERM_SYM / 16 + 1],
    pub e_permccomb_prun: [u64; N_COMB * N_PERM_SYM / 16 + 1],
    pub twist_flip_prun: [u64; (N_FLIP * N_TWIST_SYM) / 16 + 1], //Search.USE_COMBP_PRUN
    pub cube_sym: [CubieCube; 16],
    pub move_cube: [CubieCube; 18],
    pub ud_slice_conj: [[u16; 8]; N_SLICE],
    pub ud_slice_move: [[u16; N_MOVES]; N_SLICE],
    pub twist_move: [[u16; N_MOVES]; N_TWIST_SYM],
    pub flip_move: [[u16; N_MOVES]; N_FLIP_SYM],
    pub c_perm_move: [[u16; N_MOVES2]; N_PERM_SYM],
    pub e_perm_move: [[u16; N_MOVES2]; N_PERM_SYM],
    pub m_perm_move: [[u16; N_MOVES2]; N_MPERM],
    pub m_perm_conj: [[u16; 16]; N_MPERM],
    pub ccomb_p_move: [[u16; N_MOVES2]; N_COMB],
    pub ccomb_p_conj: [[u16; 16]; N_COMB],
    pub move_cube_sym: [u64; 18],
    pub first_move_sym: [u32; 48],
    pub sym_mult: [[u8; 16]; 16],
    pub sym_mult_inv: [[u8; 16]; 16],
    pub sym_move: [[u8; 18]; 16],
    pub sym_8move: [u8; 8 * 18],
    pub sym_move_ud: [[u8; 18]; 16],
    pub flip_s2r: [u16; N_FLIP_SYM],
    pub twist_s2r: [u16; N_TWIST_SYM],
    pub eperm_s2r: [u16; N_PERM_SYM],
    pub perm2combp: [u8; N_PERM_SYM],
    pub perm_inv_edge_sym: [u16; N_PERM_SYM],
    pub m_perm_inv: [u8; N_MPERM],
    pub flip_r2s: [u16; N_FLIP],
    pub twist_r2s: [u16; N_TWIST],
    pub flip_s2rf: [u16; N_FLIP_SYM * 8],
    pub eperm_r2s: [u16; N_PERM],
    pub sym_state_twist: [u16; N_TWIST_SYM],
    pub sym_state_flip: [u16; N_FLIP_SYM],
    pub sym_state_perm: [u16; N_PERM_SYM],
}

impl CubeTable {
    #[inline]
    pub fn flip_sym_move(&self, flip: u16, mv: u8) -> u16 {
        // eprintln!("mv: {}, tsym: {}", mv ,  twist & 3);
        // eprintln!("sym_8({})", ((mv<< 3) | (twist & 3) as u32));
        // eprintln!("TWIST_AXIS({},{})", (twist >> 3) as usize,self.sym_8move[((mv<< 3) | (twist & 7) as u32) as usize]);
        let mx = (mv << 3) as u16 | (flip & 7);
        unsafe {
            let x = *self
                .flip_move
                .get_unchecked((flip >> 3) as usize)
                .get_unchecked(*self.sym_8move.get_unchecked(mx as usize) as usize);
            x ^ (flip & 7)
        }
    }
    #[inline]
    pub fn twist_sym_move(&self, twist: u16, mv: u8) -> u16 {
        // eprintln!("mv: {}, tsym: {}", mv ,  twist & 3);
        // eprintln!("sym_8({})", ((mv<< 3) as u16 | (twist & 3) as u16));
        // eprintln!("TWIST_AXIS({},{})", (twist >> 3) as usize,self.sym_8move[((mv<< 3) as u32 | (twist & 7) as u32) as usize]);
        let mx = (mv << 3) as u16 | (twist & 7);
        unsafe {
            let x = *self
                .twist_move
                .get_unchecked((twist >> 3) as usize)
                .get_unchecked(*self.sym_8move.get_unchecked(mx as usize) as usize);
            x ^ (twist & 7)
        }
        // eprintln!("TWIST_GOT: {}", x);
    }
    #[inline]
    pub fn prune_ud_twist(&self, slice: u16, twist: u16) -> u32 {
        let mindex: usize = ((twist >> 3) as usize) * N_SLICE
            + self.ud_slice_conj[slice as usize][(twist & 7) as usize] as usize;
        ((self.ud_slice_twist_prun[mindex >> 4] >> ((mindex & 0xf) << 2)) & 0xf) as u32
    }
    #[inline]
    pub fn prune_ud_flip(&self, slice: u16, flip: u16) -> u32 {
        let mindex: usize = ((flip >> 3) as usize) * N_SLICE
            + self.ud_slice_conj[slice as usize][(flip & 7) as usize] as usize;
        ((self.ud_slice_flip_prun[mindex >> 4] >> ((mindex & 0xf) << 2)) & 0xf) as u32
    }

    #[inline]
    pub fn un_prune_twist_flip(&self, index: u32) -> u32 {
        let mindex: usize = index as usize;
        unsafe {
            ((*self.twist_flip_prun.get_unchecked(mindex >> 4) >> ((mindex & 0xf) << 2)) & 0xf)
                as u32
        }
    }

    #[inline]
    pub fn prune_eperm(&self, index: u32) -> u32 {
        let mindex: usize = index as usize;
        ((self.e_permccomb_prun[mindex >> 4] >> ((mindex & 0xf) << 2)) & 0xf) as u32
    }

    #[inline]
    pub fn prune_cperm(&self, index: u32) -> u32 {
        let mindex: usize = index as usize;
        ((self.mc_perm_prun[mindex >> 4] >> ((mindex & 0xf) << 2)) & 0xf) as u32
    }
}

struct PtableIter {
    sel_mask: u64,
    bh: u64,
    index: usize,
}
impl PtableIter {
    fn next(&mut self, ptable: &[u64]) -> Option<u32> {
        while self.bh == 0 {
            self.index += 1;
            if self.index < ptable.len() {
                let zfnd = unsafe { ptable.get_unchecked(self.index) } ^ self.sel_mask;
                self.bh = fill_64xmask4(zfnd);
            } else {
                return None;
            }
        }
        let i = (self.bh.trailing_zeros() >> 2) + (self.index as u32 * 16);
        self.bh &= self.bh - 1;
        Some(i)
    }
    fn new(ptable: &[u64], select: u32) -> PtableIter {
        let sel_mask = !((select as u64) * 0x11111111_11111111);
        PtableIter {
            bh: fill_64xmask4(ptable[0] ^ sel_mask),
            index: 0,
            sel_mask,
        }
    }
}

fn fill_64xmask4_addtion(mask: u64) -> u64 {
    ((((0x7777_7777_7777_7777 & mask) + 0x11111111_11111111) & mask) >> 3) & 0x11111111_11111111
}
//          |   4 bits  |   4 bits  |   4 bits  |  2 bits | 1b |  1b |   4 bits  |
//flags: | MIN_DEPTH | MAX_DEPTH | INV_DEPTH | Padding | P2 | E2C | SYM_shift |
#[inline]
fn init_raw_sym_prun(
    ptable: &mut [u64],
    raw_move: impl Fn(usize, usize) -> u16,
    raw_conj: impl Fn(usize, usize) -> u16,
    sym_move: impl Fn(usize, usize) -> u16,
    sym_state: &[u16],
    raw_len: usize,
    sym_len: usize,
    flags: u32,
) {
    let sub_pruning = |table: &mut [u64], index: u32| unsafe {
        *table.get_unchecked_mut((index >> 4) as usize) -= 1 << ((index & 0xf) << 2);
    };
    let get_pruning = |table: &mut [u64], index: u32| -> u32 {
        unsafe {
            ((table.get_unchecked((index >> 4) as usize) >> ((index & 0xf) << 2)) & 0xf) as u32
        }
    };
    let sym_shift = flags & 0xf;
    let sym_e2c_magic = if ((flags >> 4) & 1) == 1 {
        0x00DDDD00
    } else {
        0x00000000
    }; // togo make sri
    let is_phase2 = ((flags >> 5) & 1) == 1;
    let inv_depth = (flags >> 8) & 0xf;
    let max_depth = (flags >> 12) & 0xf;
    let search_depth = max_depth;
    let sym_mask = (1 << sym_shift) - 1;
    let n_raw = raw_len as u32;
    let n_size = n_raw * sym_len as u32;
    let n_moves_v = if is_phase2 { 10 } else { 18 };
    let blen = (n_size / 16) as usize;
    let mut depth: u32 = get_pruning(ptable, n_size);
    // long tt = System.nanoTime();

    for e in ptable.iter_mut() {
        *e = 0x22222222_22222222;
    }
    ptable[0] -= 2;
    while depth <= inv_depth {
        let mask: u64 = !((depth as u64 + 1) * 0x11111111_11111111u64);
        for e in ptable.iter_mut() {
            *e += fill_64xmask4_addtion(*e ^ mask);
        }
        depth += 1;
        let mut entries = PtableIter::new(ptable, depth - 1);
        while let Some(i) = entries.next(&ptable[..blen]) {
            let (raw, sym) = (i % n_raw, i / n_raw);
            for m in 0..n_moves_v {
                let mut symx = sym_move(sym as usize, m);
                let rawx = raw_conj(
                    raw_move(raw as usize, m) as usize,
                    (symx & sym_mask) as usize,
                );
                symx >>= sym_shift;
                let idx = symx as u32 * n_raw + rawx as u32;
                let prun = get_pruning(ptable, idx);
                if prun != depth + 1 {
                    continue;
                }
                sub_pruning(ptable, idx);
                let mut sym_mask = *unsafe_index!(sym_state[symx]) & !1u16;
                let idxx = symx as u32 * n_raw;
                while sym_mask != 0 {
                    let j = sym_mask.trailing_zeros();
                    sym_mask &= sym_mask - 1;
                    let idxxx = idxx
                        + raw_conj(
                            rawx as usize,
                            (j ^ ((sym_e2c_magic >> (j + j)) & 3)) as usize,
                        ) as u32;
                    if get_pruning(ptable, idxxx) as u32 == (depth + 1) {
                        sub_pruning(ptable, idxxx);
                    }
                }
            }
        }
    }

    while depth < search_depth {
        let mask: u64 = !((depth as u64 + 1) * 0x11111111_11111111u64);
        for e in ptable.iter_mut() {
            *e += fill_64xmask4_addtion(*e ^ mask);
        }
        depth += 1;

        let mut entries = PtableIter::new(ptable, depth + 1);
        while let Some(i) = entries.next(&ptable[..blen]) {
            let (raw, sym) = (i % n_raw, i / n_raw);
            for m in 0..n_moves_v {
                let mut symx = sym_move(sym as usize, m);
                let rawx = raw_conj(
                    raw_move(raw as usize, m) as usize,
                    (symx & sym_mask) as usize,
                );
                symx >>= sym_shift;
                let idx = symx as u32 * n_raw + rawx as u32;
                let prun = get_pruning(ptable, idx);
                if prun != depth - 1 {
                    continue;
                }
                sub_pruning(ptable, i);
                break;
            }
        }
    }
    // System.out.println(String.format("%2d%10d%10f", depth, done, (System.nanoTime() - tt) / 1e6d));
}
/// Incrementally builds a [`CubeTable`] one step at a time.
///
/// There are 7 initialization steps. Each call to [`next`](CubeTableEventedBuilder::next)
/// performs one step and returns the number of steps remaining.
/// This allows callers to show progress or yield between steps.
pub struct CubeTableEventedBuilder {
    pub table: Box<CubeTable>,
    pub remaining: i32,
}
impl Default for CubeTableEventedBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CubeTableEventedBuilder {
    /// Creates a new builder with uninitialized tables.
    pub fn new() -> CubeTableEventedBuilder {
        CubeTableEventedBuilder {
            table: CubeTable::new_uninit(),
            remaining: 7,
        }
    }

    /// Returns the number of steps remaining after this step (6 down to 0).
    /// Returns 0 when fully initialized or if called again after completion.
    pub fn next(&mut self) -> i32 {
        if self.remaining <= 0 {
            return 0;
        }
        match self.remaining {
            7 => {
                self.table.init_move();
                self.table.init_sym();
                self.table.init_perm_sym_2_raw();
                self.table.init_cperm_move();
                self.table.init_eperm_move();
                self.table.init_mperm_move_conj();
                self.table.init_combpmove_conj();
                self.table.init_flip_sym_2_raw();
            }
            6 => {
                self.table.init_twist_sym_2_raw();
                self.table.init_flip_move();
                self.table.init_twist_move();
                self.table.init_ud_slice_move_conj();
            }
            5 => self.table.init_mc_perm_prun(),
            4 => self.table.init_perm_compb_prun(),
            3 => self.table.init_slice_twist_prun(),
            2 => self.table.init_slice_flip_prun(),
            1 => self.table.init_twist_flip_prun(),
            _ => return 0,
        };
        self.remaining -= 1;
        self.remaining
    }
}

impl CubeTable {
    /// Allocates a zeroed, uninitialized table on the heap.
    ///
    /// # Safety
    ///
    /// The returned table must be fully initialized before use.
    pub fn new_uninit() -> Box<CubeTable> {
        let layout = std::alloc::Layout::new::<CubeTable>();
        let ptr = unsafe { std::alloc::alloc_zeroed(layout) };
        unsafe { Box::from_raw(ptr.cast()) }
    }

    fn init_ud_slice_move_conj(&mut self) {
        let mut c = CubieCube::default();
        let mut d = CubieCube::default();
        for i in 0..N_SLICE {
            c.set_ud_slice(i as u32);
            for jx in 0..N_MOVES / 3 {
                let j = jx * 3;
                c.edge_mult(&self.move_cube[j], &mut d);
                self.ud_slice_move[i][j] = d.get_ud_slice() as u16;
            }
            for jx in 0..8 {
                let j = jx * 2;
                c.edge_conjugate(self, self.sym_mult_inv[0][j] as u32, &mut d);
                self.ud_slice_conj[i][j >> 1] = d.get_ud_slice() as u16;
            }
        }
        for i in 0..N_SLICE {
            for jx in 0..N_MOVES / 3 {
                let j = jx * 3;
                let mut udslice = self.ud_slice_move[i][j];
                for k in 1..3 {
                    udslice = self.ud_slice_move[udslice as usize][j];
                    self.ud_slice_move[i][j + k] = udslice;
                }
                // c.edge_mult(&self.move_cube[j], &mut d);
                // self.ud_slice_move[i][j] = d.get_ud_slice() as u16;
            }
        }
    }
    fn init_twist_move(&mut self) {
        let mut c = CubieCube::default();
        let mut d = CubieCube::default();
        for i in 0..N_TWIST_SYM {
            c.set_twist(self.twist_s2r[i] as u32);
            for j in 0..N_MOVES {
                c.corn_mult(&self.move_cube[j], &mut d);
                self.twist_move[i][j] = d.get_twist_sym(self);
            }
        }
    }

    fn init_cperm_move(&mut self) {
        let mut c = CubieCube::default();
        let mut d = CubieCube::default();
        for i in 0..N_PERM_SYM {
            c.set_c_perm(self.eperm_s2r[i] as u32);
            for j in 0..N_MOVES2 {
                c.corn_mult(&self.move_cube[util::UD2STD[j] as usize], &mut d);
                self.c_perm_move[i][j] = d.get_c_perm_sym(self) as u16;
            }
        }
    }

    fn init_eperm_move(&mut self) {
        let mut c = CubieCube::default();
        let mut d = CubieCube::default();
        for i in 0..N_PERM_SYM {
            c.set_e_perm(self.eperm_s2r[i] as u32);
            for j in 0..N_MOVES2 {
                c.edge_mult(&self.move_cube[util::UD2STD[j] as usize], &mut d);
                self.e_perm_move[i][j] = d.get_e_perm_sym(self);
            }
        }
    }
    fn init_flip_move(&mut self) {
        let mut c = CubieCube::default();
        let mut d = CubieCube::default();
        for i in 0..N_FLIP_SYM {
            c.set_flip(self.flip_s2r[i] as u32);
            for j in 0..N_MOVES {
                c.edge_mult(&self.move_cube[j], &mut d);
                self.flip_move[i][j] = d.get_flip_sym(self) as u16;
            }
        }
    }
    fn init_mperm_move_conj(&mut self) {
        let mut c = CubieCube::default();
        let mut d = CubieCube::default();
        for i in 0..N_MPERM {
            c.set_m_perm(i as u32);
            for j in 0..N_MOVES2 {
                c.edge_mult(&self.move_cube[util::UD2STD[j] as usize], &mut d);
                self.m_perm_move[i][j] = d.get_m_perm() as u16;
            }
            for j in 0..16 {
                c.edge_conjugate(self, self.sym_mult_inv[0][j] as u32, &mut d);
                self.m_perm_conj[i][j] = d.get_m_perm() as u16;
            }
        }
    }
    fn init_combpmove_conj(&mut self) {
        let mut c = CubieCube::default();
        let mut d = CubieCube::default();
        for i in 0..N_COMB {
            c.set_c_comb((i as u32) % 70);
            for j in 0..N_MOVES2 {
                c.corn_mult(&self.move_cube[util::UD2STD[j] as usize], &mut d);
                self.ccomb_p_move[i][j] = (d.get_c_comb()
                    + (70 * (((P2_PARITY_MOVE >> j) & 1) ^ (i / 70))) as u32)
                    as u16;
            }
            for j in 0..16 {
                c.corn_conjugate(self, self.sym_mult_inv[0][j] as u32, &mut d);
                self.ccomb_p_conj[i][j] = (d.get_c_comb() + (70 * (i / 70)) as u32) as u16;
            }
        }
    }

    fn init_sym(&mut self) {
        let mut c = CubieCube::default();
        let mut d = CubieCube::default();

        let f2 = CubieCube::from_sym(28783, 0, 259268407, 0);
        let u4 = CubieCube::from_sym(15138, 0, 119765538, 7);
        let mut lr2 = CubieCube::from_sym(5167, 0, 83473207, 0);
        for i in 0..8 {
            lr2.corners.raw[i] |= 3 << 3;
        }
        for i in 0..16 {
            self.cube_sym[i] = c;
            c.corn_mult_full(&u4, &mut d);
            c.edge_mult(&u4, &mut d);
            swap(&mut c, &mut d);
            if i % 4 == 3 {
                c.corn_mult_full(&lr2, &mut d);
                c.edge_mult(&lr2, &mut d);
                swap(&mut c, &mut d);
            }
            if i % 8 == 7 {
                c.corn_mult_full(&f2, &mut d);
                c.edge_mult(&f2, &mut d);
                swap(&mut c, &mut d);
            }
        }
        for i in 0..16 {
            for j in 0..16 {
                self.cube_sym[i].corn_mult_full(&self.cube_sym[j], &mut c);
                for k in 0..16 {
                    if self.cube_sym[k].corners.raw == c.corners.raw {
                        self.sym_mult[i][j] = (i ^ k) as u8;
                        self.sym_mult_inv[k][j] = i as u8;
                    }
                }
            }
        }
        for j in 0..18 {
            for s in 0..16 {
                self.move_cube[j].corn_conjugate(self, self.sym_mult_inv[0][s] as u32, &mut c);
                for m in 0..18 {
                    if self.move_cube[m].corners.raw == c.corners.raw {
                        self.sym_move[s][j] = m as u8;
                        self.sym_move_ud[s][util::STD2UD[j] as usize] = util::STD2UD[m];
                        break;
                    }
                    if m == 17 {
                        eprintln!("BAD{:?}\n", self.move_cube[m].corners.raw);
                        eprintln!("BAD{:?}\n", c.corners.raw);
                    }
                }
                if s % 2 == 0 {
                    self.sym_8move[(j << 3) | (s >> 1)] = self.sym_move[s][j];
                }
            }
        }
        for i in 0..18 {
            self.move_cube_sym[i] = self.move_cube[i].self_symmetry(self);
            let mut j = i;
            for s in 0..48 {
                if (self.sym_move[s % 16][j] as usize) < i {
                    self.first_move_sym[s] |= 1 << i;
                }
                if s % 16 == 15 {
                    j = super::cube::URF_MOVE[2][j] as usize;
                }
            }
        }
    }
    fn init_slice_twist_prun(&mut self) {
        let ud_slice_move = &self.ud_slice_move;
        let ud_slice_conj = &self.ud_slice_conj;
        let twist_move = &self.twist_move;
        init_raw_sym_prun(
            &mut self.ud_slice_twist_prun,
            |a, b| *unsafe_index!(ud_slice_move[a][b]),
            |a, b| *unsafe_index!(ud_slice_conj[a][b]),
            |a, b| *unsafe_index!(twist_move[a][b]),
            &self.sym_state_twist,
            self.ud_slice_move.len(),
            self.twist_move.len(),
            0x69603,
        );
    }

    fn init_mc_perm_prun(&mut self) {
        let m_perm_move = &self.m_perm_move;
        let m_perm_conj = &self.m_perm_conj;
        let c_perm_move = &self.c_perm_move;
        init_raw_sym_prun(
            &mut self.mc_perm_prun,
            |a, b| *unsafe_index!(m_perm_move[a][b]),
            |a, b| *unsafe_index!(m_perm_conj[a][b]),
            |a, b| *unsafe_index!(c_perm_move[a][b]),
            &self.sym_state_perm,
            self.m_perm_move.len(),
            self.c_perm_move.len(),
            0x8ea34,
        );
    }
    fn init_slice_flip_prun(&mut self) {
        let ud_slice_move = &self.ud_slice_move;
        let ud_slice_conj = &self.ud_slice_conj;
        let flip_move = &self.flip_move;
        init_raw_sym_prun(
            &mut self.ud_slice_flip_prun,
            |a, b| *unsafe_index!(ud_slice_move[a][b]),
            |a, b| *unsafe_index!(ud_slice_conj[a][b]),
            |a, b| *unsafe_index!(flip_move[a][b]),
            &self.sym_state_flip,
            self.ud_slice_move.len(),
            self.flip_move.len(),
            0x69603,
        );
    }
    fn init_perm_compb_prun(&mut self) {
        let ccomb_p_move = &self.ccomb_p_move;
        let ccomb_p_conj = &self.ccomb_p_conj;
        let e_perm_move = &self.e_perm_move;
        init_raw_sym_prun(
            &mut self.e_permccomb_prun,
            |a, b| *unsafe_index!(ccomb_p_move[a][b]),
            |a, b| *unsafe_index!(ccomb_p_conj[a][b]),
            |a, b| *unsafe_index!(e_perm_move[a][b]),
            &self.sym_state_perm,
            self.ccomb_p_move.len(),
            self.e_perm_move.len(),
            0x7d824,
        );
    }

    fn init_flip_sym_2_raw(&mut self) {
        let mut c = CubieCube::default();
        let mut d = CubieCube::default();
        //int count = 0, idx = 0;
        let is_edge = true;
        let sym_inc = 2;
        let mut count = 0;
        for i in 0..N_FLIP {
            if self.flip_r2s[i] != 0 {
                continue;
            }
            c.set_flip(i as u32);
            for s1 in 0..8 {
                let s = s1 * 2;
                if is_edge {
                    c.edge_conjugate(self, s, &mut d);
                } else {
                    c.corn_conjugate(self, s, &mut d);
                }
                let idx = d.get_flip();
                self.flip_s2rf[((count << 3) | (s >> 1)) as usize] = idx as u16; //0
                if idx == i as u32 {
                    self.sym_state_flip[count as usize] |= 1 << (s / sym_inc);
                }
                let sym_idx = (count << 4 | s) / sym_inc;
                self.flip_r2s[idx as usize] = sym_idx as u16;
            }
            self.flip_s2r[count as usize] = i as u16;
            count += 1;
        }
    }

    fn init_twist_sym_2_raw(&mut self) {
        let mut c = CubieCube::default();
        let mut d = CubieCube::default();
        //int count = 0, idx = 0;
        let is_edge = false;
        let sym_inc = 2;
        let mut count = 0;
        for i in 0..N_TWIST {
            if self.twist_r2s[i] != 0 {
                continue;
            }
            c.set_twist(i as u32);
            for s1 in 0..8 {
                let s = s1 * 2;
                if is_edge {
                    c.edge_conjugate(self, s, &mut d);
                } else {
                    c.corn_conjugate(self, s, &mut d);
                }
                let idx = d.get_twist();
                if idx == i as u32 {
                    self.sym_state_twist[count as usize] |= 1 << s1;
                }
                let sym_idx = (count << 4 | s) / sym_inc;
                self.twist_r2s[idx as usize] = sym_idx as u16;
            }
            self.twist_s2r[count as usize] = i as u16;
            count += 1;
        }
    }

    fn init_twist_flip_prun(&mut self) {
        let sub_pruning = |table: &mut [u64], index: u32| unsafe {
            *table.get_unchecked_mut((index >> 4) as usize) -= 1 << ((index & 0xf) << 2);
        };
        let get_pruning = |table: &mut [u64], index: u32| -> u64 {
            unsafe { (table.get_unchecked((index >> 4) as usize) >> ((index & 0xf) << 2)) & 0xf }
        };
        let sym_shift = 3;
        let inv_depth = 6;
        let max_depth = 9;
        let search_depth = max_depth;
        let sym_mask = (1 << sym_shift) - 1;
        let n_raw = N_FLIP as u32;
        let n_size = n_raw * self.twist_move.len() as u32;
        let n_moves_v = 18;
        let ptable = &mut self.twist_flip_prun;
        let mut depth: u32 = get_pruning(ptable, n_size) as u32;
        let blen = n_size / 16;

        if depth == 0 {
            for e in ptable.iter_mut() {
                *e = 0x11111111_11111111;
            }
            ptable[0] = 0x11111111_11111110;
            depth = 0;
        } else {
            depth = 0;
        }
        while depth < search_depth {
            let mask: u64 = !((depth as u64 + 1) * 0x11111111_11111111u64);
            for e in ptable.iter_mut() {
                *e += fill_64xmask4_addtion(*e ^ mask);
            }
            let inv = depth > inv_depth;
            let select = if inv {
                (depth + 2) as u64
            } else {
                depth as u64
            };
            let sel_mask = !(select * 0x11111111_11111111);
            let check = if inv { depth } else { depth + 2 };
            depth += 1;

            for index in 0..blen {
                let mut bh =
                    fill_64xmask4(unsafe { ptable.get_unchecked(index as usize) } ^ sel_mask);
                while bh != 0 {
                    let i = ((bh.trailing_zeros() >> 2) + (index << 4)) as u32;
                    bh &= bh - 1;
                    let raw = i % n_raw;
                    let sym = i / n_raw;

                    let mut flip = self.flip_r2s[raw as usize];
                    let fsym = flip & 7;
                    flip >>= 3;
                    for m in 0..n_moves_v {
                        let twist_move = &self.twist_move;
                        let mut symx = *unsafe_index!(twist_move[sym][m]) as u32;
                        let rawx = unsafe {
                            let kk = *self
                                .sym_8move
                                .get_unchecked((m << 3 | fsym as usize) as usize);
                            let vv = &self.flip_move.get_unchecked(flip as usize);
                            let mm = *vv.get_unchecked(kk as usize) as u32;
                            *self
                                .flip_s2rf
                                .get_unchecked((mm ^ (fsym as u32) ^ (symx & sym_mask)) as usize)
                                as u32
                        };
                        symx >>= sym_shift;
                        let idx = symx * n_raw + rawx;
                        let prun = get_pruning(ptable, idx) as u32;
                        if prun != check {
                            continue;
                        }
                        if inv {
                            sub_pruning(ptable, i);
                            break;
                        }
                        sub_pruning(ptable, idx);
                        let mut sym_mask =
                            unsafe { *self.sym_state_twist.get_unchecked(symx as usize) } & !1;
                        while sym_mask != 0 {
                            let j = sym_mask.trailing_zeros();
                            sym_mask &= sym_mask - 1;
                            let mut idxx = symx as u32 * n_raw;
                            idxx += unsafe {
                                let kk = *self.flip_r2s.get_unchecked(rawx as usize) as u32;
                                *self.flip_s2rf.get_unchecked((kk ^ j) as usize) as u32
                            };
                            if get_pruning(ptable, idxx) as u32 == check {
                                sub_pruning(ptable, idxx);
                            }
                        }
                    }
                }
            }
        }
    }
    #[inline]
    pub fn get_perm_sym_inv(&self, idx: u16, sym: u16, is_corner: bool) -> u16 {
        let mut idxi = self.perm_inv_edge_sym[idx as usize] as u32;
        if is_corner {
            idxi = esym2csym(idxi);
        }
        (idxi as u16) ^ self.sym_mult[(idxi & 0xf) as usize][sym as usize] as u16
    }

    pub fn get_skip_moves(&self, mut ssym: u64) -> u32 {
        let mut ret = 0;
        for i in 1.. {
            ssym >>= 1;
            if ssym == 0 {
                break;
            }
            if ssym & 1 == 1 {
                ret |= self.first_move_sym[i];
            }
        }
        ret
    }

    #[inline]
    pub fn depth_bound<X: Coord, Y: Coord>(&self, base: X, sym: Y) -> i32
    where
        CoordTuple<X, Y>: PruningValue,
    {
        CoordTuple { base, sym }.pruning_value(self) as i32
    }
    fn init_perm_sym_2_raw(&mut self) {
        let mut c = CubieCube::default();
        let mut d = CubieCube::default();
        //int count = 0, idx = 0;
        let is_edge = true;
        let sym_inc = 1;
        let mut count = 0;
        for i in 0..N_PERM {
            if self.eperm_r2s[i] != 0 {
                continue;
            }
            c.set_e_perm(i as u32);
            for s in 0..16 {
                if is_edge {
                    c.edge_conjugate(self, s, &mut d);
                } else {
                    c.corn_conjugate(self, s, &mut d);
                }
                let idx = d.get_e_perm();
                if idx == i as u32 {
                    self.sym_state_perm[count as usize] |= 1 << (s / sym_inc);
                }
                let sym_idx = (count << 4 | s) / sym_inc;
                self.eperm_r2s[idx as usize] = sym_idx as u16;
            }
            self.eperm_s2r[count as usize] = i as u16;
            count += 1;
        }

        let mut cc = CubieCube::default();
        for i in 0..N_PERM_SYM {
            cc.set_e_perm(self.eperm_s2r[i] as u32);
            self.perm2combp[i] = (util::get_comb(&cc.edges.raw, 0, true)
                + util::get_parity(self.eperm_s2r[i] as u32, 8) * 70)
                as u8;
            cc = cc.invert();
            self.perm_inv_edge_sym[i] = cc.get_e_perm_sym(self);
        }
        for i in 0..N_MPERM {
            cc.set_m_perm(i as u32);
            cc = cc.invert();
            self.m_perm_inv[i] = cc.get_m_perm() as u8;
        }
    }
    fn init_move(&mut self) {
        self.move_cube[0] = CubieCube::from_sym(15120, 0, 119750400, 0);
        self.move_cube[3] = CubieCube::from_sym(21021, 1494, 323403417, 0);
        self.move_cube[6] = CubieCube::from_sym(8064, 1236, 29441808, 550);
        self.move_cube[9] = CubieCube::from_sym(9, 0, 5880, 0);
        self.move_cube[12] = CubieCube::from_sym(1230, 412, 2949660, 0);
        self.move_cube[15] = CubieCube::from_sym(224, 137, 328552, 137);
        for i in 0..6 {
            let a = 3 * i;
            for p in 0..2 {
                self.move_cube[a + p + 1].corners = self.move_cube[a + p]
                    .corners
                    .mult(self.move_cube[a].corners);
                self.move_cube[a + p + 1].edges =
                    self.move_cube[a + p].edges.mult(&self.move_cube[a].edges);
            }
        }
    }
}

fn esym2csym(idx: u32) -> u32 {
    idx ^ ((0x00DDDD00 >> ((idx & 0xf) << 1)) & 3)
}
