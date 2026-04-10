use pretty_assertions_sorted::assert_eq;

use super::*;

#[test]
fn bytes_round_trip() {
    let original = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D,
        0x1E, 0x1F,
    ];
    let hash = Felt::from_be_bytes(original);
    let bytes = hash.to_be_bytes();
    assert_eq!(bytes, original);
}

mod from_be_slice {
    use crate::felt::MODULUS_BE_BYTES;
    use crate::{Felt, OverflowError};
    use pretty_assertions_sorted::assert_eq;

    #[test]
    fn round_trip() {
        let original = Felt::from_hex("abcdef0123456789").unwrap();
        let bytes = original.to_be_bytes();
        let result = Felt::from_be_bytes_slice(&bytes[..]).unwrap();

        assert_eq!(result, original);
    }

    #[test]
    fn too_long() {
        let original = Felt::from_hex("abcdef0123456789").unwrap();
        let mut bytes = original.to_be_bytes().to_vec();
        bytes.push(0);
        Felt::from_be_bytes_slice(&bytes[..]).unwrap_err();
    }

    #[test]
    fn short_slice() {
        let original = Felt::from_hex("abcdef0123456789").unwrap();
        let bytes = original.to_be_bytes();
        let result = Felt::from_be_bytes_slice(&bytes[24..]);

        assert_eq!(result, Ok(original));
    }

    #[test]
    fn max() {
        let mut max_val = MODULUS_BE_BYTES;
        max_val[31] -= 1;
        Felt::from_be_bytes_slice(&max_val[..]).unwrap();
    }

    #[test]
    fn modulus_accepted() {
        // No modulus check — from_be_bytes_slice accepts any ≤32-byte slice.
        Felt::from_be_bytes_slice(&MODULUS_BE_BYTES[..]).unwrap();
    }

    #[test]
    fn overflow_too_long() {
        let too_long = [0u8; 33];
        assert_eq!(Felt::from_be_bytes_slice(&too_long[..]), Err(OverflowError));
    }
}

mod fmt {
    use pretty_assertions_sorted::assert_eq;

    use super::Felt;

    #[test]
    fn debug() {
        let hex_str = "1234567890abcdef000edcba0987654321";
        let felt = Felt::from_hex(hex_str).unwrap();
        let result = format!("{felt:?}");

        let expected = format!("0x{felt}");

        assert_eq!(result, expected);
    }

    #[test]
    fn fmt() {
        let hex_str = "1234567890abcdef000edcba0987654321";
        let starkhash = Felt::from_hex(hex_str).unwrap();
        let result = format!("{starkhash:x}");

        // We don't really care which casing is used by fmt.
        assert_eq!(result.to_lowercase(), hex_str.to_lowercase());
    }

    #[test]
    fn lower_hex() {
        let hex_str = "1234567890abcdef000edcba0987654321";
        let starkhash = Felt::from_hex(hex_str).unwrap();
        let result = format!("{starkhash:x}");

        assert_eq!(result, hex_str.to_lowercase());
    }

    #[test]
    fn upper_hex() {
        let hex_str = "1234567890abcdef000edcba0987654321";
        let starkhash = Felt::from_hex(hex_str).unwrap();
        let result = format!("{starkhash:X}");

        assert_eq!(result, hex_str.to_uppercase());
    }
}

mod from_hex_str {
    use assert_matches::assert_matches;
    use pretty_assertions_sorted::assert_eq;

    use super::*;

    /// Test hex string with its expected [Felt].
    fn test_data() -> (&'static str, Felt) {
        let mut expected = [0; 32];
        expected[31] = 0xEF;
        expected[30] = 0xCD;
        expected[29] = 0xAB;
        expected[28] = 0xef;
        expected[27] = 0xcd;
        expected[26] = 0xab;
        expected[25] = 0x89;
        expected[24] = 0x67;
        expected[23] = 0x45;
        expected[22] = 0x23;
        expected[21] = 0x01;
        let expected = Felt::from_be_bytes(expected);

        ("0123456789abcdefABCDEF", expected)
    }

    #[test]
    fn simple() {
        let (test_str, expected) = test_data();
        let uut = Felt::from_hex(test_str).unwrap();
        assert_eq!(uut, expected);
    }

