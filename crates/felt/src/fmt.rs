use super::Felt;

impl std::fmt::Debug for Felt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{self:x}")
    }
}

impl std::fmt::Display for Felt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Decimal conversion of a 256-bit big-endian integer (max 78 digits).
        let mut buf = [0u8; 78];
        let mut pos = buf.len();
        let mut tmp = self.0;

        loop {
            let mut rem: u16 = 0;
            for byte in tmp.iter_mut() {
                let dividend = rem * 256 + *byte as u16;
                *byte = (dividend / 10) as u8;
                rem = dividend % 10;
            }
            pos -= 1;
            buf[pos] = b'0' + rem as u8;

            if tmp.iter().all(|&b| b == 0) {
                break;
            }
        }

        // SAFETY: buf only contains ASCII digit bytes
        let s = unsafe { core::str::from_utf8_unchecked(&buf[pos..]) };
        f.pad_integral(true, "", s)
    }
}

impl std::fmt::LowerHex for Felt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = &self.0;
        let first = bytes.iter().position(|&b| b != 0).unwrap_or(31);
        // Build the raw hex digits into a stack buffer
        let mut buf = [0u8; 64];
        let mut pos = 0;
        pos += fmt_byte_lower_first(bytes[first], &mut buf[pos..]);
        for &b in &bytes[first + 1..] {
            let hi = b >> 4;
            let lo = b & 0x0f;
            buf[pos] = hex_lower(hi);
            buf[pos + 1] = hex_lower(lo);
            pos += 2;
        }
        // SAFETY: hex_lower only produces ASCII bytes
        let s = unsafe { core::str::from_utf8_unchecked(&buf[..pos]) };
        f.pad_integral(true, "0x", s)
    }
}

impl std::fmt::UpperHex for Felt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = &self.0;
        let first = bytes.iter().position(|&b| b != 0).unwrap_or(31);
        let mut buf = [0u8; 64];
        let mut pos = 0;
        pos += fmt_byte_upper_first(bytes[first], &mut buf[pos..]);
        for &b in &bytes[first + 1..] {
            let hi = b >> 4;
            let lo = b & 0x0f;
            buf[pos] = hex_upper(hi);
            buf[pos + 1] = hex_upper(lo);
            pos += 2;
        }
        let s = unsafe { core::str::from_utf8_unchecked(&buf[..pos]) };
        f.pad_integral(true, "0x", s)
    }
}

#[inline]
fn hex_lower(nibble: u8) -> u8 {
    if nibble < 10 {
        b'0' + nibble
    } else {
        b'a' + nibble - 10
    }
}

#[inline]
fn hex_upper(nibble: u8) -> u8 {
    if nibble < 10 {
        b'0' + nibble
    } else {
        b'A' + nibble - 10
    }
}

/// Formats the first byte without a leading zero, returns number of bytes written.
#[inline]
fn fmt_byte_lower_first(b: u8, buf: &mut [u8]) -> usize {
    let hi = b >> 4;
    let lo = b & 0x0f;
    if hi != 0 {
        buf[0] = hex_lower(hi);
        buf[1] = hex_lower(lo);
        2
    } else {
        buf[0] = hex_lower(lo);
        1
    }
}

#[inline]
fn fmt_byte_upper_first(b: u8, buf: &mut [u8]) -> usize {
    let hi = b >> 4;
    let lo = b & 0x0f;
    if hi != 0 {
        buf[0] = hex_upper(hi);
        buf[1] = hex_upper(lo);
        2
    } else {
        buf[0] = hex_upper(lo);
        1
    }
}
