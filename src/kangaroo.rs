#![allow(non_snake_case)]

pub mod generator;
pub mod presets;
pub mod solver;

use anyhow::{Context, Result};
use curve25519_dalek_ng::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek_ng::scalar::Scalar;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Kangaroo {
    pub parameters: Parameters,
    pub table: Table,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Table {
    pub s: Vec<RistrettoPoint>,
    pub slog: Vec<Scalar>,
    #[serde_as(as = "Vec<(_, _)>")]
    pub table: HashMap<CompressedRistretto, Scalar>,
}

#[derive(Serialize, Deserialize)]
pub struct Parameters {
    pub i: u64,
    pub W: u64,
    pub N: u64,
    pub R: u64,
    pub secret_size: u8,
}

impl Kangaroo {
    pub fn from_table(parameters: Parameters, table: Table) -> Kangaroo {
        // TODO: Add checks that the passed parameters are correct.
        Kangaroo { parameters, table }
    }

    pub fn from_parameters(parameters: Parameters) -> Kangaroo {
        let table = Table::generate(&parameters);

        Kangaroo { parameters, table }
    }

    pub fn from_secret_size(secret_size: u8) -> Result<Kangaroo> {
        let kangaroo_bytes = match secret_size {
            1..=16 => presets::KANGAROO_16,
            17..=32 => presets::KANGAROO_32,
            33..=48 => presets::KANGAROO_48,
            _ => return Err(anyhow::anyhow!("invalid secret size")),
        };

        let kangaroo =
            bincode::deserialize(kangaroo_bytes).with_context(|| "failed to deserialize table")?;

        Ok(kangaroo)
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