    #[test]
    fn prefix() {
        let (test_str, expected) = test_data();
        let uut = Felt::from_hex(&format!("0x{test_str}")).unwrap();
        assert_eq!(uut, expected);
    }

    #[test]
    fn leading_zeros() {
        let (test_str, expected) = test_data();
        let uut = Felt::from_hex(&format!("000000000{test_str}")).unwrap();
        assert_eq!(uut, expected);
    }

    #[test]
    fn prefix_and_leading_zeros() {
        let (test_str, expected) = test_data();
        let uut = Felt::from_hex(&format!("0x000000000{test_str}")).unwrap();
        assert_eq!(uut, expected);
    }

    #[test]
    fn invalid_nibble() {
        assert_matches!(Felt::from_hex("0x123z").unwrap_err(), StrError::InvalidNibble(n) => assert_eq!(n, b'z'));
    }

    #[test]
    fn invalid_len() {
        assert_matches!(Felt::from_hex(&"1".repeat(65)).unwrap_err(), StrError::InvalidLength{max: 64, actual: n} => assert_eq!(n, 65));
    }

    #[test]
    fn modulus_accepted() {
        // No modulus check — from_hex accepts any valid hex up to 64 nibbles.
        let modulus = "0x800000000000011000000000000000000000000000000000000000000000001";
        let felt = Felt::from_hex(modulus).unwrap();
        assert!(!felt.is_valid());
    }
}

mod to_hex_string {
    use pretty_assertions_sorted::assert_eq;

    use super::*;

    const ODD: &str = "0x1234567890abcde";
    const EVEN: &str = "0x1234567890abcdef";
    const MAX: &str = "0x800000000000011000000000000000000000000000000000000000000000000";

    #[test]
    fn zero() {
        assert_eq!(Felt::ZERO.to_hex_string(), "0x0");
        let mut buf = [0u8; 66];
        assert_eq!(Felt::ZERO.as_hex_str(&mut buf), "0x0");
    }

    #[test]
    fn odd() {
        let hash = Felt::from_hex(ODD).unwrap();
        assert_eq!(hash.to_hex_string(), ODD);
        let mut buf = [0u8; 66];
        assert_eq!(hash.as_hex_str(&mut buf), ODD);
    }

    #[test]
    fn even() {
        let hash = Felt::from_hex(EVEN).unwrap();
        assert_eq!(hash.to_hex_string(), EVEN);
        let mut buf = [0u8; 66];
        assert_eq!(hash.as_hex_str(&mut buf), EVEN);
    }

    #[test]
    fn max() {
        let hash = Felt::from_hex(MAX).unwrap();
        assert_eq!(hash.to_hex_string(), MAX);
        let mut buf = [0u8; 66];
        assert_eq!(hash.as_hex_str(&mut buf), MAX);
    }

    #[test]
    #[should_panic(expected = "buffer size is 65, expected at least 66")]
    fn buffer_too_small() {
        let mut buf = [0u8; 65];
        Felt::ZERO.as_hex_str(&mut buf);
    }
}

mod exceeds_251_bits {
    use super::*;

    #[test]
    fn has_251_bits() {
        let mut bytes = [0xFFu8; 32];
        bytes[0] = 0x07;
        let h = Felt::from_be_bytes(bytes);
        assert!(!h.exceeds_251_bits());
    }

    #[test]
    fn has_252_bits() {
        let mut bytes = [0u8; 32];
        bytes[0] = 0x08;
        let h = Felt::from_be_bytes(bytes);
        assert!(h.exceeds_251_bits());
    }
}

mod add {
    use crate::felt::{Felt, MODULUS_BE_BYTES};
    use starknet_types_core::felt::Felt as SnFelt;

    fn assert_add_matches(a: Felt, b: Felt) {
        let our_result = a + b;
        let sn_result = SnFelt::from_bytes_be(&a.0) + SnFelt::from_bytes_be(&b.0);
        assert_eq!(
            our_result.0,
            sn_result.to_bytes_be(),
            "mismatch for {a:?} + {b:?}"
        );
    }

