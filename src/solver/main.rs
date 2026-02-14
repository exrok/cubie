#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_upper_case_globals)]
use std::env;
use cubesolver::util::*;
use cubesolver::util;
use cubesolver::db::*;
use cubesolver::cube::*;
use cubesolver::coord::*;
use cubesolver::search::*;
use oorandom::Rand32;
//U,R2,F2,D,L2,B2
fn rand_cube(db:&CubeTable, rng: &mut Rand32) -> CubieCube {
    let mut n = CubieCube::default();
    for _ in 0..101 {
        n = n.turn(&db, (rng.rand_u32() % 18) as u8);
    }
    n
}

fn rand_p2_cube(db:&CubeTable, rng: &mut Rand32) -> CubieCube {
    let mut n = CubieCube::default();
    for _ in 0..100 {
        n = n.turn(&db, util::ud2std[(rng.rand_u32() % 10) as usize]);
    }
    n
}
fn xxx(l:u32) -> u32 {
    (l>> 1) + (0b11 &(0b10011010010110_00_0101_010110_00_0101_u32 >> (l << 1)))
}
fn invert_move(t: u8) -> u8 {
    let k = t % 3;
    if k == 1 {
        t 
    } else if k==0 {
        t+2
    } else {
        t-2
    }
}
fn conj(db: &CubeTable,cube: CubieCube, turn: u8) -> CubieCube{
    let mut out = db.move_cube[turn as usize];
    out.edges = out.edges.mult(&cube.edges);
    out.corners = out.corners.mult(cube.corners);
    out.turn(db, invert_move(turn))
}

fn min_conj_order(db: &CubeTable, cube: CubieCube) {
    let mut min_order = 10000;
    let mut combo = (0,0);
    for t in 0..18 {
        let (o1,o2,x) = cube.turn(&db,t).order();
        let mk = std::cmp::max(o1,o2);
        if mk < min_order {
            combo = (99,t);
            min_order =mk;
            eprintln!("{:?} : {}",combo,mk)
        }
    }
    for ct in 0..18 {
        let cube_conj = conj(db,cube,ct);
        for t in 0..18 {
            let (o1,o2,x) = cube_conj.turn(&db,t).order();
            let mk = std::cmp::max(o1,o2);
            if mk <= min_order {
                combo = (ct,t);
                min_order =mk;
                eprintln!("{:?} : {}",combo,mk)
            }
        }
    }

} 
use cubesolver::corner::{FTurn,CornerSet};
use cubesolver::edge::{EdgeSet};
fn extract_sym(turns: &[FTurn]) {
    let mut mask = 0b1111_1111;
    let base = CornerSet::default();
    let get = |base: &CornerSet, a:u32| {
        (base.set.get() >> (a*5)) & 0b111
    };
    for turn in turns {
        let after = base.apply(*turn);

        let mut atlus_a = [0,0,0,0,0,0,0,0];
        for i in 0..8 {
            atlus_a[(get(&base,i) & 0b111) as usize] = i ;//| (at(i) as u8 & 0b11000);
        }
        let mut atlus_b = [0,0,0,0,0,0,0,0];
        for i in 0..8 {
            atlus_b[(get(&after,i) & 0b111) as usize] = i ;//| (at(i) as u8 & 0b11000);
        }
        let mut cmask = 0;
        for i in 0..8 {
            let x = atlus_a[i]; 
            let y = atlus_b[i]; 
            if x != y {
	            cmask |= 1 << i;
            }
        }
        mask &= cmask;
    }
    for i in 0..8 {
        if (1 << i) & mask != 0 {
            eprintln!("BIT{:?}: HAS:{}",turns, i);
        }
    }

}
fn extract_syme(turns: &[FTurn]) {
    let mut mask:u32 = 0b1111_1111_1111;
    let base = EdgeSet::default();
    let get = |base: &EdgeSet, a:u32| {
        (base.set >> (a*5)) & 0b1111
    };
    for turn in turns {
        let after = base.apply(*turn);

        let mut atlus_a = [0,0,0,0,0,0,0,0,0,0,0,0];
        for i in 0..12 {
            atlus_a[(get(&base,i) & 0b1111) as usize] = i ;//| (at(i) as u8 & 0b11000);
        }
        let mut atlus_b = [0,0,0,0,0,0,0,0,0,0,0,0];
        for i in 0..12 {
            atlus_b[(get(&after,i) & 0b1111) as usize] = i ;//| (at(i) as u8 & 0b11000);
        }
        let mut cmask = 0;
        for i in 0..12 {
            let x = atlus_a[i]; 
            let y = atlus_b[i]; 
            if x != y {
	            cmask |= 1 << i;
            }
        }
        mask &= cmask;
    }
    if mask.count_ones() == 1 {
    for i in 0..12 {
        if (1 << i) & mask != 0 {
            eprintln!("BIT{:?}: HAS:{}",turns, i);
        }
    }
    }

}

