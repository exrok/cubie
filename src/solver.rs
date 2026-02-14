mod coord;
mod cube;
pub mod db;
mod search;
mod solution;
mod util;
use crate::{CornerMap, EdgeMap, Move};
use cube::CubieCube;
use db::CubeTableEventedBuilder;

/// A two-phase Kociemba solver for the 3x3x3 Rubik's cube.
///
/// Pruning tables are built automatically on the first call to
/// [`Solver::search`]. To build them ahead of time (e.g. to show
/// a progress indicator), use [`Solver::initialize_tables_incremental`].
///
/// # Examples
///
/// ```no_run
/// use cubie::{Cube, Move::*};
///
/// let mut solver = cubie::Solver::default();
/// // Tables are initialized automatically on first search.
/// let scrambled = Cube::default() * R1 * U1 * F2;
/// let solution = solver.search(scrambled);
/// ```
#[derive(Default)]
pub struct Solver {
    db: CubeTableEventedBuilder,
    searcher: search::Search,
}

fn conv_e(set: EdgeMap) -> cube::CubieEdges {
    let mut d = cube::CubieEdges::default();
    let at = |a: u8| (set.raw() >> (a * 5)) & 0b11111;
    let mut atlus = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for i in 0..12 {
        atlus[(at(i) & 0b1111) as usize] = i | (at(i) as u8 & 0b10000);
    }

    let conv = [2, 6, 0, 4, 10, 9, 8, 11, 3, 5, 1, 7];
    let mut conv_inv = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for i in 0..12 {
        conv_inv[conv[i] as usize] = i; //| (at(i) as u8 & 0b11000);
    }
    for i in 0..12 {
        let bits = atlus[conv_inv[i]];
        let loc = conv[(bits & 0b1111) as usize] as u8;
        d.raw[i] = (loc << 1) | (bits >> 4);
    }
    d
}

fn conv(set: CornerMap) -> cube::CubieCorners {
    let mut d = cube::CubieCorners::default();
    use crate::Corner;
    let map = |c| match c {
        Corner::DLB => 6,
        Corner::ULB => 2,
        Corner::DRB => 7,
        Corner::URB => 3,
        Corner::DLF => 5,
        Corner::ULF => 1,
        Corner::DRF => 4,
        Corner::URF => 0,
    };
    for (edge, (pos, twist)) in set.iter() {
        let edge = map(edge);
        let pos = map(pos);
        d.raw[pos as usize] = edge | ((twist.inverse() as u8) << 3);
    }
    d
}

use crate::FaceMove;
fn replace_with_slice(mut turns: impl Iterator<Item = FaceMove>) -> Vec<Move> {
    use crate::moves::{MoveAngle::*, MoveKind};
    let mut out = Vec::<Move>::default();
    let mut prev: Move = if let Some(turn) = turns.next() {
        turn.into()
    } else {
        return out;
    };
    for next in turns {
        let opposite_faces = prev.kind() == MoveKind::Face && prev.face().opposite() == next.face();
        match (opposite_faces, prev.angle(), next.angle()) {
            (true, Two, Two) => prev = Move::new(MoveKind::Slice, prev.face(), Two),
            (true, Cw, Ccw) => prev = Move::new(MoveKind::Slice, next.face(), Cw),
            (true, Ccw, Cw) => prev = Move::new(MoveKind::Slice, next.face(), Ccw),
            _ => {
                out.push(prev);
                prev = next.into();
            }
        }
    }

    out.push(prev);
    let mut rotation = crate::CenterMap::default();
    for mv in &mut out {
        *mv = mv.projection(rotation);
        rotation *= *mv;
    }
    out
}

impl Solver {
    /// Performs one step of pruning table initialization.
    ///
    /// There are 7 initialization steps in total. Returns the number of
    /// steps remaining, or 0 when fully initialized. Calling this after
    /// initialization is complete is a no-op that returns 0.
    ///
    /// This is useful for showing progress during initialization.
    /// Tables are also built automatically by [`Solver::search`], so
    /// calling this is only necessary when incremental control is needed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let mut solver = cubie::Solver::default();
    /// while solver.initialize_tables_incremental() > 0 {
    ///     // update progress bar, yield to event loop, etc.
    /// }
    /// ```
    pub fn initialize_tables_incremental(&mut self) -> i32 {
        self.db.next()
    }

    /// Finds a solution for the given cube state.
    ///
    /// Returns a sequence of [`Move`]s that, when applied to `cube`,
    /// produces a solved cube. If the pruning tables have not yet been
    /// initialized, they are built automatically before searching.
    /// To pre-initialize them incrementally, see
    /// [`Solver::initialize_tables_incremental`].
    ///
    /// The solver targets solutions of at most 21 moves and may include
    /// slice moves when consecutive opposite-face moves can be combined.
    ///
    /// # Panics
    ///
    /// Panics if no solution is found within the internal probe limit.
    ///
    /// [`Move`]: crate::Move
    pub fn search(&mut self, cube: crate::Cube) -> Vec<crate::Move> {
        let cubex: crate::FixedCentersCube = cube.into();
        let cc = CubieCube {
            edges: conv_e(cubex.edges()),
            corners: conv(cubex.corners()),
        };
        if self.db.remaining > 0 {
            //insure initializied
            while self.db.next() > 0 {}
        }
        let solve_turns = self
            .searcher
            .solve_cc(&self.db.table, cc, 21, 100000, 25, 0);
        use crate::FaceMove::*;
        let map = [
            U1, U2, U3, R1, R2, R3, F1, F2, F3, D1, D2, D3, L1, L2, L3, B1, B2, B3,
        ];
        let rotation = cube.centers();
        replace_with_slice(
            solve_turns
                .unwrap()
                .iter()
                .map(|mv| map[*mv as usize].projection(rotation)),
        )
    }
}