    #[test]
    fn zero_plus_zero() {
        assert_add_matches(Felt::ZERO, Felt::ZERO);
    }

    #[test]
    fn zero_plus_one() {
        assert_add_matches(Felt::ZERO, Felt::ONE);
    }

    #[test]
    fn one_plus_one() {
        assert_add_matches(Felt::ONE, Felt::ONE);
    }

    #[test]
    fn small_values() {
        let a = Felt::from_u64(12345);
        let b = Felt::from_u64(67890);
        assert_add_matches(a, b);
    }

    #[test]
    fn large_values_no_wrap() {
        let a = Felt::from_u128(u128::MAX);
        let b = Felt::from_u128(u128::MAX);
        assert_add_matches(a, b);
    }

    #[test]
    fn wraps_around_modulus() {
        // p - 1
        let max = Felt::from_be_bytes({
            let mut m = MODULUS_BE_BYTES;
            m[31] -= 1;
            m
        });
        assert_add_matches(max, Felt::ONE);
        assert_add_matches(max, max);
    }

    #[test]
    fn near_modulus() {
        let max = Felt::from_be_bytes({
            let mut m = MODULUS_BE_BYTES;
            m[31] -= 1;
            m
        });
        let half = Felt::from_be_bytes({
            let mut m = MODULUS_BE_BYTES;
            m[0] = 4;
            m[31] = 0;
            m
        });
        assert_add_matches(max, half);
        assert_add_matches(half, half);
    }

    #[test]
    fn carry_propagation() {
        // 0x00..00FF_FFFF_FFFF_FFFF + 1 forces a carry across limb boundary
        let a = Felt::from_u64(u64::MAX);
        let b = Felt::ONE;
        assert_add_matches(a, b);
    }

    #[test]
    fn multi_limb_carry() {
        // Large value spanning multiple limbs
        let a =
            Felt::from_hex("0x0000000000000000FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF")
                .unwrap();
        let b = Felt::ONE;
        assert_add_matches(a, b);
    }
}

mod neg {
    use crate::felt::MODULUS_BE_BYTES;
    use crate::Felt;
    use starknet_types_core::felt::Felt as SnFelt;

    fn assert_neg_matches(a: Felt) {
        let our_result = -a;
        let sn_result = -SnFelt::from_bytes_be(&a.0);
        assert_eq!(
            our_result.0,
            sn_result.to_bytes_be(),
            "mismatch for -({a:?})"
        );
    }

    #[test]
    fn zero() {
        assert_neg_matches(Felt::ZERO);
    }

    #[test]
    fn one() {
        assert_neg_matches(Felt::ONE);
    }

    #[test]
    fn small() {
        assert_neg_matches(Felt::from_u64(12345));
    }

    #[test]
    fn large() {
        assert_neg_matches(Felt::from_u128(u128::MAX));
    }

    #[test]
    fn max_felt() {
        let max = Felt::from_be_bytes({
            let mut m = MODULUS_BE_BYTES;
            m[31] -= 1;
            m
        });
        assert_neg_matches(max);
    }

    #[test]
    fn double_neg_is_identity() {
        let a = Felt::from_u64(42);
        assert_eq!(-(-a), a);
    }
}

mod sub {
    use crate::felt::MODULUS_BE_BYTES;

    use super::Felt;
    use starknet_types_core::felt::Felt as SnFelt;

    fn assert_sub_matches(a: Felt, b: Felt) {
        let our_result = a - b;
        let sn_result = SnFelt::from_bytes_be(&a.0) - SnFelt::from_bytes_be(&b.0);
        assert_eq!(
            our_result.0,
            sn_result.to_bytes_be(),
            "mismatch for {a:?} - {b:?}"
        );
    }

    #[test]
    fn zero_minus_zero() {
        assert_sub_matches(Felt::ZERO, Felt::ZERO);
    }

    #[test]
    fn one_minus_one() {
        assert_sub_matches(Felt::ONE, Felt::ONE);
    }

    #[test]
    fn a_minus_zero() {
        assert_sub_matches(Felt::from_u64(42), Felt::ZERO);
    }

    #[test]
    fn zero_minus_one() {
        // Should wrap: 0 - 1 = p - 1
        assert_sub_matches(Felt::ZERO, Felt::ONE);
    }

