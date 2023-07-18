use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day05.txt").expect("couldn't read input file");

    let mut nice: u64 = 0;
    let mut new_nice: u64 = 0;
    for line in input.lines() {
        if is_nice_string(&line.to_owned()) {
            nice += 1;
        }

        if is_new_nice_string(&line.to_owned()) {
            new_nice += 1;
        }
    }

    (Solution::from(nice), Solution::from(new_nice))
}

#[test]
fn test() {
    assert_eq!(has_forbidden_strings(&"adwytgowhe".to_string()), false);
    assert_eq!(has_forbidden_strings(&"abweiriieiowery".to_string()), true);
    assert_eq!(has_forbidden_strings(&"ababwwwwww".to_string()), true);
    assert_eq!(has_forbidden_strings(&"".to_string()), false);
    assert_eq!(has_forbidden_strings(&"cdabwwwwww".to_string()), true);
    assert_eq!(has_forbidden_strings(&"adadxywwww".to_string()), true);

    assert_eq!(atleast_three_vowels(&"adwytgowhe".to_string()), true);
    assert_eq!(atleast_three_vowels(&"cdabwwwwww".to_string()), false);
    assert_eq!(atleast_three_vowels(&"".to_string()), false);
    assert_eq!(atleast_three_vowels(&"adadxywwwwa".to_string()), true);

    assert_eq!(atleast_one_double(&"abcdefgh".to_string()), false);
    assert_eq!(atleast_one_double(&"".to_string()), false);
    assert_eq!(atleast_one_double(&"z".to_string()), false);
    assert_eq!(atleast_one_double(&"abccdef".to_string()), true);

    assert_eq!(is_nice_string(&"ugknbfddgicrmopn".to_string()), true);
    assert_eq!(is_nice_string(&"aaa".to_string()), true);
    assert_eq!(is_nice_string(&"jchzalrnumimnmhp".to_string()), false);
    assert_eq!(is_nice_string(&"haegwjzuvuyypxyu".to_string()), false);
    assert_eq!(is_nice_string(&"dvszwmarrgswjxmb".to_string()), false);

    assert_eq!(double_pair_two(&"xyxy".to_string()), true);
    assert_eq!(double_pair_two(&"aabcdefgaa".to_string()), true);
    assert_eq!(double_pair_two(&"aaa".to_string()), false);

    assert_eq!(three_long_palindrome(&"xyx".to_string()), true);
    assert_eq!(three_long_palindrome(&"abcdefeghi".to_string()), true);
    assert_eq!(three_long_palindrome(&"aaa".to_string()), true);
    assert_eq!(three_long_palindrome(&"".to_string()), false);
    assert_eq!(three_long_palindrome(&"aa".to_string()), false);
    assert_eq!(three_long_palindrome(&"affa".to_string()), false);
}

fn is_nice_string(input: &String) -> bool {
    !has_forbidden_strings(input) && atleast_three_vowels(input) && atleast_one_double(input)
}

fn is_new_nice_string(input: &String) -> bool {
    double_pair_two(input) && three_long_palindrome(input)
}

fn has_forbidden_strings(input: &String) -> bool {
    let ab: Vec<_> = input.match_indices("ab").collect();
    let cd: Vec<_> = input.match_indices("cd").collect();
    let pq: Vec<_> = input.match_indices("pq").collect();
    let xy: Vec<_> = input.match_indices("xy").collect();

    ab.len() + cd.len() + pq.len() + xy.len() != 0
}

fn atleast_three_vowels(input: &String) -> bool {
    let a: Vec<_> = input.match_indices("a").collect();
    let e: Vec<_> = input.match_indices("e").collect();
    let i: Vec<_> = input.match_indices("i").collect();
    let o: Vec<_> = input.match_indices("o").collect();
    let u: Vec<_> = input.match_indices("u").collect();

    a.len() + e.len() + i.len() + o.len() + u.len() >= 3
}

fn atleast_one_double(input: &String) -> bool {
    for (i, c) in input.chars().enumerate() {
        if i == 0 {
            continue;
        }

        if input
            .chars()
            .nth(i - 1)
            .expect("unable to get previous character")
            == c
        {
            return true;
        }
    }
    false
}

fn double_pair_two(input: &String) -> bool {
    for (i, c) in input.chars().enumerate() {
        if i == 0 {
            continue;
        }

        if input
            .match_indices(&format!("{}{}", input.chars().nth(i - 1).unwrap(), c))
            .collect::<Vec<_>>()
            .len()
            > 1
        {
            return true;
        }
    }
    false
}

fn three_long_palindrome(input: &String) -> bool {
    for (i, c) in input.chars().enumerate() {
        if i == 0 || i == 1 {
            continue;
        }

        if input.chars().nth(i - 2).unwrap() == c {
            return true;
        }
    }
    false
}