fn extract_syme_m2p(db: &CubeTable, turns: &[FTurn]) {
    let mut mask:u32 = 0b1111_1111_1111;
    let base = CubieCube::default();
    let get = |base: &CubieCube, a:u32| {
        base.edges.raw[a as usize]
    };
    for turn in turns {
        let after = base.turn(db,*turn as u8);
        let mut cmask = 0;
        for i in 0..12 {
            let x = get(&base, i); 
            let y = get(&after, i); 
            if x != y {
	            cmask |= 1 << i;
            }
        }
        mask &= cmask;
    }

    if mask.count_ones() == 1 {
        for i in 0..12 {
            if (1 << i) & mask != 0 {
                eprintln!("M2P{:?}: HAS:{}\n",turns, i);
            }
    }
    }

}
fn extract_sym_m2p(db: &CubeTable, turns: &[FTurn]) {
    let mut mask = 0b1111_1111;
    let base = CubieCube::default();
    let get = |base: &CubieCube, a:u32| {
        base.corners.raw[a as usize]
    };
    for turn in turns {
        let after = base.turn(db,*turn as u8);
        let mut cmask = 0;
        for i in 0..8 {
            let x = get(&base, i); 
            let y = get(&after, i); 
            if x != y {
	            cmask |= 1 << i;
            }
        }
        mask &= cmask;
    }
    for i in 0..8 {
        if (1 << i) & mask != 0 {
            eprintln!("M2P{:?}: HAS:{}",turns, i);
        }
    }

}

