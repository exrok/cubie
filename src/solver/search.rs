use std::cmp::{min,max};
use super::coord::Phase1Cube;
use super::coord::*;
use super::db::CubeTable;

use super::solution::Solution;
use super::util;
use super::cube::CubieCube;

const MAX_PRE_MOVES:u32 = 20;
const MIN_P1LENGTH_PRE:u32 = 7;
const MAX_DEPTH2:u32 = 12;
#[derive(Default)]
pub struct Search {
    conj_mask:u32,
    pub urf_idx: u32,

    length1:u32,
    depth1:u32,

    max_dep2:u32,
    sol_len: u32,
    verbose:u32,
    probe:u64,
    probe_max:u64,
    probe_min:u64,
    self_sym: u64,

    cc: CubieCube,
    urf_cubie: [CubieCube;6],
    phase1_cubie: CubieCube,


    pub pre_move_len: i32,
    max_pre_moves: i32,
        
    pub cnt: usize,
    pub solution: Solution,
    pub moves: [Turn; 31],
    pre_moves: [Turn;MAX_PRE_MOVES as usize],
    is_rec: bool,
    allow_shorter:bool,
}

impl Search {


   pub fn solve_cc(&mut self, db: &CubeTable, cc: CubieCube, max_depth: u32, probe_max: u64, probe_min: u64, verbose: u32) -> Option<Vec<u8>> {
       self.cc = cc;
       self.sol_len = max_depth + 1;
       self.probe = 0;
       self.probe_max = probe_max;
       self.probe_min = probe_min;
       self.verbose = verbose;
       self.solution.len = 0;
       self.is_rec = false;

       self.init_search(db);
       self.search(db)
    }
    fn search(&mut self, db: &CubeTable) -> Option<Vec<u8>> {
        self.length1 = if self.is_rec { self.length1 } else { 0};
        while self.length1 < self.sol_len {
            self.max_dep2 = min(MAX_DEPTH2, self.sol_len - self.length1 - 1);
            if !self.is_rec {
                self.urf_idx = 0;
            }
            while self.urf_idx < 6  {
                if (self.conj_mask & (1 << self.urf_idx)) != 0 {
                    self.urf_idx +=1;
                    continue;
                }
                if self.phase1_pre_moves(db, self.max_pre_moves as u32, -12, self.urf_cubie[self.urf_idx as usize], (self.self_sym & 0xffff) as u32) == 0 {
                    return Some(self.solution.to_vec());
                }
                self.urf_idx+=1;
            }
            self.length1+=1;
        }
        Some(self.solution.to_vec())
    }
    fn init_search(&mut self, db: &CubeTable) {
        self.conj_mask = 0;
        self.self_sym = self.cc.self_symmetry(db);
        if ((self.self_sym >> 16) & 0xffff) != 0 { self.conj_mask |= 0x12;}
        if ((self.self_sym >> 32) & 0xffff) != 0 { self.conj_mask |= 0x24;}
        if ((self.self_sym >> 48) & 0xffff) != 0 { self.conj_mask |= 0x38;}
        self.self_sym &= 0xffffffffffffu64;// NULL OP?

        self.max_pre_moves = if self.conj_mask > 7  { 0} else {MAX_PRE_MOVES as i32};

        for i in 0..6 {
            self.urf_cubie[i] = self.cc;
            self.cc.urf_conjugate();
            if i % 3 == 2 {
                self.cc = self.cc.invert();
            }
        }
    }
    fn phase1_pre_moves(&mut self,db:&CubeTable, maxl: u32, lm: i32, cc: CubieCube,  ssym: u32)  -> i32{
        self.pre_move_len = self.max_pre_moves - maxl as i32;
        let mut node = Phase1Cube::default();
        if if self.is_rec {self.depth1 as i32  == self.length1 as i32 - self.pre_move_len}
        else {self.pre_move_len == 0 || ((0x32FB7 >> lm) & 1) == 0} {
            self.depth1 = self.length1 - self.pre_move_len as u32;
            self.phase1_cubie = cc;
            self.allow_shorter = self.depth1 == MIN_P1LENGTH_PRE && self.pre_move_len != 0;
            let viable = node.set_with_prun(db, &cc, self.depth1);
            if  viable &&
                self.phase1(db, &node, self.depth1, -12) == 0{
                    return 0;
                }
        }
        // if self.urf_idx >= 2 && self.length1 > 0 {
        //    p/anic!("") 
        // }
        // TODO FIX
        if maxl == 0 || self.pre_move_len + MIN_P1LENGTH_PRE as i32 >= self.length1 as i32 {
            return 1;
        }

        let mut skip_moves =db.get_skip_moves(ssym as u64);

        if maxl ==1 || self.pre_move_len + MIN_P1LENGTH_PRE as i32 + 1 >= self.length1 as i32 {
            skip_moves |= 0x36FB7;
        }

        let lmx = lm / 3 * 3;
        // let mut m = 0;
        // while m < 18 {
        //     if (m == lmx || m == lmx - 9 || m == lmx +9) {
        //         m+=3;
        //         continue;
        //     }
            
        //     if  (skip_moves & (1 << m)) != 0 {
        //         m+=1;
        //         continue;
        //     }
        //     db.move_cube[m as usize].corn_mult( &cc, &mut self.pre_move_cubes[maxl as usize]);
        //     db.move_cube[m as usize].edge_mult( &cc, &mut self.pre_move_cubes[maxl as usize]);
        //     self.pre_moves[(self.max_pre_moves - maxl as i32) as usize] = m as u32;
        //     let ret = self.phase1_pre_moves(db,maxl - 1, m, self.pre_move_cubes[maxl as usize].clone(),
        //                                     ssym & db.move_cube_sym[m as usize] as u32);
        //     if ret == 0 {
        //         return 0;
        //     }
        //     m+=1;
        // }
//        eprintln!("{} {}",(18-lmx), lmx);
        let axis_skip = 0b111000000111000000111000000000u32>>(18-lmx);
        skip_moves |= axis_skip;
        let mut move_mask = (!(skip_moves)) & 0x3ffff; // TODO OPTIMIZE SO WE DON"T HAVE TO INVETR"
        while move_mask != 0 {
            let m = move_mask.trailing_zeros() as i32;
            move_mask &= move_mask -1;
            // if self.is_rec && m != self.pre_moves[(self.max_pre_moves - maxl as i32) as usize] as i32 {
            //     continue;
            // }
            let mut next = CubieCube::default();
            db.move_cube[m as usize].corn_mult(&cc, &mut next);
            db.move_cube[m as usize].edge_mult(&cc, &mut next);
            self.pre_moves[(self.max_pre_moves - maxl as i32) as usize] =Turn(m as u8);
            let ret = self.phase1_pre_moves(db,maxl - 1, m, next,
                                            ssym & db.move_cube_sym[m as usize] as u32);
            if ret == 0 {
                return 0;
            }
        }
        1
    }

