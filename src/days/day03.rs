use crate::{Solution, SolutionPair};
use std::{collections::HashMap, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day03.txt").expect("Could not read file");

    let (sol1, _) = house_delivery_counts(&input, &mut HashMap::new());
    let (santa, robo) = split_directions(&input);
    let map = &mut HashMap::new();
    let (_, map) = house_delivery_counts(&santa, map);
    let (sol2, _) = house_delivery_counts(&robo, map);

    (Solution::from(sol1), Solution::from(sol2))
}

#[test]
fn test() {
    let input1 = "^v".to_string();
    let input2 = "^>v<".to_string();
    let input3 = "^v^v^v^v^v".to_string();

    let (sol1, _) = house_delivery_counts(&">".to_string(), &mut HashMap::new());
    let (sol2, _) = house_delivery_counts(&input2, &mut HashMap::new());
    let (sol3, _) = house_delivery_counts(&input3, &mut HashMap::new());

    let (santa1, robo1) = split_directions(&input1);
    let (santa2, robo2) = split_directions(&input2);
    let (santa3, robo3) = split_directions(&input3);

    let map1 = &mut HashMap::new();
    let map2 = &mut HashMap::new();
    let map3 = &mut HashMap::new();

    let (_, map1) = house_delivery_counts(&santa1, map1);
    let (sol4, _) = house_delivery_counts(&robo1, map1);
    let (_, map2) = house_delivery_counts(&santa2, map2);
    let (sol5, _) = house_delivery_counts(&robo2, map2);
    let (_, map3) = house_delivery_counts(&santa3, map3);
    let (sol6, _) = house_delivery_counts(&robo3, map3);

    assert_eq!(sol1, 2);
    assert_eq!(sol2, 4);
    assert_eq!(sol3, 2);
    assert_eq!(sol4, 3);
    assert_eq!(sol5, 3);
    assert_eq!(sol6, 11);
}

fn house_delivery_counts<'a>(
    directions: &String,
    map: &'a mut HashMap<(i64, i64), bool>,
) -> (u64, &'a mut HashMap<(i64, i64), bool>) {
    let mut x: i64 = 0;
    let mut y: i64 = 0;

    let _house = map.entry((x, y)).or_insert(true);

    for dir in directions.chars() {
        match dir {
            '>' => x += 1,
            '<' => x -= 1,
            '^' => y += 1,
            'v' => y -= 1,
            _ => panic!("Invalid direction"),
        }

        let _house = map.entry((x, y)).or_insert(true);
    }

    (map.keys().len() as u64, map)
}

fn split_directions(directions: &String) -> (String, String) {
    let mut santa_directions = String::new();
    let mut robo_santa_directions = String::new();

    for (i, dir) in directions.chars().enumerate() {
        if i % 2 == 0 {
            santa_directions.push(dir);
        } else {
            robo_santa_directions.push(dir);
        }
    }

    (santa_directions, robo_santa_directions)
}
