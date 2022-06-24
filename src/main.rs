mod brain;
use brain::*;
fn main() {
    let mut brain = Brain::new();
    for i in 0..100 {
        brain.input(Inputs {
            current_line: Vec::new(),
        });
        println!("{:?}", brain.cells[100]);
    }
}
