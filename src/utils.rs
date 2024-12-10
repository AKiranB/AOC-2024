pub struct SplitArrays {
   pub left: Vec<i32>,
   pub right: Vec<i32>
   }

   
pub fn split_into_two_arrs(data: &str) -> SplitArrays {
    let mut left: Vec<i32> = Vec::new();
    let mut right:Vec<i32> = Vec::new();

    for line in data.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|n: &str| n.parse::<i32>().ok())
            .collect();

        if numbers.len() == 2 {
            left.push(numbers[0]);
            right.push(numbers[1]);
        }
    }
    SplitArrays { left, right }
}

pub fn get_report(data: &str) -> Vec<Vec<i32>> {

    let mut return:

    for line in data.lines(){

    }

}
