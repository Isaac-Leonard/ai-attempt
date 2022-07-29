mod brain;
use brain::*;
use rand::{Rng, SeedableRng};
fn main() {
    let mut rng = rand_chacha::ChaChaRng::seed_from_u64(0);
    let mut brain = Brain::new(rng.clone());
    let mut inputs = Inputs { hunger: 0, food: 0 };
    let mut last_eat_time = 0;
    let mut set = false;
    for i in 0..1000 {
        brain.input(inputs.clone());
        if brain.cells[131].last_fired == 0 && set {
            inputs.food += 1;
        }
        if brain.cells[132].last_fired == 0 && set {
            inputs.food -= 1;
        }
        if inputs.food == 0 && set {
            inputs.hunger = 0;
            last_eat_time = i;
            set = false;
        }
        if i - last_eat_time > 5 && !set {
            inputs.hunger = 1;
            inputs.food = rng.gen_range(-25..5);
            set = true;
        }
        println!("{}, {:?}", set, inputs);
    }
}
