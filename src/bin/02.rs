use std::collections::HashSet;

advent_of_code::solution!(2);

fn is_report_safe(report: &Vec<u64>) -> bool {
    let increasing = report.into_iter().is_sorted_by(|x, y| x < y); // both cover the case that adj. levels must differ by at least one
    let decreasing = report.into_iter().is_sorted_by(|x, y| x > y);
    // println!("increasing? {}", increasing);
    // println!("decreasing? {}", decreasing);
    if !(increasing || decreasing) {
        return false;
    };

    for x in 0..(report.len() - 1) {
        let curr = report.get(x).unwrap();
        let next = report.get(x + 1).unwrap();
        // println!("curr ({}) = {}, next ({}) = {}", x, curr, x + 1, next);
        if (next > curr && next - curr > 3) || (next < curr && curr - next > 3) {
            // return false if the distance is > 3
            return false;
        }
    }

    true // return true if we made it to this point!
}

/**
 * identifies dangerous levels, and tests if a report is safe after removing one of the dangerous levels
 */
fn is_report_safe_dampened(report: &Vec<u64>) -> bool {
    if is_report_safe(report) {
        return true;
    };

    let mut bad_level_inds = HashSet::<usize>::new();

    // increasing checks
    for x in 0..(report.len() - 1) {
        let curr = report.get(x);
        let next = report.get(x + 1);
        if curr < next {
            bad_level_inds.insert(x);
            bad_level_inds.insert(x + 1);
        }
    }

    // decreasing checks
    for x in 0..(report.len() - 1) {
        let curr = report.get(x);
        let next = report.get(x + 1);
        if curr > next {
            bad_level_inds.insert(x);
            bad_level_inds.insert(x + 1);
        }
    }

    // distance checks
    for x in 0..(report.len() - 1) {
        let curr = report.get(x).unwrap();
        let next = report.get(x + 1).unwrap();
        // println!("curr ({}) = {}, next ({}) = {}", x, curr, x + 1, next);
        if (next > curr && next - curr > 3) || (next < curr && curr - next > 3) {
            // not safe if the distance is > 3
            bad_level_inds.insert(x);
            bad_level_inds.insert(x + 1);
        }
    }

    // println!("{:?}", report);

    // let res = bad_level_inds.into_iter().any(| x | { // return true if no bad inds (safe to begin with)
    //     let mut new_report = report.clone();
    //     let removed = new_report.remove(x);
    //     if is_report_safe(&new_report) {
    //         println!("Report is safe by removing {} (at index {})", removed, x);
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }); // return true if there exists some report where removing one of the indexes makes it safe, false otherwise
    // if !res {
    //     println!("Report could not be saved.");
    // }
    // res

    bad_level_inds.into_iter().any(|x| {
        // return true if no bad inds (safe to begin with)
        let mut new_report = report.clone();
        new_report.remove(x);
        is_report_safe(&new_report)
    }) // return true if there exists some report where removing one of the indexes makes it safe, false otherwise
}

pub fn part_one(input: &str) -> Option<u64> {
    let reports = input.lines().map(|ln: &str| {
        ln.split_ascii_whitespace()
            .map(|num: &str| -> u64 { num.parse().unwrap() })
            .collect()
    });

    let result = reports.filter(|r| is_report_safe(r)).count() as u64;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let reports = input.lines().map(|ln: &str| {
        ln.split_ascii_whitespace()
            .map(|num: &str| -> u64 { num.parse().unwrap() })
            .collect()
    });

    let result = reports.filter(|r| is_report_safe_dampened(r)).count() as u64;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
