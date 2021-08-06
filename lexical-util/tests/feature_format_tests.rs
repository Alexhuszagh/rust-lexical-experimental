#![cfg(feature = "format")]

use lexical_util::format;

#[test]
fn ignore_test() {
    let fmt = format::NumberFormat::<{ format::IGNORE }> {};
    assert_eq!(fmt.flags(), format::DIGIT_SEPARATOR_FLAG_MASK);
    assert_eq!(fmt.digit_separator(), b'_');
    assert_eq!(fmt.decimal_point(), b'.');
    assert_eq!(fmt.exponent(), b'e');
    assert_eq!(fmt.required_integer_digits(), false);
    assert_eq!(fmt.required_fraction_digits(), false);
    assert_eq!(fmt.required_exponent_digits(), false);
    assert_eq!(fmt.required_mantissa_digits(), false);
    assert_eq!(fmt.required_digits(), false);
    assert_eq!(fmt.no_positive_mantissa_sign(), false);
    assert_eq!(fmt.required_mantissa_sign(), false);
    assert_eq!(fmt.no_exponent_notation(), false);
    assert_eq!(fmt.no_positive_exponent_sign(), false);
    assert_eq!(fmt.required_exponent_sign(), false);
    assert_eq!(fmt.no_exponent_without_fraction(), false);
    assert_eq!(fmt.no_special(), false);
    assert_eq!(fmt.case_sensitive_special(), false);
    assert_eq!(fmt.no_integer_leading_zeros(), false);
    assert_eq!(fmt.no_float_leading_zeros(), false);
    assert_eq!(fmt.required_exponent_notation(), false);
    assert_eq!(fmt.integer_internal_digit_separator(), true);
    assert_eq!(fmt.fraction_internal_digit_separator(), true);
    assert_eq!(fmt.exponent_internal_digit_separator(), true);
    assert_eq!(fmt.internal_digit_separator(), true);
    assert_eq!(fmt.integer_leading_digit_separator(), true);
    assert_eq!(fmt.fraction_leading_digit_separator(), true);
    assert_eq!(fmt.exponent_leading_digit_separator(), true);
    assert_eq!(fmt.leading_digit_separator(), true);
    assert_eq!(fmt.integer_trailing_digit_separator(), true);
    assert_eq!(fmt.fraction_trailing_digit_separator(), true);
    assert_eq!(fmt.exponent_trailing_digit_separator(), true);
    assert_eq!(fmt.trailing_digit_separator(), true);
    assert_eq!(fmt.integer_consecutive_digit_separator(), true);
    assert_eq!(fmt.fraction_consecutive_digit_separator(), true);
    assert_eq!(fmt.exponent_consecutive_digit_separator(), true);
    assert_eq!(fmt.consecutive_digit_separator(), true);
    assert_eq!(fmt.special_digit_separator(), true);
}

fn test_flag<const FORMAT: u128>() {
    let fmt = format::NumberFormat::<FORMAT> {};
    assert_eq!(fmt.flags(), FORMAT);
    assert_eq!(fmt.digit_separator(), 0);
}

