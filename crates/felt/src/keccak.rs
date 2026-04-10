/// Const-evaluable Keccak-256 implementation.
///
/// Produces the same output as the standard Keccak-256 (used by Ethereum / Starknet).

const ROUND_CONSTANTS: [u64; 24] = [
    0x0000000000000001, 0x0000000000008082, 0x800000000000808A, 0x8000000080008000,
    0x000000000000808B, 0x0000000080000001, 0x8000000080008081, 0x8000000000008009,
    0x000000000000008A, 0x0000000000000088, 0x0000000080008009, 0x000000008000000A,
    0x000000008000808B, 0x800000000000008B, 0x8000000000008089, 0x8000000000008003,
    0x8000000000008002, 0x8000000000000080, 0x000000000000800A, 0x800000008000000A,
    0x8000000080008081, 0x8000000000008080, 0x0000000080000001, 0x8000000080008008,
];

const ROTATION_OFFSETS: [[u32; 5]; 5] = [
    [0, 1, 62, 28, 27],
    [36, 44, 6, 55, 20],
    [3, 10, 43, 25, 39],
    [41, 45, 15, 21, 8],
    [18, 2, 61, 56, 14],
];

const fn keccak_f(state: &mut [u64; 25]) {
    let mut round = 0;
    while round < 24 {
        // θ (theta)
        let mut c = [0u64; 5];
        let mut x = 0;
        while x < 5 {
            c[x] = state[x] ^ state[x + 5] ^ state[x + 10] ^ state[x + 15] ^ state[x + 20];
            x += 1;
        }
        let mut d = [0u64; 5];
        x = 0;
        while x < 5 {
            d[x] = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);
            x += 1;
        }
        x = 0;
        while x < 5 {
            let mut y = 0;
            while y < 5 {
                state[x + 5 * y] ^= d[x];
                y += 1;
            }
            x += 1;
        }

        // ρ (rho) and π (pi)
        let mut b = [0u64; 25];
        x = 0;
        while x < 5 {
            let mut y = 0;
            while y < 5 {
                b[y + 5 * ((2 * x + 3 * y) % 5)] =
                    state[x + 5 * y].rotate_left(ROTATION_OFFSETS[y][x]);
                y += 1;
            }
            x += 1;
        }

        // χ (chi)
        x = 0;
        while x < 5 {
            let mut y = 0;
            while y < 5 {
                state[x + 5 * y] = b[x + 5 * y] ^ (!b[(x + 1) % 5 + 5 * y] & b[(x + 2) % 5 + 5 * y]);
                y += 1;
            }
            x += 1;
        }

        // ι (iota)
        state[0] ^= ROUND_CONSTANTS[round];
        round += 1;
    }
}

/// Const keccak-256: absorb `data`, return 32-byte digest.
pub const fn keccak256(data: &[u8]) -> [u8; 32] {
    const RATE: usize = 136; // (1600 - 2*256) / 8

    let mut state = [0u64; 25];
    let len = data.len();

    // Absorb full blocks
    let mut offset = 0;
    while offset + RATE <= len {
        let mut i = 0;
        while i < RATE / 8 {
            let base = offset + i * 8;
            let word = u64::from_le_bytes([
                data[base],
                data[base + 1],
                data[base + 2],
                data[base + 3],
                data[base + 4],
                data[base + 5],
                data[base + 6],
                data[base + 7],
            ]);
            state[i] ^= word;
            i += 1;
        }
        keccak_f(&mut state);
        offset += RATE;
    }

    // Pad: copy remaining bytes into a block, apply keccak padding (0x01...0x80)
    let remaining = len - offset;
    let mut last_block = [0u8; RATE];
    let mut i = 0;
    while i < remaining {
        last_block[i] = data[offset + i];
        i += 1;
    }
    last_block[remaining] = 0x01;
    last_block[RATE - 1] |= 0x80;

    // XOR last block into state
    i = 0;
    while i < RATE / 8 {
        let base = i * 8;
        let word = u64::from_le_bytes([
            last_block[base],
            last_block[base + 1],
            last_block[base + 2],
            last_block[base + 3],
            last_block[base + 4],
            last_block[base + 5],
            last_block[base + 6],
            last_block[base + 7],
        ]);
        state[i] ^= word;
        i += 1;
    }
    keccak_f(&mut state);

    // Squeeze: extract 32 bytes (4 lanes)
    let mut out = [0u8; 32];
    i = 0;
    while i < 4 {
        let bytes = state[i].to_le_bytes();
        out[i * 8] = bytes[0];
        out[i * 8 + 1] = bytes[1];
        out[i * 8 + 2] = bytes[2];
        out[i * 8 + 3] = bytes[3];
        out[i * 8 + 4] = bytes[4];
        out[i * 8 + 5] = bytes[5];
        out[i * 8 + 6] = bytes[6];
        out[i * 8 + 7] = bytes[7];
        i += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        // Known keccak256("") = c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470
        let hash = keccak256(b"");
        let expected = [
            0xc5, 0xd2, 0x46, 0x01, 0x86, 0xf7, 0x23, 0x3c,
            0x92, 0x7e, 0x7d, 0xb2, 0xdc, 0xc7, 0x03, 0xc0,
            0xe5, 0x00, 0xb6, 0x53, 0xca, 0x82, 0x27, 0x3b,
            0x7b, 0xfa, 0xd8, 0x04, 0x5d, 0x85, 0xa4, 0x70,
        ];
        assert_eq!(hash, expected);
    }

    #[test]
    fn transfer() {
        // keccak256("Transfer") — used as a Starknet event selector
        let hash = keccak256(b"Transfer");
        // Verify it at compile time too
        const HASH: [u8; 32] = keccak256(b"Transfer");
        assert_eq!(hash, HASH);
    }

    #[test]
    fn hello_world() {
        // Known keccak256("hello world")
        let hash = keccak256(b"hello world");
        let expected = [
            0x47, 0x17, 0x32, 0x85, 0xa8, 0xd7, 0x34, 0x1e,
            0x5e, 0x97, 0x2f, 0xc6, 0x77, 0x28, 0x63, 0x84,
            0xf8, 0x02, 0xf8, 0xef, 0x42, 0xa5, 0xec, 0x5f,
            0x03, 0xbb, 0xfa, 0x25, 0x4c, 0xb0, 0x1f, 0xad,
        ];
        assert_eq!(hash, expected);
    }
}
