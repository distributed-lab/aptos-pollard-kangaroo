#![allow(non_snake_case)]

use anyhow::Result;
use curve25519_dalek_ng::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek_ng::scalar::Scalar;
use std::collections::HashMap;

pub struct Kangaroo {
    pub(crate) s: Vec<RistrettoPoint>,
    pub(crate) slog: Vec<Scalar>,
    pub(crate) table: HashMap<CompressedRistretto, Scalar>,
}

impl Kangaroo {
    pub fn new(
        s: Vec<RistrettoPoint>,
        slog: Vec<Scalar>,
        table: Vec<(CompressedRistretto, Scalar)>,
    ) -> Kangaroo {
        let mut t = HashMap::new();

        table.into_iter().for_each(|(point, value)| {
            t.insert(point, value);
        });

        Kangaroo { s, slog, table: t }
    }

    // fn s(&self, idx: usize) -> Result<&CompressedRistretto> {
    //     self.s
    //         .get(idx)
    //         .ok_or_else(|| anyhow::anyhow!("out of bounds"))
    // }
    //
    // fn slog(&self, idx: usize) -> Result<&Scalar> {
    //     self.slog
    //         .get(idx)
    //         .ok_or_else(|| anyhow::anyhow!("out of bounds"))
    // }
    //
    // fn value(&self, point: &CompressedRistretto) -> Option<&Scalar> {
    //     self.table.get(point)
    // }
}

pub fn hash(point: &CompressedRistretto, r: u64) -> usize {
    let point = point_to_u64(point);
    (point & (r - 1)) as usize
}

pub fn is_distinguished(point: &CompressedRistretto, w: u64) -> bool {
    let point = point_to_u64(point);
    (point & (w - 1)) == 0
}

pub fn point_to_u64(p: &CompressedRistretto) -> u64 {
    let (_, u64_bytes) = p.as_bytes().split_at(32 - size_of::<u64>());
    u64::from_be_bytes(u64_bytes.try_into().unwrap())
}

pub fn scalar_to_u64(s: &Scalar) -> u64 {
    let (u64_bytes, _) = s.as_bytes().split_at(size_of::<u64>());
    u64::from_le_bytes(u64_bytes.try_into().unwrap())
}