//! DualHash Library
//!
//! Provides two functions:
//! - [`dualhash1024`] -> 1024-bit digest
//! - [`dualhash512_trunc`] -> 512-bit digest (XOR of halves)
//!
//! Construction:
//! ```
//! DualHash-1024(M) = SHA3-512(0x01 || M) || BLAKE3-512(0x02 || M)
//! DualHash-512-trunc(M) = SHA3-512(0x01 || M) XOR BLAKE3-512(0x02 || M)
//! ```

use sha3::{Digest, Sha3_512};
use blake3;

/// Compute 1024-bit digest: SHA3-512 || BLAKE3-512.
pub fn dualhash1024(msg: &[u8]) -> Vec<u8> {
    let mut sha = Sha3_512::new();
    sha.update(&[0x01]);
    sha.update(msg);
    let a = sha.finalize().to_vec();

    let mut bl = blake3::Hasher::new();
    bl.update(&[0x02]);
    bl.update(msg);
    let mut b = vec![0u8; 64];
    bl.finalize_xof().fill(&mut b);

    [a, b].concat()
}

/// Compute 512-bit digest: XOR of SHA3-512 and BLAKE3-512 outputs.
pub fn dualhash512_trunc(msg: &[u8]) -> Vec<u8> {
    let mut sha = Sha3_512::new();
    sha.update(&[0x01]);
    sha.update(msg);
    let a = sha.finalize().to_vec();

    let mut bl = blake3::Hasher::new();
    bl.update(&[0x02]);
    bl.update(msg);
    let mut b = vec![0u8; 64];
    bl.finalize_xof().fill(&mut b);

    a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dualhash_empty() {
        let d = dualhash1024(b"");
        assert_eq!(d.len(), 128);
    }

    #[test]
    fn test_trunc_length() {
        let d = dualhash512_trunc(b"abc");
        assert_eq!(d.len(), 64);
    }
}
