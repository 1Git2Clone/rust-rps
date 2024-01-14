// functions.rs
// Author: 1Kill2Steal
// 14.01.2023 (DD/MM/YYYY)

use std::io::{self, Write};
use std::collections::HashMap;
use rand::Rng;

// ../enums/enums.rs import
use crate::enums::enums::GameOptions;

// Match the user input to the hash table's game state char.
pub fn handle_game(
    user_choice: GameOptions,
    computer_choice: GameOptions,
    score: &mut i32,
    outcomes: &HashMap<(GameOptions, GameOptions), char>,
) -> bool {

    match outcomes.get(&(user_choice, computer_choice)) {
        Some(result) => {
            match result {
                &'d' => println!("Draw."),
                &'l' => {
                    println!("Loss.");
                    *score -= 1;
                }
                &'w' => {
                    println!("Win.");
                    *score += 1;
                }
                _ => unreachable!(),
            }
        }
        None => unreachable!(),
    }

    println!("Current score: {}", *score);

    *score >= 0
}

// Tuple of two enums (../enums/enums.rs) for the player vs computer variants
pub fn generate_game_outcomes() -> HashMap<(GameOptions, GameOptions), char> {

    let mut outcomes = HashMap::new();

    // d = draw, w = win, l = loss
    outcomes.insert((GameOptions::Rock, GameOptions::Rock), 'd');
    outcomes.insert((GameOptions::Rock, GameOptions::Paper), 'l');
    outcomes.insert((GameOptions::Rock, GameOptions::Scissors), 'w');

    outcomes.insert((GameOptions::Paper, GameOptions::Rock), 'w');
    outcomes.insert((GameOptions::Paper, GameOptions::Paper), 'd');
    outcomes.insert((GameOptions::Paper, GameOptions::Scissors), 'l');

    outcomes.insert((GameOptions::Scissors, GameOptions::Rock), 'l');
    outcomes.insert((GameOptions::Scissors, GameOptions::Paper), 'w');
    outcomes.insert((GameOptions::Scissors, GameOptions::Scissors), 'd');

    outcomes

}

pub fn main_game_loop(
    rng: &mut rand::rngs::ThreadRng,
    outcomes: &HashMap<(GameOptions, GameOptions), char>,
    score: &mut i32
) {
    loop {
        let mut input = String::new();
        print!("Enter 1 (rock), 2 (paper) or 3 (scissors) exit = 0: ");
        io::stdout().flush().unwrap(); // directly print the upper statement.

        io::stdin().read_line(&mut input)
            .expect("Failed to read line.");

        let Ok(number) = input.trim().parse::<u8>() else {
            eprintln!("Error: invalid input");
            continue;
        };

        let user_choice = GameOptions::match_value_to_enum(number);

        match user_choice {
            GameOptions::Exit => {
                break;
            }
            GameOptions::Rock | GameOptions::Paper | GameOptions::Scissors => {
                let computer_choice: GameOptions = GameOptions::match_value_to_enum(rng.gen_range(1..=3));

                // println!("Entered valid number. ({})", number);
                // println!("Computer choice: {}", computer_choice);

                if handle_game(user_choice, computer_choice, score, &outcomes) {
                    continue;
                } else {
                    println!("Skill issue, you got less than 0 points.");
                    break;
                }
            }
        }
    }
}
