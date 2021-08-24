#![cfg(not(feature = "compact"))]

use lexical_write_float::{algorithm, Options, RoundMode};

fn floor_shift(integer: u32, fraction: u64, shift: i32) -> i32 {
    ((integer << shift) | (fraction >> (64 - shift)) as u32) as i32
}

fn dragonbox_log5_2(q: i32) -> i32 {
    let c = floor_shift(0, 0x6e40d1a4143dcb94, 20);
    let s = floor_shift(0, 0, 20);
    (q * c - s) >> 20
}

fn dragonbox_log10_2(q: i32) -> i32 {
    let c = floor_shift(0, 0x4d104d427de7fbcc, 22);
    let s = floor_shift(0, 0, 22);
    (q * c - s) >> 22
}

fn dragonbox_log2_10(q: i32) -> i32 {
    let c = floor_shift(3, 0x5269e12f346e2bf9, 19);
    let s = floor_shift(0, 0, 19);
    (q * c - s) >> 19
}

fn dragonbox_log5_2_sub_log5_3(q: i32) -> i32 {
    let c = floor_shift(0, 0x6e40d1a4143dcb94, 20);
    let s = floor_shift(0, 0xaebf47915d443b24, 20);
    (q * c - s) >> 20
}

fn dragonbox_log10_2_sub_log10_4_div3(q: i32) -> i32 {
    let c = floor_shift(0, 0x4d104d427de7fbcc, 22);
    let s = floor_shift(0, 0x1ffbfc2bbc780375, 22);
    (q * c - s) >> 22
}

#[test]
fn floor_log5_pow2_test() {
    for q in -1492i32..=1492 {
        let actual = algorithm::floor_log5_pow2(q);
        let expected = dragonbox_log5_2(q);
        assert_eq!(actual, expected);
    }
}

#[test]
fn floor_log10_pow2_test() {
    for q in -1700i32..=1700 {
        let actual = algorithm::floor_log10_pow2(q);
        let expected = dragonbox_log10_2(q);
        assert_eq!(actual, expected);
    }
}

#[test]
fn floor_log2_pow10_test() {
    for q in -1233i32..=1233 {
        let actual = algorithm::floor_log2_pow10(q);
        let expected = dragonbox_log2_10(q);
        assert_eq!(actual, expected);
    }
}

#[test]
fn floor_log5_pow2_minus_log5_3_test() {
    for q in -2427i32..=2427 {
        let actual = algorithm::floor_log5_pow2_minus_log5_3(q);
        let expected = dragonbox_log5_2_sub_log5_3(q);
        assert_eq!(actual, expected);
    }
}

#[test]
fn floor_log10_pow2_minus_log10_4_over_3_test() {
    for q in -1700i32..=1700 {
        let actual = algorithm::floor_log10_pow2_minus_log10_4_over_3(q);
        let expected = dragonbox_log10_2_sub_log10_4_div3(q);
        assert_eq!(actual, expected);
    }
}

#[test]
fn max_power_test() {
    assert_eq!(algorithm::max_power::<f32>(), 7);
    assert_eq!(algorithm::max_power::<f64>(), 16);
}

#[test]
fn pow32_test() {
    assert_eq!(algorithm::pow32(10, 1), 10);
    assert_eq!(algorithm::pow32(10, 2), 100);
}

#[test]
fn pow64_test() {
    assert_eq!(algorithm::pow64(10, 1), 10);
    assert_eq!(algorithm::pow64(10, 2), 100);
}

#[test]
fn count_factors_test() {
    assert_eq!(algorithm::count_factors(5, 25), 2);
    assert_eq!(algorithm::count_factors(5, 30), 1);
    assert_eq!(algorithm::count_factors(5, 125), 3);
    assert_eq!(algorithm::count_factors(5, 126), 0);
}

#[test]
fn floor_log2_test() {
    assert_eq!(algorithm::floor_log2(25), 4);
    assert_eq!(algorithm::floor_log2(30), 4);
    assert_eq!(algorithm::floor_log2(125), 6);
    assert_eq!(algorithm::floor_log2(126), 6);
    assert_eq!(algorithm::floor_log2(128), 7);
}

fn to_extended_f32(float: f32, round: bool) -> (u64, i32) {
    let mut builder = Options::builder();
    if round {
        builder = builder.round_mode(RoundMode::Round);
    } else {
        builder = builder.round_mode(RoundMode::Truncate);
    }
    let options = builder.build().unwrap();
    let fp = algorithm::to_extended(float, &options);
    (fp.mant, fp.exp)
}

fn to_extended_f64(float: f64, round: bool) -> (u64, i32) {
    let mut builder = Options::builder();
    if round {
        builder = builder.round_mode(RoundMode::Round);
    } else {
        builder = builder.round_mode(RoundMode::Truncate);
    }
    let options = builder.build().unwrap();
    let fp = algorithm::to_extended(float, &options);
    (fp.mant, fp.exp)
}

#[test]
fn to_extended_test() {
    assert_eq!(to_extended_f32(0.0, true), (0, 0));
    assert_eq!(to_extended_f32(0.5, true), (5, -1));
    assert_eq!(to_extended_f32(1.0, true), (1, 0));
    assert_eq!(to_extended_f32(1.5, true), (15, -1));
    assert_eq!(to_extended_f32(1.23456, true), (123456, -5));
    assert_eq!(to_extended_f32(0.0, false), (0, 0));
    assert_eq!(to_extended_f32(0.5, false), (5, -1));
    assert_eq!(to_extended_f32(1.0, false), (1, 0));
    assert_eq!(to_extended_f32(1.5, false), (15, -1));
    assert_eq!(to_extended_f32(1.23456, false), (12345601, -7));

    assert_eq!(to_extended_f64(0.0, true), (0, 0));
    assert_eq!(to_extended_f64(0.5, true), (5000000000000000, -16));
    assert_eq!(to_extended_f64(1.0, true), (1000000000000000, -15));
    assert_eq!(to_extended_f64(1.5, true), (1500000000000000, -15));
    assert_eq!(to_extended_f64(1.23456, true), (1234560000000000, -15));
    assert_eq!(to_extended_f64(0.0, false), (0, 0));
    assert_eq!(to_extended_f64(0.5, false), (500000000000000, -15));
    assert_eq!(to_extended_f64(1.0, false), (1000000000000000, -15));
    assert_eq!(to_extended_f64(1.5, false), (1500000000000000, -15));
    assert_eq!(to_extended_f64(1.23456, false), (12345600000000002, -16));
}