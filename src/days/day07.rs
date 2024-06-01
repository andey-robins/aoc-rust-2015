use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day07.txt").expect("couldn't read input file");

    let circuit = driver(&input.to_string());

    (
        Solution::from(circuit.get_final_signal_value()),
        Solution::from(0),
    )
}

#[test]
fn test() {
    let input =
        read_to_string("input/test/day07.txt").expect("Expected to find test input file for day 7");

    let circuit = driver(&input);

    assert_eq!(
        circuit.get_signal("d").expect("expected d to have value"),
        72
    );
    assert_eq!(
        circuit.get_signal("e").expect("expected e to have value"),
        507
    );
    assert_eq!(
        circuit.get_signal("f").expect("expected f to have value"),
        492
    );
    assert_eq!(
        circuit.get_signal("g").expect("expected g to have value"),
        114
    );
    assert_eq!(
        circuit.get_signal("h").expect("expected h to have value"),
        65412
    );
    assert_eq!(
        circuit.get_signal("i").expect("expected i to have value"),
        65079
    );
    assert_eq!(
        circuit.get_signal("x").expect("expected x to have value"),
        123
    );
    assert_eq!(
        circuit.get_signal("y").expect("expected y to have value"),
        456
    );
}

fn driver(input: &String) -> Circuit {
    let mut circuit = Circuit::new();

    for line in input.lines() {
        let command = parse_command(&line.to_string());
        println!("{:?}", line);
        command.execute(&mut circuit);
    }

    circuit
}

fn parse_command(line: &String) -> Box<dyn Command> {
    let mut parts: std::str::Split<'_, &str> = line.split(" -> ");
    let command: &str = parts.next().expect("Expected a command");
    let wire_name: &str = parts.next().expect("Expected a wire name");

    let command_parts: Vec<&str> = command.split(" ").collect::<Vec<&str>>();

    if command_parts.len() == 1 {
        return Box::new(ValueCommand {
            value: command_parts[0]
                .to_string()
                .parse()
                .expect("Expected a number"),
            wire_name: wire_name.to_string(),
        });
    }

    if command_parts.len() == 2 {
        return Box::new(NotCommand {
            target_wire_id: command_parts[1].to_string(),
            dest_wire_id: wire_name.to_string(),
        });
    }

    match command_parts[1] {
        "AND" => Box::new(AndCommand {
            left_target_id: command_parts[0].to_string(),
            right_target_id: command_parts[2].to_string(),
            dest_wire_id: wire_name.to_string(),
        }),
        "OR" => Box::new(OrCommand {
            left_target_id: command_parts[0].to_string(),
            right_target_id: command_parts[2].to_string(),
            dest_wire_id: wire_name.to_string(),
        }),
        "LSHIFT" => Box::new(LShiftCommand {
            target_wire_id: command_parts[0].to_string(),
            shift_amount: command_parts[2]
                .to_string()
                .parse()
                .expect("Expected a number"),
            dest_wire_id: wire_name.to_string(),
        }),
        "RSHIFT" => Box::new(RShiftCommand {
            target_wire_id: command_parts[0].to_string(),
            shift_amount: command_parts[2]
                .to_string()
                .parse()
                .expect("Expected a number"),
            dest_wire_id: wire_name.to_string(),
        }),
        _ => panic!("Unknown command"),
    }
}

struct Wire {
    identifier: String,
    signal: u16,
}

struct Circuit {
    wires: Vec<Wire>,
}

impl Circuit {
    fn new() -> Self {
        Self { wires: Vec::new() }
    }

    fn get_signal(&self, wire_name: &str) -> Option<u16> {
        for wire in self.wires.iter() {
            if wire.identifier == wire_name {
                return Some(wire.signal);
            }
        }

        None
    }

    // the "a" labeled wire is defined by the problem to be the final value
    // that our code is searching for, this function is a wrapper around
    // `get_signal` to make that process more clear
    fn get_final_signal_value(&self) -> u16 {
        self.get_signal("a")
            .expect("expected final value to have 'a' wire")
    }
}