    #[test]
    fn small_values() {
        assert_sub_matches(Felt::from_u64(100), Felt::from_u64(42));
    }

    #[test]
    fn wraps_around() {
        // small - large wraps around the modulus
        assert_sub_matches(Felt::from_u64(1), Felt::from_u64(2));
    }

    #[test]
    fn large_values() {
        assert_sub_matches(Felt::from_u128(u128::MAX), Felt::from_u128(u128::MAX));
    }

    #[test]
    fn max_minus_one() {
        let max = Felt::from_be_bytes({
            let mut m = MODULUS_BE_BYTES;
            m[31] -= 1;
            m
        });
        assert_sub_matches(max, Felt::ONE);
    }

    #[test]
    fn add_sub_inverse() {
        let a = Felt::from_u64(12345);
        let b = Felt::from_u64(67890);
        assert_eq!((a + b) - b, a);
        assert_eq!((a - b) + b, a);
    }
}

mod selector {
    use super::Felt;
    use starknet::core::utils::starknet_keccak;

    fn assert_selector_matches(name: &str) {
        let ours = Felt::selector(name);
        let reference = starknet_keccak(name.as_bytes());
        assert_eq!(
            ours.0,
            reference.to_bytes_be(),
            "selector mismatch for {name:?}"
        );
    }

    #[test]
    fn transfer() {
        assert_selector_matches("Transfer");
    }

    #[test]
    fn transfer_from() {
        assert_selector_matches("transferFrom");
    }

    #[test]
    fn approve() {
        assert_selector_matches("Approval");
    }

    #[test]
    fn balance_of() {
        assert_selector_matches("balanceOf");
    }

    #[test]
    fn long_name() {
        assert_selector_matches("some_really_long_function_name_that_exceeds_31_bytes_easily");
    }

    #[test]
    fn single_char() {
        assert_selector_matches("a");
    }

    #[test]
    fn empty() {
        assert_selector_matches("");
    }

    #[test]
    fn const_eval() {
        const TRANSFER: Felt = Felt::selector("Transfer");
        assert_selector_matches("Transfer");
        assert_eq!(TRANSFER, Felt::selector("Transfer"));
    }
}

mod from_dec_str {
    use assert_matches::assert_matches;
    use pretty_assertions_sorted::assert_eq;
    use starknet_types_core::felt::Felt as SnFelt;

    use super::*;

    #[test]
    fn zero() {
        assert_eq!(Felt::from_dec_str("0").unwrap(), Felt::ZERO);
    }

    #[test]
    fn one() {
        assert_eq!(Felt::from_dec_str("1").unwrap(), Felt::ONE);
    }

    #[test]
    fn small() {
        assert_eq!(Felt::from_dec_str("255").unwrap(), Felt::from_u64(255));
    }

    #[test]
    fn large() {
        // 2^128
        let s = "340282366920938463463374607431768211456";
        let ours = Felt::from_dec_str(s).unwrap();
        let reference = SnFelt::from_dec_str(s).unwrap();
        assert_eq!(ours.0, reference.to_bytes_be());
    }

    #[test]
    fn max_felt() {
        // p - 1
        let s = "3618502788666131213697322783095070105623107215331596699973092056135872020480";
        let ours = Felt::from_dec_str(s).unwrap();
        let reference = SnFelt::from_dec_str(s).unwrap();
        assert_eq!(ours.0, reference.to_bytes_be());
    }

    #[test]
    fn modulus_accepted() {
        // p itself — accepted but not a valid field element
        let s = "3618502788666131213697322783095070105623107215331596699973092056135872020481";
        let felt = Felt::from_dec_str(s).unwrap();
        assert!(!felt.is_valid());
    }

    #[test]
    fn overflow_large() {
        // 2^256
        let s = "115792089237316195423570985008687907853269984665640564039457584007913129639936";
        assert_matches!(Felt::from_dec_str(s).unwrap_err(), StrError::Overflow(_));
    }

    #[test]
    fn empty() {
        assert_matches!(Felt::from_dec_str("").unwrap_err(), StrError::EmptyString);
    }

