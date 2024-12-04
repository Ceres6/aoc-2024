mod dec1;
mod dec2;

fn main() {
    println!("diff {}", dec1::diff());
    println!("Similarity score {}", dec1::similarity_score());
    println!("Safe reports {}", dec2::safe_count());
    println!("Dampened safe reports {}", dec2::dampened_count());
}
