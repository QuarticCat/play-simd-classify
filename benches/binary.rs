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
                let chunk = u8x16::from_slice(chunk);
                let mask = chunk.simd_eq(u8x16::splat(b' '))
                    | chunk.simd_eq(u8x16::splat(b'\r'))
                    | chunk.simd_eq(u8x16::splat(b'\n'))
                    | chunk.simd_eq(u8x16::splat(b'\t'));
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
            for chunk in data.chunks_exact(16) {
                let chunk = u8x16::from_slice(chunk);
                let nib_hi = chunk >> u8x16::splat(4);
                let nib_lo = chunk & u8x16::splat(0xf);
                let cls_hi = unsafe { u8x16::from(_mm_shuffle_epi8(lut_hi, nib_hi.into())) };
                let cls_lo = unsafe { u8x16::from(_mm_shuffle_epi8(lut_lo, nib_lo.into())) };
                let classes = cls_hi & cls_lo;
                if classes > u8x16::splat(0) {
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
