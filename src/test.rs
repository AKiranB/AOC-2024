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

    let level_sequences:Vec<Vec<i32>> = vec![
        vec![7, 6, 4, 2, 1],
        vec![1, 2, 7, 8, 9],
        vec![9, 7, 6, 2, 1, 6],
        vec![1, 3, 2, 4, 5, 3],
        vec![8, 6, 4, 4, 1],
        vec![1, 3, 6, 7, 9],
    ];
        let res = find_safe_levels(level_sequences.to_vec());
        assert_eq!(res, 2)
    }
}