    #[test]
    fn invalid_char() {
        assert_matches!(
            Felt::from_dec_str("123a").unwrap_err(),
            StrError::InvalidDigit(b'a')
        );
    }

    #[test]
    fn leading_zeros() {
        assert_eq!(Felt::from_dec_str("007").unwrap(), Felt::from_u64(7));
    }
}

mod from_str {
    use pretty_assertions_sorted::assert_eq;
    use starknet_types_core::felt::Felt as SnFelt;

    use super::*;

    #[test]
    fn hex_prefix() {
        let felt: Felt = "0xff".parse().unwrap();
        assert_eq!(felt, Felt::from_u64(255));
    }

    #[test]
    fn hex_prefix_upper_is_decimal_parse() {
        // 0X prefix is not recognized as hex — treated as decimal, fails
        assert!("0XFF".parse::<Felt>().is_err());
    }

    #[test]
    fn decimal() {
        let felt: Felt = "255".parse().unwrap();
        assert_eq!(felt, Felt::from_u64(255));
    }

    #[test]
    fn large_decimal() {
        let s = "3618502788666131213697322783095070105623107215331596699973092056135872020480";
        let felt: Felt = s.parse().unwrap();
        let reference = SnFelt::from_dec_str(s).unwrap();
        assert_eq!(felt.0, reference.to_bytes_be());
    }

    #[test]
    fn large_hex() {
        let s = "0x800000000000011000000000000000000000000000000000000000000000000";
        let felt: Felt = s.parse().unwrap();
        assert_eq!(felt, Felt::from_hex(s).unwrap());
    }

    #[test]
    fn zero_decimal() {
        let felt: Felt = "0".parse().unwrap();
        assert_eq!(felt, Felt::ZERO);
    }
}

mod from_short_ascii_str {
    use assert_matches::assert_matches;
    use pretty_assertions_sorted::assert_eq;
    use starknet::core::utils::cairo_short_string_to_felt;

    use super::*;

    fn assert_matches_reference(s: &str) {
        let ours = Felt::from_short_ascii_str(s).unwrap();
        let reference = cairo_short_string_to_felt(s).unwrap();
        assert_eq!(ours.0, reference.to_bytes_be(), "mismatch for {s:?}");
    }

    #[test]
    fn empty() {
        assert_matches_reference("");
    }

    #[test]
    fn single_char() {
        assert_matches_reference("a");
    }

    #[test]
    fn hello() {
        assert_matches_reference("hello");
    }

    #[test]
    fn max_length() {
        assert_matches_reference("1234567890123456789012345678901");
    }

    #[test]
    fn too_long() {
        assert_matches!(
            Felt::from_short_ascii_str("12345678901234567890123456789012").unwrap_err(),
            StrError::InvalidLength {
                max: 31,
                actual: 32
            }
        );
    }

    #[test]
    fn digits() {
        assert_matches_reference("0123456789");
    }

    #[test]
    fn special_ascii() {
        assert_matches_reference("!@#$%^&*()");
    }

    #[test]
    fn spaces() {
        assert_matches_reference("hello world");
    }

    #[test]
    fn non_ascii_rejected() {
        assert_matches!(
            Felt::from_short_ascii_str("café").unwrap_err(),
            StrError::NonAsciiCharacter
        );
    }

    #[test]
    fn unchecked_matches() {
        let s = "Transfer";
        let checked = Felt::from_short_ascii_str(s).unwrap();
        let unchecked = Felt::from_short_ascii_str_unchecked(s);
        assert_eq!(checked, unchecked);
    }

    #[test]
    fn unchecked_empty() {
        assert_eq!(Felt::from_short_ascii_str_unchecked(""), Felt::ZERO);
    }

    #[test]
    fn const_eval() {
        const TRANSFER: Felt = Felt::from_short_ascii_str_unchecked("Transfer");
        assert_eq!(TRANSFER, Felt::from_short_ascii_str("Transfer").unwrap());
    }
}

mod as_short_ascii_str {
    use assert_matches::assert_matches;
    use pretty_assertions_sorted::assert_eq;
    use starknet::core::utils::parse_cairo_short_string;

    use super::*;