fn conv_e(set: EdgeSet) -> CubieEdges {
    let mut d = CubieEdges::default();
    let at = |a:u8|(set.set >> (a*5))& 0b11111;
    let mut atlus = [0,0,0,0,0,0,0,0,0,0,0,0];
    for i in 0..12 {
        atlus[(at(i) & 0b1111) as usize] = i | (at(i) as u8 & 0b10000);
    }
    // for i in 0..8 {
    //     eprintln!("F({}) := {}",i,at(i));
    //     eprintln!("F'({}) := {}",at(i),atlus[at(i) as usize]);
    // }
    let conv = [2,6,0,4,10,9,8,11,3,5,1,7];
    let mut conv_inv = [0,0,0,0,0,0,0,0,0,0,0,0];
    for i in 0..12 {
        conv_inv[conv[i] as usize] = i;//| (at(i) as u8 & 0b11000);
    }
    for i in 0..12 {
        let bits = atlus[conv_inv[i] as usize] as u8; 
        let loc = conv[(bits & 0b1111) as usize] as u8;
        d.raw[i] = (loc << 1) | (bits >> 4);
    }
    d
} 
fn conv(set: CornerSet) -> CubieCorners {
    let mut d = CubieCorners::default();
        let at = |a:u8|(set.set.get() >> (a*5))& 0b11111;
        let mut atlus = [0,0,0,0,0,0,0,0];
        for i in 0..8 {
            atlus[(at(i) & 0b111) as usize] = i | (at(i) as u8 & 0b11000);
        }
        // for i in 0..8 {
        //     eprintln!("F({}) := {}",i,at(i));
        //     eprintln!("F'({}) := {}",at(i),atlus[at(i) as usize]);
        // }
        let conv = [6,2,5,1,7,3,4,0];
        let mut conv_inv = [0,0,0,0,0,0,0,0];
        for i in 0..8 {
            conv_inv[conv[i] as usize] = i;//| (at(i) as u8 & 0b11000);
        }
        let swapo = |o:u8| { ((o >> 1) & 0b1) | ( (o << 1) & 0b10) };
        for i in 0..8 {
            let bits = atlus[conv_inv[i] as usize] as u8; 
            let loc = conv[(bits & 0b111) as usize] as u8;
            let ori = match (i,bits >> 3) {
                (0,a) => swapo(a),
                (7,a) => swapo(a),
                (5,a) => swapo(a),
                (2,a) => swapo(a),
                (_,ori) => ori
            };
            d.raw[i] = loc | (ori << 3);
        }
    d
} 
fn main() {
    let db = CubeTable::new();
    let mut c1 = CubieCube::default(); 
    let mut c2 = CornerSet::default(); 
    let mut e2 = EdgeSet::default(); 
    let mut rng = Rand32::new(0xdead);
    // for _ in 0..100000 {
    //      // c1 = c1.turn(&db, (rng.rand_u32() % 18) as u8);
    //     let turn = (rng.rand_u32() % 18) as u8;
    //     let turn = unsafe{std::mem::transmute::<u8, FTurn>(turn)};
    //     c2 = c2.apply(turn);
    //     e2 = e2.apply(turn);
    // } 
    // // eprintln!("{}",c2.index_permutation());
    // for (i,v) in db.flip_s2rf.iter().enumerate() {
    //     eprintln!("{} {} {}",i,v, i as i32 - *v as i32);
    // }
    
    // for (i,v) in db.flip_s2rf.iter().enumerate() {
    //     eprintln!("{} {:?}",i,v);
    // }
    for _ in 0..1000 {
        let turn = (rng.rand_u32() % 18) as u8;
        let turn = unsafe{std::mem::transmute::<u8, FTurn>(turn)};
        c1 = c1.turn(&db, turn as u8);
        e2 = e2.apply(turn);
        assert_eq!(c1.edges.raw, conv_e(e2).raw);
    }
//     for _ in 0..1000 {
//         let turn = (rng.rand_u32() % 18) as u8;
//         let turn = unsafe{std::mem::transmute::<u8, FTurn>(turn)};
//         c1 = c1.turn(&db, turn as u8);
//         c2 = c2.apply(turn);
//         assert_eq!(c1.corners.raw, conv(c2).raw);
//         c1.corners.print();
//     }

// eprintln!("======");
    c1.corners.print();
    c2.print_conv();
    for i in 0..5 {
        for j in i..6 {
            let turn1 = unsafe{std::mem::transmute::<u8, FTurn>(i*3 + 1)};
            let turn2 = unsafe{std::mem::transmute::<u8, FTurn>(j*3 + 1)};
            extract_syme(&[turn1,turn2]);
            extract_syme_m2p(&db,&[turn1,turn2]);
        }

    }
    // eprintln!("");
    // extract_sym(&[FTurn::D2,FTurn::B2,FTurn::L2]);
    // extract_sym_m2p(&db,&[FTurn::D2,FTurn::B2,FTurn::L2]);
    // eprintln!("");
    // extract_sym(&[FTurn::D2,FTurn::F2,FTurn::R2]);
    // extract_sym_m2p(&db,&[FTurn::D2,FTurn::F2,FTurn::R2]);
    // eprintln!("");
    // extract_sym(&[FTurn::D2,FTurn::F2,FTurn::L2]);
    // extract_sym_m2p(&db,&[FTurn::D2,FTurn::F2,FTurn::L2]);

    // eprintln!("");
    // extract_sym(&[FTurn::U2,FTurn::B2,FTurn::R2]);
    // extract_sym_m2p(&db,&[FTurn::U2,FTurn::B2,FTurn::R2]);
    // eprintln!("");
    // extract_sym(&[FTurn::U2,FTurn::B2,FTurn::L2]);
    // extract_sym_m2p(&db,&[FTurn::U2,FTurn::B2,FTurn::L2]);
    // eprintln!("");
    // extract_sym(&[FTurn::U2,FTurn::F2,FTurn::R2]);
    // extract_sym_m2p(&db,&[FTurn::U2,FTurn::F2,FTurn::R2]);
    // eprintln!("");
    // extract_sym(&[FTurn::U2,FTurn::F2,FTurn::L2]);
    // extract_sym_m2p(&db,&[FTurn::U2,FTurn::F2,FTurn::L2]);
}
// fn main() {
//     let seed:u64 = if let Some(var) = env::args().skip(1).next() {
//         var.parse().unwrap_or(0xdead)
//     } else {
//         0xdead
//     };
//     let db = CubeTable::new();
//     let mut rng = Rand32::new(seed);
//     let mut solver = Search::default();
//     for i in 0..10 {
//         eprintln!("{:04b}: {} : {} {}",i,i,(((0x42 >> i) & 3) & 0) as i32,(((0x42 >> i) & 3) & -1i32) as i32);
//     }
//     eprintln!("FUCKING_SIZE:{:?}", std::mem::size_of::<Search>());
//     eprintln!("FUCKING_SIZE:{:?}", std::mem::size_of::<cubesolver::solution::Solution>());
//     let now = std::time::Instant::now();
//     let mut sz = 0;
//     eprintln!("{}", N_COMB * N_PERM_SYM);
//     for i in 0..2000 {
//         let mut rr = rand_cube(&db, &mut rng);
//         let mvs = solver.solve_cc(&db, rr, 20, 10000, 0, 0).unwrap();
//         assert!(mvs.len() > 0);
//         sz += mvs.len();
//         // for mv in mvs {
//         //     rr = rr.turn(&db, mv);
//         // }
//         // assert!(rr.is_solved());
//     }

