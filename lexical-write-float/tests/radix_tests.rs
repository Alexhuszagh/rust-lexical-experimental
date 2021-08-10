#![cfg(feature = "radix")]

mod parse_radix;

use approx::{assert_relative_eq, relative_eq};
use core::num;
use lexical_util::constants::{FormattedSize, BUFFER_SIZE};
use lexical_util::format::{NumberFormat, NumberFormatBuilder};
use lexical_util::num::Float;
use lexical_write_float::options::RoundMode;
use lexical_write_float::{radix, Options};
use lexical_write_integer::write::WriteInteger;
use parse_radix::{parse_f32, parse_f64};
use proptest::prelude::*;
use quickcheck::quickcheck;

const BASE3: u128 = NumberFormatBuilder::from_radix(3);
const BASE5: u128 = NumberFormatBuilder::from_radix(5);

const F32_DATA: [f32; 31] = [
    0.,
    0.1,
    1.,
    1.1,
    12.,
    12.1,
    123.,
    123.1,
    1234.,
    1234.1,
    12345.,
    12345.1,
    123456.,
    123456.1,
    1234567.,
    1234567.1,
    12345678.,
    12345678.1,
    123456789.,
    123456789.1,
    123456789.12,
    123456789.123,
    123456789.1234,
    123456789.12345,
    1.2345678912345e8,
    1.2345e+8,
    1.2345e+11,
    1.2345e+38,
    1.2345e-8,
    1.2345e-11,
    1.2345e-38,
];
const F64_DATA: [f64; 33] = [
    0.,
    0.1,
    1.,
    1.1,
    12.,
    12.1,
    123.,
    123.1,
    1234.,
    1234.1,
    12345.,
    12345.1,
    123456.,
    123456.1,
    1234567.,
    1234567.1,
    12345678.,
    12345678.1,
    123456789.,
    123456789.1,
    123456789.12,
    123456789.123,
    123456789.1234,
    123456789.12345,
    1.2345678912345e8,
    1.2345e+8,
    1.2345e+11,
    1.2345e+38,
    1.2345e+308,
    1.2345e-8,
    1.2345e-11,
    1.2345e-38,
    1.2345e-299,
];

fn write_float<T: Float, const FORMAT: u128>(f: T, options: &Options, expected: &str)
where
    <T as Float>::Unsigned: WriteInteger + FormattedSize,
{
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let count = unsafe { radix::write_float::<_, FORMAT>(f, &mut buffer, options) };
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, expected);
}

#[test]
fn write_float_test() {
    // Check no formatting, binary, and when exponent notation is used.
    let options = Options::builder().build().unwrap();
    write_float::<_, BASE3>(0.0f64, &options, "0.0");
    write_float::<_, BASE3>(1.0f64, &options, "1.0");
    write_float::<_, BASE3>(2.0f64, &options, "2.0");
    write_float::<_, BASE3>(0.49999999999f64, &options, "0.111111111111111111111101200020121");
    write_float::<_, BASE3>(0.5f64, &options, "0.1111111111111111111111111111111112");
    write_float::<_, BASE3>(0.75f64, &options, "0.202020202020202020202020202020202");
    write_float::<_, BASE3>(0.9998475842097241f64, &options, "0.22222222");

    // Adapted from bugs in quickcheck.
    write_float::<_, BASE3>(
        1.7976931348623157e+308f64,
        &options,
        "1.0020200012020012100112000100111212e212221",
    );
    // Adapted from bugs in quickcheck.
    write_float::<_, BASE3>(3.4028235e+38f32, &options, "2.022011021210002e2222");

    // Try changing the exponent limits.
    let options = Options::builder()
        .negative_exponent_break(num::NonZeroI32::new(-6))
        .positive_exponent_break(num::NonZeroI32::new(10))
        .build()
        .unwrap();
    write_float::<_, BASE3>(1501.2344967901236f64, &options, "2001121.02002222112101212200212222");
    write_float::<_, BASE3>(
        0.02290702051986883f64,
        &options,
        "0.000121200212201201002110120212011",
    );
    write_float::<_, BASE3>(10e9f64, &options, "2.21210220202122010101e202");

    // Check max digits.
    let options =
        Options::builder().max_significant_digits(num::NonZeroUsize::new(5)).build().unwrap();
    write_float::<_, BASE3>(0.0f64, &options, "0.0");
    write_float::<_, BASE3>(1.0f64, &options, "1.0");
    write_float::<_, BASE3>(2.0f64, &options, "2.0");
    write_float::<_, BASE3>(0.49999999999f64, &options, "0.11111");
    write_float::<_, BASE3>(0.5f64, &options, "0.11112");
    write_float::<_, BASE3>(0.75f64, &options, "0.20202");
    write_float::<_, BASE3>(0.9998475842097241f64, &options, "1.0");

    // Check min digits.
    let options =
        Options::builder().min_significant_digits(num::NonZeroUsize::new(5)).build().unwrap();
    write_float::<_, BASE3>(0.0f64, &options, "0.0000");
    write_float::<_, BASE3>(1.0f64, &options, "1.0000");
    write_float::<_, BASE3>(2.0f64, &options, "2.0000");
    write_float::<_, BASE3>(0.49999999999f64, &options, "0.111111111111111111111101200020121");

    // Check max digits and trim floats.
    let options = Options::builder()
        .max_significant_digits(num::NonZeroUsize::new(5))
        .trim_floats(true)
        .build()
        .unwrap();
    write_float::<_, BASE3>(0.2345678901234567890f64, &options, "0.0201");
    write_float::<_, BASE3>(23.45678901234567890f64, &options, "212.11");
    write_float::<_, BASE3>(93.82715604938272f64, &options, "10111");
    write_float::<_, BASE3>(375.3086241975309f64, &options, "111220");

    // Test the round mode.
    let truncate = Options::builder()
        .max_significant_digits(num::NonZeroUsize::new(2))
        .round_mode(RoundMode::Truncate)
        .build()
        .unwrap();
    let round = Options::builder()
        .max_significant_digits(num::NonZeroUsize::new(2))
        .round_mode(RoundMode::Round)
        .build()
        .unwrap();
    write_float::<_, BASE3>(23.45678901234567890f64, &round, "220.0");
    write_float::<_, BASE3>(23.45678901234567890f64, &truncate, "210.0");
}