    fn assert_matches_reference(s: &str) {
        let felt = Felt::from_short_ascii_str(s).unwrap();
        let ours = felt.as_short_ascii_str().unwrap();
        let sn_felt = starknet::core::types::Felt::from_bytes_be(&felt.0);
        let reference = parse_cairo_short_string(&sn_felt).unwrap();
        assert_eq!(ours, reference, "mismatch for {s:?}");
    }

    #[test]
    fn empty() {
        assert_eq!(Felt::ZERO.as_short_ascii_str().unwrap(), "");
    }

    #[test]
    fn single_char() {
        assert_matches_reference("a");
    }

    #[test]
    fn hello() {
        assert_matches_reference("hello");
    }

    #[test]
    fn max_length() {
        assert_matches_reference("1234567890123456789012345678901");
    }

    #[test]
    fn digits() {
        assert_matches_reference("0123456789");
    }

    #[test]
    fn special_ascii() {
        assert_matches_reference("!@#$%^&*()");
    }

    #[test]
    fn spaces() {
        assert_matches_reference("hello world");
    }

    #[test]
    fn non_ascii_rejected() {
        // Byte 0x80 is > 127
        let felt = Felt::from_be_bytes_slice_unchecked(&[0x80]);
        assert_matches!(
            felt.as_short_ascii_str().unwrap_err(),
            StrError::NonAsciiCharacter
        );
    }

    #[test]
    fn round_trip() {
        let s = "Transfer";
        let felt = Felt::from_short_ascii_str(s).unwrap();
        assert_eq!(felt.as_short_ascii_str().unwrap(), s);
    }

    #[test]
    fn unchecked_matches_checked() {
        let s = "balanceOf";
        let felt = Felt::from_short_ascii_str(s).unwrap();
        let checked = felt.as_short_ascii_str().unwrap();
        let unchecked = felt.as_short_ascii_str_unchecked();
        assert_eq!(checked, unchecked);
    }

    #[test]
    fn unchecked_empty() {
        assert_eq!(Felt::ZERO.as_short_ascii_str_unchecked(), "");
    }
}

mod as_be_bytes_trimmed {
    use pretty_assertions_sorted::assert_eq;

    use super::*;

    #[test]
    fn zero() {
        let empty: &[u8] = &[];
        assert_eq!(Felt::ZERO.as_be_bytes_trimmed(), empty);
    }

    #[test]
    fn one() {
        assert_eq!(Felt::ONE.as_be_bytes_trimmed(), &[1u8] as &[u8]);
    }

    #[test]
    fn two() {
        assert_eq!(Felt::TWO.as_be_bytes_trimmed(), &[2u8] as &[u8]);
    }

    #[test]
    fn small_value() {
        let felt = Felt::from_u64(0xff);
        assert_eq!(felt.as_be_bytes_trimmed(), &[0xffu8] as &[u8]);
    }

    #[test]
    fn two_bytes() {
        let felt = Felt::from_u64(0x0102);
        assert_eq!(felt.as_be_bytes_trimmed(), &[0x01u8, 0x02] as &[u8]);
    }

    #[test]
    fn u64_max() {
        let felt = Felt::from_u64(u64::MAX);
        assert_eq!(felt.as_be_bytes_trimmed(), &[0xffu8; 8] as &[u8]);
    }

    #[test]
    fn leading_zero_nibble() {
        // 0x0a should still trim to a single byte
        let felt = Felt::from_u64(0x0a);
        assert_eq!(felt.as_be_bytes_trimmed(), &[0x0au8] as &[u8]);
    }

    #[test]
    fn large_value() {
        let felt = Felt::from_hex_unchecked(
            "0x800000000000010ffffffffffffffffb781126dcae7b2321e66a241adc64d2f",
        );
        assert_eq!(felt.as_be_bytes_trimmed().len(), 32);
        assert_eq!(felt.as_be_bytes_trimmed(), felt.as_be_bytes().as_slice());
    }

    #[test]
    fn round_trip_consistency() {
        let felt = Felt::from_u64(12345);
        let trimmed = felt.as_be_bytes_trimmed();
        let restored = Felt::from_be_bytes_slice(trimmed).unwrap();
        assert_eq!(felt, restored);
    }
}
