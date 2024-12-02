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
        if increasing && (next - curr > 3) {
            // return false if the distance is > 3
            return false;
        } else if decreasing && (curr - next > 3) {
            return false;
        }
    }

    true
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
    None
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
        assert_eq!(result, None);
    }
}