#[test]
fn flags_test() {
    test_flag::<{ format::REQUIRED_INTEGER_DIGITS }>();
    test_flag::<{ format::REQUIRED_FRACTION_DIGITS }>();
    test_flag::<{ format::REQUIRED_EXPONENT_DIGITS }>();
    test_flag::<{ format::REQUIRED_MANTISSA_DIGITS }>();
    test_flag::<{ format::NO_POSITIVE_MANTISSA_SIGN }>();
    test_flag::<{ format::REQUIRED_MANTISSA_SIGN }>();
    test_flag::<{ format::NO_EXPONENT_NOTATION }>();
    test_flag::<{ format::NO_POSITIVE_EXPONENT_SIGN }>();
    test_flag::<{ format::REQUIRED_EXPONENT_SIGN }>();
    test_flag::<{ format::NO_EXPONENT_WITHOUT_FRACTION }>();
    test_flag::<{ format::NO_SPECIAL }>();
    test_flag::<{ format::CASE_SENSITIVE_SPECIAL }>();
    test_flag::<{ format::NO_INTEGER_LEADING_ZEROS }>();
    test_flag::<{ format::NO_FLOAT_LEADING_ZEROS }>();
    test_flag::<{ format::REQUIRED_EXPONENT_NOTATION }>();
    test_flag::<{ format::CASE_SENSITIVE_EXPONENT }>();
    test_flag::<{ format::CASE_SENSITIVE_BASE_PREFIX }>();
    test_flag::<{ format::CASE_SENSITIVE_BASE_SUFFIX }>();
    test_flag::<{ format::INTEGER_INTERNAL_DIGIT_SEPARATOR }>();
    test_flag::<{ format::FRACTION_INTERNAL_DIGIT_SEPARATOR }>();
    test_flag::<{ format::EXPONENT_INTERNAL_DIGIT_SEPARATOR }>();
    test_flag::<{ format::INTEGER_LEADING_DIGIT_SEPARATOR }>();
    test_flag::<{ format::FRACTION_LEADING_DIGIT_SEPARATOR }>();
    test_flag::<{ format::EXPONENT_LEADING_DIGIT_SEPARATOR }>();
    test_flag::<{ format::INTEGER_TRAILING_DIGIT_SEPARATOR }>();
    test_flag::<{ format::FRACTION_TRAILING_DIGIT_SEPARATOR }>();
    test_flag::<{ format::EXPONENT_TRAILING_DIGIT_SEPARATOR }>();
    test_flag::<{ format::INTEGER_CONSECUTIVE_DIGIT_SEPARATOR }>();
    test_flag::<{ format::FRACTION_CONSECUTIVE_DIGIT_SEPARATOR }>();
    test_flag::<{ format::EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR }>();
    test_flag::<{ format::SPECIAL_DIGIT_SEPARATOR }>();
}

