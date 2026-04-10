use crate::keccak::keccak256;

use crate::error::{OverflowError, StrError};

/// Starknet Field Element.
///
/// A field element is a number 0..p-1 with p=2^{251}+17*2^{192}+1, and it forms
/// the basic building block of most Starknet interactions.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct Felt(pub(crate) [u8; 32]);

pub const MODULUS_BE_LIMBS: [u64; 4] = [576460752303423505u64, 0, 0, 1];
pub const MODULUS_LE_LIMBS: [u64; 4] = [1, 0, 0, 576460752303423505u64];
pub const MODULUS_BE_BYTES: [u8; 32] = [
    8, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
];
pub const MODULUS_LE_BYTES: [u8; 32] = [
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 8,
];

impl AsRef<Felt> for Felt {
    fn as_ref(&self) -> &Felt {
        self
    }
}

impl AsRef<[u8; 32]> for Felt {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl Felt {
    pub const ZERO: Felt = Felt([0u8; 32]);
    pub const ONE: Felt = Self::from_u64(1);
    pub const TWO: Felt = Self::from_u64(2);
    pub const THREE: Felt = Self::from_u64(3);

    /// Return true if the element is zero.
    pub const fn is_zero(&self) -> bool {
        let mut i = 0;
        while i < 32 {
            if self.0[i] != 0 {
                return false;
            }
            i += 1;
        }
        true
    }

    /// Returns `true` if the value is in the valid field range `[0, p)`.
    pub const fn is_valid(&self) -> bool {
        !bytes_ge_modulus(&self.0)
    }

    /// Reduces the value modulo the field prime, returning a valid field element.
    ///
    /// If the value is already < p this is a no-op. Otherwise subtracts p
    /// until it is (at most 31 iterations since the value fits in 256 bits).
    pub const fn reduce(self) -> Self {
        let mut bytes = self.0;
        while bytes_ge_modulus(&bytes) {
            bytes = sub_modulus(bytes);
        }
        Felt(bytes)
    }

    pub const fn valid(self) -> Result<Self, OverflowError> {
        if self.is_valid() {
            Ok(self)
        } else {
            Err(OverflowError)
        }
    }

    /// Returns the big-endian representation of this [Felt].
    pub const fn to_be_bytes(self) -> [u8; 32] {
        self.0
    }

    /// Returns the little-endian representation of this [Felt].
    pub const fn to_le_bytes(self) -> [u8; 32] {
        let b = self.0;
        [
            b[31], b[30], b[29], b[28], b[27], b[26], b[25], b[24], b[23], b[22], b[21], b[20],
            b[19], b[18], b[17], b[16], b[15], b[14], b[13], b[12], b[11], b[10], b[9], b[8], b[7],
            b[6], b[5], b[4], b[3], b[2], b[1], b[0],
        ]
    }

    /// Big-endian representation of this [Felt].
    pub const fn as_be_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub const fn as_be_bytes_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn to_be_bytes_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn to_le_bytes_vec(&self) -> Vec<u8> {
        let mut tmp = self.0;
        tmp.reverse();
        tmp.to_vec()
    }

    /// Big-endian mutable representation of this [Felt].
    pub fn as_mut_be_bytes(&mut self) -> &mut [u8; 32] {
        &mut self.0
    }

    /// Returns 4 big-endian u64 limbs: `[0]` is the most significant word.
    pub const fn to_be_words(&self) -> [u64; 4] {
        [
            u64::from_be_bytes([
                self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6],
                self.0[7],
            ]),
            u64::from_be_bytes([
                self.0[8], self.0[9], self.0[10], self.0[11], self.0[12], self.0[13], self.0[14],
                self.0[15],
            ]),
            u64::from_be_bytes([
                self.0[16], self.0[17], self.0[18], self.0[19], self.0[20], self.0[21], self.0[22],
                self.0[23],
            ]),
            u64::from_be_bytes([
                self.0[24], self.0[25], self.0[26], self.0[27], self.0[28], self.0[29], self.0[30],
                self.0[31],
            ]),
        ]
    }

