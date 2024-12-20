mod tests {
    use crate::calculate_similarity_score;
    use crate::find_safe_levels;

    #[test]

    // fn test_calculate_similarity_score() {
    //     let left: Vec<i32> = vec![3, 4, 2, 1, 3, 3];
    //     let right: Vec<i32> = vec![4, 3, 5, 3, 9, 3];
    //     let score = calculate_similarity_score(&left, &right);
    //     println!("Score: {}", score);
    //     assert_eq!(score, 31); // 3*3 + 4*1 = 31
    // }

    // Original example from problem statement (all positive integers)
    // #[test]
    // fn test_sample_from_problem_statement() {
    //     let level_sequences = vec![
    //         vec![7, 6, 4, 2, 1], // Safe
    //         vec![1, 2, 7, 8, 9], // Unsafe (2->7 is +5)
    //         vec![9, 7, 6, 2, 1], // Unsafe (6->2 is 4)
    //         vec![1, 3, 2, 4, 5], // Unsafe (direction change)
    //         vec![8, 6, 4, 4, 1], // Unsafe (4->4 is diff 0)
    //         vec![1, 3, 6, 7, 9], // Safe
    //     ];
    //     let res = find_safe_levels(level_sequences);
    //     assert_eq!(res, 2);
    // }

    #[test]
    fn test_large_input_sample() {
        let level_sequences: Vec<Vec<i32>> = vec![
            vec![16, 19, 21, 24, 21],
            vec![15, 18, 19, 22, 24, 25, 25],
            vec![80, 81, 83, 84, 87, 89, 93],
            vec![6, 7, 8, 9, 10, 13, 18],
            vec![60, 62, 61, 64, 66, 67],
            vec![76, 79, 81, 84, 82, 80],
            vec![70, 73, 72, 74, 74],
            vec![67, 68, 71, 74, 73, 77],
            vec![56, 57, 60, 59, 61, 64, 67, 74],
            vec![37, 38, 39, 40, 40, 43],
            vec![90, 92, 95, 95, 96, 97, 94],
            vec![80, 83, 86, 86, 86],
            vec![44, 47, 49, 49, 51, 54, 58],
            vec![69, 71, 74, 74, 81],
            vec![66, 68, 72, 75, 77],
            vec![34, 35, 39, 41, 38],
            vec![58, 60, 63, 67, 70, 72, 72],
            vec![43, 46, 47, 51, 52, 53, 56, 60],
            vec![35, 36, 37, 41, 44, 50],
            vec![63, 64, 67, 69, 71, 72, 78, 80],
            vec![19, 22, 23, 30, 28],
            vec![20, 21, 24, 30, 30],
            vec![75, 78, 80, 83, 90, 91, 95],
            vec![16, 17, 20, 22, 23, 29, 31, 36],
            vec![22, 21, 24, 26, 28],
            vec![87, 84, 87, 88, 86],
            vec![48, 46, 48, 51, 51],
            vec![40, 37, 40, 43, 44, 48],
            vec![77, 75, 78, 79, 81, 84, 86, 91],
            vec![43, 41, 40, 42, 44],
            vec![32, 30, 31, 32, 35, 32, 29],
            vec![87, 84, 81, 83, 83],
            vec![43, 41, 44, 47, 48, 45, 49],
            vec![49, 48, 51, 53, 54, 57, 54, 61],
            vec![68, 66, 69, 69, 72, 75, 77],
            vec![9, 7, 8, 10, 10, 12, 11],
            vec![77, 74, 77, 77, 77],
            vec![5, 2, 4, 4, 8],
            vec![34, 33, 36, 36, 42],
            vec![11, 9, 13, 15, 18, 21],
            vec![22, 21, 22, 23, 27, 24],
            vec![68, 67, 70, 74, 77, 80, 82, 82],
            vec![87, 86, 87, 91, 95],
            vec![26, 23, 27, 28, 30, 35],
            vec![23, 20, 21, 28, 31],
            vec![85, 83, 89, 92, 95, 96, 94],
            vec![32, 31, 38, 40, 40],
            vec![40, 38, 40, 41, 43, 49, 51, 55],
        ];
    
        let result = find_safe_levels(level_sequences);
        assert_eq!(result, 0);
    }
}