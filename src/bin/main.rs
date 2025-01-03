use pollard_kangaroo::kangaroo::Kangaroo;
use pollard_kangaroo::utils;

use anyhow::Result;
use std::time::Instant;

fn test(secret_size: u8, secrets_count: u32) -> Result<()> {
    let kangaroo = Kangaroo::from_secret_size(secret_size)?;

    let mut time = 0;

    for i in 0..secrets_count {
        let now = Instant::now();

        let (sk, pk) = utils::generate_keypair(secret_size)?;

        println!("------------------");
        println!("test #{}", i + 1);
        println!("------------------");

        println!("secret key: {:x?}", sk.to_bytes());
        println!("public key: {:x?}", pk.compress().as_bytes());

        if let Some(expected_sk) = kangaroo.solve_dlp(&pk, Some(2000))? {
            println!("expected secret key: {:?}", expected_sk);
            println!("actual secret key: {:?}", utils::scalar_to_u64(&sk)?);
        } else {
            println!("run out of time");
        }

        let elapsed = now.elapsed();

        println!("elapsed: {:.2?}\n", elapsed.as_millis());

        time += elapsed.as_millis();
    }

    println!("average time: {:.2?}", time as f64 / secrets_count as f64);

    Ok(())
}

fn main() -> Result<()> {
    test(48, 10)?;

    Ok(())
}