    /// Returns 4 little-endian u64 limbs: `[0]` is the least significant word.
    pub const fn to_le_words(&self) -> [u64; 4] {
        [
            u64::from_be_bytes([
                self.0[24], self.0[25], self.0[26], self.0[27], self.0[28], self.0[29], self.0[30],
                self.0[31],
            ]),
            u64::from_be_bytes([
                self.0[16], self.0[17], self.0[18], self.0[19], self.0[20], self.0[21], self.0[22],
                self.0[23],
            ]),
            u64::from_be_bytes([
                self.0[8], self.0[9], self.0[10], self.0[11], self.0[12], self.0[13], self.0[14],
                self.0[15],
            ]),
            u64::from_be_bytes([
                self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6],
                self.0[7],
            ]),
        ]
    }

    /// Creates a [Felt] from 4 big-endian u64 limbs: `words[0]` is the most significant.
    pub const fn from_be_words(words: [u64; 4]) -> Self {
        let w0 = words[0].to_be_bytes();
        let w1 = words[1].to_be_bytes();
        let w2 = words[2].to_be_bytes();
        let w3 = words[3].to_be_bytes();
        Self([
            w0[0], w0[1], w0[2], w0[3], w0[4], w0[5], w0[6], w0[7], w1[0], w1[1], w1[2], w1[3],
            w1[4], w1[5], w1[6], w1[7], w2[0], w2[1], w2[2], w2[3], w2[4], w2[5], w2[6], w2[7],
            w3[0], w3[1], w3[2], w3[3], w3[4], w3[5], w3[6], w3[7],
        ])
    }

    /// Creates a [Felt] from 4 little-endian u64 limbs: `words[0]` is the least significant.
    pub const fn from_le_words(words: [u64; 4]) -> Self {
        let w0 = words[3].to_be_bytes();
        let w1 = words[2].to_be_bytes();
        let w2 = words[1].to_be_bytes();
        let w3 = words[0].to_be_bytes();
        Self([
            w0[0], w0[1], w0[2], w0[3], w0[4], w0[5], w0[6], w0[7], w1[0], w1[1], w1[2], w1[3],
            w1[4], w1[5], w1[6], w1[7], w2[0], w2[1], w2[2], w2[3], w2[4], w2[5], w2[6], w2[7],
            w3[0], w3[1], w3[2], w3[3], w3[4], w3[5], w3[6], w3[7],
        ])
    }

    /// Creates a [Felt] from big-endian bytes. Does not validate against the
    /// field modulus. Use [`is_valid`](Self::is_valid) or [`reduce`](Self::reduce)
    /// if the input may be >= p.
    pub const fn from_be_bytes(bytes: [u8; 32]) -> Self {
        Felt(bytes)
    }

    /// Creates a Felt from a big-endian byte slice of up to 32 bytes.
    /// Returns [`OverflowError`] if the slice is longer than 32 bytes.
    /// Does not validate against the field modulus.
    pub fn from_be_bytes_slice(bytes: &[u8]) -> Result<Self, OverflowError> {
        if bytes.len() > 32 {
            return Err(OverflowError);
        }
        let mut buf = [0u8; 32];
        buf[32 - bytes.len()..].copy_from_slice(bytes);
        Ok(Self::from_be_bytes(buf))
    }

    /// Creates a Felt from a little-endian byte slice of up to 32 bytes.
    /// Returns [`OverflowError`] if the slice is longer than 32 bytes.
    /// Does not validate against the field modulus.
    pub fn from_le_bytes_slice(bytes: &[u8]) -> Result<Self, OverflowError> {
        if bytes.len() > 32 {
            return Err(OverflowError);
        }
        let mut buf = [0u8; 32];
        for (i, &b) in bytes.iter().enumerate() {
            buf[31 - i] = b;
        }
        Ok(Self::from_be_bytes(buf))
    }

    /// Creates a Felt from a big-endian byte slice of up to 32 bytes.
    /// Panics if slice is longer than 32 bytes. Does not check for overflow.
    pub fn from_be_bytes_slice_unchecked(bytes: &[u8]) -> Self {
        assert!(bytes.len() <= 32, "slice too long");
        let mut buf = [0u8; 32];
        buf[32 - bytes.len()..].copy_from_slice(bytes);
        Self(buf)
    }

    /// Creates a Felt from a little-endian byte slice of up to 32 bytes.
    /// Panics if slice is longer than 32 bytes. Does not check for overflow.
    pub fn from_le_bytes_slice_unchecked(bytes: &[u8]) -> Self {
        assert!(bytes.len() <= 32, "slice too long");
        let mut buf = [0u8; 32];
        for (i, &b) in bytes.iter().enumerate() {
            buf[31 - i] = b;
        }
        Self(buf)
    }

    /// Returns `true` if the value exceeds 251 bits (i.e. is larger than `2^251 - 1`).
    ///
    /// Every [`Felt`] that is used to traverse a Merkle-Patricia Tree
    /// must not exceed 251 bits, since 251 is the height of the tree.
    pub const fn exceeds_251_bits(&self) -> bool {
        self.0[0] & 0b1111_1000 > 0
    }
    /// Returns the number of leading zero bytes. If the value is zero, returns 32.
    pub const fn first_non_zero_byte(&self) -> usize {
        let mut i = 0;
        while i < 32 {
            if self.0[i] != 0 {
                return i;
            }
            i += 1;
        }
        32
    }

    pub const fn as_be_bytes_trimmed(&self) -> &[u8] {
        self.0.as_slice().split_at(self.first_non_zero_byte()).1
    }

    pub const fn from_u64(u: u64) -> Self {
        let bytes = u.to_be_bytes();
        Self([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, bytes[0],
            bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ])
    }

    pub const fn from_u128(u: u128) -> Self {
        let bytes = u.to_be_bytes();
        Self([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7], bytes[8], bytes[9], bytes[10], bytes[11],
            bytes[12], bytes[13], bytes[14], bytes[15],
        ])
    }
    /// Like [`from_hex`](Self::from_hex) but does not accept a "0x" prefix.
    pub const fn from_hex_unprefixed(hex_str: &str) -> Result<Self, StrError> {
        Self::from_hex_bytes(hex_str.as_bytes())
    }
    pub const fn from_hex_bytes(bytes: &[u8]) -> Result<Self, StrError> {
        const fn nibble(b: u8) -> Result<u8, StrError> {
            match b {
                b'0'..=b'9' => Ok(b - b'0'),
                b'A'..=b'F' => Ok(b - b'A' + 10),
                b'a'..=b'f' => Ok(b - b'a' + 10),
                other => Err(StrError::InvalidNibble(other)),
            }
        }
        let len = bytes.len();
        if len > 64 {
            return Err(StrError::InvalidLength {
                max: 64,
                actual: bytes.len(),
            });
        }
        let mut buf = [0u8; 32];
        let mut pos = 32 - (len + 1) / 2;
        let mut i = 0;
        if len % 2 == 1 {
            buf[pos] = match nibble(bytes[i]) {
                Ok(b) => b,
                Err(e) => return Err(e),
            };
            pos += 1;
            i += 1;
        }

        while i < len {
            let hi = match nibble(bytes[i]) {
                Ok(b) => b,
                Err(e) => return Err(e),
            };
            let lo = match nibble(bytes[i + 1]) {
                Ok(b) => b,
                Err(e) => return Err(e),
            };
            buf[pos] = (hi << 4) | lo;
            pos += 1;
            i += 2;
        }

        Ok(Felt::from_be_bytes(buf))
    }

    /// A convenience function which parses a hex string into a [Felt].
    ///
    /// Supports both upper and lower case hex strings, as well as an optional
    /// "0x" prefix.
    pub const fn from_hex(hex_str: &str) -> Result<Self, StrError> {
        let bytes = hex_str.as_bytes();
        let start = if bytes.len() >= 2 && bytes[0] == b'0' && bytes[1] == b'x' {
            2
        } else {
            0
        };

        Self::from_hex_bytes(&bytes.split_at(start).1)
    }

    pub const fn from_hex_unchecked(hex_str: &str) -> Self {
        const_expect!(
            Self::from_hex(hex_str),
            "invalid hex string or value exceeds field modulus"
        )
    }

    /// Parses a decimal string into a [Felt].
    ///
    /// Returns [FromStrError] if the string contains non-digit characters,
    /// is empty, or the value exceeds the field modulus.
    pub fn from_dec_str(dec_str: &str) -> Result<Self, StrError> {
        let bytes = dec_str.as_bytes();
        if bytes.is_empty() {
            return Err(StrError::EmptyString);
        }

        // Accumulate into 4 big-endian u64 limbs via multiply-by-10 + add-digit.
        let mut limbs = [0u64; 4];
        for &b in bytes {
            let digit = match b {
                b'0'..=b'9' => (b - b'0') as u64,
                _ => return Err(StrError::InvalidDigit(b)),
            };

            // limbs = limbs * 10 + digit, propagating carries from low to high.
            let mut carry = digit;
            let mut i: usize = 3;
            loop {
                let wide = (limbs[i] as u128) * 10 + carry as u128;
                limbs[i] = wide as u64;
                carry = (wide >> 64) as u64;
                if i == 0 {
                    break;
                }
                i -= 1;
            }
            if carry != 0 {
                return Err(StrError::Overflow(carry));
            }
        }

        Ok(Felt::from_be_words(limbs))
    }

    /// The first stage of conversion - skip leading zeros.
    /// Caller must ensure `self` is not zero.
    fn skip_zeros(&self) -> (impl Iterator<Item = &u8>, usize, usize) {
        // Skip all leading zero bytes
        let it = self.0.iter().skip_while(|&&b| b == 0);
        let num_bytes = it.clone().count();
        let skipped = self.0.len() - num_bytes;
        // The first high nibble can be 0
        let start = if self.0[skipped] < 0x10 { 1 } else { 2 };
        // Number of characters to display
        let len = start + num_bytes * 2;
        (it, start, len)
    }

    /// The second stage of conversion - map bytes to hex str
    fn it_to_hex_str<'a>(
        it: impl Iterator<Item = &'a u8>,
        start: usize,
        len: usize,
        buf: &'a mut [u8],
    ) -> &'a [u8] {
        const LUT: [u8; 16] = *b"0123456789abcdef";
        buf[0] = b'0';
        // Same small lookup table is ~25% faster than hex::encode_from_slice 🤷
        it.enumerate().for_each(|(i, &b)| {
            let idx = b as usize;
            let pos = start + i * 2;
            let x = [LUT[(idx & 0xf0) >> 4], LUT[idx & 0x0f]];
            buf[pos..pos + 2].copy_from_slice(&x);
        });
        buf[1] = b'x';
        &buf[..len]
    }

    /// A convenience function which produces a "0x" prefixed hex str slice in a
    /// given buffer `buf` from a [Felt].
    /// Panics if `self.0.len() * 2 + 2 > buf.len()`
    pub fn as_hex_str<'a>(&'a self, buf: &'a mut [u8]) -> &'a str {
        let expected_buf_len = self.0.len() * 2 + 2;
        assert!(
            buf.len() >= expected_buf_len,
            "buffer size is {}, expected at least {}",
            buf.len(),
            expected_buf_len
        );

        if self.is_zero() {
            return "0x0";
        }

        let (it, start, len) = self.skip_zeros();
        let res = Self::it_to_hex_str(it, start, len, buf);
        // Safety: buf contains only ASCII hex digits, '0', and 'x'.
        unsafe { core::str::from_utf8_unchecked(res) }
    }

    /// A convenience function which produces a "0x" prefixed hex string from a
    /// [Felt].
    pub fn to_hex_string(&self) -> String {
        if self.is_zero() {
            return "0x0".to_string();
        }
        let (it, start, len) = self.skip_zeros();
        let mut buf = vec![0u8; len];
        Self::it_to_hex_str(it, start, len, &mut buf);
        // Unwrap is safe as the buffer contains valid utf8
        String::from_utf8(buf).unwrap()
    }

    /// Computes a Starknet selector from a function/event name.
    ///
    /// This is `keccak256(name)` with the top 6 bits cleared, matching
    /// `starknet::core::utils::get_selector_from_name`.
    pub const fn selector(name: &str) -> Self {
        let mut hash = keccak256(name.as_bytes());
        hash[0] &= 0b0000_0011;
        Felt(hash)
    }

    pub const fn from_short_ascii_str(s: &str) -> Result<Self, StrError> {
        let bytes = s.as_bytes();
        if bytes.len() > 31 {
            return Err(StrError::InvalidLength {
                max: 31,
                actual: bytes.len(),
            });
        }
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] > 127 {
                return Err(StrError::NonAsciiCharacter);
            }
            i += 1;
        }
        let mut buf = [0u8; 32];
        let offset = 32 - bytes.len();
        i = 0;
        while i < bytes.len() {
            buf[offset + i] = bytes[i];
            i += 1;
        }
        Ok(Self(buf))
    }

    /// Like [`from_short_ascii_str`](Self::from_short_ascii_str) but without
    /// length or ASCII validation. The caller must ensure `s.len() <= 31`
    /// and all bytes are ASCII.
    pub const fn from_short_ascii_str_unchecked(s: &str) -> Self {
        let bytes = s.as_bytes();
        debug_assert!(bytes.len() <= 31);
        let mut buf = [0u8; 32];
        let offset = 32 - bytes.len();
        let mut i = 0;
        while i < bytes.len() {
            debug_assert!(bytes[i] <= 127);
            buf[offset + i] = bytes[i];
            i += 1;
        }
        Self(buf)
    }

    pub const fn as_short_ascii_str(&self) -> Result<&str, StrError> {
        let bytes = &self.0;
        let mut start = 0;
        while start < 32 {
            if bytes[start] != 0 {
                break;
            }
            start += 1;
        }
        if start == 32 {
            return Ok("");
        }
        let mut i = start;
        while i < 32 {
            if bytes[i] > 127 {
                return Err(StrError::NonAsciiCharacter);
            }
            i += 1;
        }
        // Safety: all bytes from start..32 are verified ≤ 127, which is valid UTF-8.
        let (_, tail) = bytes.split_at(start);
        Ok(unsafe { core::str::from_utf8_unchecked(tail) })
    }

    /// Like [`as_short_ascii_str`](Self::as_short_ascii_str) but without
    /// ASCII validation. The caller must ensure the non-zero bytes are valid ASCII.
    pub const fn as_short_ascii_str_unchecked(&self) -> &str {
        let bytes = &self.0;
        let mut start = 0;
        while start < 32 {
            if bytes[start] != 0 {
                break;
            }
            start += 1;
        }
        // Safety: caller guarantees all non-zero bytes are ASCII (≤ 127).
        let (_, tail) = bytes.split_at(start);
        unsafe { core::str::from_utf8_unchecked(tail) }
    }

    pub fn to_fixed_hex_string(&self) -> String {
        const LUT: [u8; 16] = *b"0123456789abcdef";
        let mut buf = [0u8; 64];
        let mut i = 0;
        while i < 32 {
            let b = self.0[i] as usize;
            buf[i * 2] = LUT[b >> 4];
            buf[i * 2 + 1] = LUT[b & 0x0f];
            i += 1;
        }
        // Safety: buf contains only ASCII hex digits.
        unsafe { String::from_utf8_unchecked(buf.to_vec()) }
    }
}

