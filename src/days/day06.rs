use crate::{Solution, SolutionPair};
use regex::Regex;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq)]
struct DroneGrid {
    width: usize,
    height: usize,
    grid: Vec<Vec<Drone>>,
}

#[derive(Clone, Debug, PartialEq)]
struct Drone {
    x: usize,
    y: usize,
    led_on: bool,
    brightness: u64,
}

#[derive(Clone, Debug, PartialEq)]
struct DroneCommand {
    command: String,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day06.txt").expect("File not found");
    driver(input)
}

fn driver(input: String) -> SolutionPair {
    let lines: Vec<String> = input.lines().map(str::to_string).collect();
    let mut grid = DroneGrid::new(1000, 1000);
    for line in lines {
        match parse_drone_command(line.as_str()) {
            Ok(dc) => match dc.command.as_str() {
                "turn on" => {
                    grid.turn_on_rect(dc.x1, dc.y1, dc.x2, dc.y2);
                }
                "turn off" => {
                    grid.turn_off_rect(dc.x1, dc.y1, dc.x2, dc.y2);
                }
                "toggle" => {
                    grid.toggle_rect(dc.x1, dc.y1, dc.x2, dc.y2);
                }
                _ => {
                    panic!("Unknown command: {}", dc.command);
                }
            },
            Err(_) => {
                println!("Error parsing command: {}", line);
            }
        }
    }
    let sol1 = grid.count_lit();
    let sol2: u64 = grid.brightness();
    (Solution::from(sol1), Solution::from(sol2))
}

#[test]
fn test() {
    let input = "turn on 0,0 through 999,999";
    let bad_input = "fail this 0, 0 and 999, 999";

    // test the grid and rectangle structs
    let mut grid = DroneGrid::new(1000, 1000);
    grid.turn_on_rect(0, 0, 999, 999);
    assert_eq!(grid.count_lit(), 1_000_000);

    let mut grid = DroneGrid::new(1000, 1000);
    grid.toggle_rect(0, 0, 999, 0);
    assert_eq!(grid.count_lit(), 1000);

    let mut grid = DroneGrid::new(1000, 1000);
    grid.turn_on_rect(0, 0, 999, 999);
    grid.turn_off_rect(499, 499, 500, 500);
    assert_eq!(grid.count_lit(), 999_996);

    // test the parser
    match parse_drone_command(input) {
        Ok(dc) => {
            assert_eq!(dc.command, "turn on");
            assert_eq!(dc.x1, 0);
            assert_eq!(dc.y1, 0);
            assert_eq!(dc.x2, 999);
            assert_eq!(dc.y2, 999);
        }
        Err(_) => {
            panic!("Error parsing drone command: {}", input);
        }
    }

    match parse_drone_command(bad_input) {
        Ok(_) => {
            panic!("Should have failed to parse: {}", bad_input);
        }
        Err(_) => {}
    }

    // integration tests
    let input = read_to_string("input/test/day06.txt").unwrap();
    let (sol1, sol2) = driver(input);
    assert_eq!(sol1, Solution::U64(1_000_000 - 1_000 - 4));
    assert_eq!(sol2, Solution::U64(1_001_996));

    let input = "turn on 0,0 through 0,0\ntoggle 0,0 through 999,999".to_string();
    let (sol1, sol2) = driver(input);
    assert_eq!(sol1, Solution::U64(999_999));
    assert_eq!(sol2, Solution::U64(2_000_001));
}

impl Drone {
    fn turn_on(self: &mut Drone) -> &mut Drone {
        self.led_on = true;
        self.brightness += 1;
        self
    }

    fn turn_off(self: &mut Drone) -> &mut Drone {
        self.led_on = false;
        if self.brightness > 0 {
            self.brightness -= 1;
        }
        self
    }

    fn toggle(self: &mut Drone) -> &mut Drone {
        self.led_on = !self.led_on;
        self.brightness += 2;
        self
    }
}

impl DroneGrid {
    fn new(width: usize, height: usize) -> DroneGrid {
        DroneGrid {
            width,
            height,
            grid: vec![
                vec![
                    Drone {
                        x: 0,
                        y: 0,
                        led_on: false,
                        brightness: 0
                    };
                    width
                ];
                height
            ],
        }
    }

    fn turn_on_rect(self: &mut DroneGrid, x1: usize, y1: usize, x2: usize, y2: usize) {
        for y in y1..=y2 {
            for x in x1..=x2 {
                self.grid[y][x].turn_on();
            }
        }
    }

    fn turn_off_rect(self: &mut DroneGrid, x1: usize, y1: usize, x2: usize, y2: usize) {
        for y in y1..=y2 {
            for x in x1..=x2 {
                self.grid[y][x].turn_off();
            }
        }
    }

    fn toggle_rect(self: &mut DroneGrid, x1: usize, y1: usize, x2: usize, y2: usize) {
        for y in y1..=y2 {
            for x in x1..=x2 {
                self.grid[y][x].toggle();
            }
        }
    }

    fn count_lit(self: &mut DroneGrid) -> u64 {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x].led_on {
                    count += 1;
                }
            }
        }
        count
    }

    fn brightness(self: &mut DroneGrid) -> u64 {
        let mut total_brightness = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                total_brightness += self.grid[y][x].brightness;
            }
        }
        total_brightness
    }
}

fn parse_drone_command(string: &str) -> Result<DroneCommand, ()> {
    let re = Regex::new(r"^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap();
    for (_, [command, x1, y1, x2, y2]) in re.captures_iter(string).map(|c| c.extract()) {
        return Ok(DroneCommand {
            command: command.to_string(),
            x1: x1.parse::<usize>().unwrap(),
            y1: y1.parse::<usize>().unwrap(),
            x2: x2.parse::<usize>().unwrap(),
            y2: y2.parse::<usize>().unwrap(),
        });
    }

    Err(())
}