//     // eprintln!("{}", N_COMB * N_PERM_SYM);
//     // let mut times = Vec::<u64>::with_capacity(1000000);
//     // for i in 0..1000 {
//     //     let mut rr = rand_cube(&db, &mut rng);
//     //     let now = std::time::Instant::now();
//     //     let mvs = solver.solve_cc(&db, rr, 20, 100000, 0, 0).unwrap();
//     //     assert!(mvs.len() > 0);
//     //     let v = (now.elapsed().as_secs_f64()*10000.0) as u64;
//     //     if v > 100{
//     //         eprintln!("{},SELF_SYM:{}",v,rr.self_symmetry(&db));
//     //     }
//     //     times.push(v);
//     //     // for mv in mvs {
//     //     //     rr = rr.turn(&db, mv);
//     //     // }
//     //     // assert!(rr.is_solved());
//     // }
//     // times.sort_unstable();
//     // for i in 0..1000 {
//     //     let r = rand_cube(&db, &mut rng);
//     //     let mvs = solver.solve_cc(&db, r, 21, 100000, 0, 0).unwrap();
//     //     let mut rr = r;
//     //     eprintln!("URF: {}",solver.urf_idx);
//     //     eprintln!("mvoes: {:?}",solver.pre_move_len);
//     //     eprintln!("mvoes: {:?}",mvs);
//     //     solver.solution.urf_idx =0;
//     //     eprintln!("mvoes: {:?}",solver.solution.to_vec());
//     //     for mv in &mvs {
//     //         rr = rr.turn(&db, *mv);
//     //         let mut p1c = Phase1Cube::from(&db, &rr);
//     //         p1c.update_pruning(&db);
//     //         eprintln!("P:{:?}", p1c.prun);
//     //     }
//     //     assert!(rr.is_solved());
//     // // }
//     println!("21 move target PHASE1+PHASE2: {}ms", now.elapsed().as_secs_f64()*1000.0/2000.0);
//     eprintln!("AVG_LEN: {:?}",sz as f64/2000.0);
//     eprintln!("COUNTER: {}",solver.cnt);
//     //    // eprintln!("{:?}",cc);



// }