trait Command {
    fn execute(&self, circuit: &mut Circuit);
}

#[derive(Debug)]
struct ValueCommand {
    value: u16,
    wire_name: String,
}

impl Command for ValueCommand {
    fn execute(&self, circuit: &mut Circuit) {
        for wire in circuit.wires.iter_mut() {
            if wire.identifier == self.wire_name {
                wire.signal = self.value;
                return;
            }
        }

        circuit.wires.push(Wire {
            identifier: self.wire_name.clone(),
            signal: self.value,
        });
    }
}

#[derive(Debug)]
struct NotCommand {
    target_wire_id: String,
    dest_wire_id: String,
}

impl Command for NotCommand {
    fn execute(&self, circuit: &mut Circuit) {
        let signal: u16 = circuit
            .get_signal(&self.target_wire_id)
            .expect("Expected a valid signal");

        for wire in circuit.wires.iter_mut() {
            if wire.identifier == self.dest_wire_id {
                wire.signal = !signal;
                return;
            }
        }

        circuit.wires.push(Wire {
            identifier: self.dest_wire_id.clone(),
            signal: !signal,
        });
    }
}

#[derive(Debug)]
struct AndCommand {
    left_target_id: String,
    right_target_id: String,
    dest_wire_id: String,
}

impl Command for AndCommand {
    fn execute(&self, circuit: &mut Circuit) {
        let left_signal: u16 = circuit
            .get_signal(&self.left_target_id)
            .expect("Expected a valid signal");
        let right_signal: u16 = circuit
            .get_signal(&self.right_target_id)
            .expect("Expected a valid signal");

        for wire in circuit.wires.iter_mut() {
            if wire.identifier == self.dest_wire_id {
                wire.signal = left_signal & right_signal;
                return;
            }
        }

        circuit.wires.push(Wire {
            identifier: self.dest_wire_id.clone(),
            signal: left_signal & right_signal,
        });
    }
}

#[derive(Debug)]
struct OrCommand {
    left_target_id: String,
    right_target_id: String,
    dest_wire_id: String,
}

impl Command for OrCommand {
    fn execute(&self, circuit: &mut Circuit) {
        let left_signal: u16 = circuit
            .get_signal(&self.left_target_id)
            .expect("Expected a valid signal");
        let right_signal: u16 = circuit
            .get_signal(&self.right_target_id)
            .expect("Expected a valid signal");

        for wire in circuit.wires.iter_mut() {
            if wire.identifier == self.dest_wire_id {
                wire.signal = left_signal | right_signal;
                return;
            }
        }

        circuit.wires.push(Wire {
            identifier: self.dest_wire_id.clone(),
            signal: left_signal | right_signal,
        });
    }
}

#[derive(Debug)]
struct LShiftCommand {
    target_wire_id: String,
    dest_wire_id: String,
    shift_amount: u16,
}

impl Command for LShiftCommand {
    fn execute(&self, circuit: &mut Circuit) {
        let signal: u16 = circuit
            .get_signal(&self.target_wire_id)
            .expect("Expected a valid signal");

        for wire in circuit.wires.iter_mut() {
            if wire.identifier == self.dest_wire_id {
                wire.signal = signal << self.shift_amount;
                return;
            }
        }

        circuit.wires.push(Wire {
            identifier: self.dest_wire_id.clone(),
            signal: signal << self.shift_amount,
        });
    }
}

#[derive(Debug)]
struct RShiftCommand {
    target_wire_id: String,
    dest_wire_id: String,
    shift_amount: u16,
}

impl Command for RShiftCommand {
    fn execute(&self, circuit: &mut Circuit) {
        let signal: u16 = circuit
            .get_signal(&self.target_wire_id)
            .expect("Expected a valid signal");

        for wire in circuit.wires.iter_mut() {
            if wire.identifier == self.dest_wire_id {
                wire.signal = signal >> self.shift_amount;
                return;
            }
        }

        circuit.wires.push(Wire {
            identifier: self.dest_wire_id.clone(),
            signal: signal >> self.shift_amount,
        });
    }
}
