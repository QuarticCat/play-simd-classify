#![allow(clippy::needless_range_loop)]

use std::collections::HashMap;
use std::mem::transmute;

pub fn build_binary_tables(classes: &[bool; 256]) -> Option<([u8; 16], [u8; 16])> {
    let classes: &[[bool; 16]; 16] = unsafe { transmute(classes) };

    let mut tbl_map = HashMap::new();
    for sub_classes in classes {
        let len = tbl_map.len();
        tbl_map.entry(sub_classes).or_insert(len);
    }
    if tbl_map.len() > 8 {
        return None;
    }

    let mut upper_tbl = [0; 16];
    let mut lower_tbl = [0; 16];
    for i in 0..16 {
        let shift = tbl_map[&classes[i]];
        upper_tbl[i] = 1 << shift;
        for j in 0..16 {
            lower_tbl[j] |= (classes[i][j] as u8) << shift;
        }
    }
    Some((upper_tbl, lower_tbl))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary() {
        let mut classes = Box::new([false; 256]);
        classes[b' ' as usize] = true;
        classes[b'\r' as usize] = true;
        classes[b'\n' as usize] = true;
        classes[b'\t' as usize] = true;

        let (upper_tbl, lower_tbl) = build_binary_tables(&classes).unwrap();

        for i in 0..16 {
            for j in 0..16 {
                assert_eq!(classes[i << 4 | j], (upper_tbl[i] & lower_tbl[j]) > 0);
            }
        }
    }
}
