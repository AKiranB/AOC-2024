//  Maybe the lists are only off by a small amount! To find out,
//  pair up the numbers and measure how far apart they are.
//  Pair up the smallest number in the left list with the smallest number
//  in the right list, then the second-smallest left number with
//  the second-smallest right number, and so on.

//  Within each pair, figure out how far apart the
//  two numbers are; you'll need to add up all of those
//  distances. For example, if you pair up a 3 from the left
//  list with a 7 from the right list, the distance apart is 4;
//  if you pair up a 9 with a 3, the distance apart is 6.



// God knows if this is built in rust

fn sort_list(list: Vec<i32>) -> Vec<i32>{
    let mut array: Vec<i32> = list.clone(); 
    array.sort();
    array
}

// this too lol

fn handle_sum(a: i32, b:i32) -> i32{
    if a > b {return a - b}
    return b - a
}


pub fn consolidate_lists(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let sorted_left: Vec<i32> = sort_list(left);
    let sorted_right: Vec<i32> = sort_list(right);

    let mut sum: i32 = 0;

    for n in 0..sorted_left.len() {
        let a: i32 = sorted_left[n];
        let b: i32 = sorted_right[n];
        sum += handle_sum(a, b);
    }
    sum 
}