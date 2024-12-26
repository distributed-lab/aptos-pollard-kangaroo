#![allow(non_snake_case)]

use std::time::Instant;
use anyhow::{anyhow, Result};
use pollard_kangaroo::generator::Generator;
use pollard_kangaroo::kangaroo::Kangaroo;
use pollard_kangaroo::solver::Solver;
use pollard_kangaroo::{kangaroo, utils};

fn test(secret_size: u8, secrets_count: u32) -> Result<()> {
    let (i, W, N, R) = match secret_size {
        16 => (8, 8, 8000, 64),
        32 => (8, 2048, 4000, 128),
        48 => (8, 65536, 40000, 128),
        _ => return Err(anyhow!("unsupported secret_size")),
    };

    let kangaroo = Kangaroo::generate(i, W, N, R, secret_size);

    let mut time = 0;

    for _ in 0..secrets_count {
        let now = Instant::now();

        let (sk, pk) = utils::generate_keypair(secret_size);

        println!("Secret key: {:?}", hex::encode(&sk.to_bytes()));
        println!("Public key: {:?}", hex::encode(&pk.as_bytes()));
        println!("Public key compressed: {:?}", &pk.as_bytes());

        let expected_sk = kangaroo.solve_dlp(&pk.decompress().unwrap(), i, W, R, secret_size);

        println!("Expected secret key: {:?}", expected_sk);
        println!("Actual secret key: {:?}", kangaroo::scalar_to_u64(&sk));

        let elapsed = now.elapsed();

        println!("Elapsed: {:.2?}", elapsed.as_millis());

        time += elapsed.as_millis();
    }

    println!("Average time: {:.2?}", time as f64 / secrets_count as f64);

    Ok(())
}

fn main() -> Result<()> {
    test(32, 200)?;
    //test(16, 200)?;

    Ok(())
}

