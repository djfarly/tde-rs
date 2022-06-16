pub mod rolls;
use rolls::{calc_chance, skill_check, Attributes, Rolls};
pub mod dice;
use dice::d_20;

use std::time::Instant;

fn main() {
    let attributes: Attributes = [16, 16, 14];
    let skill_points: i8 = 13;
    let modifier: i8 = -5;
    let now = Instant::now();

    let chance = calc_chance(&attributes, &skill_points, &modifier);

    println!("Check took: {} microseconds", now.elapsed().as_micros());

    println!("{:?}", chance);

    let rolls: Rolls = [d_20(), d_20(), d_20()];
    let check = skill_check(&rolls, &attributes, &skill_points, &modifier);
    println!("Rolls: {:?} -> {:?}", rolls, check);
}
