use std::{collections::HashMap, iter::zip};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let split_lines = input.lines().map(| ln: &str | -> Vec<u64> { 
        ln.split_ascii_whitespace().map(| num: &str | -> u64 { 
            num.parse().unwrap() 
        }).collect()
    });

    let mut first_row_sorted: Vec<u64> = split_lines.clone().map(| x | { *x.get(0).unwrap() }).collect();
    first_row_sorted.sort();

    let mut second_row_sorted: Vec<u64> = split_lines.clone().map(| x | { *x.get(1).unwrap() }).collect();
    second_row_sorted.sort();

    let result: u64 = zip(first_row_sorted, second_row_sorted).fold(0, | acc, (x, y) | { 
        acc + if x > y { x - y } else { y - x } 
    });

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let split_lines = input.lines().map(| ln: &str | -> Vec<u64> { 
        ln.split_ascii_whitespace().map(| num: &str | -> u64 { 
            num.parse().unwrap() 
        }).collect()
    });

    let first_row = split_lines.clone().map(| x | { *x.get(0).unwrap() });
    let second_row = split_lines.clone().map(| x | { *x.get(1).unwrap() });

    let mut count_table = HashMap::new();
    for x in second_row {
        match count_table.get(&x) {
            None => count_table.insert(x, 1),
            Some(val) => count_table.insert(x, val + 1)
        };
    }

    let result = first_row.fold(0, | acc, x | { acc + 
        match count_table.get(&x) {
            None => 0,
            Some(val) => val * x
        }
    });

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
