use rand::Rng;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Cell {
    pub last_fired: u32,
    pub charge: i8,
    pub target: i8,
    pub reset_time: u8,
    pub decay: u8,
}
impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "lastfired: {}, charge: {}", self.last_fired, self.charge)
    }
}
impl Cell {
    pub fn new(target: i8, reset_time: u8, decay: u8) -> Self {
        Self {
            target,
            reset_time,
            decay,
            last_fired: 0,
            charge: 0,
        }
    }
    pub fn to_bytes(&self) -> [u8; 3] {
        [self.target as u8, self.reset_time, self.decay]
    }
}
#[derive(Debug)]
pub struct Synapse {
    pub strength: u8,
    pub adds: bool,
    pub to: usize,
}
impl Synapse {
    pub fn new(strength: u8, adds: bool, to: usize) -> Self {
        Self { strength, adds, to }
    }

    pub fn new_basic(to: usize) -> Self {
        Self {
            strength: 1,
            adds: true,
            to,
        }
    }

    pub fn to_bytes(&self) -> [u8; 10] {
        let to = self.to.to_be_bytes();
        [
            self.strength,
            self.adds as u8,
            to[0],
            to[1],
            to[2],
            to[3],
            to[4],
            to[5],
            to[6],
            to[7],
        ]
    }
}

#[derive(Debug)]
pub struct Brain {
    pub cells: Vec<Cell>,
    pub connections: Vec<Vec<Synapse>>,
}
impl Brain {
    pub fn new(mut prng: impl Rng) -> Self {
        let mut brain = Brain {
            cells: Vec::new(),
            connections: Vec::new(),
        };
        // Set up time processing
        for i in 0..128 {
            brain.cells.push(Cell {
                charge: 0,
                target: 1,
                last_fired: 0,
                reset_time: 0,
                decay: 2,
            })
        }

        brain.cells.push(Cell {
            charge: 0,
            target: 3,
            last_fired: 0,
            reset_time: 2,
            decay: 6,
        });

        brain.cells.push(Cell {
            charge: 0,
            target: 3,
            last_fired: 0,
            reset_time: 2,
            decay: 6,
        });
        brain.cells.push(Cell {
            charge: 0,
            target: 3,
            last_fired: 0,
            reset_time: 2,
            decay: 6,
        });
        brain.cells.push(Cell {
            charge: 0,
            target: 3,
            last_fired: 0,
            reset_time: 2,
            decay: 3,
        });
        brain.cells.push(Cell {
            charge: 0,
            target: 2,
            last_fired: 0,
            reset_time: 1,
            decay: 3,
        });
        brain.cells.push(Cell {
            charge: 0,
            target: 1,
            last_fired: 0,
            reset_time: 3,
            decay: 3,
        });
        brain.cells.push(Cell {
            charge: 0,
            target: 1,
            last_fired: 0,
            reset_time: 3,
            decay: 6,
        });
        brain.cells.push(Cell {
            charge: 0,
            target: 1,
            last_fired: 0,
            reset_time: 3,
            decay: 3,
        });
        brain.cells.push(Cell {
            charge: 0,
            target: 1,
            last_fired: 0,
            reset_time: 3,
            decay: 6,
        });

        for _ in 0..brain.cells.len() {
            brain.connections.push(Vec::new())
        }

        brain.connections[128].push(Synapse::new_basic(131));
        brain.connections[128].push(Synapse::new_basic(132));
        brain.connections[129].push(Synapse::new_basic(131));
        brain.connections[130].push(Synapse::new_basic(132));
        brain.connections[129].push(Synapse::new_basic(133));
        brain.connections[130].push(Synapse::new_basic(134));
        brain.connections[133].push(Synapse::new_basic(131));
        brain.connections[134].push(Synapse::new_basic(132));
        brain.connections[133].push(Synapse::new_basic(135));
        brain.connections[134].push(Synapse::new_basic(136));
        brain.connections[135].push(Synapse::new_basic(131));
        brain.connections[136].push(Synapse::new_basic(132));

        brain
    }

    pub fn input(&mut self, inputs: Inputs) {
        if inputs.hunger > 0 {
            self.cells[128].charge += 1;
        }

        if inputs.food_coord < 0 {
            self.cells[129].charge += 1;
        }

        if inputs.food_coord > 0 {
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
                && self.cells[i].last_fired as u8 > self.cells[i].reset_time
            {
                for connection in &self.connections[i] {
                    self.cells[connection.to].charge += 1;
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

    pub fn random_mutate(&mut self, mut rng: impl Rng) {
        let type_mutation = rng.gen_range(0..2);
        if type_mutation == 0 {
            let cell = Cell::new(
                rng.gen_range(-10..10),
                rng.gen_range(0..10),
                rng.gen_range(0..10),
            );
            self.cells.push(cell);
            self.connections.push(vec![Synapse::new_basic(
                rng.gen_range(0..self.cells.len() - 1),
            )]);
        } else {
            self.connections[rng.gen_range(0..self.cells.len())]
                .push(Synapse::new_basic(rng.gen_range(0..self.cells.len())));
        }
    }
    pub fn serialise(&self) -> Vec<u8> {
        let mut output = Vec::new();
        output.append(&mut self.cells.len().to_be_bytes().to_vec());
        for cell in &self.cells {
            output.append(&mut cell.to_bytes().to_vec())
        }
        for connection in &self.connections {
            output.append(&mut connection.len().to_be_bytes().to_vec());
            for connection in connection {
                output.append(&mut connection.to_bytes().to_vec())
            }
        }
        output
    }

    pub fn deserialise(data: &[u8]) -> Self {
        let mut brain = Brain {
            cells: Vec::new(),
            connections: Vec::new(),
        };
        let cell_count_bytes = &data[0..8];
        let mut count = 0;
        for i in 0..8 {
            count += (cell_count_bytes[i] as usize) << (i * 8);
        }
        let last_cell = count + 8;
        for i in (8..last_cell).step_by(3) {
            brain
                .cells
                .push(Cell::new(data[i] as i8, data[i + 1], data[i + 2]))
        }
        brain
    }
}

#[derive(Debug, Clone)]
pub struct Inputs {
    pub hunger: u8,
    pub food_coord: i8,
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
