use crate::kangaroo::{self, Parameters, Table};
use crate::utils;

use curve25519_dalek_ng::{
    constants::RISTRETTO_BASEPOINT_POINT, ristretto::RistrettoPoint, scalar::Scalar,
};
use std::collections::HashMap;
use std::ops::{AddAssign, Mul};

impl Table {
    pub fn generate(parameters: &Parameters) -> Table {
        let (slog, s) = Self::s_values_init(parameters);

        let mut table = HashMap::new();

        while table.len() < parameters.N as usize {
            let mut wlog = utils::generate_random_scalar(parameters.secret_size).unwrap();
            let mut w = RISTRETTO_BASEPOINT_POINT.mul(wlog);

            for _ in 0..parameters.i * parameters.W {
                let w_compressed = w.compress();

                if kangaroo::is_distinguished(&w_compressed, parameters) {
                    table.insert(w_compressed, wlog);

                    break;
                }

                let h = kangaroo::hash(&w_compressed, parameters) as usize;

                wlog.add_assign(slog[h]);
                w.add_assign(s[h]);
            }
        }

        Table { s, slog, table }
    }

    fn s_values_init(parameters: &Parameters) -> (Vec<Scalar>, Vec<RistrettoPoint>) {
        let slog_size = ((1 << (parameters.secret_size as u64 - 2)) / parameters.W).ilog2() as u8;

        (0..parameters.R)
            .map(|_| {
                let slog = utils::generate_random_scalar(slog_size).unwrap();
                let s = RISTRETTO_BASEPOINT_POINT.mul(slog);

                (slog, s)
            })
            .collect()
    }
}