    fn init_phase2_pre(&mut self, db: &CubeTable) -> u32 {
        self.is_rec = false;
        let probe_bound = if self.solution.len > 0 { self.probe_min } else { self.probe_max};
        if self.probe >= probe_bound {
            return 0;
        }
        self.probe+=1;
        let mut node = self.phase1_cubie;

        for i in 0_usize.. self.depth1 as usize { //todo renable valid optimization
            node.corners = node.corners.mult(db.move_cube[self.moves[i].0 as usize].corners);
            node.edges = node.edges.mult(&db.move_cube[self.moves[i].0 as usize].edges);
        }
        // for i in 0 as usize.. self.depth1 as usize { //todo renable valid optimization
        //     self.phase1_cubie[i+1].corners = self.phase1_cubie[i].corners.mult(
        //         db.move_cube[self.moves[i as usize].0 as usize].corners);
        //     self.phase1_cubie[i+1].edges = self.phase1_cubie[i].edges.mult(
        //         &db.move_cube[self.moves[i].0 as usize].edges);
        // }
//        let node = &self.phase1_cubie[self.depth1 as usize];
        let mut corn = SymCPerm::from(db, &node);
        let mut edge = SymEPerm::from(db, &node);
        let mut mid = MPerm::from(db, &node);
        let mut edge_inv = edge.inverse(db);
        let mut corn_inv = corn.inverse(db);
        let mut ret = 0;

        let _node = Phase2Cube{mperm:mid,eperm:edge, cperm:corn, last_turn: UDTurn(0)};


        //Always

        //mask 0
        //IF,depth 1

        //IF,depth 2
        
        //IF,depth 2



        let last_move = if self.depth1 == 0 { -1 } else  {self.moves[(self.depth1 - 1) as usize].0 as i32};
        let last_pre = if self.pre_move_len == 0 { -1 } else  {self.pre_moves[(self.pre_move_len - 1) as usize].0 as i32};
        let p2switch_max = (if self.pre_move_len == 0 {1} else {2}) * (if self.depth1 == 0 {1} else {2});

        let mut p2switch = 0;
        let mut mask = (1i32 << p2switch_max) - 1;
        while p2switch < p2switch_max {
            if (mask >> p2switch) & 1 != 0 {
                mask &= !( 1 << p2switch);
                ret = self.init_phase2(db,corn, edge, mid, edge_inv, corn_inv);
                if ret == 0 || ret > 2 {
                    break;
                } 
                if ret == 2 {
                    mask &= 0x4 << p2switch; // 0->2; 1=>3; 2=>N/A
                }
            }
            if mask == 0 {
                break;
            }
            
            if (p2switch & 1) == 0 && self.depth1 > 0 {
                let mv = conjugate_move(last_move as u32) as usize; // 

                self.moves[(self.depth1 - 1) as usize] =//TODO: CLEAN UP
                   Turn( (util::UD2STD[mv] << 1) - self.moves[(self.depth1 - 1) as usize].0);

                let turn = UDTurn(mv as u8);
                mid = mid.turn(db,turn);
                corn = corn.turn(db, turn);
                edge = edge.turn(db, turn);

                corn_inv = corn.inverse(db);
                edge_inv = edge.inverse(db);
            }
            else if self.pre_move_len > 0 {
                let mv = conjugate_move(last_pre as u32) as usize;

                self.pre_moves[(self.pre_move_len - 1) as usize] = //TODO: CLEAN UP
                    Turn(util::UD2STD[mv] * 2 - self.pre_moves[(self.pre_move_len - 1) as usize].0);
                let turn = UDTurn(mv as u8);
                mid = mid.turn_inverse(db, turn);
                corn_inv = corn_inv.turn(db, turn);
                edge_inv = edge_inv.turn(db, turn);

                corn = corn_inv.inverse(db);
                edge = edge_inv.inverse(db);
            }

            p2switch +=1;
        }
        if self.depth1 > 0 {
            self.moves[(self.depth1 -1) as usize] = Turn(last_move as u8);
        }

        if self.pre_move_len > 0 {
            self.pre_moves[(self.pre_move_len -1) as usize] = Turn(last_pre as u8);
        }
        
        if ret == 0 {
            0
        } else {
            2
        }
        
    }

