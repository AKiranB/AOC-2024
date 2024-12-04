mod tests {
    use crate::calculate_similarity_score;
    use crate::find_safe_levels;

    #[test]

    fn test_calculate_similarity_score() {
        let left: Vec<i32> = vec![3, 4, 2, 1, 3, 3];
        let right: Vec<i32> = vec![4, 3, 5, 3, 9, 3];
        let score = calculate_similarity_score(&left, &right);
        println!("Score: {}", score);
        assert_eq!(score, 31); // 3*3 + 4*1 = 31
    }

    #[test]
    fn test_find_safe_levels(){
     let level_sequences = [
    [7, 6, 4, 2, 1], // Safe because the levels are all decreasing by 1 or 2.
    [1, 2, 7, 8, 9], // Unsafe because 2 7 is an increase of 5.
    [9, 7, 6, 2, 1], // Unsafe because 6 2 is a decrease of 4.
    [1, 3, 2, 4, 5], // Unsafe because 1 3 is increasing but 3 2 is decreasing.
    [8, 6, 4, 4, 1], // Unsafe because 4 4 is neither an increase nor a decrease.
    [1, 3, 6, 7, 9], // Safe because the levels are all increasing by 1, 2, or 3.
];
        let res =find_safe_levels(level_sequences.to_vec());
        println!("res: {}", res); 
        assert_eq!(res, 2)
    }
}