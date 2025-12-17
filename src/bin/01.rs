advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let sum: u64 = input
        .lines()
        .filter_map(|l| l.parse::<u64>().ok())
        .map(|val| (val / 3) - 2)
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum: u64 = input
        .lines()
        .filter_map(|l| l.parse::<u64>().ok())
        .map(|val| {
            let mut sum = 0;
            let mut gas = val;
            loop {
                gas = (gas / 3) - 2;
                sum += gas;
                if gas < 6 {
                    break;
                };
            }
            sum
        })
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34241));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51316));
    }
}