    fn init_phase2(&mut self, db: &CubeTable, corn: SymCPerm, edge: SymEPerm, mid: MPerm,
                     edge_inv:SymEPerm, corn_inv:SymCPerm)  -> u32{
        let prun = max(prune_value(db, corn_inv, edge_inv),
                       max(prune_value(db, mid, corn),
                           prune_value(db, corn, edge))) as u32;

        if prun > self.max_dep2 {
            return prun - self.max_dep2;
        }
        let mut depth2 =self.max_dep2 as i32;
        while depth2 >= prun as i32 {
            // let ret = self.phase2(db,(edge.0 >> 4) as u32, (edge.0 &0xf) as u32 ,(corn.0 >> 4) as u32, (corn.0 &0xf) as u32,mid.0 as u32, depth2, self.depth1, 10);
            let node = Phase2Cube{mperm:mid, cperm:corn, eperm:edge, last_turn:UDTurn(10)};
            let ret = self.phase2(db,node, depth2, self.depth1);
            if ret < 0 { break; }
            depth2 -= ret;
            self.sol_len = 0;
            self.solution.len = 0;
            self.solution.set_args(self.verbose,self.urf_idx,self.depth1);
            for i in 0..(self.depth1 +depth2 as u32) as usize {
                self.solution.push(self.moves[i].0 as u32);
            }

            for i in (0..(self.pre_move_len as usize)).rev() {
                self.solution.push(self.pre_moves[i].0 as u32);
            }
            self.sol_len = self.solution.len;
            depth2 -= 1;
        }
        if depth2 != self.max_dep2 as i32 {
            self.max_dep2 = min(MAX_DEPTH2, self.sol_len - self.length1 -1);
            if self.probe >= self.probe_min {
                0
            } else {
                1
            }
        } else {
            1
        }
    }
    fn phase1(&mut self, db: &CubeTable, node: &Phase1Cube, max_depth:u32, lm:i32 ) -> i32 {
        if node.prun == 0 && max_depth < 5 {
            if self.allow_shorter || max_depth == 0 {
                self.depth1 -= max_depth;
                let ret = self.init_phase2_pre(db);//init
                self.depth1 += max_depth;
                return -(ret as i32);
            } else {
                return -1;
            }
        }

        let mut turns = TurnIterator::new(lm); 
        let mut next = Phase1Cube::default();
        
        while let Some(turn) = turns.next() {
            let turn = Turn(turn);
            next.twist = node.twist.turn(db,turn);
            next.twistc = node.twistc.turn(db,turn.conjugate(db));

            next.flip = node.flip.turn(db,turn);
            next.flipc = node.flipc.turn(db,turn.conjugate(db));

            let px1 =max(db.depth_bound(next.twistc, next.flipc),
                         db.depth_bound(next.twist, next.flip));
            let prun = max_depth as i32 - px1 -1;
            if prun == -1 { continue; }
            if prun <= -2 { turns.skip_turn_axis(turn.0); continue;}

            next.slice = node.slice.turn(db,turn);
            let px3 = max(next.p2(db), next.p1(db));
            let prun = max_depth as i32 - px3 -1;
            if prun == -1 { continue; }
            if prun <= -2 { turns.skip_turn_axis(turn.0); continue;}

            next.prun = max(px1,px3) as i8;
            self.moves[(self.depth1 - max_depth) as usize] = turn;
            let axis = turn.0 - (turn.0 % 3);

            let ret = self.phase1(db, &next, max_depth - 1, axis as i32);
            if ret == -1 { continue; }
            if ret <= -2 { turns.skip_turn_axis(turn.0); continue; } 

            return 0;
        }
//         for __axis in 0..6 {
//             let axis = __axis * 3;
//             if axis == lm || axis == lm.wrapping_sub(9) {
//                 continue;
//             }
//             for power in 0..3 {
                
//                 let m = axis + power;

//                 next.apply_move(db, node, m as u8);
//                 next.update_pruning(db);

//                 let maximum_surplus = maxl as i32 - next.prun as i32 - 1;
//                 if maximum_surplus == -1 {continue;}
//                 if maximum_surplus <= -2 {break;}

//                 self.moves[(self.depth1 - maxl) as usize] = Turn(m as u8);
// //                self.valid1 = min(self.valid1, self.depth1 - maxl);

//                 let solve_surplus = self.phase1(db, &next, maxl - 1, axis);
//                 if solve_surplus == -1 {continue;}
//                 if solve_surplus <= -2 {break;}

//                 return 0;
//             }
//         }  
        -1
    }

