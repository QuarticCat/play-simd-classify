#![allow(clippy::needless_range_loop)]

use std::collections::HashMap;
use std::mem::transmute;

pub fn build_binary_lut(classes: &[bool; 256]) -> Option<([u8; 16], [u8; 16])> {
    let classes: &[[bool; 16]; 16] = unsafe { transmute(classes) };

    let mut tbl_map = HashMap::new();
    for sub_classes in classes {
        let len = tbl_map.len();
        tbl_map.entry(sub_classes).or_insert(len);
    }
    if tbl_map.len() > 8 {
        return None;
    }

    let mut lut_hi = [0; 16];
    let mut lut_lo = [0; 16];
    for i in 0..16 {
        let shift = tbl_map[&classes[i]];
        lut_hi[i] = 1 << shift;
        for j in 0..16 {
            lut_lo[j] |= (classes[i][j] as u8) << shift;
        }
    }
    Some((lut_hi, lut_lo))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary() {
        let mut classes = Box::new([false; 256]);
        for i in [b' ', b'\r', b'\n', b'\t'] {
            classes[i as usize] = true;
        }

        let (lut_hi, lut_lo) = build_binary_lut(&classes).unwrap();

        for i in 0..16 {
            for j in 0..16 {
                assert_eq!(classes[i << 4 | j], (lut_hi[i] & lut_lo[j]) > 0);
            }
        }
    }
}
