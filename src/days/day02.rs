use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day02.txt").expect("Could not read file");
    calculate(&input)
}

#[test]
fn test() {
    let input: String = read_to_string("input/test/day02.txt").expect("Could not read file");

    let (test_sol1, test_sol2) = calculate(&input);

    assert_eq!(test_sol1, Solution::U64(101));
    assert_eq!(test_sol2, Solution::U64(48));
}

fn calculate(input: &String) -> SolutionPair {
    let mut total_paper: u64 = 0;
    let mut total_ribbon: u64 = 0;
    for line in input.lines() {
        let side_lengths = line.split("x").collect::<Vec<&str>>();
        let l = side_lengths[0].parse::<u64>().unwrap();
        let w = side_lengths[1].parse::<u64>().unwrap();
        let h = side_lengths[2].parse::<u64>().unwrap();

        let lw = l * w;
        let wh = w * h;
        let hl = h * l;

        let smallest_perimeter: u64;

        let smallest_side = if lw < wh && lw < hl {
            smallest_perimeter = 2 * (l + w);
            lw
        } else if wh < hl {
            smallest_perimeter = 2 * (w + h);
            wh
        } else {
            smallest_perimeter = 2 * (h + l);
            hl
        };

        total_paper += 2 * (lw + wh + hl);
        total_paper += smallest_side;

        total_ribbon += smallest_perimeter;
        total_ribbon += l * w * h;
    }

    (Solution::U64(total_paper), Solution::U64(total_ribbon))
}