    fn phase2(&mut self, db: &CubeTable, cube: Phase2Cube, max_depth: i32, depth: u32) -> i32 {
        if cube.is_solved() {
            return max_depth;
        }

        let mut turns = UDTurnIterator::new(cube.last_turn);
        while let Some(turn) = turns.next() {
            let next_cube = cube.turn(db, turn);

            let expected_remaining = max_depth - next_cube.prune_direct(db) - 1;
            if expected_remaining == -1 { continue; }
            if expected_remaining == -2 { turns.skip_turn_axis(turn); continue; }

            let expected_remaining = max_depth - next_cube.prune_inverse(db) - 1;
            if expected_remaining == -1 { continue; }
            if expected_remaining == -2 { turns.skip_turn_axis(turn); continue; }
            if expected_remaining <= -3 { return expected_remaining + 2; }

            let remaining = self.phase2(db, next_cube, max_depth - 1, depth + 1);
            if remaining == -1 { continue; }
            if remaining == -2 { turns.skip_turn_axis(turn); continue; }
            if remaining <= -3 { return remaining + 2;}

            self.moves[depth as usize] = turn.into();
            return remaining;
        }

        -1
    }
    // fn phase2m(&mut selfj db: &CubeTable, node: &Phase2Cube, max_depth: i32, depth_offset: u32) -> i32 {
    //     assert!(max_depth <= MAX_DEPTH2 as i32);
    //     let mut stack: [(Phase2Cube, u16); MAX_DEPTH2 as usize] = Default::default();
    //     let mut depth = 0;

    //     if node.is_solved() {
    //         return max_depth as i32;
    //     }

    //     stack[0] = (*node, node.last_turn.skip_mask());

    //     'dfs: loop {
    //         let (prev_cube, move_mask) = &mut stack[depth]; 
    //         while let Some(turn) = next_turn(move_mask) { 
    //             let mut cube = prev_cube.turn(db, turn);

    //             let surplus = max_depth - (cube.prune(db) + depth as i32);
    //             if surplus == -1 { 
    //                 *move_mask &= !turn.axis_mask(); // Try next axis
    //                 continue;
    //             }
    //             if surplus == 0 { continue; }
    //             if surplus < -1 {
    //                 if surplus == -2 { break; }
    //                 if surplus == -3 {
    //                     if depth == 0 { return -2; }
    //                     depth -= 1;
    //                     stack[depth].1 &= !prev_cube.last_turn.axis_mask();
    //                     continue 'dfs;
    //                 }
    //                 return surplus + 1;
    //             }
    //             depth += 1;
    //             if cube.is_solved() {
    //                 for (depth, (cube, _)) in stack[1..depth].iter().enumerate()  {
    //                     let t:Turn = cube.last_turn.into();
    //                     self.moves[depth_offset as usize + depth ] = t.0 as u32;
    //                     eprintln!("move[{}]={}",depth_offset as usize + depth ,t.0);
    //                 }
    //                 let t:Turn = turn.into();
    //                 self.moves[depth_offset as usize + depth ] = t.0 as u32;
    //                     eprintln!("move[{}]={}",depth_offset as usize + depth ,t.0);
    //                 return max_depth  - depth as i32;
    //             }
    //             stack[depth] = (cube, cube.last_turn.skip_mask());
    //             continue 'dfs;
    //         }

