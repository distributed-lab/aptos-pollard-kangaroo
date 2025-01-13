#![allow(non_snake_case)]

use curve25519_dalek_ng::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek_ng::scalar::Scalar;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Kangaroo {
    pub parameters: Parameters,
    pub table: Table,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Parameters {
    pub i: u64,
    pub W: u64,
    pub N: u64,
    pub R: u64,
    pub secret_size: u8,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", serde_as)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Table {
    pub s: Vec<RistrettoPoint>,
    pub slog: Vec<Scalar>,
    #[cfg_attr(feature = "serde", serde_as(as = "Vec<(_, _)>"))]
    pub table: HashMap<CompressedRistretto, Scalar>,
}

// pub const KANGAROO_16: &[u8] = include_bytes!("rsc/table_16");
// pub const KANGAROO_32: &[u8] = include_bytes!("rsc/table_32");
pub const KANGAROO_48: &[u8] = include_bytes!("rsc/table_48");

fn main() {
    let kangaroo: Kangaroo =
        bincode::deserialize(KANGAROO_48).expect("failed to deserialize table");

    let path = Path::new(".").join("codegen.rs");
    println!("path: {:?}", path);
    let mut file = BufWriter::new(File::create(&path).unwrap());

    writeln!(
        &mut file,
        "static KANGAROO_48_S: [[u8; 32]; {}] = {:?};",
        kangaroo.table.s.len(),
        kangaroo
            .table
            .s
            .into_iter()
            .map(|p| p.compress().to_bytes())
            .collect::<Vec<_>>()
    )
    .unwrap();

    writeln!(
        &mut file,
        "static KANGAROO_48_SLOG: [[u8; 32]; {}] = {:?};",
        kangaroo.table.slog.len(),
        kangaroo
            .table
            .slog
            .into_iter()
            .map(|s| s.to_bytes())
            .collect::<Vec<_>>()
    )
    .unwrap();

    writeln!(
        &mut file,
        "static KANGAROO_48_TABLE: [([u8; 32], [u8; 32]); {}] = {:?};",
        kangaroo.table.table.len(),
        kangaroo
            .table
            .table
            .into_iter()
            .map(|(k, v)| (k.to_bytes(), v.to_bytes()))
            .collect::<Vec<_>>()
    )
    .unwrap();
}
