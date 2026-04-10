use crate::felt::{limbs_ge_modulus, Felt, MODULUS_BE_LIMBS};

impl std::ops::Neg for Felt {
    type Output = Felt;

    fn neg(self) -> Felt {
        if self == Felt::ZERO {
            return Felt::ZERO;
        }
        // p - self
        let a = MODULUS_BE_LIMBS;
        let b = self.to_be_words();

        let (d3, borrow) = a[3].overflowing_sub(b[3]);
        let (d2, b1) = a[2].overflowing_sub(b[2]);
        let (d2, b2) = d2.overflowing_sub(borrow as u64);
        let borrow = b1 | b2;
        let (d1, b1) = a[1].overflowing_sub(b[1]);
        let (d1, b2) = d1.overflowing_sub(borrow as u64);
        let borrow = b1 | b2;
        let (d0, _) = a[0].overflowing_sub(b[0]);
        let (d0, _) = d0.overflowing_sub(borrow as u64);

        Felt::from_be_words([d0, d1, d2, d3])
    }
}

impl std::ops::Sub for Felt {
    type Output = Felt;

    fn sub(self, rhs: Self) -> Felt {
        self + (-rhs)
    }
}

impl std::ops::Add for Felt {
    type Output = Felt;

    fn add(self, rhs: Self) -> Felt {
        // Interpret as 4 big-endian u64 limbs (most significant first).
        let a = self.to_be_words();
        let b = rhs.to_be_words();

        // Add limbs right-to-left with carry.
        let (s3, carry) = a[3].overflowing_add(b[3]);
        let (s2, c1) = a[2].overflowing_add(b[2]);
        let (s2, c2) = s2.overflowing_add(carry as u64);
        let carry = c1 | c2;
        let (s1, c1) = a[1].overflowing_add(b[1]);
        let (s1, c2) = s1.overflowing_add(carry as u64);
        let carry = c1 | c2;
        let (s0, c1) = a[0].overflowing_add(b[0]);
        let (s0, c2) = s0.overflowing_add(carry as u64);
        let overflow = c1 | c2;

        // Conditionally subtract the modulus. Since both operands are < p,
        // the sum is < 2p, so at most one subtraction is needed.
        let mut sum = [s0, s1, s2, s3];
        if overflow || limbs_ge_modulus(&sum) {
            let (d3, borrow) = sum[3].overflowing_sub(MODULUS_BE_LIMBS[3]);
            let (d2, b1) = sum[2].overflowing_sub(MODULUS_BE_LIMBS[2]);
            let (d2, b2) = d2.overflowing_sub(borrow as u64);
            let borrow = b1 | b2;
            let (d1, b1) = sum[1].overflowing_sub(MODULUS_BE_LIMBS[1]);
            let (d1, b2) = d1.overflowing_sub(borrow as u64);
            let borrow = b1 | b2;
            let (d0, _) = sum[0].overflowing_sub(MODULUS_BE_LIMBS[0]);
            let (d0, _) = d0.overflowing_sub(borrow as u64);
            sum = [d0, d1, d2, d3];
        }

        Felt::from_be_words(sum)
    }
}
