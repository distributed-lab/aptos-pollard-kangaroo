#![allow(non_snake_case)]

use crate::kangaroo::Kangaroo;
use crate::{kangaroo, utils};
use core::ops::{Add, AddAssign, Mul, Sub};
use curve25519_dalek_ng::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek_ng::ristretto::{RistrettoPoint};

pub trait Solver {
    fn solve_dlp(&self, pk: &RistrettoPoint, i: u64, W: u64, R: u64, secret_size: u8) -> u64;
}

impl Solver for Kangaroo {
    fn solve_dlp(&self, pk: &RistrettoPoint, i: u64, W: u64, R: u64, secret_size: u8) -> u64 {
        loop {
            let mut wdist = utils::generate_random_scalar(secret_size - 8);
            let mut w = pk.add(RISTRETTO_BASEPOINT_POINT.mul(wdist));

            for _ in 0..i * W {
                let w_compressed = w.compress();

                if kangaroo::is_distinguished(&w_compressed, W) {
                    if let Some(table_entry) = self.table.get(&w_compressed) {
                        let sk = table_entry.sub(wdist);

                        if RISTRETTO_BASEPOINT_POINT.mul(sk).eq(pk) {
                            return kangaroo::scalar_to_u64(&sk);
                        }
                    }

                    break;
                }

                let h = kangaroo::hash(&w_compressed, R);

                wdist.add_assign(&self.slog[h]);
                w.add_assign(&self.s[h]);
            }
        }
    }
}
