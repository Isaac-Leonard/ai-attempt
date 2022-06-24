use std::collections::HashMap;
#[derive(Debug)]
pub struct Cell {
    pub last_fired: u32,
    pub connections: Vec<usize>,
    pub charge: i32,
    pub target: i32,
    pub adds: bool,
    pub reset_time: u16,
}

#[derive(Debug)]
pub struct Brain {
    pub cells: Vec<Cell>,
    pub connections: HashMap<(usize, usize), i16>,
}
impl Brain {
    pub fn new() -> Self {
        let mut brain = Brain {
            cells: Vec::new(),
            connections: HashMap::new(),
        };
        for i in 0..1000000 {
            let adds = i % 1000 > 500;
            brain.cells.push(Cell {
                charge: 0,
                adds,
                connections: Vec::new(),
                target: 10,
                last_fired: 0,
                reset_time: 4,
            })
        }
        for i in 0..100 {
            for j in 0..10000 {
                brain.cells[i * 10000 + j].connections.append(&mut vec![
                    i * 10000 + 10000 - j,
                    (i * 10000 + 10000 + j * 2) % 1000000,
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
            if self.cells[i].charge > self.cells[i].target {
                for connection in self.cells[i].connections.clone() {
                    self.cells[connection].charge += 1;
                }
                self.cells[i].last_fired = 0;
                self.cells[i].charge = 0;
            } else {
                self.cells[i].last_fired += 1;
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
