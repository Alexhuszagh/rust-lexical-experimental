#![cfg(feature = "power-of-two")]

use lexical_parse_float::binary::binary;
use lexical_parse_float::number::Number;
use lexical_util::format::NumberFormatBuilder;

const BINARY: u128 = NumberFormatBuilder::from_radix(2);
const BASE4: u128 = NumberFormatBuilder::from_radix(4);
const OCTAL: u128 = NumberFormatBuilder::from_radix(8);
const HEX: u128 = NumberFormatBuilder::from_radix(16);
const BASE32: u128 = NumberFormatBuilder::from_radix(32);

fn compute_float32<const FORMAT: u128>(q: i64, w: u64, many_digits: bool) -> (i32, u64) {
    let num = Number {
        exponent: q,
        mantissa: w,
        is_negative: false,
        many_digits,
    };
    let fp = binary::<f32, FORMAT>(&num, false);
    (fp.exp, fp.mant)
}

fn compute_float64<const FORMAT: u128>(q: i64, w: u64, many_digits: bool) -> (i32, u64) {
    let num = Number {
        exponent: q,
        mantissa: w,
        is_negative: false,
        many_digits,
    };
    let fp = binary::<f64, FORMAT>(&num, false);
    (fp.exp, fp.mant)
}

#[test]
fn halfway_round_down_test() {
    // Halfway, round-down tests
    assert_eq!(compute_float64::<BINARY>(0, 9007199254740992, false), (1076, 0));
    assert_eq!(compute_float64::<BINARY>(0, 9007199254740993, false), (1076, 0));
    assert_eq!(compute_float64::<BINARY>(0, 9007199254740994, false), (1076, 1));

    assert_eq!(compute_float64::<BINARY>(0, 18014398509481984, false), (1077, 0));
    assert_eq!(compute_float64::<BINARY>(0, 18014398509481986, false), (1077, 0));
    assert_eq!(compute_float64::<BINARY>(0, 18014398509481988, false), (1077, 1));

    assert_eq!(compute_float64::<BINARY>(0, 9223372036854775808, false), (1086, 0));
    assert_eq!(compute_float64::<BINARY>(0, 9223372036854776832, false), (1086, 0));
    assert_eq!(compute_float64::<BINARY>(0, 9223372036854777856, false), (1086, 1));

    // Add a 0 but say we're truncated.
    assert_eq!(compute_float64::<BINARY>(-10, 9223372036854775808, true), (1076, 0));
    assert_eq!(compute_float64::<BINARY>(-10, 9223372036854776832, true), (-1, 0));
    assert_eq!(compute_float64::<BINARY>(-10, 9223372036854777856, true), (1076, 1));

    // Check other bases.
    assert_eq!(compute_float64::<BASE4>(-2, 144115188075855872, false), (1076, 0));
    assert_eq!(compute_float64::<BASE4>(-2, 144115188075855888, false), (1076, 0));
    assert_eq!(compute_float64::<BASE4>(-2, 144115188075855904, false), (1076, 1));

    assert_eq!(compute_float64::<OCTAL>(-2, 576460752303423488, false), (1076, 0));
    assert_eq!(compute_float64::<OCTAL>(-2, 576460752303423552, false), (1076, 0));
    assert_eq!(compute_float64::<OCTAL>(-2, 576460752303423616, false), (1076, 1));

    assert_eq!(compute_float64::<HEX>(-1, 144115188075855872, false), (1076, 0));
    assert_eq!(compute_float64::<HEX>(-1, 144115188075855888, false), (1076, 0));
    assert_eq!(compute_float64::<HEX>(-1, 144115188075855904, false), (1076, 1));

    assert_eq!(compute_float64::<BASE32>(-1, 288230376151711744, false), (1076, 0));
    assert_eq!(compute_float64::<BASE32>(-1, 288230376151711776, false), (1076, 0));
    assert_eq!(compute_float64::<BASE32>(-1, 288230376151711808, false), (1076, 1));
}

#[test]
fn test_halfway_round_up() {
    // Halfway, round-up tests
    assert_eq!(compute_float64::<BINARY>(0, 9007199254740994, false), (1076, 1));
    assert_eq!(compute_float64::<BINARY>(0, 9007199254740995, false), (1076, 2));
    assert_eq!(compute_float64::<BINARY>(0, 9007199254740996, false), (1076, 2));

    assert_eq!(compute_float64::<BINARY>(0, 18014398509481988, false), (1077, 1));
    assert_eq!(compute_float64::<BINARY>(0, 18014398509481990, false), (1077, 2));
    assert_eq!(compute_float64::<BINARY>(0, 18014398509481992, false), (1077, 2));

    assert_eq!(compute_float64::<BINARY>(0, 9223372036854777856, false), (1086, 1));
    assert_eq!(compute_float64::<BINARY>(0, 9223372036854778880, false), (1086, 2));
    assert_eq!(compute_float64::<BINARY>(0, 9223372036854779904, false), (1086, 2));

    // Add a 0 but say we're truncated.
    assert_eq!(compute_float64::<BINARY>(-10, 9223372036854777856, true), (1076, 1));
    assert_eq!(compute_float64::<BINARY>(-10, 9223372036854778879, true), (1076, 1));
    assert_eq!(compute_float64::<BINARY>(-10, 9223372036854778880, true), (1076, 2));
    assert_eq!(compute_float64::<BINARY>(-10, 9223372036854779904, true), (1076, 2));

    // Check other bases.
    assert_eq!(compute_float64::<BASE4>(-2, 144115188075855904, false), (1076, 1));
    assert_eq!(compute_float64::<BASE4>(-2, 144115188075855920, false), (1076, 2));
    assert_eq!(compute_float64::<BASE4>(-2, 144115188075855936, false), (1076, 2));

    assert_eq!(compute_float64::<OCTAL>(-2, 576460752303423616, false), (1076, 1));
    assert_eq!(compute_float64::<OCTAL>(-2, 576460752303423680, false), (1076, 2));
    assert_eq!(compute_float64::<OCTAL>(-2, 576460752303423744, false), (1076, 2));

    assert_eq!(compute_float64::<HEX>(-1, 144115188075855904, false), (1076, 1));
    assert_eq!(compute_float64::<HEX>(-1, 144115188075855920, false), (1076, 2));
    assert_eq!(compute_float64::<HEX>(-1, 144115188075855936, false), (1076, 2));

    assert_eq!(compute_float64::<BASE32>(-1, 288230376151711808, false), (1076, 1));
    assert_eq!(compute_float64::<BASE32>(-1, 288230376151711840, false), (1076, 2));
    assert_eq!(compute_float64::<BASE32>(-1, 288230376151711872, false), (1076, 2));
}