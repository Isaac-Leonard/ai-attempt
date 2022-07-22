mod brain;
use brain::*;
use rand::{prelude::*, rngs::*, Rng, SeedableRng};
fn main() {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);
    let mut brain = Brain::new(rng);
    for _ in 0..1000 {
        brain.input(Inputs {
            current_line: Vec::new(),
        });
        println!("{}", brain.cells[104000]);
    }
}

fn debug_out_of_range_connections(brain: &Brain) {
    for i in 0..brain.cells.len() {
        for con in &brain.cells[i].connections {
            if *con > 1000000 {
                println!["{}: {}", i, con]
            }
        }
    }
}
