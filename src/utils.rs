use anyhow::Result;
use core::ops::Mul;
use curve25519_dalek_ng::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek_ng::ristretto::RistrettoPoint;
use curve25519_dalek_ng::scalar::Scalar;
use rand_core::{OsRng, RngCore};

/// Generates a random scalar with the specified number of bits.
pub fn generate_random_scalar(bits: u8) -> Result<Scalar> {
    if bits > 64 {
        return Err(anyhow::anyhow!("bits must be less than or equal to 64"));
    }

    let mut key = [0u8; 32];

    let last_byte = ((bits + 7) >> 3) as usize;
    OsRng.fill_bytes(&mut key[..last_byte]);

    if bits & 0x07 != 0 {
        key[last_byte - 1] &= (1 << (bits & 0x07)) - 1;
    }

    Scalar::from_canonical_bytes(key)
        .ok_or_else(|| anyhow::anyhow!("failed to generate random scalar"))
}

/// Converts a scalar to an u64.
pub fn scalar_to_u64(scalar: &Scalar) -> Result<u64> {
    let (u64_bytes, _) = scalar.as_bytes().split_at(size_of::<u64>());

    Ok(u64::from_le_bytes(u64_bytes.try_into()?))
}

/// Generates a random scalar and its corresponding "public key".
pub fn generate_keypair(bits: u8) -> Result<(Scalar, RistrettoPoint)> {
    let sk = generate_random_scalar(bits)?;

    Ok((sk, RISTRETTO_BASEPOINT_POINT.mul(sk)))
}