macro_rules! test_radix {
    ($parse:ident, $f:ident, $radix:expr, $buffer:ident, $options:ident) => {{
        const FORMAT: u128 = NumberFormatBuilder::from_radix($radix);
        let format = NumberFormat::<FORMAT> {};
        let count = unsafe { radix::write_float::<_, FORMAT>($f, &mut $buffer, &$options) };
        let roundtrip = $parse(&$buffer[..count], $radix, format.exponent());
        assert_relative_eq!($f, roundtrip, epsilon = 1e-6, max_relative = 3e-6);
    }};
}

macro_rules! test_all {
    ($parse:ident, $f:ident, $buffer:ident, $options:ident) => {{
        test_radix!($parse, $f, 3, $buffer, $options);
        test_radix!($parse, $f, 5, $buffer, $options);
        test_radix!($parse, $f, 6, $buffer, $options);
        test_radix!($parse, $f, 7, $buffer, $options);
        test_radix!($parse, $f, 9, $buffer, $options);
        test_radix!($parse, $f, 11, $buffer, $options);
        test_radix!($parse, $f, 12, $buffer, $options);
        test_radix!($parse, $f, 13, $buffer, $options);
        test_radix!($parse, $f, 14, $buffer, $options);
        test_radix!($parse, $f, 15, $buffer, $options);
        test_radix!($parse, $f, 17, $buffer, $options);
        test_radix!($parse, $f, 18, $buffer, $options);
        test_radix!($parse, $f, 19, $buffer, $options);
        test_radix!($parse, $f, 20, $buffer, $options);
        test_radix!($parse, $f, 21, $buffer, $options);
        test_radix!($parse, $f, 22, $buffer, $options);
        test_radix!($parse, $f, 23, $buffer, $options);
        test_radix!($parse, $f, 24, $buffer, $options);
        test_radix!($parse, $f, 25, $buffer, $options);
        test_radix!($parse, $f, 26, $buffer, $options);
        test_radix!($parse, $f, 27, $buffer, $options);
        test_radix!($parse, $f, 28, $buffer, $options);
        test_radix!($parse, $f, 29, $buffer, $options);
        test_radix!($parse, $f, 30, $buffer, $options);
        test_radix!($parse, $f, 31, $buffer, $options);
        test_radix!($parse, $f, 33, $buffer, $options);
        test_radix!($parse, $f, 34, $buffer, $options);
        test_radix!($parse, $f, 35, $buffer, $options);
        test_radix!($parse, $f, 36, $buffer, $options);
    }};
}

#[test]
fn f32_radix_roundtrip_test() {
    let mut buffer = [b'\x00'; 1200];
    let options = Options::new();
    for &f in F32_DATA.iter() {
        test_all!(parse_f32, f, buffer, options);
    }
}

