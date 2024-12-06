mod dec1;
mod dec2;
mod dec3;
mod dec4;
mod dec5;

fn main() -> anyhow::Result<()> {
    println!("diff {}", dec1::diff());
    println!("Similarity score {}", dec1::similarity_score());
    println!("Safe reports {}", dec2::safe_count());
    println!("Dampened safe reports {}", dec2::dampened_count());
    println!("Safe mul {}", dec3::sum_mul(false)?);
    println!("Safe enabled mul {}", dec3::sum_mul(true)?);
    println!("XMAS count {}", dec4::count_xmas()?);
    println!("MAS cross count {}", dec4::count_cross_mas()?);
    println!("Middle page ordered updates sum brute force {}", dec5::middle_page_ordered_updates_sum()?);
    println!("Middle page ordered updates sum by predecessors and antecessors struct {}", dec5::middle_page_sum(true)?);
    println!("Middle page unordered updates sum by predecessors and antecessors struct {}", dec5::middle_page_sum(false)?);
    println!("Middle page ordered updates sum by custom ordering function {}", dec5::page_custom_order(true)?);
    println!("Middle page unordered updates sum by custom ordering function {}", dec5::page_custom_order(true)?);
    Ok(())
}
