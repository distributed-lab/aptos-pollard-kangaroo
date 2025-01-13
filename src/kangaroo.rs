#![allow(non_snake_case)]

pub mod generator;
#[cfg(feature = "presets")]
pub mod presets;
pub mod solver;

#[cfg(feature = "presets")]
use crate::kangaroo::presets::Presets;

use anyhow::{Context, Result};
use curve25519_dalek_ng::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek_ng::scalar::Scalar;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use serde_with::serde_as;
use std::collections::HashMap;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Kangaroo {
    pub parameters: Parameters,
    pub table: Table,
}

#[cfg_attr(feature = "serde", serde_as)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Table {
    pub s: Vec<RistrettoPoint>,
    pub slog: Vec<Scalar>,
    #[cfg_attr(feature = "serde", serde_as(as = "Vec<(_, _)>"))]
    pub table: HashMap<CompressedRistretto, Scalar>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Parameters {
    pub i: u64,
    pub W: u64,
    pub N: u64,
    pub R: u64,
    pub secret_size: u8,
}

impl Kangaroo {
    pub fn from_parameters(parameters: Parameters) -> Result<Kangaroo> {
        let table = Table::generate(&parameters).context("failed to generate table")?;

        Ok(Kangaroo { parameters, table })
    }

    #[cfg(feature = "presets")]
    pub fn from_preset(preset: Presets) -> Result<&'static Kangaroo> {
        Ok(&presets::KANGAROO_48)
    }
}

fn is_distinguished(compressed_point: &CompressedRistretto, parameters: &Parameters) -> bool {
    let point_bytes = get_last_point_bytes(compressed_point);

    (point_bytes & (parameters.W - 1)) == 0
}

fn hash(compressed_point: &CompressedRistretto, parameters: &Parameters) -> u64 {
    let point_bytes = get_last_point_bytes(compressed_point);

    point_bytes & (parameters.R - 1)
}

fn get_last_point_bytes(compressed_point: &CompressedRistretto) -> u64 {
    let (_, point_bytes) = compressed_point.as_bytes().split_at(32 - size_of::<u64>());

    u64::from_be_bytes(point_bytes.try_into().unwrap())
}
