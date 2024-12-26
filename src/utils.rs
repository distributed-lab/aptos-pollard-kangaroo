use core::ops::Mul;
use curve25519_dalek_ng::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek_ng::ristretto::CompressedRistretto;
use curve25519_dalek_ng::scalar::Scalar;
use rand_core::{OsRng, RngCore};

/// Generates a random scalar with the specified number of bits.
pub fn generate_random_scalar(bits: u8) -> Scalar {
    if bits > 64 {
        panic!("Bits must be less than or equal to 64");
    }

    let mut key = [0u8; 32];

    let last_byte = ((bits + 7) >> 3) as usize;
    OsRng.fill_bytes(&mut key[..last_byte]);

    if bits & 0x07 != 0 {
        key[last_byte - 1] &= 1 << (bits & 0x07) - 1;
    }

    Scalar::from_canonical_bytes(key).unwrap_or_else(|| panic!("Failed to generate random scalar"))
}

pub fn generate_keypair(bits: u8) -> (Scalar, CompressedRistretto) {
    let sk = generate_random_scalar(bits);

    (sk, RISTRETTO_BASEPOINT_POINT.mul(sk).compress())
}
