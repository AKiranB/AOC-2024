mod tests {
    use crate::calculate_similarity_score;

    #[test]
    fn test_basic_functionality() {
        let left: Vec<i32> = vec![3, 4, 2, 1, 3, 3];
        let right: Vec<i32> = vec![4, 3, 5, 3, 9, 3];

        let score = calculate_similarity_score(&left, &right);
        println!("Score: {}", score);
        assert_eq!(score, 31); // 3*3 + 4*1 = 31
    }

    
}