mod dec1;
mod dec2;
mod dec3;
mod dec4;
mod dec5;
mod dec6;
mod dec7;
mod dec8;
mod dec9;
mod dec10;
mod dec11;
mod dec12;

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
    println!("Guard unique position count {}", dec6::guard_position_count()?);
    println!("Possible loops with new obstacle {}", dec6::count_possible_loops()?);
    println!("Feasible equations sum {}", dec7::feasible_equations()?);
    println!("Antinodes count {}", dec8::count_antinodes()?);
    println!("Resonating antinodes count {}", dec8::count_resonating_antinodes()?);
    println!("Compact checksum {}", dec9::compact_checksum()?);
    println!("Unfragmented compact checksum {}", dec9::unfragmented_compact_checksum()?);
    println!("Total trail score {}", dec10::sum_trailhead_scores(true)?);
    println!("Total trail rating {}", dec10::sum_trailhead_scores(false)?);
    println!("Stone count {}", dec11::count_stones()?);
    println!("Fence price {}", dec12::fence_price(false)?);
    println!("Fence price with bulk discount {}", dec12::fence_price(true)?);
    Ok(())
}
