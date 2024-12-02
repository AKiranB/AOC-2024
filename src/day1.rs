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

//pt1

fn sort_list(list: &Vec<i32>) -> Vec<i32>{
    let mut array: Vec<i32> = list.clone(); 
    array.sort();
    array
}

fn handle_sum(a: i32, b:i32) -> i32{
    if a > b {return a - b}
    return b - a
}

pub fn consolidate_lists(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
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

// --- Part Two ---
// Your analysis only confirmed what everyone feared: the two lists of location IDs are indeed very different.

// Or are they?

// The Historians can't agree on which group made the mistakes or how to read most of the Chief's handwriting, but in the commotion you notice an interesting detail: a lot of location IDs appear in both lists! Maybe the other numbers aren't location IDs at all but rather misinterpreted handwriting.

// This time, you'll need to figure out exactly how often each number from the left list appears in the right list. Calculate a total similarity score by adding up each number in the left list after multiplying it by the number of times that number appears in the right list.

// Here are the same example lists again:

// 3   4
// 4   3
// 2   5
// 1   3
// 3   9
// 3   3
// For these example lists, here is the process of finding the similarity score:

// The first number in the left list is 3. It appears in the right list three times, so the similarity score increases by 3 * 3 = 9.
// The second number in the left list is 4. It appears in the right list once, so the similarity score increases by 4 * 1 = 4.
// The third number in the left list is 2. It does not appear in the right list, so the similarity score does not increase (2 * 0 = 0).
// The fourth number, 1, also does not appear in the right list.
// The fifth number, 3, appears in the right list three times; the similarity score increases by 9.
// The last number, 3, appears in the right list three times; the similarity score again increases by 9.
// So, for these example lists, the similarity score at the end of this process is 31 (9 + 4 + 0 + 0 + 9 + 9).

// Once again consider your left and right lists. What is their similarity score?

pub fn calculate_similarity_score (left: &Vec<i32>, right:&Vec<i32>) -> i32 {

    let sorted_left: Vec<i32> = sort_list(left);
    let sorted_right: Vec<i32> = sort_list(right);

    let mut similarity_score: i32 = 0;

    let mut i:usize= 0;
    let mut j:usize = 0;
    let mut seen_count:i32 = 0;
     
    while i < sorted_left.len() && j < sorted_right.len() {
        if sorted_left[i] == sorted_right[j]{
            seen_count += 1;
            j += 1 
        }else if sorted_right[j]> sorted_left[i] {
            similarity_score += sorted_left[i] * seen_count;
            seen_count = 0;
            i += 1
        }else {
            j+= 1;
            i+= 1;
        }
    }
    
    similarity_score
}