use rand::Rng;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
#[derive(Debug)]
pub struct Cell {
    pub last_fired: u32,
    pub connections: Vec<usize>,
    pub charge: i32,
    pub target: i32,
    pub adds: bool,
    pub reset_time: u32,
}
impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "lastfired: {}, charge: {}", self.last_fired, self.charge)
    }
}

#[derive(Debug)]
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
            let adds = i % 1000 > 50;
            brain.cells.push(Cell {
                charge: 0,
                adds,
                connections: vec![
                    prng.gen_range(128..1000000),
                    prng.gen_range(128..1000000),
                    prng.gen_range(128..1000000),
                ],
                target: 1,
                last_fired: 0,
                reset_time: 0,
            })
        }

        for i in 128..1000000 {
            let adds = i % 1000 > 50;
            brain.cells.push(Cell {
                charge: 0,
                adds,
                connections: Vec::new(),
                target: 1,
                last_fired: 0,
                reset_time: 2,
            })
        }
        for i in 2..10000 {
            for j in 0..100 {
                brain.cells[i * 100 + j]
                    .connections
                    .push(prng.gen_range(0..100) + i * 100);
                brain.cells[i * 100 + j]
                    .connections
                    .push(prng.gen_range(0..100) + i * 100);
                brain.cells[i * 100 + j]
                    .connections
                    .push(prng.gen_range(0..100) + i * 100);
            }
        }
        for i in 1..100 {
            for j in 0..10000 {
                brain.cells[i * 10000 + j].connections.append(&mut vec![
                    prng.gen_range(0..10000) + i * 10000,
                    prng.gen_range(0..10000) + i * 10000,
                    prng.gen_range(0..10000) + i * 10000,
                    prng.gen_range(0..10000) + i * 10000,
                    prng.gen_range(0..10000) + i * 10000,
                    prng.gen_range(0..10000) + i * 10000,
                    prng.gen_range(0..10000) + i * 10000,
                    prng.gen_range(0..10000) + i * 10000,
                    prng.gen_range(128..1000000),
                    prng.gen_range(128..1000000),
                    (prng.gen_range(0..100000) + i * 10000) % 1000000,
                ]);
            }
        }
        brain
    }

    pub fn input(&mut self, inputs: Inputs) {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_le_bytes();
        for (i, byte) in time.iter().enumerate() {
            let bits = u8_to_bools(*byte);
            for (j, on) in bits.iter().enumerate() {
                let loc = 100000 + i * 8 + j;
                if *on {
                    self.cells[loc].charge += 1
                }
            }
        }
        for i in 0..1000 {}
        for i in 0..self.cells.len() {
            self.cells[i].last_fired += 1;
            if self.cells[i].charge >= self.cells[i].target
                && self.cells[i].last_fired < self.cells[i].reset_time
            {
                for connection in self.cells[i].connections.clone() {
                    self.cells[connection].charge += 1;
                }
                self.cells[i].last_fired = 0;
                self.cells[i].charge = 0;
            }
        }
    }
}

pub struct Inputs {
    pub current_line: Vec<u8>,
}

fn u8_to_bools(mut num: u8) -> [bool; 8] {
    let mut arr: [bool; 8] = [false, false, false, false, false, false, false, false];
    for i in 0..8 {
        num = num >> i;
        arr[i] = num & 1 > 0;
    }
    arr
}
