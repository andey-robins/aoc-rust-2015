use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = worker(&"ckczppom".to_string(), 5);
    let sol2: u64 = worker(&"ckczppom".to_string(), 6);

    (Solution::from(sol1), Solution::from(sol2))
}

#[test]
fn test() {
    assert_eq!(worker(&"abcdef".to_string(), 5), 609043);
    assert_eq!(worker(&"pqrstuv".to_string(), 5), 1048970);
}

fn worker(key: &String, pow: usize) -> u64 {
    let target_string = "0".repeat(pow);

    // check if already a valid key and hash
    let mut _result = format!("{:x}", md5::compute(key));

    // search for the addon which has proof of work
    let mut counter: u64 = 0;
    while _result.chars().take(pow).collect::<String>() != target_string {
        counter += 1;
        _result = format!("{:x}", md5::compute(format!("{}{}", key, counter)));
    }
    counter
}
