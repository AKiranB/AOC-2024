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

    #[test]
    fn test_no_overlap() {
        let left: Vec<i32> = vec![1, 2, 3, 4];
        let right: Vec<i32> = vec![5, 6, 7, 8];

        let score = calculate_similarity_score(&left, &right);
        println!("Score: {}", score);
        assert_eq!(score, 0); // No overlap, score is 0
    }

    #[test]
    fn test_full_overlap() {
        let left: Vec<i32> = vec![1, 1, 2, 2, 3, 3];
        let right: Vec<i32> = vec![1, 1, 2, 2, 3, 3];

        let score = calculate_similarity_score(&left, &right);
        println!("Score: {}", score);
        assert_eq!(score, 12); // (1*2 + 2*2 + 3*2) = 12
    }

    #[test]
    fn test_large_numbers() {
        let left: Vec<i32> = vec![10, 20, 30, 40, 50];
        let right: Vec<i32> = vec![50, 40, 30, 20, 10];

        let score = calculate_similarity_score(&left, &right);
        println!("Score: {}", score);
        assert_eq!(score, 150); // 10*1 + 20*1 + 30*1 + 40*1 + 50*1 = 150
    }

    #[test]
    fn test_repeated_large_numbers() {
        let left: Vec<i32> = vec![10, 10, 20, 20, 30, 30];
        let right: Vec<i32> = vec![10, 10, 10, 20, 30, 30];

        let score = calculate_similarity_score(&left, &right);
        println!("Score: {}", score);
        assert_eq!(score, 120); // (10*2 + 20*1 + 30*2) = 120
    }

    #[test]
    fn test_edge_case_empty_arrays() {
        let left: Vec<i32> = vec![];
        let right: Vec<i32> = vec![];

        let score = calculate_similarity_score(&left, &right);
        println!("Score: {}", score);
        assert_eq!(score, 0); // Empty arrays, score is 0
    }

    #[test]
    fn test_edge_case_single_element_overlap() {
        let left: Vec<i32> = vec![5];
        let right: Vec<i32> = vec![5];

        let score = calculate_similarity_score(&left, &right);
        println!("Score: {}", score);
        assert_eq!(score, 5); // One element, overlap score is 5
    }

    #[test]
    fn test_large_input() {
        let left: Vec<i32> = (1..=1000).collect();
        let right: Vec<i32> = (500..=1500).collect();

        let score = calculate_similarity_score(&left, &right);
        println!("Score: {}", score);
        assert!(score > 0); // Ensure it calculates something
    }

    #[test]
    fn test_skewed_distribution() {
        let left: Vec<i32> = vec![1, 1, 1, 2, 2, 3];
        let right: Vec<i32> = vec![1, 1, 2, 2, 2, 2, 3, 3, 3, 3];

        let score = calculate_similarity_score(&left, &right);
        println!("Score: {}", score);
        assert_eq!(score, 19); // (1*2 + 2*2 + 3*3) = 19
    }
}