impl Default for Felt {
    fn default() -> Self {
        Felt::ZERO
    }
}

macro_rules! const_expect {
    ($e:expr, $why:expr) => {{
        match $e {
            Ok(x) => x,
            Err(_) => panic!(concat!("Expectation failed: ", $why)),
        }
    }};
}

use const_expect;

/// Returns true if limbs >= MODULUS_BE_LIMBS (big-endian comparison).
pub const fn limbs_ge_modulus(limbs: &[u64; 4]) -> bool {
    let mut i = 0;
    while i < 4 {
        if limbs[i] < MODULUS_BE_LIMBS[i] {
            return false;
        }
        if limbs[i] > MODULUS_BE_LIMBS[i] {
            return true;
        }
        i += 1;
    }
    true // equal
}

pub const fn bytes_ge_modulus(bytes: &[u8; 32]) -> bool {
    let mut i = 0;
    while i < 32 {
        if bytes[i] < MODULUS_BE_BYTES[i] {
            return false;
        }
        if bytes[i] > MODULUS_BE_BYTES[i] {
            return true;
        }
        i += 1;
    }
    true // equal to modulus
}

/// Subtracts the field modulus from a 256-bit big-endian value.
const fn sub_modulus(bytes: [u8; 32]) -> [u8; 32] {
    let mut result = [0u8; 32];
    let mut borrow: u16 = 0;
    let mut i = 31;
    loop {
        let val = bytes[i] as u16;
        let sub = MODULUS_BE_BYTES[i] as u16 + borrow;
        if val >= sub {
            result[i] = (val - sub) as u8;
            borrow = 0;
        } else {
            result[i] = (256 + val - sub) as u8;
            borrow = 1;
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
    result
}