#[test]
fn constants_test() {
    // Don't check the actual values: just check they're defined.
    let _: u128 = format::RUST_LITERAL;
    let _: u128 = format::RUST_STRING;
    let _: u128 = format::PYTHON_LITERAL;
    let _: u128 = format::PYTHON_STRING;
    let _: u128 = format::PYTHON3_LITERAL;
    let _: u128 = format::PYTHON3_STRING;
    let _: u128 = format::PYTHON36_LITERAL;
    let _: u128 = format::PYTHON35_LITERAL;
    let _: u128 = format::PYTHON2_LITERAL;
    let _: u128 = format::PYTHON2_STRING;
    let _: u128 = format::CXX17_LITERAL;
    let _: u128 = format::CXX17_STRING;
    let _: u128 = format::CXX14_LITERAL;
    let _: u128 = format::CXX14_STRING;
    let _: u128 = format::CXX11_LITERAL;
    let _: u128 = format::CXX11_STRING;
    let _: u128 = format::CXX03_LITERAL;
    let _: u128 = format::CXX03_STRING;
    let _: u128 = format::CXX98_LITERAL;
    let _: u128 = format::CXX98_STRING;
    let _: u128 = format::C18_LITERAL;
    let _: u128 = format::C18_STRING;
    let _: u128 = format::C11_LITERAL;
    let _: u128 = format::C11_STRING;
    let _: u128 = format::C99_LITERAL;
    let _: u128 = format::C99_STRING;
    let _: u128 = format::C90_LITERAL;
    let _: u128 = format::C90_STRING;
    let _: u128 = format::C89_LITERAL;
    let _: u128 = format::C89_STRING;
    let _: u128 = format::RUBY_LITERAL;
    let _: u128 = format::RUBY_STRING;
    let _: u128 = format::SWIFT_LITERAL;
    let _: u128 = format::SWIFT_STRING;
    let _: u128 = format::GO_LITERAL;
    let _: u128 = format::GO_STRING;
    let _: u128 = format::HASKELL_LITERAL;
    let _: u128 = format::HASKELL_STRING;
    let _: u128 = format::JAVASCRIPT_LITERAL;
    let _: u128 = format::JAVASCRIPT_STRING;
    let _: u128 = format::PERL_LITERAL;
    let _: u128 = format::PERL_STRING;
    let _: u128 = format::PHP_LITERAL;
    let _: u128 = format::PHP_STRING;
    let _: u128 = format::JAVA_LITERAL;
    let _: u128 = format::JAVA_STRING;
    let _: u128 = format::R_LITERAL;
    let _: u128 = format::R_STRING;
    let _: u128 = format::KOTLIN_LITERAL;
    let _: u128 = format::KOTLIN_STRING;
    let _: u128 = format::JULIA_LITERAL;
    let _: u128 = format::JULIA_STRING;
    let _: u128 = format::CSHARP7_LITERAL;
    let _: u128 = format::CSHARP7_STRING;
    let _: u128 = format::CSHARP6_LITERAL;
    let _: u128 = format::CSHARP6_STRING;
    let _: u128 = format::CSHARP5_LITERAL;
    let _: u128 = format::CSHARP5_STRING;
    let _: u128 = format::CSHARP4_LITERAL;
    let _: u128 = format::CSHARP4_STRING;
    let _: u128 = format::CSHARP3_LITERAL;
    let _: u128 = format::CSHARP3_STRING;
    let _: u128 = format::CSHARP2_LITERAL;
    let _: u128 = format::CSHARP2_STRING;
    let _: u128 = format::CSHARP1_LITERAL;
    let _: u128 = format::CSHARP1_STRING;
    let _: u128 = format::KAWA_LITERAL;
    let _: u128 = format::KAWA_STRING;
    let _: u128 = format::GAMBITC_LITERAL;
    let _: u128 = format::GAMBITC_STRING;
    let _: u128 = format::GUILE_LITERAL;
    let _: u128 = format::GUILE_STRING;
    let _: u128 = format::CLOJURE_LITERAL;
    let _: u128 = format::CLOJURE_STRING;
    let _: u128 = format::ERLANG_LITERAL;
    let _: u128 = format::ERLANG_STRING;
    let _: u128 = format::ELM_LITERAL;
    let _: u128 = format::ELM_STRING;
    let _: u128 = format::SCALA_LITERAL;
    let _: u128 = format::SCALA_STRING;
    let _: u128 = format::ELIXIR_LITERAL;
    let _: u128 = format::ELIXIR_STRING;
    let _: u128 = format::FORTRAN_LITERAL;
    let _: u128 = format::FORTRAN_STRING;
    let _: u128 = format::D_LITERAL;
    let _: u128 = format::D_STRING;
    let _: u128 = format::COFFEESCRIPT_LITERAL;
    let _: u128 = format::COFFEESCRIPT_STRING;
    let _: u128 = format::COBOL_LITERAL;
    let _: u128 = format::COBOL_STRING;
    let _: u128 = format::FSHARP_LITERAL;
    let _: u128 = format::FSHARP_STRING;
    let _: u128 = format::VB_LITERAL;
    let _: u128 = format::VB_STRING;
    let _: u128 = format::OCAML_LITERAL;
    let _: u128 = format::OCAML_STRING;
    let _: u128 = format::OBJECTIVEC_LITERAL;
    let _: u128 = format::OBJECTIVEC_STRING;
    let _: u128 = format::REASONML_LITERAL;
    let _: u128 = format::REASONML_STRING;
    let _: u128 = format::OCTAVE_LITERAL;
    let _: u128 = format::OCTAVE_STRING;
    let _: u128 = format::MATLAB_LITERAL;
    let _: u128 = format::MATLAB_STRING;
    let _: u128 = format::ZIG_LITERAL;
    let _: u128 = format::ZIG_STRING;
    let _: u128 = format::SAGE_LITERAL;
    let _: u128 = format::SAGE_STRING;
    let _: u128 = format::JSON;
    let _: u128 = format::TOML;
    let _: u128 = format::YAML;
    let _: u128 = format::XML;
    let _: u128 = format::SQLITE;
    let _: u128 = format::POSTGRESQL;
    let _: u128 = format::MYSQL;
    let _: u128 = format::MONGODB;
}