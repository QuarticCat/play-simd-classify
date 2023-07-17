#![feature(portable_simd)]

use std::arch::x86_64::*;
use std::simd::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;

use simd_classify::build_binary_lut;

fn whitespace(c: &mut Criterion) {
    let rng = &mut thread_rng();
    let data = (0..16 * 1000000)
        .map(|_| *[b' ', b'\r', b'\n', b'\t'].choose(rng).unwrap())
        .collect::<Vec<_>>();

    c.bench_function("simd_cmp", |b| {
        b.iter(|| {
            let mut cursor = 0;
            for chunk in data.chunks_exact(16) {
                let vchunk = u8x16::from_slice(chunk);
                let mask = vchunk.simd_eq(u8x16::splat(b' '))
                    | vchunk.simd_eq(u8x16::splat(b'\r'))
                    | vchunk.simd_eq(u8x16::splat(b'\n'))
                    | vchunk.simd_eq(u8x16::splat(b'\t'));
                if mask == mask8x16::splat(true) {
                    cursor += 16;
                } else {
                    cursor += mask.to_bitmask().trailing_ones();
                    break;
                }
            }
            black_box(cursor);
        })
    });

    let mut classes = Box::new([false; 256]);
    for i in [b' ', b'\r', b'\n', b'\t'] {
        classes[i as usize] = true;
    }
    let (lut_hi, lut_lo) = build_binary_lut(&classes).unwrap();
    let lut_hi = u8x16::from_array(lut_hi).into();
    let lut_lo = u8x16::from_array(lut_lo).into();
    c.bench_function("simd_lookup", |b| {
        b.iter(|| {
            let mut cursor = 0;
            for chunk in data.chunks_exact(32) {
                let vchunk0 = u8x16::from_slice(&chunk[..16]);
                let vchunk1 = u8x16::from_slice(&chunk[16..]);
                let nib_hi0 = vchunk0 >> u8x16::splat(4);
                let nib_hi1 = vchunk1 >> u8x16::splat(4);
                let nib_lo0 = vchunk0 & u8x16::splat(0xf);
                let nib_lo1 = vchunk1 & u8x16::splat(0xf);
                let cls_hi0 = unsafe { u8x16::from(_mm_shuffle_epi8(lut_hi, nib_hi0.into())) };
                let cls_hi1 = unsafe { u8x16::from(_mm_shuffle_epi8(lut_hi, nib_hi1.into())) };
                let cls_lo0 = unsafe { u8x16::from(_mm_shuffle_epi8(lut_lo, nib_lo0.into())) };
                let cls_lo1 = unsafe { u8x16::from(_mm_shuffle_epi8(lut_lo, nib_lo1.into())) };
                let classes0 = cls_hi0 & cls_lo0;
                let classes1 = cls_hi1 & cls_lo1;
                if classes0 > u8x16::splat(0) {
                    cursor += 16;
                } else {
                    todo!();
                }
                if classes1 > u8x16::splat(0) {
                    cursor += 16;
                } else {
                    todo!();
                }
            }
            black_box(cursor);
        })
    });
}

criterion_group!(benches, whitespace);
criterion_main!(benches);
