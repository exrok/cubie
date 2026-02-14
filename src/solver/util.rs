pub const UX1: u8 = 0;
pub const UX2: u8 = 1;
pub const UX3: u8 = 2;
pub const RX1: u8 = 3;
pub const RX2: u8 = 4;
pub const RX3: u8 = 5;
pub const FX1: u8 = 6;
pub const FX2: u8 = 7;
pub const FX3: u8 = 8;
pub const DX1: u8 = 9;
pub const DX2: u8 = 10;
pub const DX3: u8 = 11;
pub const LX1: u8 = 12;
pub const LX2: u8 = 13;
pub const LX3: u8 = 14;
pub const BX1: u8 = 15;
pub const BX2: u8 = 16;
pub const BX3: u8 = 17;

pub const UD2STD: [u8; 18] = [
    UX1, UX2, UX3, RX2, FX2, DX1, DX2, DX3, LX2, BX2, RX1, RX3, FX1, FX3, LX1, LX3, BX1, BX3,
];

pub const STD2UD: [u8; 18] = [0, 1, 2, 10, 3, 11, 12, 4, 13, 5, 6, 7, 14, 8, 15, 16, 9, 17];
pub const CKMV2BIT: [u16; 11] = [7, 7, 7, 8, 16, 231, 231, 231, 264, 528, 0];
pub const CNK: [[u32; 13]; 13] = [
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 3, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 4, 6, 4, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 5, 10, 10, 5, 1, 0, 0, 0, 0, 0, 0, 0],
    [1, 6, 15, 20, 15, 6, 1, 0, 0, 0, 0, 0, 0],
    [1, 7, 21, 35, 35, 21, 7, 1, 0, 0, 0, 0, 0],
    [1, 8, 28, 56, 70, 56, 28, 8, 1, 0, 0, 0, 0],
    [1, 9, 36, 84, 126, 126, 84, 36, 9, 1, 0, 0, 0],
    [1, 10, 45, 120, 210, 252, 210, 120, 45, 10, 1, 0, 0],
    [1, 11, 55, 165, 330, 462, 462, 330, 165, 55, 11, 1, 0],
    [1, 12, 66, 220, 495, 792, 924, 792, 495, 220, 66, 12, 1],
];

pub fn get_parity(mut idx: u32, n: u32) -> u32 {
    let mut p = 0;
    for i in (0..n - 1).rev() {
        p ^= idx % (n - i);
        idx /= n - i;
    }
    p & 1
}
pub fn set_val(val0: u32, val: u32, is_edge: bool) -> u32 {
    if is_edge {
        (val << 1) | (val0 & 1)
    } else {
        val | (val0 & !7)
    } 
}

pub fn get_val(val0: u32, is_edge: bool) -> u32 {
    if is_edge {
        val0 >> 1
    } else {
        val0 & 7
    }
}
pub fn set_perm(arr: &mut [u8], mut idx: u32, is_edge: bool) {
    let mut val = 0xFEDCBA9876543210u64;
    let mut extract = 0u64;
    for p in 2..arr.len() as u64 + 1 {
        extract = (extract << 4) | ((idx as u64) % p);
        idx /= p as u32;
    }
    let n = arr.len();
    for e in &mut arr[0..n - 1] {
        let v = ((extract & 0xf) << 2) as u32;
        extract >>= 4;
        *e = set_val(*e as u32, ((val >> v) & 0xf) as u32, is_edge) as u8;
        let m = (1u64 << v) - 1;
        val = val & m | (val >> 4) & !m; // TODO verfiy
    }
    arr[n - 1] = set_val(arr[n - 1] as u32, (val & 0xf) as u32, is_edge) as u8;
}

pub fn get_perm(arr: &[u8], is_edge: bool) -> u32 {
    let mut idx: u32 = 0;
    let mut val = 0xFEDCBA9876543210u64;
    for (i, e) in arr[0..arr.len() - 1].iter().enumerate() {
        let v = get_val(*e as u32, is_edge) << 2;
        idx = (arr.len() as u32 - i as u32) * idx + ((val >> v) & 0xf) as u32;
        val -= 0x1111111111111110u64 << v;
    }
    idx
}
pub fn get_comb(arr: &[u8], mask: u32, is_edge: bool) -> u32 {
    let end = arr.len() - 1;
    let mut r: u32 = 4;
    let mut idxc = 0;
    for i in (0u32..(end + 1) as u32).rev() {
        let perm = get_val(arr[i as usize] as u32, is_edge) as u8;
        if (perm & 0xc) == mask as u8 {
            idxc += CNK[i as usize][r as usize];
            r -= 1;
        }
    }
    idxc
}
pub fn set_comb(arr: &mut [u8], mut idxc: u32, mask: u32, is_edge: bool) {
    let end = arr.len() - 1;
    let mut r: u32 = 4;
    let mut fill: i32 = end as i32;

    for i in (0u32..arr.len() as u32).rev() {
        if idxc >= CNK[i as usize][r as usize] {
            idxc -= CNK[i as usize][r as usize];
            r -= 1;
            arr[i as usize] = set_val(arr[i as usize] as u32, r | mask, is_edge) as u8;
        } else {
            if fill as u32 & 0xc == mask {
                fill -= 4;
            }
            arr[i as usize] = set_val(arr[i as usize] as u32, fill as u32, is_edge) as u8;
            fill -= 1;
        }
    }
}
