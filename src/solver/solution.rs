use super::cube::URF_MOVE;
#[derive(Default)]
pub struct Solution  {
    pub len: u32,
    pub depth: u32,
    pub verbose: u32,
    pub urf_idx: u32,
    pub moves: [u8;31]
}

impl Solution {
    pub fn set_args(&mut self,  verbose:u32,  urf_idx:u32,  depth:u32) {
        self.verbose = verbose;
        self.urf_idx = urf_idx;
        self.depth = depth;
    } 
    pub  fn to_vec(&self) -> Vec<u8> {
//        eprintln!("VERBOS:{}",self.verbose);
        if self.urf_idx < 3 {
            self.moves.iter().take(self.len as usize).map(|mv|{
                URF_MOVE[self.urf_idx as usize][*mv as usize]
            }).collect()
        } else {
            self.moves.iter().take(self.len as usize).rev().map(|mv|{
                URF_MOVE[self.urf_idx as usize][*mv as usize]
            }).collect()
        }
    }
    pub fn push(&mut self, cmove: u32) {
        // if self.len == 0 {
        //     self.moves[self.len as usize] = cmove as u8;
        //     self.len += 1;
        //     return;
        // }

        // let axis_cur = cmove / 3;
        // let axis_last = self.moves[(self.len - 1) as usize] as u32 /3;
        // if axis_cur == axis_last {
        //     let pow = ((cmove % 3) + (self.moves[(self.len -1) as usize] as u32 % 3) + 1)%4;
        //     if pow == 3 {
        //         self.len -= 1;
        //     } else {
        //         self.moves[(self.len - 1) as usize] = (axis_cur * 3 + pow) as u8;
        //     }
        //     return;
        // }

        // if self.len > 1
        //     && axis_cur % 3 == axis_last % 3
        //     && axis_cur == self.moves[(self.len - 2) as usize] as u32 / 3 {
        //         let pow = ((cmove +self.moves[(self.len -2) as usize] as u32) %3 +1)%4;
        //         if pow == 3 {
        //             self.moves[(self.len - 2) as usize] = self.moves[(self.len - 1) as usize];
        //             self.len -= 1;
        //         } else {
        //             self.moves[(self.len - 2) as usize] = (axis_cur * 3 + pow) as u8; 
        //         }
        //         return;
        // }
        self.moves[self.len as usize] = cmove as u8;
        self.len += 1;
    }

}
