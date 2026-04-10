use super::Felt;

impl std::fmt::Debug for Felt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{self:x}")
    }
}

impl std::fmt::Display for Felt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:x}")
    }
}

impl std::fmt::LowerHex for Felt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = &self.0;
        let first = bytes.iter().position(|&b| b != 0).unwrap_or(31);
        write!(f, "{:x}", bytes[first])?;
        bytes[first + 1..]
            .iter()
            .try_for_each(|&b| write!(f, "{b:02x}"))
    }
}

impl std::fmt::UpperHex for Felt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = &self.0;
        let first = bytes.iter().position(|&b| b != 0).unwrap_or(31);
        write!(f, "{:X}", bytes[first])?;
        bytes[first + 1..]
            .iter()
            .try_for_each(|&b| write!(f, "{b:02X}"))
    }
}
