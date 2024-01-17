// /main.rs
// Author: 1Kill2Steal
// 14.01.2023 (DD/MM/YYYY)

// ./lib.rs
mod enums;
mod functions;
// end of ./lib.rs


// ./functions/functions.rs
use functions::functions::{generate_game_outcomes, main_game_loop};

fn main() {

    let mut rng = rand::thread_rng();
    let outcomes = generate_game_outcomes();

    // value decreases on loss and increases on win.
    let mut score: i32 = 0;

    main_game_loop(&mut rng, &outcomes, &mut score);

}
