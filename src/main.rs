mod day1;
use crate::day1::consolidate_lists;

fn main() {
   
   let left = [1, 4, 5, 3, 4].to_vec();
   let right = [6, 2, 2, 2, 4].to_vec();

   let outcome:i32 = consolidate_lists(left, right);

   println!("{:?}", outcome);

}