// This example data contains six reports each containing five levels.

// The engineers are trying to figure out which reports are safe. The Red-Nosed reactor safety systems can only tolerate levels that are either gradually increasing or gradually decreasing. So, a report only counts as safe if both of the following are true:

// The levels are either all increasing or all decreasing.
// Any two adjacent levels differ by at least one and at most three.
// In the example above, the reports can be found safe or unsafe by checking those rules:

// 7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
// 1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
// 9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
// 1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
// 8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
// // 1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.

// const levelSequences = [
//     [7, 6, 4, 2, 1], // Safe because the levels are all decreasing by 1 or 2.
//     [1, 2, 7, 8, 9], // Unsafe because 2 7 is an increase of 5.
//     [9, 7, 6, 2, 1], // Unsafe because 6 2 is a decrease of 4.
//     [1, 3, 2, 4, 5], // Unsafe because 1 3 is increasing but 3 2 is decreasing.
//     [8, 6, 4, 4, 1], // Unsafe because 4 4 is neither an increase nor a decrease.
//     [1, 3, 6, 7, 9], // Safe because the levels are all increasing by 1, 2, or 3.
// ];

fn safely_find_diff(n: i32, z: i32) -> i32 {
    println!("Calculating difference between n: {} and z: {}", n, z);
    let res = if n > z {
        n - z
    } else {
        z - n
    };
    println!("Result (res): {}", res);
    res
}

pub fn find_safe_levels(reports: Vec<[i32; 5]>) -> i32 {
    let mut safe_levels: i32 = 0;

    for (report_index, report) in reports.iter().enumerate() {
        let mut is_safe: bool = true;
        println!("Analyzing report #{}: {:?}", report_index + 1, report);

        for window in report.windows(2) {
            println!("Window: {:?}", window);
            let diff = safely_find_diff(window[0], window[1]);
            if diff > 3 {
                println!("Unsafe difference found: {}", diff);
                is_safe = false;
                break
            }
        }

        if is_safe {
            println!("Report #{} is safe.", report_index + 1);
            safe_levels += 1;
        } else {
            println!("Report #{} is unsafe.", report_index + 1);
        }
    }

    println!("Total safe levels: {}", safe_levels);
    safe_levels
}
