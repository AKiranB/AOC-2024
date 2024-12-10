use day1::{calculate_similarity_score, consolidate_lists};
use day2::{find_safe_levels};
use utils::{split_into_two_arrs, SplitArrays};
use day_1_dataset::{SPLIT_ARRS};
use day_2_dataset::{LEVEL_REPORTS};

mod day1;
mod utils;
mod test;
mod day2;
mod day_2_dataset;
mod day_1_dataset;



fn main() {
    let arrays:SplitArrays = split_into_two_arrs(SPLIT_ARRS);
    let result:i32 = consolidate_lists(&arrays.left, &arrays.right);
    let result2:i32 = calculate_similarity_score(&arrays.left, &arrays.right);
    let result3 = find_safe_levels(vec![
        vec![1,2,3,4,5],
        vec![1,5,2,4,2]
    ]);
    println!("{:?}", result);
    println!("{:?}", result2);
    println!("{:?}", result3);
}
