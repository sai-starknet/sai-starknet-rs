use super::{Felt, StrError, PrimitiveFromFeltError};

impl From<bool> for Felt {
    fn from(value: bool) -> Self {
        if value {
            Self::ONE
        } else {
            Self::ZERO
        }
    }
}

impl From<u8> for Felt {
    fn from(value: u8) -> Self {
        Self::from_u64(value as u64)
    }
}

impl From<u16> for Felt {
    fn from(value: u16) -> Self {
        Self::from_u64(value as u64)
    }
}

impl From<u32> for Felt {
    fn from(value: u32) -> Self {
        Self::from_u64(value as u64)
    }
}

impl From<u64> for Felt {
    fn from(value: u64) -> Self {
        Self::from_u64(value)
    }
}

impl From<usize> for Felt {
    fn from(value: usize) -> Self {
        Self::from_u64(value.try_into().expect("ptr size is 64 bits"))
    }
}

impl From<u128> for Felt {
    fn from(value: u128) -> Self {
        Self::from_u128(value)
    }
}

impl From<[u8; 32]> for Felt {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

impl From<[u8; 31]> for Felt {
    fn from(bytes: [u8; 31]) -> Self {
        let mut buf = [0u8; 32];
        buf[1..].copy_from_slice(&bytes);
        Self(buf)
    }
}

impl From<i128> for Felt {
    fn from(value: i128) -> Self {
        let abs = Felt::from_u128(value.unsigned_abs());
        if value.is_negative() {
            -abs
        } else {
            abs
        }
    }
}

impl From<i64> for Felt {
    fn from(value: i64) -> Self {
        Felt::from(value as i128)
    }
}

impl From<i32> for Felt {
    fn from(value: i32) -> Self {
        Felt::from(value as i128)
    }
}

impl From<i16> for Felt {
    fn from(value: i16) -> Self {
        Felt::from(value as i128)
    }
}

impl From<i8> for Felt {
    fn from(value: i8) -> Self {
        Felt::from(value as i128)
    }
}

impl From<Felt> for Vec<u8> {
    fn from(value: Felt) -> Self {
        value.0.to_vec()
    }
}

impl From<&Felt> for Vec<u8> {
    fn from(value: &Felt) -> Self {
        value.0.to_vec()
    }
}

impl From<Felt> for [u8; 32] {
    fn from(value: Felt) -> Self {
        value.0
    }
}

impl From<&Felt> for [u8; 32] {
    fn from(value: &Felt) -> Self {
        value.0
    }
}

impl TryFrom<Felt> for u8 {
    type Error = PrimitiveFromFeltError;

    fn try_from(felt: Felt) -> Result<u8, Self::Error> {
        match felt.0 {
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, v] => {
                Ok(v)
            }
            _ => Err(PrimitiveFromFeltError),
        }
    }
}

impl TryFrom<Felt> for u16 {
    type Error = PrimitiveFromFeltError;

    fn try_from(felt: Felt) -> Result<u16, Self::Error> {
        if felt.0[..30] != [0u8; 30] {
            return Err(PrimitiveFromFeltError);
        }
        Ok(u16::from_be_bytes([felt.0[30], felt.0[31]]))
    }
}

impl TryFrom<Felt> for u32 {
    type Error = PrimitiveFromFeltError;

    fn try_from(felt: Felt) -> Result<u32, Self::Error> {
        if felt.0[..28] != [0u8; 28] {
            return Err(PrimitiveFromFeltError);
        }
        Ok(u32::from_be_bytes([
            felt.0[28], felt.0[29], felt.0[30], felt.0[31],
        ]))
    }
}

impl TryFrom<Felt> for u64 {
    type Error = PrimitiveFromFeltError;

    fn try_from(felt: Felt) -> Result<u64, Self::Error> {
        if felt.0[..24] != [0u8; 24] {
            return Err(PrimitiveFromFeltError);
        }
        Ok(u64::from_be_bytes([
            felt.0[24], felt.0[25], felt.0[26], felt.0[27], felt.0[28], felt.0[29], felt.0[30],
            felt.0[31],
        ]))
    }
}

impl TryFrom<Felt> for u128 {
    type Error = PrimitiveFromFeltError;

    fn try_from(felt: Felt) -> Result<u128, Self::Error> {
        if felt.0[..16] != [0u8; 16] {
            return Err(PrimitiveFromFeltError);
        }
        Ok(u128::from_be_bytes([
            felt.0[16], felt.0[17], felt.0[18], felt.0[19], felt.0[20], felt.0[21], felt.0[22],
            felt.0[23], felt.0[24], felt.0[25], felt.0[26], felt.0[27], felt.0[28], felt.0[29],
            felt.0[30], felt.0[31],
        ]))
    }
}

impl TryFrom<Felt> for i128 {
    type Error = PrimitiveFromFeltError;

    fn try_from(felt: Felt) -> Result<i128, Self::Error> {
        // Positive: fits in u128 and within i128 range
        if let Ok(v) = u128::try_from(felt) {
            if v <= i128::MAX as u128 {
                return Ok(v as i128);
            }
            return Err(PrimitiveFromFeltError);
        }

        // Negative: negate to get absolute value, then check range
        let abs = u128::try_from(-felt).map_err(|_| PrimitiveFromFeltError)?;
        if abs <= i128::MAX as u128 {
            Ok(-(abs as i128))
        } else if abs == i128::MIN.unsigned_abs() {
            Ok(i128::MIN)
        } else {
            Err(PrimitiveFromFeltError)
        }
    }
}

impl TryFrom<Felt> for i64 {
    type Error = PrimitiveFromFeltError;

    fn try_from(felt: Felt) -> Result<i64, Self::Error> {
        i128::try_from(felt)?
            .try_into()
            .map_err(|_| PrimitiveFromFeltError)
    }
}

impl TryFrom<Felt> for i32 {
    type Error = PrimitiveFromFeltError;

    fn try_from(felt: Felt) -> Result<i32, Self::Error> {
        i128::try_from(felt)?
            .try_into()
            .map_err(|_| PrimitiveFromFeltError)
    }
}

impl TryFrom<Felt> for i16 {
    type Error = PrimitiveFromFeltError;

    fn try_from(felt: Felt) -> Result<i16, Self::Error> {
        i128::try_from(felt)?
            .try_into()
            .map_err(|_| PrimitiveFromFeltError)
    }
}

impl TryFrom<Felt> for i8 {
    type Error = PrimitiveFromFeltError;

    fn try_from(felt: Felt) -> Result<i8, Self::Error> {
        i128::try_from(felt)?
            .try_into()
            .map_err(|_| PrimitiveFromFeltError)
    }
}

impl std::str::FromStr for Felt {
    type Err = StrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("0x") {
            Self::from_hex(s)
        } else {
            Self::from_dec_str(s)
        }
    }
}
