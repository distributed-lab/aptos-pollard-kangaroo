#![allow(non_snake_case)]

use crate::kangaroo::{self, Kangaroo};
use crate::utils;
use core::ops::Mul;
use curve25519_dalek_ng::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek_ng::ristretto::RistrettoPoint;
use curve25519_dalek_ng::scalar::Scalar;
use std::collections::HashMap;
use std::ops::AddAssign;

pub trait Generator {
    fn generate(i: u64, W: u64, N: usize, R: u64, secret_size: u8) -> Kangaroo;
}

impl Generator for Kangaroo {
    fn generate(i: u64, W: u64, N: usize, R: u64, secret_size: u8) -> Kangaroo {
        let (slog, s) = s_values_init(R, W, secret_size);

        let mut table = HashMap::new();
        
        while table.len() < N {
            let mut wlog = utils::generate_random_scalar(secret_size);
            let mut w = RISTRETTO_BASEPOINT_POINT.mul(wlog);

            for _ in 0..i * W {
                let w_compressed = w.compress();

                if kangaroo::is_distinguished(&w_compressed, W) {
                    println!("Distinguished point found: {:?}", table.len() + 1);
                    
                    table.insert(w_compressed, wlog);

                    break;
                }

                let h = kangaroo::hash(&w_compressed, R);
                wlog.add_assign(slog[h]);
                w.add_assign(s[h]);
            }
        }

        Kangaroo::new(
            s,
            slog,
            table.into_iter().map(|(k, v)| (k, v)).collect(),
        )
    }
}

fn s_values_init(R: u64, W: u64, secret_size: u8) -> (Vec<Scalar>, Vec<RistrettoPoint>) {
    let secret_size = ((1 << (secret_size as u64 - 2)) / W).ilog2() as u8;   
    
    (0..R)
        .map(|_| {
            let slog = utils::generate_random_scalar(secret_size);
            let s = RISTRETTO_BASEPOINT_POINT.mul(slog);

            (slog, s)
        })
        .collect()
}
