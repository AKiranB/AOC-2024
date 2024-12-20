use day1::{calculate_similarity_score, consolidate_lists};
use day2::{find_safe_levels};
use utils::{split_into_two_arrs, SplitArrays, get_report};
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
    let day1_pt_1_res:i32 = consolidate_lists(&arrays.left, &arrays.right);
    let day1_pt_2_rs:i32 = calculate_similarity_score(&arrays.left, &arrays.right);

    let reports = get_report(LEVEL_REPORTS);


    let reports_len:usize = reports.len();
    let day2_pt_1_res:i32 = find_safe_levels(reports);

    
    println!("{:?}", day2_pt_1_res); 



}
