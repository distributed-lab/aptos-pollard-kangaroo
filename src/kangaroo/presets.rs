use crate::kangaroo::{Kangaroo, Parameters, Table};
use curve25519_dalek_ng::ristretto::CompressedRistretto;
use curve25519_dalek_ng::scalar::Scalar;
use lazy_static::lazy_static;

pub enum Presets {
    #[cfg(feature = "table16")]
    Kangaroo16,
    #[cfg(feature = "table32")]
    Kangaroo32,
    #[cfg(feature = "table48")]
    Kangaroo48,
}

include!(concat!("../../.", "/codegen.rs"));

lazy_static! {
    pub static ref KANGAROO_48: Kangaroo = Kangaroo {
        table: Table {
            s: KANGAROO_48_S
                .iter()
                .map(|p: &[u8; 32]| CompressedRistretto((*p).clone()).decompress().unwrap())
                .collect(),
            slog: KANGAROO_48_SLOG
                .iter()
                .map(|s: &[u8; 32]| Scalar::from_canonical_bytes(*s).unwrap())
                .collect(),
            table: KANGAROO_48_TABLE
                .iter()
                .map(|(c, s): &([u8; 32], [u8; 32])| (
                    CompressedRistretto((*c).clone()),
                    Scalar::from_canonical_bytes((*s).clone()).unwrap()
                ))
                .collect(),
        },
        parameters: PARAMETERS_48,
    };
}

#[cfg(feature = "table16")]
pub const PARAMETERS_16: Parameters = Parameters {
    i: 8,
    W: 8,
    N: 8000,
    R: 64,
    secret_size: 16,
};

#[cfg(feature = "table32")]
pub const PARAMETERS_32: Parameters = Parameters {
    i: 8,
    W: 2048,
    N: 4000,
    R: 128,
    secret_size: 32,
};

#[cfg(feature = "table48")]
pub const PARAMETERS_48: Parameters = Parameters {
    i: 8,
    W: 65536,
    N: 40000,
    R: 128,
    secret_size: 48,
};

// #[cfg(feature = "table16")]
// pub const KANGAROO_16: &[u8] = include_bytes!("rsc/table_16");
//
// #[cfg(feature = "table32")]
// pub const KANGAROO_32: &[u8] = include_bytes!("rsc/table_32");
//
// #[cfg(feature = "table48")]
// pub const KANGAROO_48: &[u8] = include_bytes!("rsc/table_48");
