use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day01.txt").expect("Could not read file");

    let sol1: i32 = one(&input);
    let sol2: i32 = two(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

#[test]
fn test() {
    let input1 = "(())".to_string();
    let input2 = "()()".to_string();
    let input3 = "(((".to_string();
    let input4 = "(()(()(".to_string();
    let input5 = "))(((((".to_string();
    let input6 = "())".to_string();
    let input7 = "))(".to_string();
    let input8 = ")))".to_string();
    let input9 = ")())())".to_string();
    let input10 = ")".to_string();
    let input11: String = "()())".to_string();

    assert_eq!(one(&input1), 0);
    assert_eq!(one(&input2), 0);
    assert_eq!(one(&input3), 3);
    assert_eq!(one(&input4), 3);
    assert_eq!(one(&input5), 3);
    assert_eq!(one(&input6), -1);
    assert_eq!(one(&input7), -1);
    assert_eq!(one(&input8), -3);
    assert_eq!(one(&input9), -3);

    assert_eq!(two(&input10), 1);
    assert_eq!(two(&input11), 5);
}

fn one(input: &String) -> i32 {
    count_char(&input, '(') - count_char(&input, ')')
}

fn two(input: &String) -> i32 {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }

        if floor < 0 {
            return (i + 1) as i32;
        }
    }
    // return only a -1 error code since the inputs can assumed to be good for AoC
    // alternatively we return a Result and unwrap which will be functionally
    // the same for these happy inputs
    -1
}

fn count_char(input: &String, chr: char) -> i32 {
    let mut count: i32 = 0;
    for c in input.chars() {
        if c == chr {
            count += 1;
        }
    }
    count
}
