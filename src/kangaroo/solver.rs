use crate::kangaroo::{self, Kangaroo};
use crate::utils;

use anyhow::Result;
use curve25519_dalek_ng::{constants::RISTRETTO_BASEPOINT_POINT, ristretto::RistrettoPoint};
use std::ops::{Add, AddAssign, Mul, Sub};
use std::time::{Duration, Instant};

impl Kangaroo {
    pub fn solve_dlp(&self, pk: &RistrettoPoint, max_time: Option<u64>) -> Result<Option<u64>> {
        let start_time = max_time.map(|_| Instant::now());

        loop {
            // wdist = r + slog_1 + slog_2 ...
            let mut wdist = utils::generate_random_scalar(self.parameters.secret_size - 8)?;
            // w = sk * G + r * G + slog_1 * G + slog_2 * G ... = sk * G + wdist * G
            let mut w = pk.add(RISTRETTO_BASEPOINT_POINT.mul(wdist));

            for _ in 0..self.parameters.i * self.parameters.W {
                let w_compressed = w.compress();

                if kangaroo::is_distinguished(&w_compressed, &self.parameters) {
                    if let Some(value) = self.table.table.get(&w_compressed) {
                        // value * G = sk * G + wdist * G => sk = value - wdist
                        let sk = value.sub(wdist);

                        assert!(RISTRETTO_BASEPOINT_POINT.mul(sk).eq(pk));

                        return Ok(Some(utils::scalar_to_u64(&sk)?));
                    }

                    break;
                }

                if let Some(max_time) = max_time {
                    if start_time.unwrap().elapsed() >= Duration::from_millis(max_time) {
                        return Ok(None);
                    }
                }

                let h = kangaroo::hash(&w_compressed, &self.parameters) as usize;

                wdist.add_assign(&self.table.slog[h]);
                w.add_assign(&self.table.s[h]);
            }
        }
    }
}