#[test]
fn f64_radix_roundtrip_test() {
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let options = Options::new();
    for &f in F64_DATA.iter() {
        test_all!(parse_f64, f, buffer, options);
    }
}

//  NOTE:
//      Due to how we round-up by default, for min or max values, the output
//      frequently rounds up to infinity, meaning we can't roundtrip. These
//      should be inf, or F::MAX, but we can't guarantee it, so just skip them.

quickcheck! {
    #[cfg_attr(miri, ignore)]
    fn f32_base3_quickcheck(f: f32) -> bool {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        if f.is_special() || f == f32::MAX || f == f32::MIN {
            true
        } else {
            let f = f.abs();
            let count = unsafe { radix::write_float::<_, BASE3>(f, &mut buffer, &options) };
            let roundtrip = parse_f32(&buffer[..count], 3, b'e');
            relative_eq!(f, roundtrip, epsilon=1e-6, max_relative=1e-6)
        }
    }

    #[cfg_attr(miri, ignore)]
    fn f32_base5_quickcheck(f: f32) -> bool {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        if f.is_special() || f == f32::MAX || f == f32::MIN {
            true
        } else {
            let f = f.abs();
            let count = unsafe { radix::write_float::<_, BASE5>(f, &mut buffer, &options) };
            let roundtrip = parse_f32(&buffer[..count], 5, b'e');
            relative_eq!(f, roundtrip, epsilon=1e-6, max_relative=1e-6)
        }
    }

    #[cfg_attr(miri, ignore)]
    fn f64_base3_quickcheck(f: f64) -> bool {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        if f.is_special() || f == f64::MAX || f == f64::MIN {
            true
        } else {
            let f = f.abs();
            let count = unsafe { radix::write_float::<_, BASE3>(f, &mut buffer, &options) };
            let roundtrip = parse_f64(&buffer[..count], 3, b'e');
            relative_eq!(f, roundtrip, epsilon=1e-6, max_relative=1e-6)
        }
    }

    #[cfg_attr(miri, ignore)]
    fn f64_base5_quickcheck(f: f64) -> bool {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        if f.is_special() || f == f64::MAX || f == f64::MIN {
            true
        } else {
            let f = f.abs();
            let count = unsafe { radix::write_float::<_, BASE5>(f, &mut buffer, &options) };
            let roundtrip = parse_f64(&buffer[..count], 5, b'e');
            relative_eq!(f, roundtrip, epsilon=1e-6, max_relative=1e-6)
        }
    }
}

proptest! {
    #[test]
    #[cfg_attr(miri, ignore)]
    fn f32_base3_proptest(f in f32::MIN..f32::MAX) {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        if !(f.is_special() || f == f32::MAX || f == f32::MIN) {
            let f = f.abs();
            let count = unsafe { radix::write_float::<_, BASE3>(f, &mut buffer, &options) };
            let roundtrip = parse_f32(&buffer[..count], 3, b'e');
            let equal = relative_eq!(f, roundtrip, epsilon=1e-6, max_relative=1e-6);
            prop_assert!(equal)
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn f32_base5_proptest(f in f32::MIN..f32::MAX) {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        if !(f.is_special() || f == f32::MAX || f == f32::MIN) {
            let f = f.abs();
            let count = unsafe { radix::write_float::<_, BASE5>(f, &mut buffer, &options) };
            let roundtrip = parse_f32(&buffer[..count], 5, b'e');
            let equal = relative_eq!(f, roundtrip, epsilon=1e-6, max_relative=1e-6);
            prop_assert!(equal)
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn f64_base3_proptest(f in f64::MIN..f64::MAX) {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        if !(f.is_special() || f == f64::MAX || f == f64::MIN) {
            let f = f.abs();
            let count = unsafe { radix::write_float::<_, BASE3>(f, &mut buffer, &options) };
            let roundtrip = parse_f64(&buffer[..count], 3, b'e');
            let equal = relative_eq!(f, roundtrip, epsilon=1e-6, max_relative=1e-6);
            prop_assert!(equal)
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn f64_base5_proptest(f in f64::MIN..f64::MAX) {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        if !(f.is_special() || f == f64::MAX || f == f64::MIN) {
            let f = f.abs();
            let count = unsafe { radix::write_float::<_, BASE5>(f, &mut buffer, &options) };
            let roundtrip = parse_f64(&buffer[..count], 5, b'e');
            let equal = relative_eq!(f, roundtrip, epsilon=1e-6, max_relative=1e-6);
            prop_assert!(equal)
        }
    }
}