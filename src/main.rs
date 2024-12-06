mod dec1;
mod dec2;
mod dec3;

fn main() -> anyhow::Result<()> {
    println!("diff {}", dec1::diff());
    println!("Similarity score {}", dec1::similarity_score());
    println!("Safe reports {}", dec2::safe_count());
    println!("Dampened safe reports {}", dec2::dampened_count());
    println!("Safe mul {}", dec3::sum_mul(false)?);
    println!("Safe enabled mul {}", dec3::sum_mul(true)?);
    Ok(())
}
