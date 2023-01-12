use itertools::Itertools;

#[derive(Debug)]
struct ElfRange {
    low: u8,
    high: u8,
}

impl ElfRange {
    fn contains(&self, other: &Self) -> bool {
        self.low <= other.low && self.high >= other.high
    }
}

/// Counts the numbers of pairs whose ranges overlap completely
/// # Examples
/// ```
/// use std::fs;
/// use advent_of_code_2022_04::count_fully_inclusive_pairs;
///
/// let contents = fs::read_to_string("example-input.txt").unwrap();
/// assert_eq!(count_fully_inclusive_pairs(&contents), 2);
/// ```
pub fn count_fully_inclusive_pairs(input: &String) -> u32 {
    input
        .split('\n') // iterator over lines
        .filter(|x| !x.is_empty()) // iterator over lines with text
        .map(|x| {
            x
                .split(',') // iterator over two pairs of dash delineated numbers
                .map(|x| {
                    let mut elf_range_limits = x.split('-'); //iterator over two numbers

                    ElfRange {
                        low:  elf_range_limits.next().unwrap().parse().unwrap(),
                        high: elf_range_limits.next().unwrap().parse().unwrap(),
                    }
                })
                .next_tuple().unwrap()
        }) // iterator over pairs of ElfRanges
        .filter(|x: &(ElfRange, ElfRange)| {
            x.0.contains(&x.1) || x.1.contains(&x.0)
        })
        .count().try_into().unwrap()
}