    //         if depth == 0 {
    //             return -1;
    //         }
    //         depth -=1;
    //     }

    // } 
}


fn conjugate_move(l:u32) -> u32 {
    (l>>1) + (0b11 &(0b10011010010110_00_0101_010110_00_0101u32.wrapping_shr(l<<1)))
}

#[derive(Clone,Copy)]
struct Phase2Cube {
    mperm: MPerm,
    eperm: SymEPerm,
    cperm: SymCPerm,
    last_turn: UDTurn,
}

impl Default for Phase2Cube {
    fn default() -> Phase2Cube {
        Phase2Cube{
            mperm : MPerm(0),
            eperm : SymEPerm(0),
            cperm : SymCPerm(0),
            last_turn: UDTurn(0),
        }
    }
}

struct TurnIterator {
    turn_mask: u32,
}

impl TurnIterator {
    fn new(previous_turn: i32) -> TurnIterator {
        TurnIterator{
            turn_mask: (!( 0b000000000111000000111000000000u32>>(18-previous_turn)) & 0x3ffff),
        }
    }
    fn skip_turn_axis(&mut self, turn: u8) {
        self.turn_mask &= !(0b111 << (turn - (turn % 3)));
    }
    fn next(&mut self) -> Option<u8> {
        if self.turn_mask == 0 {
            None
        } else {
            let m = self.turn_mask.trailing_zeros() as u8;
            self.turn_mask &= self.turn_mask -1;
            Some(m)
        }
    }
}
struct UDTurnIterator {
    turn_mask: u16,
}
impl UDTurnIterator {
    fn new(previous_turn: UDTurn) -> UDTurnIterator {
        UDTurnIterator{
            turn_mask:  (!util::CKMV2BIT[previous_turn.0 as usize])&0b11_111_11_111_u16,
        }
    }
    fn skip_turn_axis(&mut self, turn: UDTurn) {
        const SKIP_LUT: &[u8;8] = &[
            0b000_00_111,
            0b000_00_111,
            0b000_00_111,
            0b000_00_000,
            0b000_00_000,
            0b111_00_000,
            0b111_00_000,
            0b111_00_000,
        ];
        let filter = !((SKIP_LUT[(turn.0&0x7) as usize] as i32) as u16);
        self.turn_mask &= filter;
    }
    fn next(&mut self) -> Option<UDTurn> {
        if self.turn_mask == 0 {
            None
        } else {
            let m = self.turn_mask.trailing_zeros() as u8;
            self.turn_mask &= self.turn_mask -1;
            Some(UDTurn(m))
        }
    }
}
impl Phase2Cube {
    #[inline]
    fn turn(self, db: &CubeTable, turn: UDTurn) -> Phase2Cube {
        Phase2Cube{
            mperm : self.mperm.turn(db, turn),
            eperm : self.eperm.turn(db, turn),
            cperm : self.cperm.turn(db, turn),
            last_turn: turn,
        }
    }
    #[inline]
    fn is_solved(self) ->bool {
        self.mperm.0 == 0 && (self.eperm.0 | self.cperm.0) <= 0xf  
    }

    #[inline]
    fn prune_direct(&self,db: &CubeTable) -> i32 {
        prune_value(db, self.mperm, self.cperm) as i32
    }
    #[inline]
    fn prune_inverse(&self,db: &CubeTable) -> i32 {
        let edgei = self.eperm.inverse(db);
        let corni = self.cperm.inverse(db);
        max(prune_value(db, corni, edgei),
            prune_value(db, self.cperm, self.eperm)) as i32
    }
    // #[inline]
    // fn prune(&self,db: &CubeTable) -> i32 {
    //     let edgei = self.eperm.inverse(db);
    //     let corni = self.cperm.inverse(db);
    //     max( prune_value(db, corni, edgei), max(prune_value(db, self.mperm, self.cperm),
    //                        prune_value(db, self.cperm, self.eperm))) as i32
    // }
}
