use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug)]
pub struct Cell {
    pub last_fired: u32,
    pub connections: Vec<usize>,
    pub charge: i32,
    pub target: i32,
    pub adds: bool,
    pub reset_time: u32,
    decay: u8,
}
impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "lastfired: {}, charge: {}", self.last_fired, self.charge)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Brain {
    pub cells: Vec<Cell>,
    pub connections: HashMap<(usize, usize), i16>,
}
impl Brain {
    pub fn new(mut prng: impl Rng) -> Self {
        let mut brain = Brain {
            cells: Vec::new(),
            connections: HashMap::new(),
        };
        // Set up time processing
        for i in 0..128 {
            brain.cells.push(Cell {
                charge: 0,
                adds: true,
                connections: vec![],
                target: 1,
                last_fired: 0,
                reset_time: 0,
                decay: 2,
            })
        }

        brain.cells.push(Cell {
            charge: 0,
            adds: true,
            connections: vec![],
            target: 3,
            last_fired: 0,
            reset_time: 2,
            decay: 6,
        });

        brain.cells.push(Cell {
            charge: 0,
            adds: true,
            connections: vec![],
            target: 3,
            last_fired: 0,
            reset_time: 2,
            decay: 6,
        });
        brain.cells.push(Cell {
            charge: 0,
            adds: true,
            connections: vec![],
            target: 3,
            last_fired: 0,
            reset_time: 2,
            decay: 6,
        });
        brain.cells.push(Cell {
            charge: 0,
            adds: true,
            connections: vec![],
            target: 3,
            last_fired: 0,
            reset_time: 2,
            decay: 3,
        });
        brain.cells.push(Cell {
            charge: 0,
            adds: true,
            connections: vec![],
            target: 2,
            last_fired: 0,
            reset_time: 1,
            decay: 3,
        });
        brain.cells.push(Cell {
            charge: 0,
            adds: true,
            connections: vec![],
            target: 1,
            last_fired: 0,
            reset_time: 3,
            decay: 3,
        });
        brain.cells.push(Cell {
            charge: 0,
            adds: true,
            connections: vec![],
            target: 1,
            last_fired: 0,
            reset_time: 3,
            decay: 6,
        });
        brain.cells.push(Cell {
            charge: 0,
            adds: true,
            connections: vec![],
            target: 1,
            last_fired: 0,
            reset_time: 3,
            decay: 3,
        });
        brain.cells.push(Cell {
            charge: 0,
            adds: true,
            connections: vec![],
            target: 1,
            last_fired: 0,
            reset_time: 3,
            decay: 6,
        });
        brain.cells[128].connections.push(131);
        brain.cells[128].connections.push(132);
        brain.cells[129].connections.push(131);
        brain.cells[130].connections.push(132);
        brain.cells[129].connections.push(133);
        brain.cells[130].connections.push(134);
        brain.cells[133].connections.push(131);
        brain.cells[134].connections.push(132);
        brain.cells[133].connections.push(135);
        brain.cells[134].connections.push(136);
        brain.cells[135].connections.push(131);
        brain.cells[136].connections.push(132);

        brain
    }

    pub fn input(&mut self, inputs: Inputs) {
        if inputs.hunger > 0 {
            self.cells[128].charge += 1;
        }

        if inputs.food < 0 {
            self.cells[129].charge += 1;
        }

        if inputs.food > 0 {
            self.cells[130].charge += 1;
        }

        for i in 0..self.cells.len() {
            self.cells[i].last_fired += 1;
            if self.cells[i].last_fired % self.cells[i].decay as u32 == 0 {
                if self.cells[i].charge > 0 {
                    self.cells[i].charge -= 1;
                }
                if self.cells[i].charge < 0 {
                    self.cells[i].charge += 1
                }
            }
            if self.cells[i].charge >= self.cells[i].target
                && self.cells[i].last_fired > self.cells[i].reset_time
            {
                for connection in self.cells[i].connections.clone() {
                    self.cells[connection].charge += 1;
                }
                self.cells[i].last_fired = 0;
                self.cells[i].charge = 0;
            }
        }
    }
    fn fire_time(&mut self) {
        let time = get_time();
        for (i, byte) in time.iter().enumerate() {
            let bits = u8_to_bools(*byte);
            for (j, on) in bits.iter().enumerate() {
                let loc = 100000 + i * 8 + j;
                if *on {
                    self.cells[loc].charge += 1
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Inputs {
    pub hunger: u8,
    pub food: i8,
}

fn u8_to_bools(mut num: u8) -> [bool; 8] {
    let mut arr: [bool; 8] = [false, false, false, false, false, false, false, false];
    for i in 0..8 {
        num >>= i;
        arr[i] = num & 1 > 0;
    }
    arr
}
fn get_time() -> [u8; 16] {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_le_bytes()
}
