use rand::prelude::*;

pub mod ast;
pub mod parse;
pub mod roll;

use roll::expr;


fn roll(dice: &[(i32, i32)]) {
    let mut rng = StdRng::seed_from_u64(expr::TEST_SEED);
    for &(num, sides) in dice.iter() {
        let tot: Vec<i32> = (&mut rng)
            .sample_iter(rand::distributions::Uniform::new_inclusive(1, sides))
            .take(num as usize)
            .collect();
        print!("{}d{}: {:?} = {}\t", num, sides, tot, tot.iter().sum::<i32>());
    }
    println!()
}

fn main() {
    let to_roll: Vec<&[(i32, i32)]> = vec![
        &[(1, 20)],
        &[(6, 6)],
        &[(4, 12)],
        &[(2, 10), (3, 4)],
        &[(1, 10)],
        &[(1, 6)],
        &[(1, 12)],
        &[(2, 4)],
        &[(1, 20), (1, 4)],
        &[(3, 20)],
    ];

    for &dice in to_roll.iter() {
        roll(dice);
    }
}